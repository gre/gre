use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn parametric(t: f64) -> (f64, f64) {
    return (
        0.6 * (2. * PI * t).cos()
            + 0.58 * (80. * PI * t).cos(),
        0.6 * (2. * PI * t).sin()
            - 0.58 * (80. * PI * t).sin(),
    );
}

fn art(_seed: f64) -> Vec<Group> {
    let colors = vec!["pink", "yellow"];
    let pad = 20.0;
    let width = 210.0;
    let height = 210.0;
    let size = 60.0;
    let bounds = (pad, pad, width - pad, height - pad);

    let line_length = 200.0;
    let granularity = 0.5;
    let samples = 6000;

    let get_angle = |_p, initial_angle, length| {
        initial_angle + 0.15 - length as f64 * 0.01
    };

    let samples_data: Vec<(f64, (f64, f64))> = (0..samples)
        .map(|i| {
            let sp = i as f64 / (samples as f64);
            let o = parametric(sp);
            let dt = 0.001;
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

    let mut routes =
    // lines
    build_routes_with_collision_par(
        initial_positions,
        &build_route,
    );

    // frame
    routes.push(boundaries_route(bounds));

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
            g = g.add(base_path(color, 0.35, data));
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
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
