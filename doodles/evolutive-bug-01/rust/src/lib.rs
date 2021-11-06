mod utils;
use noise::*;
use std::f64::consts::PI;
use byteorder::*;
use contour::ContourBuilder;
use geojson::Feature;
use svg::node::element::path::Data;
use svg::node::element::*;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Opts {
    pub seed: f64,
    pub v1: f64,
    pub v2: f64,
    pub v3: f64,
}

pub fn art(opts: &Opts) -> Vec<Group> {
    let samples = 60;

    let width = 200.0;
    let height = 200.0;
    let stroke_width = 0.32;
    let precision = 1.;
    let w = (width as f64 / precision) as u32;
    let h = (height as f64 / precision) as u32;
    let pad = 0.0;
    let perlin = Perlin::new();
    
    fn length(l: (f64, f64)) -> f64 {
        (l.0 * l.0 + l.1 * l.1).sqrt()
    }
    fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
        (
            a.cos() * p.0 + a.sin() * p.1,
            a.cos() * p.1 - a.sin() * p.0,
        )
    }
    fn op_union_round(a: f64, b: f64, r: f64) -> f64 {
        r.max(a.min(b)) - length(((r - a).max(0.), (r - b).max(0.)))
    }
    let  sdf_box2 = |(x,y):(f64,f64), (w, h):(f64,f64)| {
        let dx = x.abs() - w;
        let dy = y.abs() - h;
        length((dx.max(0.), dy.max(0.))) + dx.min(0.).max(dy.min(0.))
    };
    
    let colors = vec!["#000", "#000", "#000"];
    let mut layers = Vec::new();
    for (ci, &color) in colors.iter().enumerate() {
        let f = |(x, y): (f64, f64)| {
            let mut rng = rng_from_seed(opts.seed);
            let mut c = ((x-0.5) * width / height, y-0.5);
            c.0 = c.0.abs();
            c = p_r(c, 0.3 * (ci as f64 - 1.0) * PI / 3.);
            let mut s = 100f64;
            let k = 0.05;
            for _i in 0..8 {
              let mut p = (c.0, c.1);
              p.0 += rng.gen_range(-0.16, 0.16);
              p.1 += rng.gen_range(-0.16, 0.16);
              p = p_r(p, rng.gen_range(0.0, 10.0));
              let dim = (
                rng.gen_range(0.0, 0.1),
                rng.gen_range(0.0, 0.1)
              );
              s = op_union_round(s, sdf_box2(p, dim), k);
            }
            let f1 = rng.gen_range(3.0, 4.0) + 0.1 * opts.v1;
            let f3 = rng.gen_range(3.0, 4.0) + 0.1 * opts.v2;
            let a1 = 0.35;
            let a2 = 2.0;
            let n = a1 * perlin.get([
                f1 * c.0,
                f1 * c.1,
                3.7 * opts.seed +
                opts.v3 +
                a2 * perlin.get([
                    3.7 * opts.seed + 0.5 * opts.v3,
                    c.0 + 2.0 * perlin.get([f3 * c.0, f3 * c.1, 12.1 * opts.seed]),
                    8.0 * c.1 + 1.3 * perlin.get([f3 * c.0, f3 * c.1, 26.7 * opts.seed])
                  ])
                ]);
            lerp(-0.05, 0.25, s) + n
        };
        
        let thresholds: Vec<f64> = 
            (0..samples)
            .map(|i| 
                (((ci + colors.len() * i)) as f64) / ((colors.len() * samples) as f64))
            .collect();
        let res = contour(w, h, f, &thresholds);
        let routes = features_to_routes(res, precision);
        
        let inside = |from, to| {
            strictly_in_boundaries(from, (pad, pad, width-pad, height-pad)) &&
            strictly_in_boundaries(to, (pad, pad, width-pad, height-pad))
        };
        
        let mut l = Group::new()
            .set("inkscape:groupmode", "layer")
            .set("inkscape:label", color)
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", stroke_width);

        let opacity: f64 = 0.6;
        
        let opdiff = 0.15 / (routes.len() as f64);
        let mut trace = 0f64;
        for r in routes.clone() {
            trace += 1f64;
            if r.len() < 2 {
                continue;
            }
            let data = render_route_when(Data::new(), r, inside);
            l = l.add(
                Path::new()
                .set("opacity", (1000. * (opacity - trace * opdiff)).floor()/1000.0)
                .set("d", data)
            );
        }
        layers.push(l);
    }
        
    layers 
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
    let opts = val.into_serde().unwrap();

    let mut g = Group::new();
    let a = art(&opts);
    for e in a {
        g = g.add(e);
    }
    let str = g.to_string();
    return str;
}

#[inline]
pub fn project_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> (f64, f64) {
    (
        significant_str(p.0 * (boundaries.2 - boundaries.0) + boundaries.0),
        significant_str(p.1 * (boundaries.3 - boundaries.1) + boundaries.1),
    )
}

#[inline]
pub fn strictly_in_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> bool {
    p.0 > boundaries.0
        && p.0 < boundaries.2
        && p.1 > boundaries.1
        && p.1 < boundaries.3
}

