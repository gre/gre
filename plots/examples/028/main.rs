use gre::smoothstep;
use noise::{NoiseFn, Perlin};
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
    length: f64,
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

// for i in {0..100}; do cargo run --example unreleased_meri_edition $i; cp image.svg results/$i.svg; done

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(25.0);

    let perlin = Perlin::new();

    // give the field angle (not the length)
    let field = |(x, y): (f64, f64)| {
        (3.0 * perlin.get([9. * x, 9. * y, seed])
            + (0.1 + x) * 3.0 * perlin.get([20. * x, 20. * y, seed + 2.0]))
    };

    let mut data = Data::new();

    let boundaries = (10.0, 10.0, 270.0, 200.0);
    let lines = 200;
    let precision = 1.0;
    for l in 0..lines {
        let mut p = (
            boundaries.0 + (boundaries.2 - boundaries.0) * (l as f64) / (lines as f64),
            boundaries.1 + (boundaries.3 - boundaries.1) * (0.5 + 0.3 * (0.04 * l as f64).cos()),
        );
        let iterations = (120.0 * (1.1 - l as f64 / (lines as f64)) / precision) as usize;
        let mut first = true;
        for _i in 0..iterations {
            let normalized = normalize(p, boundaries);
            let angle = field(normalized);
            let (px, py) = p;
            p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
            if out_of_bound(p, boundaries) {
                break;
            }
            let x = px + 0.1 * (l as f64);
            let y = py - 0.2 * (l as f64);
            if first {
                first = false;
                data = data.move_to((x, y));
            } else {
                data = data.line_to((x, y));
            }
        }
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
        .add(gre::signature(0.8, (265.0, 195.0), "black"))
        .add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.1)
                .set("d", data),
        );

    svg::save("image.svg", &document).unwrap();
}
