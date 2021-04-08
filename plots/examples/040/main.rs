use gre::*;
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let mut groups = Vec::new();

    let mut data = Data::new();
    let get_color = image_get_color("images/dragoon.jpg").unwrap();
    let perlin = Perlin::new();

    let rot_field = |(x, y): (f64, f64)| {
        if x < 0.0 || y < 0.0 || x > 1.0 || y > 1. {
            (0.0, 0.0)
        } else {
            let g = grayscale(get_color((x, y)));
            let s = smoothstep(0.8, 0.0, g);
            let k1 = perlin.get([2. * x, 2. * y, 1.0]);
            let k2 = perlin.get([3. * x, 3. * y, 2.0]);
            let k3 = perlin.get([6. * x, 6. * y, 3.0]);
            (
                (0.05 + 0.05 * s) * k1 + s * 0.2 * k2 + 0.3 * s * (0.3 - g + 0.1 * k3),
                1. - g,
            )
        }
    };
    let boundaries = (40.0, 10.0, 240.0, 200.0);
    let height = 90;
    let width = 40;
    let precision = 1.0;
    let iterations = (20.0 / precision) as usize;

    for x in 0..width {
        for y in 0..height {
            let mut p = (
                boundaries.0
                    + (boundaries.2 - boundaries.0) * (((y % 2) as f64) * 0.5 + x as f64)
                        / (width as f64),
                boundaries.1 + (boundaries.3 - boundaries.1) * (y as f64) / (height as f64),
            );
            let mut first = true;
            let mut angle = 0.0;
            for i in 0..iterations {
                let normalized = normalize_in_boundaries(p, boundaries);
                let r = rot_field(normalized);
                if (i as f64) / (iterations as f64) > r.1 {
                    break;
                }
                angle += r.0;
                let (px, py) = p;
                p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
                if out_of_boundaries(p, boundaries) {
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
            }
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
        .add(signature(1.0, (200.0, 195.0), "black"));
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
