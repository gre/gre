use gre::smoothstep;
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn layer(id: &str) -> Group {
    return Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", id);
}

struct Config {
    seed: f64,
    lines: usize,
    rows: usize,
    length: f64,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(32.0);

    let mut groups = Vec::new();

    let configs = vec![Config {
        seed,
        lines: 150,
        rows: 100,
        length: 12.0,
    }];

    for c in configs {
        let color = "black";

        let perlin = Perlin::new();

        // give the field angle (not the length)
        let field = |(x, y): (f64, f64)| {
            (3.0 * perlin.get([2.0 * x, 2.0 * y, c.seed])
                + 1.5 * perlin.get([8.0 * x, 8.0 * y, 1.0 + c.seed])
                + 0.6 * perlin.get([40.0 * x, 40.0 * y, 2.0 + c.seed]))
        };

        let mut data = Data::new();

        let boundaries = (10.0, 10.0, 287.0, 195.0);
        let lines = c.lines;
        let rows = c.rows;
        let precision = 1.0;
        let iterations = (c.length / precision) as usize;
        for l in 0..lines {
            for r in 0..rows {
                let mut p = (
                    boundaries.0 + (boundaries.2 - boundaries.0) * (l as f64) / (lines as f64),
                    boundaries.1 + (boundaries.2 - boundaries.0) * (r as f64) / (rows as f64),
                );
                let mut first = true;
                for _i in 0..iterations {
                    let normalized = (
                        (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
                        (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
                    );
                    let angle = field(normalized);
                    let (px, py) = p;
                    p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
                    if p.0 < boundaries.0
                        || p.0 > boundaries.2
                        || p.1 < boundaries.1
                        || p.1 > boundaries.3
                    {
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

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.2)
            .set("d", data);

        groups.push(layer(color).add(path));
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(gre::signature(1.0, (260.0, 196.0), "black"));
    for g in groups {
        document = document.add(g);
    }

    svg::save("image.svg", &document).unwrap();
}
