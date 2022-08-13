use std::f64::consts::PI;

use clap::*;
use contour::ContourBuilder;
use geojson::Feature;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
    #[clap(short, long, default_value = "image.svg")]
    file: String,
    #[clap(short, long, default_value = "297.0")]
    pub width: f64,
    #[clap(short, long, default_value = "210.0")]
    pub height: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed1: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed2: f64,
    #[clap(short, long, default_value = "0.0")]
    pub seed3: f64,
}

// FIXME in all our logic, the value is reversed. it should be the distance

struct ValueMap {
    precision: f64,
    width: f64,
    height: f64,
    w: u32,
    h: u32,
    values: Vec<f64>,
    orientations: Vec<f64>,
    highest_pos: (f64, f64),
}
impl ValueMap {
    pub fn new(
        precision: f64,
        width: f64,
        height: f64,
        f: impl Fn((f64, f64)) -> f64,
    ) -> Self {
        let w = (width / precision) as u32;
        let h = (height / precision) as u32;
        let mut highest_pos = (0.0, 0.0);
        let mut highest_value = -99.;
        let mut orientations = Vec::new();
        let mut values = Vec::new();
        for y in 0..h {
            for x in 0..w {
                let p = (
                    x as f64 / (w as f64),
                    y as f64 / (h as f64),
                );
                let g = (p.0 * width, p.1 * height);
                let v = f(g);
                if v > highest_value {
                    highest_value = v;
                    highest_pos = g;
                }
                values.push(v);
                let e = 0.1; // epsilon small value
                let v = (
                    f((g.0 + e, g.1)) - f((g.0 - e, g.1)),
                    f((g.0, g.1 + e)) - f((g.0, g.1 - e)),
                );
                let l = (v.0 * v.0 + v.1 * v.1).sqrt();
                let a = v.1.atan2(v.0);
                orientations.push(a);
            }
        }
        ValueMap {
            precision,
            width,
            height,
            w,
            h,
            values,
            orientations,
            highest_pos,
        }
    }
    pub fn new_empty(
        precision: f64,
        width: f64,
        height: f64,
    ) -> Self {
        Self::new(precision, width, height, |_p| -99999.0)
    }
    fn union(self: &Self, other: &Self) -> Self {
        // IDEA we could do smooth union?
        let precision = self.precision;
        let width = self.width;
        let height = self.height;
        let w = self.w;
        let h = self.h;
        let values = self
            .values
            .iter()
            .enumerate()
            .map(|(i, v)| v.max(other.values[i]))
            .collect();
        // NB: we don't bother doing a union on intersections yet..
        let orientations = self.orientations.clone();
        let highest_pos = self.highest_pos;
        ValueMap {
            precision,
            width,
            height,
            w,
            h,
            values,
            orientations,
            highest_pos,
        }
    }
    fn index(self: &Self, (x, y): (f64, f64)) -> usize {
        let w = self.w as usize;
        let h = self.h as usize;
        let xi = ((x / self.precision).round() as usize)
            .max(0)
            .min(w - 1);
        let yi = ((y / self.precision).round() as usize)
            .max(0)
            .min(h - 1);
        yi * (w) + xi
    }
    pub fn get_highest_pos(self: &Self) -> (f64, f64) {
        self.highest_pos
    }
    pub fn get_angle(self: &Self, p: (f64, f64)) -> f64 {
        self.orientations[self.index(p)]
    }
    pub fn get_value(self: &Self, p: (f64, f64)) -> f64 {
        self.values[self.index(p)]
    }
    pub fn build_contour(
        self: &Self,
        thresholds: Vec<f64>,
    ) -> Vec<Feature> {
        ContourBuilder::new(self.w, self.h, true)
            .contours(&self.values, &thresholds)
            .unwrap_or(Vec::new())
    }
}

// make a cloud struct within the (width,height) dimension
struct CloudInput {
    width: f64,
    height: f64,
    seed: f64,
    precision: f64,
    lightness: f64,
    light_dir: (f64, f64),
}
struct CloudResult {
    routes: Vec<Vec<(f64, f64)>>,
    values: ValueMap,
}

