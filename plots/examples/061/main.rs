use geo::*;
use gre::*;
use noise::*;
use prelude::{BoundingRect, Centroid};
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::Group;
use time::Duration;
use voronoi::*;

fn art(
    seed: f64,
    pad: f64,
    samples: usize,
    poly_fill_samples: i64,
    poly_threshold: f64,
    f1: f64,
    f2: f64,
) -> Vec<Group> {
    let width = 297.;
    let height = 210.;
    let color_group_freq = 3.0;
    let colors =
        vec!["deepskyblue", "aquamarine", "hotpink"];

    let perlin = Perlin::new();
    let mut rng = SmallRng::from_seed([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    // sample a bunch of points based on perlin
    let candidates = sample_2d_candidates(
        &|(x, y)| {
            lt(0.0, perlin.get([x * f1, y * f1, seed]))
                * perlin.get([x * f2, y * f2, seed + 1.0])
                > 0.2
        },
        400,
        samples,
        &mut rng,
    );

    let mut groups = Vec::new();

    if candidates.len() < 5 {
        return groups;
    }

    let mut points = Vec::new();
    for c in candidates {
        points.push(voronoi::Point::new(
            pad + (1.0 - 2.0 * pad) * c.0,
            pad + (1.0 - 2.0 * pad) * c.1,
        ));
    }
    let dcel = voronoi(points, 1.0);
    let mut polys = make_polygons(&dcel);
    // filter out big polygons (by their "squared" bounds)
    polys.retain(|pts| {
        let poly = Polygon::new(
            pts.iter()
                .map(|p| (p.x(), p.y()))
                .collect::<Vec<_>>()
                .into(),
            vec![],
        );
        let bounds = poly.bounding_rect().unwrap();
        bounds.width().max(bounds.height()) < poly_threshold
    });

    let len = colors.len();
    for (group, color) in colors.iter().enumerate() {
        let mut data = Data::new();
        for poly in polys.clone() {
            let pts: Vec<(f64, f64)> = poly
                .iter()
                .map(|&p| {
                    (
                        (width + 10.) * p.x() - 5.,
                        (height + 10.) * p.y() - 5.,
                    )
                })
                .collect();
            let abs_poly = Polygon::new(pts.into(), vec![]);
            let center = abs_poly.centroid().unwrap();
            let v = smoothstep(
                -0.4,
                0.6,
                perlin.get([
                    color_group_freq * center.x() / width,
                    color_group_freq * center.y() / height,
                    seed,
                ]),
            );
            let selected_group =
                (v * (len as f64)) as usize;
            if selected_group != group {
                continue;
            }

            let abs_pfs = poly_fill_samples.abs() as usize;
            if abs_pfs < 2 {
                data =
                    render_polygon_stroke(data, abs_poly);
            } else if poly_fill_samples < 0 {
                data = render_polygon_fill_spiral(
                    data, abs_poly, abs_pfs, &mut rng,
                );
            } else {
                data = render_polygon_fill_tsp(
                    data,
                    abs_poly,
                    abs_pfs,
                    &mut rng,
                    Duration::seconds(1),
                );
            }
        }

        groups.push(
            layer(color).add(base_path(color, 0.4, data)),
        );
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(46.0);
    let pad = args
        .get(2)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.15);
    let samples = args
        .get(3)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(200);
    let poly_fill_samples = args
        .get(4)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(-120);
    let poly_threshold = args
        .get(5)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.1);
    let f1 = args
        .get(6)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(3.);
    let f2 = args
        .get(7)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(24.);

    let groups = art(
        seed,
        pad,
        samples,
        poly_fill_samples,
        poly_threshold,
        f1,
        f2,
    );
    let mut document = base_a4_landscape("black");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "grey",
    ));
    svg::save("image.svg", &document).unwrap();
}

fn lt(x: f64, a: f64) -> f64 {
    if x < a {
        1.0
    } else {
        0.0
    }
}
