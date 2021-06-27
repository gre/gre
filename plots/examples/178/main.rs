use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clone)]
struct Cycle {
    count: usize,
    vradius: f64,
    vx: f64,
    vy: f64,
}

#[derive(Clone)]
struct ShapeConfig {
    resolution: usize,
    initial_radius: f64,
    initial_center: (f64, f64),
    cycles: Vec<Cycle>,
    harmonies: Vec<(f64, f64)>,
    displacement: f64,
    disp_harmonies: Vec<(f64, f64)>,
    seed: f64,
}

fn shape(
    ShapeConfig {
        resolution,
        initial_radius,
        initial_center,
        cycles,
        harmonies,
        displacement,
        disp_harmonies,
        seed,
    }: ShapeConfig,
) -> Data {
    let perlin = Perlin::new();

    let mut passage = Passage2DCounter::new(1.0, 297., 210.);
    let max_passage = 6;
    let mut should_draw_line = |a: (f64,f64), b: (f64,f64)| {
        let m = (mix(a.0, b.0, 0.5), mix(a.1, b.1, 0.5));
        passage.count(m) < max_passage
    };

    let amp_multiplier = |a: f64| -> f64 {
        1. + harmonies
            .iter()
            .enumerate()
            .map(|(h, &(amp, f))| {
                amp * perlin.get([seed + h as f64, f * a])
            })
            .sum::<f64>()
    };
    let disp = |p: (f64, f64), r: f64| -> (f64, f64) {
        let a = 2.
            * PI
            * disp_harmonies
                .iter()
                .enumerate()
                .map(|(h, &(amp, f))| {
                    amp * perlin.get([
                        100. + seed + h as f64 + 0.1 * perlin.get([
                            0.0,
                            2. * f * p.0,
                            2. * f * p.1,
                        ]),
                        f * p.0,
                        f * p.1,
                    ])
                })
                .sum::<f64>();
        (
            p.0 + r * displacement * a.cos(),
            p.1 + r * displacement * a.cos(),
        )
    };

    let mut rng = rng_from_seed(seed);
    let mut routes = Vec::new();
    let mut radius = initial_radius;
    let mut center = initial_center;
    for Cycle {
        count,
        vradius,
        vx,
        vy,
    } in cycles
    {
        for _i in 0..count {
            let a_off = rng.gen_range(0.0, 1.0);
            let mut route = (0..resolution)
                .map(|j| {
                    let a = (a_off
                        + j as f64 / (resolution as f64))
                        % 1.;
                    let ang = a * 2. * PI;
                    let amp = amp_multiplier(a) * radius;
                    disp(
                        (
                            center.0 + amp * ang.cos(),
                            center.1 + amp * ang.sin(),
                        ),
                        amp,
                    )
                })
                .collect::<Vec<(f64, f64)>>();
            route.push(route[0]);
            routes.push(route);

            radius += vradius;
            center.0 += vx;
            center.1 += vy;
        }
    }

    routes.iter().fold(Data::new(), |data, route| {
        render_route_when(data, route.clone(), &mut should_draw_line)
    })
}

fn art(opts: Opts) -> Vec<Group> {
    let width = 297.;
    let height = 210.;
    let t = 2. * PI * opts.index as f64 / (opts.frames as f64);

    let configs = vec![(
        "orange",
        ShapeConfig {
            resolution: 1000,
            initial_radius: 70.,
            initial_center: (width / 2. - 40. * t.cos(), height / 2. - 10.0),
            cycles: vec![
                Cycle {
                    count: 80,
                    vradius: -0.5,
                    vx: 0.5 * t.cos(),
                    vy: 0.3,
                },
                Cycle {
                    count: 60,
                    vradius: -0.4,
                    vx: -0.3 * t.cos(),
                    vy: -0.2 - 0.1 * t.sin(),
                },
            ],
            harmonies: vec![
                (
                    0.2,
                    8.0,
                ),
                (
                    0.1,
                    28.0,
                ),
            ],
            displacement: 0.3,
            disp_harmonies: vec![
                (
                    0.6,
                    0.008,
                ),
                (
                    0.3,
                    0.016,
                ),
            ],
            seed: opts.seed,
        },
    )];

    configs
        .iter()
        .enumerate()
        .map(|(i, (color, config))| {
            let data = shape(config.clone());
            let mut g = layer(color);
            g = g.add(base_path(color, 0.2, data));
            if i == configs.len() - 1 {
                g = g.add(signature(
                    1.0,
                    (230.0, 180.0),
                    color,
                ));
            }
            g
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "4")]
    index: usize,
    #[clap(short, long, default_value = "6")]
    frames: usize,
    #[clap(short, long, default_value = "1.0")]
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
