use geo::prelude::{BoundingRect, Contains};
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seed: u8, path: &str) -> Vec<Group> {
    let voronoi_size = 2000;
    let max_samples = 60;
    let samples_r = 0.01;
    let res = 80;
    let pad = 10.;
    let width = 260.;
    let height = 190.;
    let poly_threshold = 0.5;

    let project = |(x, y): (f64, f64)| {
        (pad + x * width, pad + y * height)
    };

    let mut rng = SmallRng::from_seed([
        seed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let mut data = Data::new();

    for i in 0..4 {
        let candidates = sample_2d_candidates_f64(
            &|p| 1. - 1.5 * euclidian_dist(p, (0.5, 0.5)),
            800,
            (voronoi_size as f64 * rng.gen_range(0.1, 1.2))
                as usize,
            &mut rng,
        );

        let mut polys =
            sample_square_voronoi_polys(candidates, 0.1);

        // filter out big polygons (by their "squared" bounds)
        polys.retain(|poly| {
            poly_bounding_square_edge(poly) < poly_threshold
        });

        let get_color = image_get_color(path).unwrap();

        let pow = rng.gen_range(1.0, 3.0) as f64;
        let get = |p| {
            0.02 + smoothstep(
                1.0,
                0.0,
                grayscale(get_color(p)),
            )
            .powf(pow)
        };

        let routes: Vec<Vec<(f64, f64)>> = polys
            .par_iter()
            .map(|poly| {
                let mut rng = SmallRng::from_seed([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0,
                ]);
                let bounds = poly.bounding_rect().unwrap();
                let min = bounds.min();
                let width = bounds.width();
                let height = bounds.height();
                let map_p = |(lx, ly)| {
                    (
                        min.x + width * lx,
                        min.y + height * ly,
                    )
                };
                let mut candidates =
                    sample_2d_candidates_f64(
                        &|p| {
                            let ap = map_p(p);
                            if poly.contains(
                                &geo::Point::new(
                                    ap.0, ap.1,
                                ),
                            ) {
                                samples_r * get(ap)
                            } else {
                                0.0
                            }
                        },
                        res,
                        max_samples,
                        &mut rng,
                    );
                candidates = candidates
                    .iter()
                    .map(|&p| project(map_p(p)))
                    .collect();
                if candidates.len() < 6 {
                    vec![]
                } else {
                    if i % 4 == 0 {
                        candidates =
                            route_spiral(candidates);
                    } else if i % 4 == 1 {
                        candidates.sort_by(|&a, &b| {
                            (a.0)
                                .partial_cmp(&(b.0))
                                .unwrap()
                                .then(
                                    a.1.partial_cmp(&b.1)
                                        .unwrap(),
                                )
                        });
                    } else if i % 4 == 2 {
                        candidates.sort_by(|&a, &b| {
                            (a.1)
                                .partial_cmp(&(b.1))
                                .unwrap()
                                .then(
                                    a.0.partial_cmp(&b.0)
                                        .unwrap(),
                                )
                        });
                    }

                    candidates
                }
            })
            .collect();

        let should_draw_line =
            |a: (f64, f64), b: (f64, f64)| {
                let dx = a.0 - b.0;
                let dy = a.1 - b.1;
                (dx * dx + dy * dy).sqrt() < 20.0
            };

        for route in routes {
            data = render_route_when(
                data,
                route,
                &should_draw_line,
            );
        }
    }

    vec![Group::new().add(
        layer("black").add(base_path("black", 0.2, data)),
    )]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(1);
    let path = args.get(2).unwrap();
    let groups = art(seed, path);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (160.0, 280.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
