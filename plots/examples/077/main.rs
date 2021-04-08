use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seed: f64) -> Vec<Group> {
    let pad = 20.;
    let width = 297.;
    let height = 210.;
    let boundaries = (pad, pad, width - pad, height - pad);
    let mut rng = SmallRng::from_seed([seed as u8; 16]);
    for _i in 0..50 {
        rng.gen::<f64>();
    }

    let ang = 6.;

    let total = rng.gen_range(100, 3000);

    let mut initial_positions: Vec<(f64, f64)> = (0..total)
        .map(|i| {
            round_point(
                (
                    rng.gen_range(
                        boundaries.0,
                        boundaries.2,
                    ),
                    rng.gen_range(
                        boundaries.1,
                        boundaries.3,
                    ),
                ),
                2.0,
            )
            /*
            let ii = (i as f64) / (total as f64);
            let a = i as f64 * golden_angle;
            let r = 100. * ii.powf(0.5);
            round_point(
                (
                    width / 2. + r * a.cos(),
                    height / 2. + r * a.sin(),
                ),
                0.1,
            )
            */
        })
        .collect();
    rng.shuffle(&mut initial_positions);

    let initial_angle: Vec<f64> = (0..total)
        .map(|_i| {
            rng.gen_range::<f64>(0., 6.0).floor() * PI / 3.0
                + rng.gen_range(-0.02, 0.02)
        })
        .collect();

    let initial_amp: Vec<f64> = (0..total)
        .map(|_i| {
            20.0 + 400.0
                * rng.gen_range::<f64>(0.0, 1.0).powf(2.0)
        })
        .collect();

    let build_route = |p: (f64, f64), i, route_i| {
        let a: f64 = initial_angle[route_i]
            + ((2. * PI) / ang) * (i as f64);
        let amp = initial_amp[route_i] - (i as f64 * 1.);
        let d = (amp * a.cos(), amp * a.sin());
        let next = (p.0 + d.0, p.1 + d.1);
        let ends = amp <= 0.0
            || out_of_boundaries(next, boundaries);
        if ends {
            return None;
        }
        if let Some(c) =
            collide_segment_boundaries(p, next, boundaries)
        {
            return Some((c, true));
        }
        Some((next, false))
    };

    let mut routes = build_routes_with_collision_seq(
        initial_positions.clone(),
        &build_route,
    );
    routes.push(boundaries_route(boundaries));

    let mut groups = Vec::new();
    for (i, color) in
        vec!["peru", "firebrick"].iter().enumerate()
    {
        let mut data = Data::new();
        for route in routes.iter() {
            if route.len() % 2 == i {
                data = render_route(data, route.clone());
            }
        }
        let mut l =
            layer(color).add(base_path(color, 0.2, data));
        if i == 1 {
            l = l.add(signature(
                1.0,
                (250.0, 190.0),
                color,
            ));
        }
        groups.push(l);
    }
    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(27.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
