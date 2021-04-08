use gre::smoothstep;
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn field((x, y): (f64, f64)) -> f64 {
    PI / 2.0 - (2.0 * x - 1.0) * y.powf(0.5) * smoothstep(0.4, 0.6, y + 0.05 * (20.0 * x).sin())
}

fn main() {
    let mut data = Data::new();
    let perlin = Perlin::new();

    let boundaries = (10.0, 10.0, 280.0, 190.0);
    let lines = 50;
    let precision = 0.2;
    for l in 0..lines {
        let mut p = (
            boundaries.0 + (0.5 + l as f64) * (boundaries.2 - boundaries.0) / (lines as f64),
            boundaries.1,
        );
        data = data.move_to(p);
        let mut t: f64 = 0.0;
        for _i in 0..10000 {
            let normalized = (
                (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
                (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
            );
            let angle = field(normalized);
            let (px, py) = p;
            p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
            if p.0 < boundaries.0 || p.0 > boundaries.2 || p.1 < boundaries.1 || p.1 > boundaries.3
            {
                break;
            }
            let (mut x, mut y) = p;
            let n = perlin.get([300.0 * normalized.0, 3.0 * normalized.1, 0.3]);
            let phase = 0.5 * smoothstep(0.3, 0.8, n);
            let amp = 1.2 * smoothstep(0.3, 0.6, n).powf(2.0);
            let da = (y - py).atan2(x - px);
            x -= amp * t.cos() * (da).sin();
            y += amp * t.cos() * (da).cos();
            data = data.line_to((x, y));
            t += phase;
        }
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.5)
        .set("d", data);

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(gre::signature(1.0, (265.0, 195.0), "black"))
        .add(path);

    svg::save("image.svg", &document).unwrap();
}
