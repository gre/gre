use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn parametric(p: f64) -> (f64, f64) {
    (
        1.0 * (2. * PI * p).cos()
            + 0.2 * (8. * PI * p).cos(),
        1.0 * (2. * PI * p).sin()
            + 0.2 * (8. * PI * p).sin(),
    )
}

fn art(
    seed: f64,
    noise_f: f64,
    angular_speed: f64,
) -> Vec<Group> {
    let colors = vec!["gold", "royalblue"];
    let pad = 10.0;
    let width = 210.0;
    let height = 210.0;
    let size = 60.0;
    let bounds = (pad, pad, width - pad, height - pad);

    let line_length = 500.0;
    let granularity = 1.0;
    let samples = 1000;

    let perlin = Perlin::new();
    let get_initial_angle =
        |(x1, y1): (f64, f64), (x2, y2): (f64, f64)| {
            (y1 - y2).atan2(x1 - x2) - PI / 2.
                + 1. * perlin.get([
                    noise_f * x1 / width,
                    noise_f * y1 / height,
                    seed,
                ])
        };

    let get_angle = |(x, y), initial_angle, length| {
        initial_angle
            + angular_speed * length
            + 2. * perlin.get([
                noise_f * x / width,
                noise_f * y / height,
                seed,
            ])
    };

    let samples_data: Vec<(f64, (f64, f64))> = (0..samples)
        .map(|i| {
            let sp = i as f64 / (samples as f64);
            let o = parametric(sp);
            let dt = 0.0001;
            let o2 = parametric(sp + dt);
            let initial_angle = get_initial_angle(o, o2);
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
                    (140.0, 160.0),
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
    let noise_f = args
        .get(2)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(7.0);
    let angular_speed = args
        .get(3)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.03);
    let groups = art(seed, noise_f, angular_speed);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }

    /*
    // debug
    document = document.add(
        Text::new()
            .set("x", 20)
            .set("y", 250)
            .set("font-family", "serif")
            .set("text-anchor", "start")
            .set("font-size", "6")
            .add(svg::node::Text::new(format!(
                "{} {} {}",
                seed, noise_f, angular_speed
            ))),
    );
    */

    svg::save("image.svg", &document).unwrap();
}
