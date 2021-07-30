use std::f64::consts::PI;
use clap::Clap;
use geo::*;
use geo::prelude::*;
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::{Group, path::Data};

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "1.0")]
    seed: f64,
}

fn make_polygon(x: f64, y: f64, size: f64, angle: f64) -> Polygon<f64> {
    let count = 4;
    Polygon::new(
        LineString::from(
            (0..count)
            .map(|i| {
                let a = angle + 2. * PI * i as f64 / (count as f64);
                (x + size * a.cos(), y + size * a.sin())
            })
            .collect::<Vec<(f64, f64)>>()
        ),
        vec![]
    )
}

fn poly_collides_in_polys(polys: &Vec<Polygon<f64>>, poly: &Polygon<f64>) -> bool {
    polys.iter().any(|p| {
        poly.intersects(p)
    })
}

fn scaling_search<F: FnMut(f64) -> bool>(
    mut f: F,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let mut from = min_scale;
    let mut to = max_scale;
    loop {
        if !f(from) {
            return None;
        }
        if to - from < 0.1 {
            return Some(from);
        }
        let middle = (to + from) / 2.0;
        if !f(middle) {
            to = middle;
        }
        else {
            from = middle;
        }
    }
}

fn search(
    container: &Polygon<f64>,
    polys: &Vec<Polygon<f64>>,
    x: f64,
    y: f64,
    angle: f64,
    min_scale: f64,
    max_scale: f64,
) -> Option<f64> {
    let overlaps = |size| {
        let p = &make_polygon(x, y, size, angle);
        container.contains(p) &&
        !poly_collides_in_polys(polys, p)
    };
    scaling_search(overlaps, min_scale, max_scale)
}

fn packing<F: FnMut(usize) -> f64>(
    seed: f64,
    iterations: usize,
    desired_count: usize,
    optimize_size: usize,
    pad: f64,
    container: &Polygon<f64>,
    min_scale: f64,
    mut max_scale: F,
) -> Vec<Polygon<f64>> {
    let mut polys = Vec::new();
    let mut tries = Vec::new();
    let mut rng = rng_from_seed(seed);
    let bounds = container.bounding_rect().unwrap();
    let (x1, y1) = bounds.min().x_y();
    let x2 = x1 + bounds.width();
    let y2 = y1 + bounds.height();
    for _i in 0..iterations {
        let x: f64 = rng.gen_range(x1, x2);
        let y: f64 = rng.gen_range(y1, y2);
        let angle = rng.gen_range(0f64, 2. * PI);
        if let Some(size) = search(&container, &polys, x, y, angle, min_scale, max_scale(polys.len())) {
            tries.push((x, y, size - pad, angle));
            if tries.len() > optimize_size {
                tries.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
                let (x, y, s, a) = tries[0];
                let p = make_polygon(x, y, s, a);
                polys.push(p);
                tries = Vec::new();
            }
        }
        if polys.len() > desired_count {
            break;
        }
    }
    polys
}

fn art(opts: Opts) -> Vec<Group> {
    let width = 297.0;
    let height = 210.0;
    let pad = 20.0;
    let stroke_width = 0.35;

    let container = Polygon::new(
        LineString::from(vec![
            (pad, pad),
            (width-pad, pad),
            (width-pad, height-pad),
            (pad, height-pad),
        ]),
        vec![]
    );

    let routes = 
        packing(
            opts.seed,
            200000,
            4000,
            4,
            0.45,
            &container,
            0.6,
            |i| 60.0 / (1.0 + i as f64 * 0.5).min(4.0)
        )
        .par_iter()
        .map(|poly| {
            let bounds = poly.bounding_rect().unwrap();
            let (x1, y1) = bounds.min().x_y();
            let x2 = x1 + bounds.width();
            let y2 = y1 + bounds.height();
            let f = |p: (f64, f64)| {
                (x1 + p.0 * (x2 - x1), y1 + p.1 * (y2 - y1))
            };
            let mut rng = rng_from_seed(opts.seed + 7.77 * x1 + y1 / 3.);
            let mut candidates =
                sample_2d_candidates(
                    &|p| {
                        let q = f(p);
                        poly.intersects(&Point::from(q))
                    },
                    400,
                    8 + (0.6 * bounds.width() * bounds.height()) as usize,
                    &mut rng,
                );
    
            candidates = candidates
                .iter()
                .map(|&p| f(p))
                .collect();
    
            let mut spiral = route_spiral(candidates);
            if spiral.len() < 3 {
                return vec![];
            }

            spiral[0] = (
                (spiral[0].0 + spiral[1].0) / 2.,
                (spiral[0].1 + spiral[1].1) / 2.,
            );

            spiral
        })
        .collect::<Vec<_>>();

    let mut layers = Vec::new();

    let colors = vec!["steelblue", "brown"];
    for (ci, color) in colors.iter().enumerate() {
        let mut l = layer(color);
        if ci == 0 {
            l = l.add(signature(
                0.8,
                (255.0, 190.0),
                color,
            ));
        }
        let mut data = Data::new();
        for (i, route) in routes.iter().enumerate() {
            if i % colors.len() == ci {
                data = render_route_curve(data, route.clone());
            }
        }
        l = l.add(base_path(color, stroke_width, data));
        layers.push(l);
    }

    layers
    
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
