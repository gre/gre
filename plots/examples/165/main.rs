use clap::Clap;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "8.0")]
    seed: f64,
    #[clap(short, long, default_value = "0.02")]
    k: f64,
    #[clap(short, long, default_value = "60")]
    count: usize,
    #[clap(short, long, default_value = "150")]
    steps: usize,
    #[clap(short, long, default_value = "0.3")]
    offset: f64,
}

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["black", "grey"];

    let mut rng = rng_from_seed(opts.seed);

    let (width, height) = (297., 210.);
    let precision = 0.2;
    let pad = 20.;
    let w = (width as f64 / precision) as u32;
    let h = (height as f64 / precision) as u32;
    let bounds = (
        pad,
        pad,
        width - pad,
        height - pad,
    );
    let count = opts.count;
    let k = opts.k;
    let steps = opts.steps;

    let entities: Vec<((f64,f64), f64)> =
      (0..count).map(|_i| {
        let x = rng.gen_range(-0.2, 1.2);
        let y = rng.gen_range(0.2, 0.8);
        ((x, y), 1.)
      }).collect();


    let ratio = width as f64 / (height as f64);
    let norm = |(x, y): (f64, f64)| (x*ratio, y);

    let f = |p| {
        let c = entities.iter().fold(100f64, |acc, e|
            f_op_union_round(
                acc,
                e.1 * euclidian_dist(norm(e.0), norm(p)),
                k
            )
        );
        c
    };

    colors
        .iter()
        .enumerate()
        .map(|(ci, color)| {
            let mut data = Data::new();
            let collider = |a, _b| {
                if !strictly_in_boundaries(a, bounds) {
                    return Some(a);
                }
                return None;
            };
            let thresholds = (0..steps).map(|i|
                (i as f64 + ci as f64 * opts.offset) / (steps as f64)
            ).collect();
            let features = contour(
                w,
                h,
                f,
                &thresholds
            );
            let mut routes = features_to_routes(features, precision);
            routes = crop_routes(&routes, bounds);

            for route in routes.clone() {
                data = render_route_collide(data, route, &collider);
            }

            data = render_route(data, boundaries_route(bounds));
            

            let mut l = layer(color);
            l = l.add(base_path(color, 0.35, data));
            if ci == colors.len() - 1 {
                l = l.add(signature(
                    1.0,
                    (250.0, 190.0),
                    color,
                ));
            }
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
