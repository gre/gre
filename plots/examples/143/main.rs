use clap::Clap;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let seed = 0.248 + opts.seed * 0.37;
    let boundaries = (10.0, 10.0, 280.0, 200.0);
    let max_passage = 3;
    let passage_precision = 0.5;
    let lines = opts.lines;
    let precision = 1.0;
    let length = 300;
    let palette = vec![
        "deepskyblue",
        "teal",
        "navy",
        "indigo",
        "darkred",
        "crimson",
        "orangered",
        "peru",
    ];
    let len = palette.len();
    let colors = vec![
        &palette[opts.index % len],
        &palette[(opts.index + 1) % len],
    ];
    let p = opts.index as f64 / opts.frames as f64;
    let p1 = (p * PI * 2.).sin();
    let p2 = (p * PI * 2.).cos();

    let mut passage = Passage2DCounter::new(
        passage_precision,
        297.,
        210.,
    );

    let perlin = Perlin::new();
    let field = |(x, y): (f64, f64), l: f64| {
        // "domain warp" technique over perlin
        let (qx, qy) = (
            perlin.get([
                0.5 * x,
                0.5 * y,
                10. + seed + opts.delta1 * p2,
            ]),
            perlin.get([x + 2.7, y + 1.3, 20. + seed]),
        );
        let (rx, ry) = (
            perlin.get([
                x + 2. * qx + 0.13,
                y + 2. * qy + 0.31,
                30. + seed,
            ]),
            perlin.get([
                x + 2. * qx + 0.347,
                y + 2. * qy + 0.858,
                40. + seed,
            ]),
        );
        opts.amp
            * perlin.get([
                opts.freq * x + opts.rfreq * rx,
                opts.freq * y
                    + opts.rfreq * ry
                    + opts.delta2 * (x + p1),
                7.6 * seed + l,
            ])
    };

    let initial_positions = (0..lines)
        .map(|l| {
            let p = (l as f64) / (lines as f64);
            (
                boundaries.0
                    + (boundaries.2 - boundaries.0) * p,
                boundaries.1
                    + (boundaries.3 - boundaries.1)
                        * (0.5
                            + 0.3
                                * (PI * p).sin()
                                * (l as f64).cos()),
            )
        })
        .collect();

    let mut last_angles: Vec<f64> = (0..lines)
        .map(|l| if l < lines / 2 { 0.0 } else { PI })
        .collect();

    let mut build_route = |p: (f64, f64), l, route_i| {
        let normalized =
            normalize_in_boundaries(p, boundaries);

        let mut angle =
            field(normalized, (l as f64) / (lines as f64));
        let last_angle: f64 = last_angles[route_i];
        if (angle - last_angle).abs() > 0.5 * PI {
            angle += PI;
        }
        last_angles[route_i] = angle;
        let next = follow_angle(p, angle, precision);
        let b =
            collide_segment_boundaries(p, next, boundaries);
        if let Some(c) = b {
            return Some((c, true));
        }
        let ends = passage.count(next) > max_passage
            || l > length
            || out_of_boundaries(next, boundaries);
        if ends {
            None
        } else {
            Some((next, false))
        }
    };

    let routes = build_routes_with_collision_par(
        initial_positions,
        &mut build_route,
    );

    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let mut data = Data::new();
            for (r, route) in routes.iter().enumerate() {
                let secondary = r % 3 == 0;
                if secondary && i == 0 || i == 1 {
                    data =
                        render_route(data, route.clone());
                }
            }

            let mut l = layer(color);
            l = l.add(base_path(color, 0.3, data));
            if i == colors.len() - 1 {
                l = l.add(signature(
                    1.0,
                    (260.0, 190.0),
                    color,
                ));
            }
            l
        })
        .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "1600")]
    lines: usize,
    #[clap(short, long, default_value = "2.5")]
    amp: f64,
    #[clap(short, long, default_value = "3.2")]
    freq: f64,
    #[clap(short, long, default_value = "0.9")]
    rfreq: f64,
    #[clap(short, long, default_value = "0.3")]
    delta1: f64,
    #[clap(short, long, default_value = "0.5")]
    delta2: f64,
    #[clap(short, long, default_value = "27.")]
    seed: f64,
    #[clap(short, long, default_value = "4")]
    index: usize,
    #[clap(short, long, default_value = "8")]
    frames: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