pub fn crop_route(
    route: &Vec<(f64, f64)>,
    boundaries: (f64, f64, f64, f64),
) -> Option<Vec<(f64, f64)>> {
    if route.len() < 2
        || route
            .iter()
            .all(|&p| !strictly_in_boundaries(p, boundaries))
    {
        return None;
    }
    return Some(route.clone());
}

pub fn crop_routes(
    routes: &Vec<Vec<(f64, f64)>>,
    boundaries: (f64, f64, f64, f64),
) -> Vec<Vec<(f64, f64)>> {
    return routes
        .iter()
        .filter_map(|route| crop_route(&route, boundaries))
        .collect();
}

#[inline]
fn significant_str (f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

pub fn sample_2d_candidates(
    f: &dyn Fn((f64, f64)) -> bool,
    dim: usize,
    samples: usize,
    rng: &mut impl Rng,
) -> Vec<(f64, f64)> {
    let mut candidates = Vec::new();
    for x in 0..dim {
        for y in 0..dim {
            let p = (
                (x as f64) / (dim as f64),
                (y as f64) / (dim as f64),
            );
            if f(p) {
                candidates.push(p);
            }
        }
    }
    rng.shuffle(&mut candidates);
    candidates.truncate(samples);
    return candidates;
}


fn render_route_when<F: FnMut(
    (f64, f64),
    (f64, f64),
) -> bool>(
    data: Data,
    route: Vec<(f64, f64)>,
    mut should_draw_line: F,
) -> Data {
    let mut first = true;
    let mut up = false;
    let mut last = (0.0, 0.0);
    let mut d = data;
    for p in route {
        if first {
            if should_draw_line(p, p) {
                first = false;
                d = d.move_to(p);
            }
        } else {
            if should_draw_line(last, p) {
                if up {
                    up = false;
                    d = d.move_to(last);
                }
                d = d.line_to(p);
            } else {
                up = true;
            }
        }
        last = p;
    }
    return d;
}

#[inline]
fn lerp(a: f64, b: f64, x: f64) -> f64 {
    (x - a) / (b - a)
}

pub struct Passage2DCounter {
    granularity: f64,
    width: f64,
    height: f64,
    counters: Vec<usize>,
}
impl Passage2DCounter {
    pub fn new(
        granularity: f64,
        width: f64,
        height: f64,
    ) -> Self {
        let wi = (width / granularity).ceil() as usize;
        let hi = (height / granularity).ceil() as usize;
        let counters = vec![0; wi * hi];
        Passage2DCounter {
            granularity,
            width,
            height,
            counters,
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
    pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
        let i = self.index(p);
        let v = self.counters[i] + 1;
        self.counters[i] = v;
        v
    }
    pub fn get(self: &Self, p: (f64, f64)) -> usize {
        self.counters[self.index(p)]
    }
}
pub fn rng_from_seed(s: f64) -> impl Rng {
    let mut bs = [0; 16];
    bs.as_mut().write_f64::<BigEndian>(s).unwrap();
    let mut rng = SmallRng::from_seed(bs);
    for _i in 0..10 {
        rng.gen::<f64>();
    }
    return rng;
}

pub fn out_of_boundaries(
    p: (f64, f64),
    boundaries: (f64, f64, f64, f64),
) -> bool {
    p.0 < boundaries.0
        || p.0 > boundaries.2
        || p.1 < boundaries.1
        || p.1 > boundaries.3
}

fn contour<F: FnMut((f64, f64)) -> f64>(
    width: u32,
    height: u32,
    mut f: F,
    thresholds: &Vec<f64>,
) -> Vec<Feature> {
    let c = ContourBuilder::new(width, height, true);
    let values = rasterize_1d(width, height, &mut f);
    c.contours(&values, &thresholds).unwrap_or(Vec::new())
}

fn features_to_routes(
    features: Vec<Feature>,
    precision: f64,
) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();
    for f in features {
        for g in f.geometry {
            let value = g.value;
            match value {
                geojson::Value::MultiPolygon(all) => {
                    for poly in all {
                        for lines in poly {
                            let mut points = lines
                                .iter()
                                .map(|p| {
                                    (
                                        precision * p[0],
                                        precision * p[1],
                                    )
                                })
                                .collect::<Vec<(f64, f64)>>(
                                );
                            let len = points.len();
                            if len < 3 {
                                continue;
                            }
                            if euclidian_dist(
                                points[0],
                                points[len - 1],
                            ) <= precision
                            {
                                points.push(points[0]);
                            }
                            routes.push(points);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    routes
}

#[inline]
fn euclidian_dist(
    (x1, y1): (f64, f64),
    (x2, y2): (f64, f64),
) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}

fn rasterize_1d<F: FnMut((f64, f64)) -> f64>(
    width: u32,
    height: u32,
    mut f: F,
) -> Vec<f64> {
    (0..height)
        .flat_map(|y| {
            (0..width)
                .map(|x| {
                    f((
                        x as f64 / width as f64,
                        y as f64 / height as f64,
                    ))
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<f64>>()
}