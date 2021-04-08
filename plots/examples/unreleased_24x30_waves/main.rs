use clap::Clap;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let colors = vec!["green", "green"];
    let pad = 20.0;
    let width = 240.0;
    let height = 300.0;
    let bounds = (pad, pad, width - pad, height - pad);

    let line_length = 500.0;
    let granularity = 1.0;
    let samples = 500;

    let perlin = Perlin::new();
    let get_angle = |(x, y): (f64, f64),
                     _length,
                     i|
     -> f64 {
        let cx = (PI * x).sin(); // 1 on center and 0 on edges
        let dt = 0.2;
        let base = if i % 2 == 0 { -dt } else { PI + dt };
        let add =
            3. * perlin.get([
                x,
                y,
                100. * opts.seed + i as f64 / opts.divisor,
            ]) + 0.5
                * perlin.get([
                    3. * x,
                    3. * y,
                    opts.seed - 100.0,
                ])
                + 0.1
                    * perlin.get([
                        200. * x,
                        200. * y,
                        opts.seed - 100.0,
                    ]);

        base + add * (0.2 + opts.power * 0.8 * cx)
    };

    let initial_positions: Vec<(f64, f64)> = (0..samples)
        .map(|s| {
            let sp = s as f64 / (samples as f64);
            (
                if s % 2 == 0 {
                    pad + 0.1
                } else {
                    width - pad - 0.1
                },
                pad + (height - 2. * pad) * sp,
            )
        })
        .collect();

    let build_route = |p: (f64, f64), l, route_i| {
        let normalized = normalize_in_boundaries(p, bounds);
        let angle = get_angle(
            normalized,
            l as f64 * granularity,
            route_i,
        );
        let next = (
            p.0 + granularity * angle.cos(),
            p.1 + granularity * angle.sin(),
        );
        let ends = l as f64 / granularity > line_length;
        if let Some(c) =
            collide_segment_boundaries(p, next, bounds)
        {
            return Some((c, true));
        }
        if ends {
            None
        } else {
            Some((next, false))
        }
    };

    let mut routes = build_routes_with_collision_par(
        initial_positions.clone(),
        &build_route,
    );

    routes = routes
        .iter()
        .map(|route| round_route(route.clone(), 0.01))
        .collect();

    routes.push(boundaries_route(bounds));

    let mut groups = Vec::new();

    for (i, color) in colors.iter().enumerate() {
        let mut data = Data::new();
        for (j, route) in routes.iter().enumerate() {
            if j % colors.len() == i {
                data = render_route(data, route.clone());
            }
        }

        let mut g = layer(color);

        g = g.add(base_path(color, 0.2, data));

        if i == colors.len() - 1 {
            g = g.add(signature(1.0, (195.0, 280.0), color))
        }

        groups.push(g);
    }

    groups
}
#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "26.0")]
    seed: f64,
    #[clap(short, long, default_value = "30.0")]
    divisor: f64,
    #[clap(short, long, default_value = "1.0")]
    power: f64,
}
fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_24x30_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
