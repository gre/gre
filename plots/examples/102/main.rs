use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Copy, Clone)]
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
    harmonies_mul: f64,
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
        harmonies_mul,
        displacement,
        disp_harmonies,
        seed,
    }: ShapeConfig,
) -> Data {
    let perlin = Perlin::new();

    let amp_multiplier = |a: f64| -> f64 {
        1. + harmonies_mul
            * harmonies
                .iter()
                .enumerate()
                .map(|(h, &(amp, f))| {
                    amp * perlin
                        .get([seed + h as f64, f * a])
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
                        100. + seed + h as f64,
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
        render_route(data, route.clone())
    })
}

fn art(opts: Opts) -> Vec<Group> {
    let height = 210.;

    let mut rng = rng_from_seed(opts.seed);

    let cycles = vec![
        Cycle {
            count: rng.gen_range(20, 40),
            vradius: -0.6,
            vx: rng.gen_range(-0.5, 0.5),
            vy: 0.0,
        },
        Cycle {
            count: rng.gen_range(0, 30),
            vradius: rng.gen_range(-2., -0.8),
            vx: rng.gen_range(-1., 1.),
            vy: 0.0,
        },
    ];
    let harmonies = vec![
        (
            rng.gen_range(0.0, 0.05),
            rng.gen_range(2.0, 20.0f64).floor(),
        ),
        (
            rng.gen_range(0.0, 0.1),
            rng.gen_range(0.5, 8.0f64).floor(),
        ),
    ];
    let disp_harmonies = vec![
        (rng.gen_range(0.0, 0.4), rng.gen_range(0.0, 0.01)),
        (
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.005, 0.02),
        ),
    ];
    let resolution = 600;
    let initial_radius = 40.;

    let configs = vec![
        (
            "gold",
            ShapeConfig {
                resolution,
                initial_radius,
                initial_center: (50., height / 2.),
                cycles: cycles.clone(),
                harmonies: harmonies.clone(),
                harmonies_mul: 0.5,
                displacement: 0.1,
                disp_harmonies: disp_harmonies.clone(),
                seed: opts.seed,
            },
        ),
        (
            "silver",
            ShapeConfig {
                resolution,
                initial_radius,
                initial_center: (145., height / 2.),
                cycles: cycles.clone(),
                harmonies: harmonies.clone(),
                harmonies_mul: 1.,
                displacement: 0.1,
                disp_harmonies: disp_harmonies.clone(),
                seed: opts.seed,
            },
        ),
        (
            "white",
            ShapeConfig {
                resolution,
                initial_radius,
                initial_center: (240., height / 2.),
                cycles: cycles.clone(),
                harmonies: harmonies.clone(),
                harmonies_mul: 2.,
                displacement: 0.1,
                disp_harmonies: disp_harmonies.clone(),
                seed: opts.seed,
            },
        ),
    ];

    configs
        .iter()
        .enumerate()
        .map(|(i, (color, config))| {
            let data = shape(config.clone());
            let mut g = layer(color);
            g = g.add(base_path(color, 0.4, data));
            if i == configs.len() - 1 {
                g = g.add(signature(
                    1.0,
                    (250.0, 160.0),
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
    #[clap(short, long, default_value = "10.0")]
    seed: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_landscape("black");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
