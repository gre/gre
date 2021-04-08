use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(seed0: u8) -> Vec<Group> {
    let mut groups = Vec::new();

    let perlin = Perlin::new();

    let mut rng = SmallRng::from_seed([seed0; 16]);
    for _i in 0..50 {
        rng.gen::<f64>();
    }
    let seed_f = rng.gen::<f64>() % 37.0;

    let field =
        |(x, y): (f64, f64), l: usize, s: usize| -> f64 {
            2. * PI * smoothstep(0.0, 8.0, l as f64)
                + 1. * perlin.get([
                    0.05 * x,
                    0.05 * y,
                    seed_f + (s as f64 * 0.001),
                ])
                + 2. * perlin.get([
                    0.02 * x,
                    0.02 * y,
                    1. + seed_f,
                ])
        };

    let samples_count = 500;
    let line_length = 80.0;
    let precision = 1.0;
    let c_radius = 0.3;
    let c_offset = 0.2;

    let boundaries = (50., 10., 250., 200.);
    let plot_boundaries = (10., 10., 277., 200.);

    let sample_f = |p| {
        smoothstep(
            c_radius,
            0.0,
            euclidian_dist(p, (0.5 - c_offset, 0.5)),
        )
        .max(smoothstep(
            c_radius,
            0.0,
            euclidian_dist(p, (0.5 + c_offset, 0.5)),
        ))
    };

    let samples = sample_2d_candidates_f64(
        &sample_f,
        200,
        samples_count,
        &mut rng,
    );

    let data = samples
        .iter()
        .enumerate()
        .map(|(s, &p)| {
            let pos = project_in_boundaries(p, boundaries);
            let mut pts = Vec::new();
            pts.push(pos);
            let mut p = pos;
            for l in 0..((line_length / precision) as usize)
            {
                let a = field(p, l, s);
                p = (
                    p.0 + precision * a.cos(),
                    p.1 + precision * a.sin(),
                );
                if out_of_boundaries(p, plot_boundaries) {
                    break;
                }
                pts.push(p);
            }
            return pts;
        })
        .fold(Data::new(), |data, route| {
            render_route(data, route)
        });

    let color = "black";
    groups.push(
        layer(color).add(base_path(color, 0.2, data)),
    );

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
