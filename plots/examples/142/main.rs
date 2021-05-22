use clap::Clap;
use core::f64;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::{path::Data, Group};

fn art(opts: Opts) -> Vec<Group> {
    let stroke_width = 0.5;
    let w = 297.;
    let h = 210.;
    let colors =
        opts.colors.split(",").collect::<Vec<&str>>();

    colors
        .iter()
        .enumerate()
        .map(|(ci, color)| {
            let mut rng = rng_from_seed(opts.seed);
            let mut l = layer(color);
            let mut pts = (0..opts.points)
                .map(|i| {
                    let r = rng.gen_range(
                        opts.radius_from,
                        opts.radius_to,
                    );
                    let a = PI / 2.
                        + PI * (2. * PI * i as f64
                            / opts.points as f64)
                            .sin();
                    (
                        w / 2. + r * a.cos(),
                        h / 2.
                            + r * a.sin()
                            + opts.offset * ci as f64,
                    )
                })
                .collect::<Vec<(f64, f64)>>();
            rng.shuffle(&mut pts);
            let data = render_route_curve(Data::new(), pts);
            l = l.add(base_path(color, stroke_width, data));
            if ci == colors.len() - 1 {
                l = l.add(signature(
                    1.5,
                    (w - 100., h - 30.),
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
    #[clap(short, long, default_value = "400")]
    points: usize,
    #[clap(short, long, default_value = "0.")]
    seed: f64,
    #[clap(short, long, default_value = "50.")]
    radius_from: f64,
    #[clap(short, long, default_value = "90.")]
    radius_to: f64,
    #[clap(short, long, default_value = "5.")]
    offset: f64,
    #[clap(short, long, default_value = "red,green")]
    colors: String,
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
