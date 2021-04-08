use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// inspired from https://github.com/gre/shaderday.com/blob/master/day/015.js
// Kudos to 0xB0nnaz for the formula
// x(t) = sin(PI * t) + 0.8 * sin(4 * PI * t) + 0.5 * sin(128 * PI * t)
// y(t) = cos(PI * t) + 0.8 * cos(4 * PI * t) + 0.5 * cos(128 * PI * t)

fn parametric(t: f64) -> (f64, f64) {
    return (
        (PI * t).sin() + 0.8 * (4. * PI * t).sin() + (128. * PI * t).sin() * 0.5,
        (PI * t).cos() + 0.8 * (4. * PI * t).cos() + (128. * PI * t).cos() * 0.5,
    );
}

fn main() {
    fn map(t: f64) -> (f64, f64) {
        let (x, y) = parametric(t);
        return (50. + 20. * x, 50. + 20. * y);
    }

    let mut data = Data::new().move_to(map(0.));
    let nb = 10000;
    for i in 1..nb {
        let t = 2.0 * (i as f64) / (nb as f64);
        data = data.line_to(map(t));
    }
    data = data.close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.2)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 100, 100)).add(path);

    svg::save("image.svg", &document).unwrap();
}
