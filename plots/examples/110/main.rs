use clap::Clap;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let height = 210f64;
    let width = 297f64;
    let granularity = 5f64;
    let counts = [1200, 40];
    let max_count = 1500;
    let colors = vec!["gold", "firebrick"];
    let mut rng = rng_from_seed(opts.seed);
    let perlin = Perlin::new();

    let cx = width / 2.;
    let cy = height / 2.;
    let radius_from = 2.0;
    let radius_to = 90.0;

    let freq = 0.01;
    let amp = 2.;

    let candidates: Vec<Vec<(f64, f64)>> = (0..max_count)
        .map(|i| {
            let mut route = Vec::new();
            let radius =
                rng.gen_range(radius_from, radius_to);
            let angle_from = rng.gen_range(0.0, 2. * PI);
            let angle_to = angle_from
                + 2. * PI * rng.gen_range(0.1, 0.6);

            let mut angle = angle_from;
            loop {
                if angle > angle_to {
                    break;
                }

                let divergence = 0.0002;

                let x = cx + radius * angle.cos();
                let y = cy + radius * angle.sin();

                let r = radius
                    + amp
                        * (0.1
                            * perlin.get([
                                10. * freq * x,
                                10. * freq * y,
                                opts.seed
                                    + i as f64 * divergence,
                            ])
                            + 0.9
                                * perlin.get([
                                    freq * x,
                                    freq * y,
                                    opts.seed
                                        - i as f64
                                            * divergence,
                                ]));

                let x = cx + r * angle.cos();
                let y = cy + r * angle.sin();
                route.push((x, y));
                angle += granularity / (radius * 2. * PI);
            }
            route
        })
        .filter(|r| r.len() >= 2)
        .collect();

    colors
        .iter()
        .enumerate()
        .map(|(g, color)| {
            let count = counts[g];
            let mut routes = candidates.clone();
            rng.shuffle(&mut routes);
            routes.truncate(count);
            let data = routes.iter().fold(
                Data::new(),
                |data, route| {
                    render_route(data, route.clone())
                },
            );
            let mut l = layer(color);
            l = l.add(base_path(color, 0.3, data));
            if g == colors.len() - 1 {
                l = l.add(signature(
                    1.0,
                    (220.0, 180.0),
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
