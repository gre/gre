use gre::{layer, signature};
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
        .unwrap_or(1.0);

    let mut groups = Vec::new();

    let mut data = Data::new();

    let perlin = Perlin::new();

    // give the field angle (not the length)
    let field = |(x, y): (f64, f64), l: f64| {
        perlin.get([2.0 * x + 0.1 * l, 2.0 * y, seed])
            + perlin.get([4.0 * x, 4.0 * y, 1.0 + seed])
            + perlin.get([8.0 * x, 8.0 * y, 2.0 + seed])
    };

    let boundaries = (10.0, 10.0, 280.0, 200.0);
    let lines = 500;
    let precision = 1.0;
    let iterations = (100.0 / precision) as usize;

    for l in 0..lines {
        let mut p = (
            boundaries.0 + (boundaries.2 - boundaries.0) * (l as f64) / (lines as f64),
            boundaries.1 + (boundaries.3 - boundaries.1) * (0.5 + 0.25 * (l as f64 * 3.0).cos()),
        );
        let mut first = true;
        let mut last_angle = if l < lines / 2 { 0.0 } else { PI };
        for _i in 0..iterations {
            let normalized = normalize(p, boundaries);
            let mut angle = field(normalized, (l as f64) / (lines as f64));
            if (angle - last_angle).abs() > 0.5 * PI {
                angle += PI;
            }
            // angle = angle + 0.8 * (last_angle - angle);
            let (px, py) = p;
            p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
            if out_of_bound(p, boundaries) {
                break;
            }
            let x = px;
            let y = py;
            if first {
                first = false;
                data = data.move_to((x, y));
            } else {
                data = data.line_to((x, y));
            }
            last_angle = angle;
        }
    }

    let color = "black";
    groups.push(
        layer(color).add(
            Path::new()
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.2)
                .set("d", data),
        ),
    );

    // Make svg
    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(signature(1.0, (260.0, 190.0), "black"));
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}

fn normalize(p: (f64, f64), boundaries: (f64, f64, f64, f64)) -> (f64, f64) {
    (
        (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
        (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
    )
}

fn out_of_bound(p: (f64, f64), boundaries: (f64, f64, f64, f64)) -> bool {
    p.0 < boundaries.0 || p.0 > boundaries.2 || p.1 < boundaries.1 || p.1 > boundaries.3
}
