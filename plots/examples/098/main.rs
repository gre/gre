use clap::Clap;
use geo::prelude::EuclideanDistance;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let boundaries = (10.0, 10.0, 280.0, 200.0);
    let center = (
        boundaries.0 + (boundaries.2 - boundaries.0) * 0.5,
        boundaries.1 + (boundaries.3 - boundaries.1) * 0.5,
    );
    let spins = 8.;
    let radius = 80.;
    let lines = 500;
    let precision = 1.0;
    let length = 200;

    let colors = vec!["black", "green"];
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut counters_passages = vec![0; 280 * 200];
            let mut passage_mm = |p: (f64, f64)| {
                let i = (p.1 as usize).max(0).min(199)
                    * 280
                    + (p.0 as usize).max(0).min(279);
                let v = counters_passages[i] + 1;
                counters_passages[i] = v;
                v
            };

            let mut data = Data::new();
            let perlin = Perlin::new();
            // give the field angle (not the length)
            let field = |(x, y): (f64, f64), l: f64| {
                4.0 * perlin.get([
                    3.0 * x,
                    3.0 * y,
                    1.0 + opts.seed + i as f64 * 0.01,
                ]) + 1.0
                    * perlin.get([
                        8.0 * x,
                        8.0 * y,
                        2.0 + opts.seed + l * 0.05,
                    ])
            };

            let initial_positions = (0..lines)
                .map(|l| {
                    let p = (l as f64 + i as f64 * 0.5)
                        / (lines as f64);
                    let amp = radius * (p);
                    let a = spins * p * 2. * PI;
                    (
                        boundaries.0
                            + (boundaries.2 - boundaries.0)
                                * 0.5
                            + amp * a.cos(),
                        boundaries.1
                            + (boundaries.3 - boundaries.1)
                                * 0.5
                            + amp * a.sin(),
                    )
                })
                .collect();

            let mut last_angles: Vec<f64> = (0..lines)
                .map(|l| {
                    (spins
                        * ((l as f64 + i as f64 * 0.5)
                            / (lines as f64))
                        * 2.
                        * PI
                        + PI)
                        % (2. * PI)
                })
                .collect();

            let mut build_route =
                |p: (f64, f64), l, route_i| {
                    let normalized =
                        normalize_in_boundaries(
                            p, boundaries,
                        );

                    let mut angle = field(
                        normalized,
                        (l as f64) / (lines as f64),
                    );
                    let last_angle: f64 =
                        last_angles[route_i];
                    if (angle - last_angle).abs() > 0.5 * PI
                    {
                        angle += PI;
                    }
                    last_angles[route_i] = angle;
                    let next = (
                        p.0 + precision * angle.cos(),
                        p.1 + precision * angle.sin(),
                    );
                    let passage = passage_mm(next);
                    let ends = euclidian_dist(next, center)
                        > radius
                        || passage > 5
                        || i > length
                        || out_of_boundaries(
                            next, boundaries,
                        );
                    if ends {
                        None
                    } else {
                        Some((next, false))
                    }
                };

            let mut routes =
                build_routes_with_collision_par(
                    initial_positions,
                    &mut build_route,
                );

            let mut croute = Vec::new();
            for c in 0..1000 {
                let p = (c as f64) / 1000.;
                let a = p * 2. * PI;
                let amp = radius - i as f64 * 0.4;
                croute.push((
                    center.0 + amp * a.cos(),
                    center.1 + amp * a.sin(),
                ));
            }
            croute.push(croute[0]);
            routes.push(croute);

            for route in routes {
                data = render_route(data, route);
            }

            let mut l = layer(color);
            l = l.add(base_path(color, 0.2, data));
            if i == colors.len() - 1 {
                l = l.add(signature(
                    1.0,
                    (260.0, 190.0),
                    color,
                ));
            }
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
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
