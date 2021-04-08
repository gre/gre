use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let perlin = Perlin::new();
    let size = 2.6;
    let seed = 0.0;
    let get = move |(x, y)| gre::smoothstep(-0.2, 0.5, perlin.get([x * size, y * size, seed]));
    let group = vectorize_as_spiralwave((148.0, 105.0), 90.0, 16.0, get, "white");

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background:black")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(gre::signature(1.0, (265.0, 195.0), "white"))
        .add(group);

    svg::save("image.svg", &document).unwrap();
}

////// vectorize function :)
fn vectorize_as_spiralwave(
    (cx, cy): (f64, f64),
    size: f64,
    turns: f64,
    get: impl Fn((f64, f64)) -> f64,
    color: &str,
) -> Group {
    let mut data = Data::new();

    let mut t = 0.0;
    let nb = (turns * 5000.0) as usize;
    for i in 0..nb {
        let p = (i as f64) / (nb as f64);
        let mut length = (0.02 + 0.98 * p) * size;
        let angle = p * turns * 2.0 * PI;
        let mut x = cx + length * angle.cos();
        let mut y = cy + length * angle.sin();

        let value = get((0.5 + (x - cx) / size, 0.5 + (y - cy) / size));
        let amp = 0.5 * (0.5 + 0.5 * p) * size / turns;
        t += 0.1 * value * (0.5 + 0.5 * p);
        length += amp * t.cos();

        x = cx + length * angle.cos();
        y = cy + length * angle.sin();
        if i == 0 {
            data = data.move_to((x, y));
        } else {
            data = data.line_to((x, y));
        }
    }

    return Group::new().add(
        Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.4)
            .set("d", data),
    );
}
