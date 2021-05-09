use clap::Clap;
use geo::algorithm::centroid::Centroid;
use geo::algorithm::map_coords::MapCoords;
use geo::prelude::{BoundingRect, Contains};
use gre::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
use soy::{Bezier, Lerper};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Clap)]
#[clap()]
struct Opts {
    // value from 0.0 to 1.0 of the loop frame
    #[clap(short, long, default_value = "0.0625")]
    f: f64,
    // rotate on different plotting techniques
    #[clap(short, long, default_value = "0")]
    technique: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a5_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}

fn art(opts: Opts) -> Vec<Group> {
    let stroke_width = 0.3;
    let colors = vec!["hotpink", "deepskyblue"];
    let mut rng = rng_from_seed(0.0);
    let w = 210.;
    let size = 120.0;
    let project = |p: (f64, f64)| {
        ((w - size) / 2. + p.0 * size, 10.0 + p.1 * size)
    };
    let project_poly = |p: geo::Polygon<f64>| {
        p.map_coords(|&c| project(c))
    };

    colors
        .iter()
        .enumerate()
        .map(|(i, &color)| {
            let f = |p| jumping_blob(opts.f, p)[i];
            let mut l = layer(color);

            if i == opts.technique % 2 {
                let mut data = render_route(
                    Data::new(),
                    boundaries_route((0., 0., 1., 1.))
                        .iter()
                        .map(|&p| project(p))
                        .collect(),
                );
                let pattern = |data, p: (f64, f64)| {
                    let mut d = data;
                    d = render_route(
                        d,
                        vec![
                            (p.0 - 1., p.1),
                            (p.0 + 1., p.1),
                        ],
                    );
                    d = render_route(
                        d,
                        vec![
                            (p.0, p.1 - 1.),
                            (p.0, p.1 + 1.),
                        ],
                    );
                    d
                };
                let count = 30;
                for y in 0..count {
                    for x in 0..(if y % 2 == 1 {
                        count - 1
                    } else {
                        count
                    }) {
                        let p = (
                            (0.5 + x as f64
                                + (y as f64 % 2.) * 0.5)
                                / (count as f64),
                            (0.5 + y as f64)
                                / (count as f64),
                        );
                        let v = f(p);
                        if v == 0.0 {
                            data =
                                pattern(data, project(p));
                        }
                    }
                }
                l = l.add(base_path(
                    color,
                    stroke_width,
                    data,
                ));
                l = l.add(signature(
                    1.0,
                    project((0.78, 1.0)),
                    color,
                ));
            }
            match opts.technique % 8 {
                0 | 1 | 7 => {
                    // vector field
                    let candidates: Vec<(f64, f64)> =
                        sample_2d_candidates_f64(
                            &f, 400, 700, &mut rng,
                        );

                    let mut passage = Passage2DCounter::new(
                        0.005, 1.0, 1.0,
                    );

                    let perlin = Perlin::new();

                    let get_angle = |p: (f64, f64), i| {
                        let mut delta = 0.;
                        delta += PI
                            * (if i % 2 == 0 {
                                1.
                            } else {
                                0.
                            });
                        if opts.technique == 1 {
                            delta -= PI / 2.;
                        }
                        delta
                            + 4. * perlin.get([
                                0.8 * p.0,
                                0.8 * p.1,
                                1. + opts.technique as f64,
                            ])
                            + (0.8
                                + opts.technique as f64
                                    * 0.3)
                                * perlin.get([
                                    2. * p.0,
                                    2. * p.1,
                                    10.,
                                ])
                            + 0.1
                                * perlin.get([
                                    200. * p.0,
                                    200. * p.1,
                                    i as f64 / 100.0,
                                ])
                    };

                    let initial_positions =
                        candidates.clone();

                    let line_length = 1.0;
                    let granularity = 0.005;

                    let mut build_route = |p, i, j| {
                        let length = i as f64 * granularity;
                        if length >= line_length {
                            return None; // line ends
                        }
                        let angle = get_angle(p, j);
                        let nextp = follow_angle(
                            p,
                            angle,
                            granularity,
                        );
                        if let Some(edge_p) =
                            collide_segment_boundaries(
                                p,
                                nextp,
                                (0.0, 0.0, 1.0, 1.0),
                            )
                        {
                            return Some((edge_p, true));
                        }
                        if f(nextp)
                            < rng.gen_range(0., 0.002)
                        {
                            return None;
                        }
                        let count = passage.count(nextp);
                        if count > 2 {
                            return None; // too much passage here
                        }
                        return Some((nextp, false));
                    };

                    let routes = build_routes(
                        initial_positions,
                        &mut build_route,
                    );

                    let mut data = Data::new();
                    for route in routes {
                        if route.len() > 4 {
                            data = render_route(
                                data,
                                route
                                    .iter()
                                    .map(|&p| project(p))
                                    .collect(),
                            );
                        }
                    }

                    l = l.add(base_path(
                        color,
                        stroke_width,
                        data,
                    ));
                }
                2 => {
                    // circles
                    for c in sample_2d_candidates_f64(
                        &f, 500, 2000, &mut rng,
                    ) {
                        let (x, y) = project(c);
                        l = l.add(
                            Circle::new()
                                .set("cx", x)
                                .set("cy", y)
                                .set("r", 1.)
                                .set("stroke", color)
                                .set(
                                    "stroke-width",
                                    stroke_width,
                                )
                                .set("fill", "none")
                                .set("style", "mix-blend-mode: multiply;"),
                        );
                    }
                }
                3 => {
                    // spiral fill
                    let candidates =
                        sample_2d_candidates_f64(
                            &f, 400, 300, &mut rng,
                        );

                    let mut polys =
                        sample_square_voronoi_polys(
                            candidates, 0.0,
                        );
                    // filter out big polygons (by their "squared" bounds)
                    polys.retain(|poly| {
                        let centroid = poly.centroid();
                        if centroid.is_none() {
                            return false;
                        }
                        strictly_in_boundaries(
                            centroid.unwrap().x_y(),
                            (0., 0., 1., 1.),
                        ) && poly_bounding_square_edge(poly)
                            < 0.4
                    });

                    let mut data = Data::new();
                    for poly in polys {
                        let bounds =
                            poly.bounding_rect().unwrap();
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
                                        0.2 * f(ap)
                                    } else {
                                        0.0
                                    }
                                },
                                80,
                                36,
                                &mut rng,
                            );
                        candidates = candidates
                            .iter()
                            .map(|&p| project(map_p(p)))
                            .collect();
                        data = render_route(
                            data,
                            route_spiral(candidates),
                        );
                    }

                    l = l.add(base_path(
                        color,
                        stroke_width,
                        data,
                    ));
                }
                4 => {
                    // fuzzy sorting voronoi samples
                    let candidates =
                        sample_2d_candidates_f64(
                            &f, 800, 320, &mut rng,
                        );

                    let mut polys =
                        sample_square_voronoi_polys(
                            candidates, 0.0,
                        );
                    // filter out big polygons (by their "squared" bounds)
                    polys.retain(|poly| {
                        let centroid = poly.centroid();
                        if centroid.is_none() {
                            return false;
                        }
                        strictly_in_boundaries(
                            centroid.unwrap().x_y(),
                            (0., 0., 1., 1.),
                        ) && poly_bounding_square_edge(poly)
                            < 0.3
                    });

                    let mut data = Data::new();
                    for poly in polys {
                        let bounds =
                            poly.bounding_rect().unwrap();
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
                                        f(ap)
                                    } else {
                                        0.0
                                    }
                                },
                                160,
                                40,
                                &mut rng,
                            );
                        candidates = candidates
                            .iter()
                            .map(|&p| project(map_p(p)))
                            .collect();

                        let mul =
                            if i == 0 { 1. } else { -1. };

                        if candidates.len() > 5 {
                            candidates.sort_by(|&a, &b| {
                                (a.0 - mul * a.1)
                                    .partial_cmp(
                                        &(b.0 - mul * b.1),
                                    )
                                    .unwrap()
                                    .then(
                                        a.1.partial_cmp(
                                            &b.1,
                                        )
                                        .unwrap(),
                                    )
                            });
                            data = render_route(
                                data, candidates,
                            );
                        }
                    }

                    l = l.add(base_path(
                        color,
                        stroke_width,
                        data,
                    ));
                }
                5 => {
                    // voronoi
                    let candidates =
                        sample_2d_candidates_f64(
                            &f, 400, 2800, &mut rng,
                        );

                    let mut polys =
                        sample_square_voronoi_polys(
                            candidates, 0.0,
                        );
                    // filter out big polygons (by their "squared" bounds)
                    polys.retain(|poly| {
                        let centroid = poly.centroid();
                        if centroid.is_none() {
                            return false;
                        }
                        poly_bounding_square_edge(poly)
                            < 0.1
                            && strictly_in_boundaries(
                                centroid.unwrap().x_y(),
                                (0., 0., 1., 1.),
                            )
                            && poly
                                .exterior()
                                .points_iter()
                                .all(|p| f(p.x_y()) > 0.001)
                    });

                    let mut data = Data::new();
                    for poly in polys {
                        data = render_polygon_stroke(
                            data,
                            project_poly(poly),
                        );
                    }

                    l = l.add(base_path(
                        color,
                        stroke_width,
                        data,
                    ));
                }
                6 => {
                    // voronoi + tsp
                    let candidates =
                        sample_2d_candidates_f64(
                            &f, 400, 100, &mut rng,
                        );

                    let mut polys =
                        sample_square_voronoi_polys(
                            candidates, 0.0,
                        );
                    // filter out big polygons (by their "squared" bounds)
                    polys.retain(|poly| {
                        let centroid = poly.centroid();
                        if centroid.is_none() {
                            return false;
                        }
                        strictly_in_boundaries(
                            centroid.unwrap().x_y(),
                            (0., 0., 1., 1.),
                        ) && poly_bounding_square_edge(poly)
                            < 0.5
                    });

                    let clouds: Vec<Vec<(f64, f64)>> =
                        polys
                            .iter()
                            .map(|poly| {
                                let bounds = poly
                                    .bounding_rect()
                                    .unwrap();
                                let min = bounds.min();
                                let width = bounds.width();
                                let height =
                                    bounds.height();
                                let map_p = |(lx, ly)| {
                                    (
                                        min.x + width * lx,
                                        min.y + height * ly,
                                    )
                                };
                                sample_2d_candidates_f64(
                                    &|p| {
                                        let ap = map_p(p);
                                        if poly.contains(
                                        &geo::Point::new(
                                            ap.0, ap.1,
                                        ),
                                    ) {
                                        f(ap)
                                    } else {
                                        0.0
                                    }
                                    },
                                    120,
                                    100,
                                    &mut rng,
                                )
                                .iter()
                                .map(|&p| project(map_p(p)))
                                .collect()
                            })
                            .collect();

                    let routes: Vec<Vec<(f64, f64)>> =
                        clouds
                            .into_par_iter()
                            .map(|pts| {
                                let mut route = tsp(
                                    pts,
                                    time::Duration::seconds(
                                        2,
                                    ),
                                );
                                route.push(route[0]);
                                route
                            })
                            .collect();

                    let mut data = Data::new();
                    for route in routes {
                        data = render_route(data, route);
                    }

                    l = l.add(base_path(
                        color,
                        stroke_width,
                        data,
                    ));
                }
                _ => {}
            }
            l
        })
        .collect()
}
// adaptation of greweb.me/shaderday/67 in Rust
// value returns is an intensity in the two colors of the plot
// for the given p position and f frame %
fn jumping_blob(f: f64, o: (f64, f64)) -> Vec<f64> {
    let mut p = o;
    let bezier = Bezier::new(0.0, 0.1, 1.0, 0.9);
    let x = bezier.calculate(f as f32) as f64;
    let t = x * 2. * PI;
    let radius = 0.18;
    let smoothing = 0.15;
    let dist = 0.2;
    p.0 -= 0.5;
    p.1 -= 0.5;
    p.1 *= -1.0;
    p = p_r(p, PI / 2.0);
    let q = p;
    p = p_r(p, -t);
    let s = f_op_difference_round(
        f_op_union_round(
            q.0.max(0.1 + q.0),
            length((p.0 + dist, p.1)) - radius,
            smoothing,
        ),
        length((p.0 - dist, p.1)) - radius,
        smoothing,
    );
    let v = smoothstep(-0.6, 0.0, s).powf(2.0)
        * (if s < 0.0 { 1.0 } else { 0.0 });
    vec![
        v * (0.001 + smoothstep(-0.5, 1.5, p.0)),
        v * (0.001 + smoothstep(1.5, -0.5, p.0)),
    ]
}
fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
    (
        a.cos() * p.0 + a.sin() * p.1,
        a.cos() * p.1 - a.sin() * p.0,
    )
}
fn length(l: (f64, f64)) -> f64 {
    (l.0 * l.0 + l.1 * l.1).sqrt()
}
fn f_op_union_round(a: f64, b: f64, r: f64) -> f64 {
    r.max(a.min(b))
        - length(((r - a).max(0.), (r - b).max(0.)))
}
fn f_op_intersection_round(a: f64, b: f64, r: f64) -> f64 {
    (-r).min(a.max(b))
        + length(((r + a).max(0.), (r + b).max(0.)))
}
fn f_op_difference_round(a: f64, b: f64, r: f64) -> f64 {
    f_op_intersection_round(a, -b, r)
}
