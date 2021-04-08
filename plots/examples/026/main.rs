use gre::{grayscale, image_get_color};
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

struct Config {
    lines: usize,
    rows: usize,
    length: f64,
}

fn normalize(p: (f64, f64), boundaries: (f64, f64, f64, f64)) -> (f64, f64) {
    (
        (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
        (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
    )
}
fn out_of_normalized_bound(p: (f64, f64)) -> bool {
    p.0 < 0.0 || p.1 < 0.0 || p.0 > 1.0 || p.1 > 1.0
}
fn out_of_bound(p: (f64, f64), boundaries: (f64, f64, f64, f64)) -> bool {
    p.0 < boundaries.0 || p.0 > boundaries.2 || p.1 < boundaries.1 || p.1 > boundaries.3
}

fn main() {
    let get_color = image_get_color("images/monalisa.jpg").unwrap();

    let field = |(x, y): (f64, f64)| {
        9.0 * (1.0 - 0.5 * x) * grayscale(get_color((x, y))) + 12.0 * (x - 0.5) * (y - 0.5)
    };

    let mut data = Data::new();

    let boundaries = (10.0, 10.0, 200.0, 280.0);
    let rows = 140;
    let precision = 1.0;
    for r in 0..rows {
        let lines = ((1.0 - 0.95 * (r as f64) / (rows as f64)) * (110 as f64)) as usize;
        for l in 0..lines {
            let mut p = (
                boundaries.0 + (boundaries.2 - boundaries.0) * (l as f64) / (lines as f64),
                boundaries.1
                    + (boundaries.3 - boundaries.1) * ((r as f64) / (rows as f64)).powf(1.5),
            );
            let normalized = normalize(p, boundaries);
            if out_of_normalized_bound(normalized) {
                break;
            }
            let length =
                (1.0 - (get_color(normalized)).0).powf(1.5) * (0.4 + 0.6 * normalized.1) * 32.0;
            let iterations = (length / precision) as usize;
            let mut first = true;
            for _i in 0..iterations {
                let normalized = normalize(p, boundaries);
                if out_of_normalized_bound(normalized) {
                    break;
                }
                let angle = field(normalized);
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
            }
        }
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 210, 297))
        .set("height", "297mm")
        .set("width", "210mm")
        .add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.2)
                .set("d", data),
        )
        .add(gre::signature(0.8, (11.0, 272.0), "black"));

    svg::save("image.svg", &document).unwrap();
}
