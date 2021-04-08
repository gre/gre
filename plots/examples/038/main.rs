use gre::{layer, signature};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn f(p: f64) -> (f64, f64, i32) {
    (
        -0.8 * (64. * 2.0 * PI * p).cos()
            - 0.4 * (128. * 2.0 * PI * p).cos()
            - 1.6 * (63. * 2.0 * PI * p).cos(),
        (2.0 * PI * p).sin()
            + 0.6 * (64. * 2.0 * PI * p).sin()
            + 0.3 * (128. * 2.0 * PI * p).sin()
            + 0.1 * (121. * 2.0 * PI * p).sin(),
        if (2.0 * PI * p).cos() > 0.0 { 2 } else { 1 },
    )
}

fn main() {
    let mut groups = Vec::new();
    let mut data1 = Data::new();
    let mut data2 = Data::new();

    let granularity = 10000;
    let mut last_c = 0;
    for i in 0..(granularity + 1) {
        let (x, y, c) = f((i as f64) / (granularity as f64));
        let p = (150.0 + 50. * x, 105.0 + 50. * y);
        if last_c != c {
            if c == 1 {
                data1 = data1.move_to(p);
            } else {
                data2 = data2.move_to(p);
            }
        } else {
            if c == 1 {
                data1 = data1.line_to(p);
            } else {
                data2 = data2.line_to(p);
            }
        }
        last_c = c;
    }

    groups.push(
        layer("gold").add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "gold")
                .set("stroke-width", 1.0)
                .set("d", data1),
        ),
    );
    groups.push(
        layer("white").add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "white")
                .set("stroke-width", 1.0)
                .set("d", data2),
        ),
    );

    // Make svg
    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: black")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(signature(1.0, (260.0, 190.0), "white"));
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
