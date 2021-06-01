use clap::Clap;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 210f64;
    let height = 297f64;
    let padx = 10f64;
    let pady = 30f64;
    let perlin = Perlin::new();
    let boundaries = (padx, pady, width - padx, height - pady);
    let samples = opts.samples;
    let f1 = (opts.f1x, opts.f1y);
    let f2 = (opts.f2x, opts.f2y);
    let amp1 = opts.amp1;
    let amp2 = opts.amp2;
    let spins = opts.spins;
    let splits = opts.splits;
    let split_threshold = opts.split_threshold;
    let pow = opts.pow;

    let colors = vec!["darkturquoise", "firebrick"];
    let mut layers = Vec::new();
    for (i, color) in colors.iter().enumerate() {
        let size = opts.size - i as f64;
        let parametric = |p: f64| {
            let p1 = (splits * p).floor();
            let p2 = splits * p - p1;
            let t = (p1 + split_threshold * p2) / splits;
            let mut t2 = (p1
                + split_threshold * p2.powf(pow))
                / splits;
            let initial = 1. / spins;
            t2 =
                (t2 - initial).max(0.) / (1. - initial);
            let scale = 1.0
                - t2 * (1.0
                    - i as f64 * opts.size_diff / size);
            let s = spins;
            let mut p = (
                scale
                    * amp1
                    * ((s * 2. * PI * t).sin()
                        + amp2
                            * mix(
                                (s * f1.1 * PI * t).sin(),
                                (s * f2.1 * PI * t).sin(),
                                t,
                            )),
                scale
                    * amp1
                    * ((s * 2. * PI * t).cos()
                        + amp2
                            * mix(
                                (s * f1.0 * PI * t).cos(),
                                (s * f2.0 * PI * t).cos(),
                                t,
                            )),
            );
            let noise_angle = p.1.atan2(p.0);
            let noise_amp = 0.005 * perlin.get([
                    5.8 * p.0 + perlin.get([
                        5.8 * p.0,
                        4.2 * p.1,
                        40. + opts.seed
                    ]),
                    5.8 * p.1 + 0.8 * perlin.get([
                        6.5 * p.0,
                        6.8 * p.1 + perlin.get([
                            15.5 * p.0,
                            15.8 * p.1,
                            200. + opts.seed,
                        ]),
                        20. + opts.seed,
                    ]),
                    100. + opts.seed + i as f64 * opts.seed_diff,
                ]) +
                opts.disp * (1. - t) * perlin.get([
                    0.7 * p.0 + perlin.get([
                        2.9 * p.0,
                        1.7 * p.1,
                        2000.0
                    ]),
                    0.7 * p.1 + perlin.get([
                        3.1 * p.0,
                        2.5 * p.1,
                        2100.0
                    ]),
                    1000.,
                ]);

            p.0 += noise_amp * noise_angle.cos();
            p.1 += noise_amp * noise_angle.sin();
            p.0 += opts.offsetx + opts.offsetx_diff * i as f64;
            p.1 += opts.offsety + opts.offsety_diff * i as f64;
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
            let mut lookup =
                normalize_in_boundaries(p, boundaries);
            lookup.0 = (lookup.0 - 0.5) / opts.sx + 0.5;
            lookup.1 = (lookup.1 - 0.5) / opts.sy + 0.5;
            if euclidian_dist(p, last) > opts.threshold
                || out_of_boundaries(p, boundaries)
            {
                if route.len() > 1 {
                    routes.push(route);
                }
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
        l = l.add(signature(0.8, (180.0, 220.0), color));
        layers.push(l);
    }
    layers
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "1.")]
    sx: f64,
    #[clap(short, long, default_value = "1.")]
    sy: f64,
    #[clap(short, long, default_value = "17.")]
    seed: f64,
    #[clap(short, long, default_value = "16.")]
    seed_diff: f64,
    #[clap(short, long, default_value = "0.")]
    offsetx: f64,
    #[clap(short, long, default_value = "-1.2")]
    offsety: f64,
    #[clap(short, long, default_value = "220.")]
    size: f64,
    #[clap(short, long, default_value = "0.8")]
    size_diff: f64,
    #[clap(short, long, default_value = "0.001")]
    offsetx_diff: f64,
    #[clap(short, long, default_value = "0.001")]
    offsety_diff: f64,
    #[clap(short, long, default_value = "12.")]
    f1x: f64,
    #[clap(short, long, default_value = "8.")]
    f1y: f64,
    #[clap(short, long, default_value = "4.")]
    f2x: f64,
    #[clap(short, long, default_value = "8.")]
    f2y: f64,
    #[clap(short, long, default_value = "1.8")]
    amp1: f64,
    #[clap(short, long, default_value = "0.1")]
    amp2: f64,
    #[clap(short, long, default_value = "500.0")]
    spins: f64,
    #[clap(short, long, default_value = "1.0")]
    splits: f64,
    #[clap(short, long, default_value = "1.0")]
    split_threshold: f64,
    #[clap(short, long, default_value = "1.1")]
    pow: f64,
    #[clap(short, long, default_value = "1.5")]
    threshold: f64,
    #[clap(short, long, default_value = "0.1")]
    disp: f64,
    #[clap(short, long, default_value = "1000000")]
    samples: usize,
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
