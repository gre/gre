use std::f64::consts::PI;
use clap::Clap;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "14.0")]
    seed: f64,
}

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["turquoise", "pink", "orange"];
    let width = 297.0;
    let height = 210.0;
    let radius = 90.0;
    let bounds = (width / 2. - radius, height / 2. - radius, width / 2. + radius, height / 2. + radius);
    let stroke_width = 0.3;
    let initial_non_full = 40.0;
    let desired_count = 2000;
    let upper_limit = 100000;
    let pad = 0.4;
    let threshold_radius = 2.0;
    let spiral_da = 1.0;

    let mut rng = rng_from_seed(opts.seed);
    let mut circles = Vec::new();
    for i in 0..upper_limit {
        let x: f64 = rng.gen_range(bounds.0, bounds.2);
        let y: f64 = rng.gen_range(bounds.1, bounds.3);
        let mut r = (x - bounds.0).min(y - bounds.1).min(bounds.2 - x).min(bounds.3 - y);
        r *= mix(smoothstep(0.0, initial_non_full, i as f64), 1.0, rng.gen_range(0.0, 1.0));
        for &(x2, y2, r2) in circles.iter() {
            r = r.min(euclidian_dist((x,y), (x2,y2)) - r2);
        }
        r -= pad;
        if r > threshold_radius {
           circles.push((x, y, r));
        }
        if circles.len() > desired_count {
            break;
        }
    }

    colors
        .iter()
        .enumerate()
        .map(|(ci, &color)| {
            let mut l = layer(color);
            let mut data = Data::new();
            for (i, &c) in circles.iter().enumerate() {
                if ci == i % colors.len() {
                    data = render_route(data, spiral(c.0, c.1, c.2, spiral_da));
                }
            }


            if ci == colors.len() - 1 {
                data = render_route(data, boundaries_route(bounds));
                l = l.add(signature(
                    1.0,
                    (212.0, 194.0),
                    color,
                ));
            }
            l = l.add(base_path(color, stroke_width, data));
            l
        })
        .collect()
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
