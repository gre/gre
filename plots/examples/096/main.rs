use clap::Clap;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 297.;
    let height = 210.;
    let pad = 10.;
    let bounds = (pad, pad, width - pad, height - pad);
    let mut rng = rng_from_seed(opts.seed);
    let colors = vec!["white", "gold"];
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();

            let amp_x_0 = rng.gen_range(0.1, 1.0);
            let amp_x_1 = rng.gen_range(0.1, 1.0);
            let freq_x_1 = rng.gen::<usize>() % 200;
            let amp_y_0 = rng.gen_range(0.1, 1.0);
            let amp_y_1 = rng.gen_range(0.1, 1.0);
            let freq_y_1 =
                freq_x_1 + ((rng.gen::<usize>() % 10) - 5);

            let scale = 0.3;

            let parametric = |t: f64| {
                (
                    amp_x_0 * (2. * PI * t).cos()
                        + amp_x_1
                            * ((freq_x_1 as f64)
                                * 2.
                                * PI
                                * t)
                                .cos(),
                    amp_y_0 * (2. * PI * t).sin()
                        + amp_y_1
                            * ((freq_y_1 as f64)
                                * 2.
                                * PI
                                * t)
                                .sin(),
                )
            };

            let samples = 10000;
            let mut route: Vec<(f64, f64)> = (0..samples)
                .map(|i| {
                    let mut p = parametric(
                        i as f64 / (samples as f64),
                    );
                    p.0 = scale * p.0 + 0.5;
                    p.1 = scale * p.1 + 0.5;
                    project_in_boundaries(p, bounds)
                })
                .collect();

            route.push(route[0]);

            data = render_route(data, route);

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