fn cloud<R: Rng>(
    mut rng: R,
    opts: CloudInput,
) -> CloudResult {
    let precision = opts.precision;
    let width = opts.width;
    let height = opts.height;
    let light_dir = opts.light_dir;
    let lightness = opts.lightness;
    let perlin = Perlin::new();
    let mut rng = rng_from_seed(opts.seed);

    let spread = width / 2.0;

    let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

    let iterations = 2
        + (rng.gen_range(0.8, 1.2) * width / 40.0) as usize;

    let noiseamp1 = rng.gen_range(0.5, 1.5);
    let noisef1 = rng.gen_range(0.01, 0.02);

    let noiseamp2 = rng.gen_range(0.2, 0.3);
    let noisef2 = rng.gen_range(0.05, 0.07);

    let noisef3 = rng.gen_range(0.2, 0.3);
    let noiseamp3 = 0.05;

    let cloud_lines_samples =
        (0.2 * width * height) as usize;
    let cloud_lines_incr = 1.2;
    let cloud_lines_maxlen = 6;

    let mut global_map =
        ValueMap::new_empty(precision, width, height);

    for j in 0..iterations {
        let jp = j as f64 / (iterations as f64 - 1.0);
        let offsetx = (if j % 2 == 0 { -1.0 } else { 1.0 })
            * 0.5
            * spread
            * (1.0 - jp);

        let offsety = 0.25 * spread * (jp - 0.5);

        let seed = opts.seed + j as f64 * 7.7;

        let f = |p: (f64, f64)| {
            let x = p.0 + offsetx;
            let y = p.1 + offsety;
            let mut n = noiseamp1
                * perlin.get([
                    noisef1 * x,
                    noisef1 * y,
                    seed + 500.5,
                ])
                + noiseamp2
                    * perlin.get([
                        noisef2 * x,
                        noisef2 * y,
                        999. + seed * 3.7,
                    ])
                + noiseamp3
                    * perlin.get([
                        noisef3 * x,
                        noisef3 * y,
                        99. + seed * 7.7,
                    ]);
            let dx: f64 = x - width / 2.0;
            let dy: f64 = y - height / 2.0;
            let distc = (dx * dx + dy * dy).sqrt();

            // TODO externalize into radius + noisy params
            // TODO could split out 2 circles instead of one for the actual "spread"
            n += 1.3 + 1.3 * jp - distc / 20.0;
            n
            /*
            if n > 0.2 {
                1.0
            } else {
                0.0
            }*/
        };

        let local_map =
            ValueMap::new(precision, width, height, f);
        global_map = global_map.union(&local_map);

        let mut copy: Vec<Vec<(f64, f64)>> = Vec::new();

        for r in routes {
            let l = r.len();
            if l > 2 {
                // ?TODO we could "glitch" a bit the collision to let the lines enter a bit!
                // copy.push(r);
                let mut route = Vec::new();
                for p in r.clone() {
                    if f(p) > 0.0 {
                        if route.len() > 1 {
                            copy.push(route);
                        }
                        route = Vec::new();
                    } else {
                        route.push(p);
                    }
                }
                if route.len() > 1 {
                    copy.push(route);
                    route = Vec::new();
                }
            } else if l == 2 {
                if f(r[0]) > 0.0 && f(r[1]) > 0.0 {
                    // remove stroke
                } else {
                    copy.push(r);
                }
            }
        }
        routes = copy;
        let pad = 2.0 * precision;
        let bounds = (pad, pad, width - pad, height - pad);
        // TODO we could cut some part of the threshold to create more variety. for now we do a double line
        let thresholds = vec![0.0, 0.02, 0.05];

        let res = local_map.build_contour(thresholds);
        let mut routes_contour =
            features_to_routes(res, precision);
        routes_contour =
            crop_routes(&routes_contour, bounds);

        for route in routes_contour {
            routes.push(route);
        }

        let f2 = f;

        //if j == 0 {
        let center = local_map.get_highest_pos();
        for i in 0..cloud_lines_samples {
            let mut x = rng.gen_range(pad, width - pad);
            let mut y = rng.gen_range(pad, height - pad);
            let mut j = 0;

            let mut route = Vec::new();
            route.push((x, y));

            loop {
                if j > cloud_lines_maxlen {
                    break;
                }

                let mut n = f((x, y));
                if n < 0.0 {
                    break;
                }
                // points on edge
                n = (1.0 - lightness * n).max(0.0);
                // TODO orientation du soleil au lieu d'avoir un absolu. il faut déterminer le centre du nuage.
                // more points on bottom
                n += light_dir.0 * (center.0 - x) / width;
                n += light_dir.1 * (center.1 - y) / height;
                if n < rng.gen_range(0.0, 1.0)
                    - j as f64 * 0.3
                {
                    break;
                }

                // TODO line orientation to follow the noise
                // calc normal vector
                // longer curvy line instead of one stroke?
                let a =
                    local_map.get_angle((x, y)) + PI / 2.0;
                /*

                PI / 2.0
                    + (y + offsety - height / 2.)
                        .atan2(
                            x + offsetx
                                - width / 2.,
                        );
                        */
                let amp = cloud_lines_incr;
                let dx = amp * a.cos();
                let dy = amp * a.sin();
                x += dx;
                y += dy;

                route.push((x, y));
                j += 1;
            }
            if route.len() > 1 {
                routes.push(route);
            }
        }
        //}
    }

    CloudResult {
        routes,
        values: global_map,
    }
}

