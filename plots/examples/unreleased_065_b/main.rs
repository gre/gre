use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn parametric(p: f64) -> (f64, f64) {
    (
        0.8 * (8. * PI * p).cos()
            + 0.1 * (4. * PI * p).sin()
            + 0.4 * (6. * PI * p).cos(),
        0.8 * (8. * PI * p).sin()
            + 0.2 * (6. * PI * p).cos()
            + 0.5 * (4. * PI * p).sin(),
    )
}

fn art(seed: f64) -> Vec<Group> {
    let colors = vec!["hotpink"];
    let pad = 10.0;
    let width = 210.0;
    let height = 297.0;
    let size = 60.0;
    let bounds = (pad, pad, width - pad, height - pad);

    let line_length = 100.0;
    let granularity = 0.5;
    let samples = 1000;

    let perlin = Perlin::new();
    let get_angle = |(x, y), initial_angle, length| {
        initial_angle
            + (2.
                + 20.
                    * (0.8
                        - euclidian_dist(
                            (x, y),
                            (width / 2., height / 2.),
                        ) / width)
                        .max(0.0)
                + (3.
                    * perlin.get([
                        0.02 * x,
                        0.02 * y,
                        seed,
                    ])
                    + 1. * perlin.get([
                        0.08 * x,
                        0.08 * y,
                        seed,
                    ])))
                * (0.1 + length / line_length)
    };

    let samples_data: Vec<(f64, (f64, f64))> = (0..samples)
        .map(|i| {
            let sp = i as f64 / (samples as f64);
            let o = parametric(sp);
            let dt = 0.0001;
            let o2 = parametric(sp + dt);
            let initial_angle =
                (o.1 - o2.1).atan2(o.0 - o2.0);
            let p = (
                width * 0.5 + size * o.0,
                height * 0.5 + size * o.1,
            );
            (initial_angle, p)
        })
        .collect();

    let initial_positions =
        samples_data.iter().map(|&(_a, p)| p).collect();

    let build_route = |p, i, j| {
        let length = i as f64 * granularity;
        if length >= line_length {
            return None; // line ends
        }
        let (initial_angle, _o) = samples_data[j];
        let angle = get_angle(p, initial_angle, length);
        let nextp = follow_angle(p, angle, granularity);
        if let Some(edge_p) =
            collide_segment_boundaries(p, nextp, bounds)
        {
            return Some((edge_p, true));
        }
        return Some((nextp, false));
    };

    let routes = build_routes_with_collision_par(
        initial_positions,
        &build_route,
    );

    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let data = routes
                .iter()
                .enumerate()
                .filter(|(j, _route)| j % colors.len() == i)
                .fold(Data::new(), |data, (_j, route)| {
                    render_route(data, route.clone())
                });

            let mut g = layer(color);
            g = g.add(base_path(color, 0.3, data));
            if i == colors.len() - 1 {
                g = g.add(signature(
                    1.0,
                    (170.0, 230.0),
                    color,
                ))
            }
            return g;
        })
        .collect()
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_portrait("black");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
