use gre::{grayscale, image_get_color};
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

// the function is a fork of 009 vectorize
fn vectorize_as_fwave_rows_pingpong(
    (width, height): (f64, f64),
    get: &Fn((f64, f64)) -> f64,
    osc_curve: &Fn(f64) -> f64,
    rows: u32,
    wave_freq: f64,
    precision: f64,
    color: &str,
) -> Group {
    let mut t = 0.0;
    let mut data = Data::new().move_to((0, height * 0.5 / (rows as f64)));
    for yi in 0..rows {
        let yp = (0.5 + yi as f64) / (rows as f64);
        let y = height * yp;
        let ltr = yi % 2 == 0;
        let nb = (width * (rows as f64) * precision) as usize;
        // TODO: this could be optimized to have less datapoints
        for i in 1..nb {
            let mut xp = (i as f64) / (nb as f64);
            if !ltr {
                xp = 1.0 - xp;
            }
            let x = width * xp;
            let angle = if ltr { 0.0 } else { PI };
            let value = get((xp, yp));
            let amp = 0.5 * (height as f64) / (rows as f64);
            t += wave_freq * (value).powf(2.0) / (nb as f64);
            let dy = amp * osc_curve(t);
            data = data.line_to((x, y + dy));
        }
    }

    return Group::new().add(
        Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.2)
            .set("d", data),
    );
}

fn main() {
    let get_color = image_get_color("images/ai.png").unwrap();
    let perlin = Perlin::new();
    let get = &|p: (f64, f64)| {
        1.1 - grayscale(get_color(p)) + 0.5 * perlin.get([0.0, 32.0 * p.0, 32.0 * p.1]).abs()
    };
    let osc = &|t: f64| t.cos() * (0.6 + 1.2 * perlin.get([0.1 * t, 0.0, 0.0]));
    let art = vectorize_as_fwave_rows_pingpong((190.0, 190.0), get, osc, 48, 800.0, 0.5, "white")
        .set("transform", "translate(10,5)");

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background:#900")
        .set("viewBox", (0, 0, 210, 210))
        .set("width", "210mm")
        .set("height", "210mm")
        .add(art)
        .add(gre::signature(1.0, (175.0, 200.0), "white"));

    svg::save("image.svg", &document).unwrap();
}
