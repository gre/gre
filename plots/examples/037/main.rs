use gre::*;
use noise::{NoiseFn, Perlin};
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_default = String::from("images/doge.png");
    let path = args.get(1).unwrap_or(&path_default);

    let perlin = Perlin::new();

    let get_color = image_get_color(path).unwrap();

    let get = &|(x, y)| (2.3 * (1. - grayscale(get_color((x, y))))).powf(2.0);

    let art = vectorize_as_fwave_rows_pingpong_3((190., 190.), get, 75, 250, "black")
        .add(signature(1.0, (164.0, 190.0), "black"))
        .set("transform", "translate(10,10) rotate(-90 95 95)");

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
        .add(art);

    svg::save("image.svg", &document).unwrap();
}

fn vectorize_as_fwave_rows_pingpong_3(
    (width, height): (f64, f64),
    get: &dyn Fn((f64, f64)) -> f64,
    rows: u32,
    cols: u32,
    color: &str,
) -> Group {
    let mut data = Data::new().move_to((0, height * 0.5 / (rows as f64)));
    for yi in 0..rows {
        let yp = (0.5 + yi as f64) / (rows as f64);
        let y = height * yp;
        let nb = cols;
        for i in 0..nb {
            let mut xp = (i as f64) / (nb as f64);
            let x = width * xp;
            let value = get((xp, yp));
            let x1 = x - 0.5 * width / (nb as f64);
            let y1 = y + ((i % 2) as f64 - 0.5) * value * (height as f64) / (rows as f64);
            if i == 0 {
                data = data.move_to((x, y));
            } else {
                if value < 0.01 {
                    data = data.line_to((x, y));
                } else {
                    data = data.quadratic_curve_to((x1, y1, x, y));
                }
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