fn art(opts: &Opts) -> Vec<Group> {
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();
            let mut rng = rng_from_seed(opts.seed);

            let seed = opts.seed;
            let width = opts.width;
            let height = opts.height;
            let pad = 10.0;
            let lightness = 0.5;
            let light_dir = (
                rng.gen_range(-4.0, 4.0),
                rng.gen_range(-4.0, 4.0),
            );
            let precision = 0.5;

            let cloud_res = cloud(
                &mut rng,
                CloudInput {
                    width,
                    height,
                    seed,
                    precision,
                    lightness,
                    light_dir,
                },
            );

            let mut routes = cloud_res.routes.clone();

            let insidepad = 20.0;
            let offset = 3.0;
            let mut y = insidepad;
            loop {
                if y > height - insidepad {
                    break;
                }

                let mut x = insidepad;
                let mut route = Vec::new();

                loop {
                    if x > width - insidepad {
                        break;
                    }
                    if cloud_res.values.get_value((x, y))
                        > 0.0
                    {
                        // inside cloud
                        if route.len() > 0 {
                            route.push((x, y));
                            routes.push(route);
                        }
                        route = Vec::new();
                    } else {
                        if route.len() == 0 {
                            route.push((x, y));
                        }
                    }

                    x += 0.5;
                }

                if route.len() > 0 {
                    route.push((x, y));
                    routes.push(route);
                }

                y += offset;
            }

            for route in routes {
                data = render_route(data, route);
            }

            let mut l = layer(color);
            l = l.add(base_path(color, 0.35, data));
            l
        })
        .collect()
}

/*
#[derive(Clone)]
pub struct OrientationMap {
    granularity: f64,
    width: f64,
    height: f64,
    orientations: Vec<f64>,
}
impl OrientationMap {
    pub fn new(
        granularity: f64,
        width: f64,
        height: f64,
        f: impl Fn((f64, f64)) -> f64,
    ) -> Self {
        let wi = (width / granularity).ceil() as usize;
        let hi = (height / granularity).ceil() as usize;
        let mut orientations = Vec::new();

        for y in 0..hi {
            for x in 0..wi {
                let p = (
                    width * x as f64 / (wi as f64),
                    height * y as f64 / (hi as f64),
                );
                let e = 0.1; // epsilon small value
                let v = (
                    f((p.0 + e, p.1)) - f((p.0 - e, p.1)),
                    f((p.0, p.1 + e)) - f((p.0, p.1 - e)),
                );
                let l = (v.0 * v.0 + v.1 * v.1).sqrt();
                let a = v.1.atan2(v.0);
                orientations.push(a);
            }
        }

        OrientationMap {
            granularity,
            width,
            height,
            orientations,
        }
    }

    fn index(self: &Self, (x, y): (f64, f64)) -> usize {
        let wi =
            (self.width / self.granularity).ceil() as usize;
        let hi = (self.height / self.granularity).ceil()
            as usize;
        let xi = ((x / self.granularity).round() as usize)
            .max(0)
            .min(wi - 1);
        let yi = ((y / self.granularity).round() as usize)
            .max(0)
            .min(hi - 1);
        yi * wi + xi
    }

    pub fn get_angle(self: &Self, p: (f64, f64)) -> f64 {
        self.orientations[self.index(p)]
    }
}
*/

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(&opts);
    let mut document =
        base_document("white", opts.width, opts.height);
    for g in groups {
        document = document.add(g);
    }
    svg::save(opts.file, &document).unwrap();
}
