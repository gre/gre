use gre::{signature, smoothstep};
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(100.0);

    let perlin = Perlin::new();

    let golden_angle = PI * (3.0 - (5.0 as f64).sqrt());

    let field = |i: f64, j: f64, (x, y): (f64, f64)| {
        let angle = (y - 0.5).atan2(x - 0.5);
        1.0 + angle
            + ((0.1 + 3.0 * j) * perlin.get([20.0 * i, 40.0 * j, seed])
                + 0.8 * perlin.get([7.0 * x, 7.0 * y, 1.0 + seed])
                + 0.3 * perlin.get([9.0 * x, 9.0 * y, 2.0 + seed]))
    };

    let mut data = Data::new();

    let boundaries = (10.0, 10.0, 287.0, 195.0);
    let precision = 0.5;
    let samples = 200;
    let radius_from = 0.0;
    let radius_to = 60.0;
    let length_base = 100.0;
    let length_unify = 0.5;
    for i in 0..samples {
        let a = golden_angle * (i as f64);
        let amp =
            radius_from + (radius_to - radius_from) * ((i as f64) / (samples as f64)).powf(0.6);
        let mut p = (
            boundaries.0 + (boundaries.2 - boundaries.0) * 0.5 + a.cos() * amp,
            boundaries.1 + (boundaries.3 - boundaries.1) * 0.5 + a.sin() * amp,
        );
        let length = length_base - length_unify * amp;
        let iterations = (length / precision) as usize;
        let mut first = true;
        for j in 0..iterations {
            let normalized = (
                (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
                (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
            );
            let angle = field(
                (i as f64) / (samples as f64),
                (j as f64) / (iterations as f64),
                normalized,
            );
            p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
            if p.0 < boundaries.0 || p.0 > boundaries.2 || p.1 < boundaries.1 || p.1 > boundaries.3
            {
                break;
            }
            if first {
                first = false;
                data = data.move_to(p);
            } else {
                data = data.line_to(p);
            }
        }
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: #111")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "gold")
                .set("stroke-width", 0.5)
                .set("d", data),
        )
        .add(signature(1.0, (260.0, 196.0), "gold"));

    svg::save("image.svg", &document).unwrap();
}
