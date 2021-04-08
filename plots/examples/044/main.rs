use geo::algorithm::area::Area;
use geo::*;
use gre::*;
use noise::*;
use prelude::BoundingRect;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::Group;
use voronoi::{make_polygons, voronoi, Point};

fn lt(x: f64, a: f64) -> f64 {
    if x < a {
        1.0
    } else {
        0.0
    }
}

fn art(
    seed: f64,
    pad: f64,
    samples: usize,
    poly_threshold: f64,
    f1: f64,
    f2: f64,
    group_sampling: f64,
) -> Vec<Group> {
    let x_off = 1.0;
    let colors = vec!["blue", "red"];

    let mut groups = Vec::new();

    let perlin = Perlin::new();
    let mut rng = SmallRng::from_seed([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let f = |(x, y)| {
        lt(0.0, perlin.get([x * f1, y * f1, seed]))
            * perlin.get([x * f2, y * f2, seed + 1.0])
            > 0.0
    };

    let mut candidates = Vec::new();
    let dim = 400;
    for x in 0..dim {
        for y in 0..dim {
            let p = (
                (x as f64) / (dim as f64),
                (y as f64) / (dim as f64),
            );
            if f(p) {
                candidates.push(p);
            }
        }
    }

    rng.shuffle(&mut candidates);
    candidates.truncate(samples);

    if candidates.len() < 5 {
        return groups;
    }

    for (i, color) in colors.iter().enumerate() {
        let mut pts = candidates.clone();
        rng.shuffle(&mut pts);
        pts.truncate(
            ((samples as f64) * (group_sampling)) as usize,
        );

        let mut points = Vec::new();
        for c in pts {
            points.push(Point::new(
                pad + (1.0 - 2.0 * pad) * c.0,
                pad + (1.0 - 2.0 * pad) * c.1,
            ));
        }
        let dcel = voronoi(points, 1.0);
        let mut polys = make_polygons(&dcel);
        polys.retain(|pts| {
            let poly = Polygon::new(
                pts.iter()
                    .map(|p| (p.x(), p.y()))
                    .collect::<Vec<_>>()
                    .into(),
                vec![],
            );
            let bounds = poly.bounding_rect().unwrap();
            bounds.width().max(bounds.height())
                < poly_threshold
        });

        // rendering
        let mut data = Data::new();
        for poly in polys {
            let mut first = true;
            for point in poly {
                let p = (
                    310.0 * point.x() - 10.0
                        + x_off
                            * (i as f64
                                - (colors.len() as f64)
                                    / 2.0),
                    230.0 * point.y() - 10.0,
                );
                if first {
                    first = false;
                    data = data.move_to(p);
                } else {
                    data = data.line_to(p);
                }
            }
            data = data.close();
        }

        groups.push(
            layer(color).add(base_path(color, 0.6, data)),
        );
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let pad = args
        .get(2)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.15);
    let samples = args
        .get(3)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(800);
    let poly_threshold = args
        .get(4)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.04);
    let f1 = args
        .get(5)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(4.);
    let f2 = args
        .get(6)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(20.);
    let group_sampling = args
        .get(7)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.9);

    let groups = art(
        seed,
        pad,
        samples,
        poly_threshold,
        f1,
        f2,
        group_sampling,
    );
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
