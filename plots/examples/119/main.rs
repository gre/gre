use clap::Clap;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 210f64;
    let height = 297f64;
    let size = 100.;
    let mut rng = rng_from_seed(opts.seed);
    let mut passage =
        Passage2DCounter::new(0.3, width, height);

    let f1 = (2 + 2 * (rng.gen::<usize>() % 30)) as f64;
    let f2 = (2 + 2 * (rng.gen::<usize>() % 60)) as f64;
    let rep = (2 + rng.gen::<usize>() % 5) as f64;
    let count = rep as usize * 8;
    let samples = (f1.max(f2) * 50.) as usize;

    let parametric = |t: f64, p: f64| {
        let p1 = (rep * p).floor();
        let p2 = rep * p - p1;
        let v = (p1 + 0.2 * p2) / (rep + 1.0);
        (
            (0.15 + v) * (2. * PI * t).sin()
                + 0.1 * (f1 * PI * t).sin(),
            (0.3 + v) * (2. * PI * t).cos()
                + 0.2 * (f2 * PI * t).cos(),
        )
    };

    let mut routes = Vec::new();
    for pass in 0..count {
        let mut route = Vec::new();
        let r = rng.gen::<f64>();
        for i in 0..(samples + 1) {
            let sp =
                (r + i as f64 / (samples as f64)) % 1.0;
            let o = parametric(
                sp,
                pass as f64 / (count as f64),
            );
            let p = (
                width * 0.5 + size * o.0,
                height * 0.5 + size * o.1,
            );
            let count = passage.count(p);
            if count > 4 {
                if route.len() > 1 {
                    routes.push(route);
                }
                route = Vec::new();
            } else {
                route.push(p);
            }
        }
        routes.push(route);
    }

    let color = "white";
    let data =
        routes.iter().fold(Data::new(), |data, route| {
            render_route(data, route.clone())
        });
    let mut l = layer(color);
    l = l.add(base_path(color, 0.1, data));
    l = l.add(signature(1.0, (160.0, 260.0), color));
    vec![l]
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "85.0")]
    seed: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_portrait("black");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
