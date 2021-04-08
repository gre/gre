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

    let perlin = Perlin::new();
    let mut rng = SmallRng::from_seed([seed as u8; 16]);
    for _i in 0..50 {
        rng.gen::<f64>();
    }

    let f = rng.gen_range(1., 6.);
    let ang = rng.gen_range(3., 5.);
    let dist = rng.gen_range(5., 15.);
    let amp = rng.gen_range(5., 15.);

    let total = rng.gen_range(500, 2000);
    let golden_angle = PI * (3.0 - (5.0 as f64).sqrt());

    let initial_positions: Vec<(f64, f64)> = (0..total)
        .map(|i| {
            let ii = (i as f64) / (total as f64);
            let a = i as f64 * golden_angle;
            let r = 100. * ii.powf(0.5);
            (
                width / 2. + r * a.cos(),
                height / 2. + r * a.sin(),
            )
        })
        .collect();

    let build_route = |p: (f64, f64), i, route_i| {
        let px = p.0 / width;
        let py = p.1 / height;
        let a = ((2. * PI) / ang)
            * (i as f64 / 10.
                + (2.
                    * perlin.get([
                        0.5 * f * px,
                        0.5 * f * py,
                        -(route_i as f64),
                    ]))
                .powf(3.))
            .round();
        let dx =
            amp * perlin.get([f * px, f * px, seed + 20.]);
        let dy =
            amp * perlin.get([f * py, f * py, seed + 40.]);
        let d = (dist * a.cos() + dx, dist * a.sin() + dy);
        let next = (p.0 + d.0, p.1 + d.1);
        let ends =
            i > 100 || out_of_boundaries(next, boundaries);
        if ends {
            None
        } else {
            Some((next, false))
        }
    };

    let routes = build_routes_with_collision_seq(
        initial_positions,
        &build_route,
    );

    let mut groups = Vec::new();
    let mut data = Data::new();
    for route in routes {
        if route.len() < 3 {
            continue;
        }
        data = render_route_curve(data, route);
    }
    let color = "firebrick";
    groups.push(
        layer(color)
            .add(base_path(color, 0.2, data))
            .add(signature(1.0, (250.0, 180.0), color)),
    );
    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(127.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
