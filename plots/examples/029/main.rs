use gre::signature;
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
        .unwrap_or(80.0);
    let height = 280.0;
    let width = 190.0;
    let line_length = 100.0;
    let lines = 500;
    let perlin = Perlin::new();
    let angle_velocity_field = |x, y, _l, _length| {
        0.15 * (0.5 * perlin.get([7.0 * x, 7.0 * y, seed])
            + 0.3 * perlin.get([11.0 * x, 11.0 * y, seed])
            + 0.2 * perlin.get([21.0 * x, 21.0 * y, seed]))
    };
    let origin = |l| {
        (
            width / 2.0 + 10.0 * (l as f64 * 3.0).sin(),
            height * (1.0 - 0.7 * (l as f64 + 0.5) / (lines as f64)),
        )
    };
    let initial_angle = |_l| -PI / 2.0;
    let art = render_angle_velocity_field(
        width,
        height,
        lines,
        line_length,
        angle_velocity_field,
        origin,
        initial_angle,
    )
    .set("transform", "translate(10,10)");
    svg::save("image.svg", &make_svg(art)).unwrap();
}

fn render_angle_velocity_field(
    width: f64,
    height: f64,
    lines: usize,
    line_max_length: f64,
    angle_velocity_field: impl Fn(f64, f64, f64, f64) -> f64,
    origin: impl Fn(usize) -> (f64, f64),
    initial_angle: impl Fn(usize) -> f64,
) -> Group {
    let mut data = Data::new();
    let step = 1.0;

    for l in 0..lines {
        let mut angle = initial_angle(l);
        let mut length = 0.0;
        let mut p = origin(l);
        data = data.move_to(p);
        loop {
            let a = angle_velocity_field(
                p.0 / width,
                p.1 / height,
                (l as f64) / (lines as f64),
                length / width.min(height),
            );
            angle += step * a;
            p.0 += step * angle.cos();
            p.1 += step * angle.sin();
            if p.0 < 0. || p.1 < 0. || p.0 > width || p.1 > height {
                break;
            }
            data = data.line_to(p);
            length += step;
            if length > line_max_length {
                break;
            }
        }
    }

    return Group::new().add(
        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.2)
            .set("d", data),
    );
}

fn make_svg(art: Group) -> Document {
    Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 210, 297))
        .set("width", "210mm")
        .set("height", "297mm")
        .add(art)
        .add(signature(1.0, (170.0, 280.0), "black"))
}
