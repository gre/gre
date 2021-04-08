use geo::prelude::{Area, BoundingRect, Contains};
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seconds: i64) -> Vec<Group> {
    let voronoi_size = 60;
    let samples = 80;
    let res = 40;
    let width = 210.;
    let height = 210.;
    let poly_threshold = 0.5;

    let project =
        |(x, y): (f64, f64)| (x * width, y * height + 40.0);

    let mut rng = SmallRng::from_seed([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let candidates = sample_2d_candidates(
        &|_p| true,
        1000,
        voronoi_size,
        &mut rng,
    );

    let mut polys =
        sample_square_voronoi_polys(candidates, 0.1);

    // filter out big polygons (by their "squared" bounds)
    polys.retain(|poly| {
        poly_bounding_square_edge(poly) < poly_threshold
    });

    let get_color =
        image_get_color("images/ai.png").unwrap();

    let get = |p| 1. - grayscale(get_color(p));

    let mut data = Data::new();

    let routes: Vec<Vec<(f64, f64)>> = polys
        .par_iter()
        .map(|poly| {
            let mut rng = SmallRng::from_seed([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0,
            ]);
            let bounds = poly.bounding_rect().unwrap();
            let min = bounds.min();
            let width = bounds.width();
            let height = bounds.height();
            let map_p = |(lx, ly)| {
                (min.x + width * lx, min.y + height * ly)
            };
            let mut candidates = sample_2d_candidates_f64(
                &|p| {
                    let ap = map_p(p);
                    if poly.contains(&geo::Point::new(
                        ap.0, ap.1,
                    )) {
                        get(ap)
                    } else {
                        0.0
                    }
                },
                res,
                samples,
                &mut rng,
            );
            candidates = candidates
                .iter()
                .map(|&p| project(map_p(p)))
                .collect();
            if candidates.len() < 5 {
                vec![]
            } else {
                tsp(
                    candidates,
                    time::Duration::seconds(seconds),
                )
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

    vec![Group::new().add(
        layer("hotpink")
            .add(base_path("hotpink", 0.4, data)),
    )]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seconds = args
        .get(1)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(1);
    let groups = art(seconds);
    let mut document = base_a4_portrait("black");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (170.0, 260.0),
        "hotpink",
    ));
    svg::save("image.svg", &document).unwrap();
}
