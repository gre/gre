use svg::node::element::*;
use svg::Document;

fn circles(x: f64, y: f64, radius: f64, decr: f64, color: &str) -> Group {
    let mut group = Group::new()
        .set("id", color)
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", color);
    let mut r = radius;
    loop {
        if r <= 0.0 {
            break;
        }
        group = group.add(
            Circle::new()
                .set("cx", x)
                .set("cy", y)
                .set("r", r)
                .set("fill", "none")
                .set("stroke-width", 1.0)
                .set("stroke", color),
        );
        r -= decr;
    }
    return group;
}

fn main() {
    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(circles(120.0, 80.0, 50.0, 4.0, "red"))
        .add(circles(150.0, 120.0, 50.0, 4.0, "blue"))
        .add(circles(180.0, 80.0, 50.0, 4.0, "green"));

    svg::save("image.svg", &document).unwrap();
}
