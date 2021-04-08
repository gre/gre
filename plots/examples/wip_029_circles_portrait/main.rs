use gre::{grayscale, image_get_color, layer, signature};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default = &String::from("images/photo-kid-hand1.jpg");
    let path = args.get(1).unwrap_or(default);
    let get_color = image_get_color(path).unwrap();

    let mut data = Data::new();

    let steps = 50000;
    let loops = 200;
    let radius = 100.0;
    let center = (105.0, 105.0);
    let mut was_drawing = false;
    data = data.move_to(center);
    for i in 0..steps {
        let p = (i as f64) / (steps as f64);
        let a = p * 2.0 * PI * (loops as f64);
        let dist = p * radius;
        let pos = (center.0 + dist * a.cos(), center.1 + dist * a.sin());
        let c = get_color((0.5 + 0.5 * p * a.cos(), 0.5 + 0.5 * p * a.sin()));
        let draw = grayscale(c) < 0.35;
        if draw {
            if !was_drawing {
                was_drawing = true;
                data = data.move_to(pos);
            } else {
                data = data.line_to(pos);
            }
        } else {
            was_drawing = false;
        }
    }

    let art = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.1)
        .set("d", data);

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
        .add(art)
        .add(signature(1.0, (260.0, 190.0), "black"));
    svg::save("image.svg", &document).unwrap();
}
