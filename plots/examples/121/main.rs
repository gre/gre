use clap::Clap;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 210f64;
    let height = 297f64;
    let perlin = Perlin::new();

    let total = 16.;
    let fp = opts.frame / total;

    let f1x = 8.0;
    let f1y = 8.0;
    let f2x = 14.;
    let f2y = 14.;
    let pingpong = (PI * fp).sin();
    let amp1 = 1. - 0.2 * pingpong;
    let amp2 = 0.2 * pingpong;

    let samples = 100000;
    let f1 = (f1x, f1y);
    let f2 = (f2x, f2y);
    let spins = opts.spins;
    let pow = opts.pow;

    let colors = vec!["firebrick"];
    let mut layers = Vec::new();
    for (i, color) in colors.iter().enumerate() {
        let size = opts.size;
        let parametric = |t: f64| {
            let mut t2 = t.powf(pow);
            let initial = 1. / spins;
            t2 = (t2 - initial).max(0.) / (1. - initial);
            t2 = (t2 / (1. - 2. * initial)).min(1.);
            t2 *= 1. - 0.2 * fp;
            let scale = 1.0 - t2;
            let s = spins;
            let dx = (2. * fp - 1.)
                * 0.3
                * (((t).powf(2.0) - 0.3).abs() - 0.3);
            let dy = 0.0;
            let mut p = (
                dx + scale
                    * amp1
                    * ((s * 2. * PI * t).sin()
                        + amp2
                            * mix(
                                (s * f1.1 * PI * t).sin(),
                                (s * f2.1 * PI * t).sin(),
                                t,
                            )),
                dy + scale
                    * amp1
                    * ((s * 2. * PI * t).cos()
                        + amp2
                            * mix(
                                (s * f1.0 * PI * t).cos(),
                                (s * f2.0 * PI * t).cos(),
                                t,
                            )),
            );
            // glitch noise on edges
            let noise_angle = 2.
                * PI
                * perlin.get([p.0, p.1, 1000.0 + fp]);
            let noise_amp = 0.01
                * (1.0 - t).powf(2.0)
                * (0.5 - p.1).max(0.0)
                * perlin
                    .get([
                        20.0 * p.0,
                        20.0 * p.1,
                        -1000. + 10. * pingpong,
                    ])
                    .max(0.);
            p.0 += noise_amp * noise_angle.cos();
            p.1 += noise_amp * noise_angle.sin();
            // med freq noise
            let noise_angle = 2.
                * PI
                * perlin.get([
                    3. * p.0,
                    3. * p.1,
                    100.0 + 4. * fp,
                ]);
            let noise_amp = 0.01
                * perlin.get([
                    6. * p.0,
                    6. * p.1,
                    100. + 10. * fp,
                ]);
            p.0 += noise_amp * noise_angle.cos();
            p.1 += noise_amp * noise_angle.sin();
            // low freq noise
            let noise_angle = 2.
                * PI
                * perlin.get([
                    0.6 * p.0,
                    0.6 * p.1,
                    50. + fp,
                ]);
            let noise_amp = 0.1
                * perlin.get([
                    1.2 * p.0,
                    1.2 * p.1,
                    -50. + fp,
                ]);
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

        let data = routes
            .iter()
            .fold(Data::new(), |data, route| {
                render_route(data, route.clone())
            });
        let mut l = layer(color);
        l = l.add(base_path(color, 0.3, data));
        if i == 0 {
            l = l.add(signature(
                1.0,
                (160.0, 230.0),
                color,
            ));
        }
        layers.push(l);
    }

    layers
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.")]
    frame: f64,
    #[clap(short, long, default_value = "90.")]
    size: f64,
    #[clap(short, long, default_value = "100.0")]
    spins: f64,
    #[clap(short, long, default_value = "1.2")]
    pow: f64,
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
