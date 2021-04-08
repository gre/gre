use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seconds: i64) -> Vec<Group> {
    let radius = 0.3;
    let ring_w = 0.2;
    let samples_count = 1200;
    let kmeans_clusters = 10;

    let mut groups = Vec::new();

    let colors = vec!["lightskyblue", "gold", "aquamarine"];

    let bounds = (50.0, 10.0, 250.0, 200.0);

    let routes: Vec<(&str, Vec<(f64, f64)>)> = colors
        .par_iter()
        .enumerate()
        .flat_map(|(i, &color)| {
            let mut rng = thread_rng();

            let a =
                i as f64 * 2. * PI / (colors.len() as f64);
            let amp = 0.5;
            let center =
                (0.5 + amp * a.cos(), 0.5 + amp * a.sin());

            let sampler = |p| {
                smoothstep(
                    0.8,
                    0.2,
                    euclidian_dist(p, center),
                ) * smoothstep(
                    ring_w,
                    0.0,
                    (euclidian_dist(p, (0.5, 0.5))
                        - radius)
                        .abs(),
                )
            };

            let samples = sample_2d_candidates_f64(
                &sampler,
                200,
                samples_count,
                &mut rng,
            );

            let groups: Vec<(&str, Vec<(f64, f64)>)> =
                group_with_kmeans(samples, kmeans_clusters)
                    .par_iter()
                    .map(|pts| {
                        let mut route = tsp(
                            pts.iter()
                                .map(|&p| {
                                    project_in_boundaries(
                                        p, bounds,
                                    )
                                })
                                .collect(),
                            time::Duration::seconds(
                                seconds,
                            ),
                        );

                        route.push(route[0]);

                        (color, route)
                    })
                    .collect();

            groups
        })
        .collect();

    for (color, route) in routes {
        let mut data = Data::new();
        data = render_route_curve(data, route);
        groups.push(
            layer(color).add(base_path(color, 0.5, data)),
        );
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seconds = args
        .get(1)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(20);
    let groups = art(seconds);
    let mut document = base_a4_landscape("black");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (220.0, 170.0),
        "white",
    ));
    svg::save("image.svg", &document).unwrap();
}
