use gre::signature;
use noise::{NoiseFn, Perlin};
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

// formula from https://www.youtube.com/watch?v=aNR4n0i2ZlM
fn heart_distance(p: (f64, f64)) -> f64 {
    let x = p.0;
    let y = 4.0 + 1.2 * p.1 - x.abs() * ((20.0 - x.abs()) / 15.0).sqrt();
    x * x + y * y - 10.0
}

// the function is a fork of 009 vectorize
fn vectorize_as_fwave_rows_pingpong_2(
    (width, height): (f64, f64),
    get: &dyn Fn((f64, f64)) -> f64,
    osc_curve: &dyn Fn(f64) -> f64,
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
        for i in 0..nb {
            let mut xp = (i as f64) / (nb as f64);
            if !ltr {
                xp = 1.0 - xp;
            }
            let x = width * xp;
            let value = get((xp, yp));
            let amp = 0.45 * (height as f64) / (rows as f64);
            t += wave_freq * (value).powf(2.0) / (nb as f64);
            let dy = amp * osc_curve(t);
            if i == 0 {
                data = data.move_to((x, y + dy));
            } else {
                data = data.line_to((x, y + dy));
            }
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
    let perlin = Perlin::new();
    let get = &|p: (f64, f64)| {
        (-0.5 * heart_distance((8.0 * (p.0 - 0.5), 8.0 * (0.2 - p.1))))
            .max(0.0)
            .min(1.0)
            + 0.5 * perlin.get([0.0, 32.0 * p.0, 32.0 * p.1]).abs()
    };
    let osc = &|t: f64| t.cos() * (0.6 + 1.2 * perlin.get([0.1 * t, 0.0, 0.0]));
    let art = vectorize_as_fwave_rows_pingpong_2((190.0, 190.0), get, osc, 32, 600.0, 0.5, "black")
        .set("transform", "translate(10,5)");

    // Make svg
    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(art)
        .add(signature(1.0, (38.0, 188.0), "black"));

    svg::save("image.svg", &document).unwrap();
}
