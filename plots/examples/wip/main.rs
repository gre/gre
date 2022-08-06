use std::f64::consts::PI;

use clap::*;
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

fn art(opts: &Opts) -> Vec<Group> {
    let colors = vec!["black"];
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();

            let perlin = Perlin::new();
            let width = opts.width;
            let height = opts.height;
            let pad = 10.0;
            let mut rng = rng_from_seed(opts.seed);

            let spread = 150.0;

            let mut routes: Vec<Vec<(f64, f64)>> =
                Vec::new();

            let iterations = rng.gen_range(4, 9);
            for j in 0..iterations {
                let noisef1 = 0.01;
                let noisef2 = 0.08;
                let noiseamp2 = rng.gen_range(0.05, 0.2);
                let jp =
                    j as f64 / (iterations as f64 - 1.0);
                let offsetx =
                    (if j % 2 == 0 { -1.0 } else { 1.0 })
                        * 0.5
                        * spread
                        * (1.0 - jp);

                let offsety = 0.25 * spread * (jp - 0.5);

                let seed = opts.seed + j as f64 * 7.7;

                let f = |p: (f64, f64)| {
                    let x = p.0 + offsetx;
                    let y = p.1 + offsety;
                    let mut n = 2.0
                        * (perlin.get([
                            noisef1 * x,
                            noisef1 * y,
                            seed + 500.5,
                        ]) + noiseamp2
                            * perlin.get([
                                noisef2 * x,
                                noisef2 * y,
                                999. + seed * 3.7,
                            ]));
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

                let mut copy: Vec<Vec<(f64, f64)>> =
                    Vec::new();

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

                // TODO we could make the contour stronger with some offset too

                let precision = 0.5;
                let w = (width / precision) as u32;
                let h = (height / precision) as u32;
                let bounds =
                    (pad, pad, width - pad, height - pad);
                // TODO we could cut some part of the threshold to create more variety. for now we do a double line
                let thresholds = vec![0.0, 0.03];
                let g = |(x, y)| f((x * width, y * height));
                let res = contour(w, h, g, &thresholds);
                let mut routes_contour =
                    features_to_routes(res, precision);
                routes_contour =
                    crop_routes(&routes_contour, bounds);

                for route in routes_contour {
                    routes.push(route);
                }

                // TODO pregen a map of orientations (field)

                //if j == 0 {
                let count = 70000;
                for i in 0..count {
                    let x = rng.gen_range(pad, width - pad);
                    let y =
                        rng.gen_range(pad, height - pad);
                    let mut n = f((x, y));
                    if n > 0.0 {
                        // points on edge
                        n = (1.0 - 0.7 * n).max(0.0);
                        if n > 0.001 {
                            // TODO orientation du soleil au lieu d'avoir un absolu. il faut déterminer le centre du nuage.
                            // more points on bottom
                            n += 0.01 * (y - height / 2.0);
                        }
                    }
                    if n > rng.gen_range(0.0, 1.0) {
                        // TODO line orientation to follow the noise
                        // calc normal vector
                        // longer curvy line instead of one stroke?
                        let a = PI / 2.0
                            + (y + offsety - height / 2.)
                                .atan2(
                                    x + offsetx
                                        - width / 2.,
                                );
                        let amp = 0.5;
                        let dx = amp * a.cos();
                        let dy = amp * a.sin();
                        routes.push(vec![
                            (x - dx, y - dy),
                            (x + dx, y + dy),
                        ]);
                    }
                }
                //}
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
