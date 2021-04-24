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

    let size = 80.;
    let f2 = (2., 4.);
    let f1 = (14., 22.);
    let amp1 = 0.8;
    let amp2 = 0.4;
    let samples = 200000;
    let spins = 120.0;
    let splits = 3.0;
    let split_threshold = 1.0;
    let pow = 2.0;
    let noise_angle_freq = 2.0;
    let noise_angle_displacement_freq = 0.02;
    let noise_angle_displacement = 0.1;

    let parametric = |p: f64| {
        let p1 = (splits * p).floor();
        let p2 = splits * p - p1;
        let t = (p1 + split_threshold * p2) / splits;
        let t2 =
            (p1 + split_threshold * p2.powf(pow)) / splits;
        let scale = 1.0 - t2;
        let s = spins;
        let mut p = (
            scale
                * amp1
                * ((s * 2. * PI * t).cos()
                    + amp2
                        * mix(
                            (s * f1.0 * PI * t).cos(),
                            (s * f2.0 * PI * t).cos(),
                            t,
                        )),
            scale
                * amp1
                * ((s * 2. * PI * t).sin()
                    + amp2
                        * mix(
                            (s * f1.1 * PI * t).sin(),
                            (s * f2.1 * PI * t).sin(),
                            t,
                        )),
        );
        let noise_angle = 2.
            * PI
            * perlin.get([
                noise_angle_freq * p.0,
                noise_angle_freq * p.1,
                100.0 + opts.seed,
            ]);
        let noise_amp = noise_angle_displacement
            * perlin
                .get([
                    noise_angle_displacement_freq * p.0,
                    noise_angle_displacement_freq * p.1,
                    opts.seed,
                ])
                .max(0.0);
        p.0 *= 1.4;
        p.0 += noise_amp * noise_angle.cos();
        p.1 += noise_amp * noise_angle.sin();
        p
    };

    let mut routes = Vec::new();
    let mut route = Vec::new();
    let mut last = (-1000.0, -1000.0);
    for i in 0..(samples + 1) {
        let sp = i as f64 / (samples as f64);
        let o = parametric(sp);
        let p = (
            width * 0.5 + size * o.0,
            height * 0.5 + size * o.1,
        );
        if euclidian_dist(p, last) > 2.0 {
            routes.push(route);
            route = Vec::new();
        }
        route.push(p);
        last = p;
    }
    routes.push(route);

    let color = "darkblue";
    let data =
        routes.iter().fold(Data::new(), |data, route| {
            render_route(data, route.clone())
        });
    let mut l = layer(color);
    l = l.add(base_path(color, 0.5, data));
    l = l.add(signature(1.5, (200.0, 180.0), color));
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
