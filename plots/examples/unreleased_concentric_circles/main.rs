use clap::Clap;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let width = 297.;
    let height = 210.;
    let color = "darkgreen";
    let perlin = Perlin::new();

    let mut routes = Vec::new();

    let data = vec![
        (500, 50, 80., 0.5, (-0.45, 0.), (0., 0.)),
        (500, 25, 5., -2.0, (-1.9, 0.), (26., 0.)),
    ];

    for (
        r_split,
        count,
        initial_amp,
        r_off,
        delta,
        offset,
    ) in data
    {
        for i in 0..count {
            let mut route: Vec<(f64, f64)> = (0..r_split)
                .map(|j| {
                    let ang = j as f64 * 2. * PI
                        / (r_split as f64);
                    let o =
                        0.01 * perlin.get([
                            opts.seed,
                            20. * ang / (2. * PI),
                        ]) + 0.02
                            * perlin.get([
                                1.0 + opts.seed,
                                5. * ang / (2. * PI),
                            ]);
                    let amp = (1. + o)
                        * (initial_amp
                            - r_off * (i as f64));
                    (
                        width / 2.
                            + amp * ang.cos()
                            + delta.0 * (i as f64)
                            + offset.0,
                        height / 2.
                            + amp * ang.sin()
                            + delta.1 * (i as f64)
                            + offset.1,
                    )
                })
                .collect();
            route.push(route[0]);
            routes.push(route);
        }
    }

    let data =
        routes.iter().fold(Data::new(), |data, route| {
            render_route(data, route.clone())
        });

    let mut g = layer(color);
    g = g.add(base_path(color, 0.2, data));
    g = g.add(signature(1.0, (230.0, 180.0), color));

    vec![g]
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
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
