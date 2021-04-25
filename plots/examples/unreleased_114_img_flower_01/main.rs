use clap::Clap;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 297f64;
    let height = 210f64;
    let pad = 10f64;
    let perlin = Perlin::new();
    let boundaries = (pad, pad, width - pad, height - pad);

    let get_color =
        image_get_color(opts.path.as_str()).unwrap();
    let samples = 100000;
    let spins = opts.spins;
    let splits = opts.splits;
    let split_threshold = opts.split_threshold;
    let pow = opts.pow;
    let noise_angle_freq = opts.noise_angle_freq;
    let noise_angle_displacement_freq =
        opts.noise_angle_displacement_freq;
    let noise_angle_displacement =
        opts.noise_angle_displacement;

    let colors = vec!["orange", "red"];
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
            let initial = 2. / spins;
            t2 = (t2 - initial).max(0.) / (1. - initial);
            let scale = 1.0 - t2;
            let s = spins;
            let mut p = (
                -0.04
                    + (i as f64 - 0.5) * 0.001
                    + scale
                        * ((s * 2. * PI * t).cos()
                            + 0.15
                                * (1. - p)
                                * mix(
                                    (s * 12. * PI * t)
                                        .cos(),
                                    (s * 2. * PI * t).cos(),
                                    t.powf(0.5),
                                )),
                (i as f64 - 0.5) * 0.002
                    + scale
                        * ((s * 2. * PI * t).sin()
                            + 0.15
                                * (1. - p)
                                * mix(
                                    (s * 12. * PI * t)
                                        .sin(),
                                    (s * 2. * PI * t).sin(),
                                    t.powf(0.5),
                                )),
            );
            let v = grayscale(get_color(
                normalize_in_boundaries(
                    p,
                    (-1.3, -1.3, 1.3, 1.3),
                ),
            ));
            let diff = i as f64 * opts.seed_diff;
            let noise_angle = 2.
                * PI
                * perlin.get([
                    noise_angle_freq * p.0,
                    noise_angle_freq * p.1,
                    -100.0 - opts.seed + diff,
                ]);
            let noise_amp = noise_angle_displacement
                * perlin.get([
                    noise_angle_displacement_freq * p.0,
                    noise_angle_displacement_freq * p.1,
                    opts.seed + diff,
                ]);
            p.0 += noise_amp * noise_angle.cos();
            p.1 += noise_amp * noise_angle.sin();
            p.0 += 9999. * (1. - v);
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
            if euclidian_dist(p, last) > 2.0
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
        l = l.add(base_path(color, 0.35, data));
        if i == 0 {
            l = l.add(signature(
                1.0,
                (230.0, 190.0),
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
    #[clap(
        short,
        long,
        default_value = "/Users/grenaudeau/Desktop/text.png"
    )]
    path: String,
    #[clap(short, long, default_value = "95.0")]
    seed: f64,
    #[clap(short, long, default_value = "10.0")]
    seed_diff: f64,
    #[clap(short, long, default_value = "120.")]
    size: f64,
    #[clap(short, long, default_value = "140.0")]
    spins: f64,
    #[clap(short, long, default_value = "1.0")]
    splits: f64,
    #[clap(short, long, default_value = "0.99")]
    split_threshold: f64,
    #[clap(short, long, default_value = "0.9")]
    pow: f64,
    #[clap(short, long, default_value = "18.8")]
    noise_angle_freq: f64,
    #[clap(short, long, default_value = "2.8")]
    noise_angle_displacement_freq: f64,
    #[clap(short, long, default_value = "0.003")]
    noise_angle_displacement: f64,
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
