use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
}

fn art(opts: Opts) -> Vec<Group> {
    let (width, height) = (210., 297.);
    let colors = vec!["black", "red"];

    colors.iter().enumerate().map(|(ci, &color)| {
        let mut rng = rng_from_seed(opts.seed);
        let circles = rng.gen_range(3, 16);
        let count = 180 / circles;
        let size = rng.gen_range(100.0, 120.0) / (circles as f64);
        let mut data = Data::new();
        let mut routes = Vec::new();
        let noise = OpenSimplex::new();
        let seed = opts.seed;
        let f = |p: (f64, f64)| {
            let mut rng = rng_from_seed(seed);
            let x = p.0 / width;
            let y = p.1 / height;
            2.0 * noise.get([
                x + rng.gen_range(0.0, 8.0) * noise.get([ rng.gen_range(0.0, 20.0) * x, rng.gen_range(0.0, 10.0) * y, 66. + 7.4 * seed ]),
                y + rng.gen_range(0.0, 8.0) * noise.get([
                    rng.gen_range(0.0, 20.0) * x,
                    rng.gen_range(0.0, 10.0) * y,
                    6. + 9.4 * seed + 2.0 * rng.gen_range(0.0, 1f64).powf(2f64) * noise.get([
                        rng.gen_range(0.0, 40.0) * x,
                        rng.gen_range(0.0, 40.0) * y,
                        66. + 7.4 * seed
                    ])
                ]),
                seed
            ])
        };
        for i in 0..count {
            let x = (i as f64) / (count as f64);
            for c in 0..circles {
                if c % colors.len() != ci {
                    continue;
                }
                let r = 4.0 + 90.0 * (c as f64) / (circles as f64);
                let splits = ((6.0 + r) * 6.0) as usize;
                let mut route = Vec::new();
                for s in 0..splits {
                    let a = 2. * PI * (s as f64) / ((splits - 1) as f64);
                    let p = (
                        width / 2.0 + a.cos() * r,
                        height / 2.0 + a.sin() * r,
                    );
                    let v = f(p) * (x - 0.5);
                    let rx = r + size * v;
                    let p = (
                        width / 2.0 + a.cos() * rx,
                        height / 2.0 + a.sin() * rx,
                    );
                    route.push(p);
                }
                routes.push(route);
            }
        }
        let mut l = layer(color);
        for r in routes {
            data = render_route(data, r);
        }
        l = l.add(base_path(color, 0.35, data));
        if ci == 0 {
            l = l.add(signature(0.6, (130., 233.), color));
        }
        l
    }).collect()
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
