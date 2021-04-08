use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

fn parametric(mut t: f64) -> (f64, f64) {
    t = t * 2.0;
    let mut x = 0.8 * (PI * t).cos() + 0.2 * (3. * PI * t).cos() + 0.5 * (300. * PI * t).cos();
    let rightExt = 0.9;
    if x > rightExt {
        x *= 1.0 + (x - rightExt);
    }
    x -= 0.4;
    let y = 0.5 * (PI * t).sin()
        + 0.3 * (3. * PI * t).sin()
        + 0.4 * (300. * PI * t).sin().powf(5.)
        + (0.4 * (1.0 - (t - 1.0).abs()).powf(8.0)) * (4000. * PI * t).sin();
    return (x, y);
}

fn layer(children: Path, id: String) -> Group {
    return Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", id)
        .add(children);
}

fn main() {
    fn map(t: f64) -> (f64, f64) {
        let (x, y) = parametric(t);
        return (210. + 100. * x, 148.5 + 100. * y);
    }
    let nb = 40000;
    let parts = 2;

    let mut paths = Vec::new();

    for p in 0..parts {
        let from = p * nb / parts;
        let to = (p + 1) * nb / parts;
        let mut data = Data::new().move_to(map((from as f64) / (nb as f64)));
        for i in (from + 1)..to {
            let t = (i as f64) / (nb as f64);
            data = data.line_to(map(t));
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", format!("hsl({}, 80%, 50%)", p * 180))
            .set("stroke-width", 0.2)
            .set("d", data);

        paths.push(layer(path, format!("Group {}", p)));
    }

    let mut document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, 420, 297))
        .set("width", "420mm")
        .set("height", "297mm");
    for path in paths {
        document = document.add(path);
    }

    svg::save("image.svg", &document).unwrap();
}
