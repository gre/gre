use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
pub struct Opts {
    #[clap(short, long, default_value = "image.svg")]
    file: String,
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
    let width = 420.;
    let height = 297.;  
    let pad = 20.0;
    let colors = vec!["#f90", "#09f"];

    let seed = opts.seed;
    let mut rng = rng_from_seed(seed);
    let dy = 0.4;
    let r = rng.gen_range(1.0, 3.0);
    let res1 = (r, r);
    let r = rng.gen_range(1.0, 3.0);
    let res2 = (r, r);
    let r1 = rng.gen_range(0.0, 1.0);
    let r2 = rng.gen_range(0.0, 1.0);
    let r3 = rng.gen_range(0.0, 1.0);
    let r4 = rng.gen_range(0.0, 1.0);
    let r5 = rng.gen_range(0.1, 10.0);

    let perlin = Perlin::new();
    let low_poly_perlin = |(xr, yr): (f64, f64), x: f64, y: f64, s: f64| {
        // quadradic interpolation between 4 noise points
        let xi = x / xr;
        let yi = y / yr;
        let x1 = xr * xi.floor();
        let y1 = yr * yi.floor();
        let x2 = xr * xi.ceil();
        let y2 = yr * yi.ceil();
        let xp = xi - xi.floor();
        let yp = yi - yi.floor();
        let p1 = perlin.get([ x1, y1, s ]);
        let p2 = perlin.get([ x2, y1, s ]);
        let p3 = perlin.get([ x2, y2, s ]);
        let p4 = perlin.get([ x1, y2, s ]);
        mix(
            mix(p1 as f64, p2 as f64, xp),
            mix(p4 as f64, p3 as f64, xp), yp)
    };

    
    colors
        .iter()
        .enumerate()
        .map(|(ci, color)| {
            let from_r = 1.0;
            let to_r = (width + height) / 2.0;

            let mut routes = Vec::new();
            let mut radius = from_r;

            let mut height_map: Vec<f64> = Vec::new();
            loop {
                if radius > to_r {
                    break;
                }
                let is_color = (radius < height * 0.25) == (ci == 0);
                let mut route = Vec::new();
                let precision = 0.01;
                let mut a = 0.0;
                let mut was_outside = true;
                let mut i = 0;
                loop {
                    if a > 2. * PI + precision {
                        break;
                    }
                    a = a.min(2. * PI);
                    let x = width / 2.0 + radius * a.cos();
                    let y = height / 2.0 + radius * a.sin();
                    let amp =
                    mix(100.0, 300.0, 1. - r1 * r1) *
                    mix(0.0, 1.0, radius / to_r);
                    let shape = low_poly_perlin(
                        res1,
                        mix(0.5, 2.0, r3) * 0.1 * radius / r5,
                        mix(0.5, 2.0, r3) * a.cos() * r5,
                        7.7 * seed + 0.3 * a.sin() + mix(0.05, 0.2, r2) *
                        low_poly_perlin(
                            res2,
                            mix(0.5, 2.0, r4) * 0.1 * x,
                            mix(0.5, 2.0, r4) * 0.1 * y,
                            seed / 3.
                        ),
                    );
                    let displacement =
                        mix(0.0008, 0.01,
                            smoothstep(-0.2, -0.5, shape).powf(2.0) *
                            (radius / to_r).max(0.0).min(1.0)) *
                        perlin.get([
                            seed * 9.3,
                            0.5 * x,
                            0.5 * y,
                        ]);
                    let r = radius + amp * (shape + displacement);
                    let mut collides = false;
                    if i >= height_map.len() {
                        height_map.push(r);
                    }
                    else {
                        if r < height_map[i] + dy * 0.5 {
                            collides = true;
                        }
                        else {
                            height_map[i] = r;
                        }
                    }
                    let x = width / 2.0 + r * a.cos();
                    let y = height / 2.0 + r * a.sin();
                    let inside = !collides &&
                    pad < x && x < width - pad &&
                    pad < y && y < height - pad;
                    if inside {
                        if was_outside {
                            if route.len() > 2 {
                                if is_color {
                                    routes.push(route);
                                }
                            }
                            route = Vec::new();
                        }
                        was_outside = false;
                        route.push((x, y));
                    }
                    else {
                        was_outside = true;
                    }
                    a += precision;
                    i += 1;
                }
                
                if is_color {
                    routes.push(route);
                }

                radius += dy;
            }
        
            let mut data = Data::new();
            for r in routes.clone() {
                data = render_route(data, r);
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
    let mut document = base_a3_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save(opts.file, &document).unwrap();
}
