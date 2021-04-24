use clap::Clap;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let height = 210f64;
    let width = 297f64;
    let perlin = Perlin::new();

    let size = 90.;
    let f1 = (8., 8.);
    let f2 = (16., 16.);
    let amp1 = 1.0;
    let amp2 = 0.05;
    let samples = 60000;
    let spins = 100.0;

    let parametric = |t: f64| {
        let scale = 1.0 - 0.7 * t;
        (
            scale
                * amp1
                * ((spins * 2. * PI * t).cos()
                    + amp2
                        * mix(
                            (spins * f1.0 * PI * t).cos(),
                            (spins * f2.0 * PI * t).cos(),
                            t,
                        )),
            scale
                * amp1
                * ((spins * 2. * PI * t).sin()
                    + amp2
                        * mix(
                            (spins * f1.1 * PI * t).sin(),
                            (spins * f2.1 * PI * t).sin(),
                            t,
                        )),
        )
    };

    let mut routes = Vec::new();
    let mut route = Vec::new();
    for i in 0..(samples + 1) {
        let sp = i as f64 / (samples as f64);
        let o = parametric(sp);
        let mut p = (
            width * 0.5 + size * o.0,
            height * 0.5 + size * o.1,
        );
        let noise_angle = 2.
            * PI
            * perlin.get([
                0.02 * p.0,
                0.02 * p.1,
                100.0 + opts.seed,
            ]);
        let noise_amp =
            perlin.get([0.01 * p.0, 0.01 * p.1, opts.seed]);
        p.0 += noise_amp * noise_angle.cos();
        p.1 += noise_amp * noise_angle.sin();
        route.push(p);
    }
    routes.push(route);

    let color = "black";
    let data =
        routes.iter().fold(Data::new(), |data, route| {
            render_route(data, route.clone())
        });
    let mut l = layer(color);
    l = l.add(base_path(color, 0.1, data));
    l = l.add(signature(1., (215.0, 185.0), color));
    vec![l]
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
