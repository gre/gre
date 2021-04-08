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
        .unwrap_or(64.0);

    let perlin = Perlin::new();

    // give the field angle (not the length)
    let field = |(x, y): (f64, f64)| {
        (0.5 + x)
            * (3.2 * perlin.get([2.0 * x, 2.0 * y, seed])
                + 0.9 * perlin.get([7.0 * x, 7.0 * y, 1.0 + seed])
                + 0.5 * perlin.get([30.0 * x, 30.0 * y, 4.0 + seed]))
    };

    let mut data = Data::new();

    let boundaries = (10.0, 10.0, 230.0, 190.0);
    let lines = 300;
    let precision = 1.0;
    let iterations = (100.0 / precision) as usize;
    for r in 0..2 {
        for l in 0..((2 - r) * lines / 2) {
            let mut p = (
                boundaries.0 + (boundaries.2 - boundaries.0) * (l as f64) / (lines as f64),
                boundaries.1 + (boundaries.3 - boundaries.1) * ((1 + 2 * r) as f64) / 4.0,
            );
            let mut first = true;
            for _i in 0..iterations {
                let normalized = normalize(p, boundaries);
                let angle = field(normalized);
                let (px, py) = p;
                p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
                if out_of_bound(p, boundaries) {
                    break;
                }
                let x = px + 0.2 * (l as f64);
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
