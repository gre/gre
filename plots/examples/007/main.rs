use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

fn parametric(mut t: f64) -> (f64, f64) {
    t = t * 2.0;
    return (
        0.5 * (PI * t).sin()
            + 0.3 * (3. * PI * t).sin()
            + 0.6 * (200. * PI * t).sin().powf(5.)
            + 0.1 * (401. * PI * t).sin(),
        0.7 * (PI * t).cos() + 0.2 * (3. * PI * t).cos() + 0.7 * (201. * PI * t).cos(),
    );
}

fn main() {
    fn map(t: f64) -> (f64, f64) {
        let (x, y) = parametric(t);
        return (50. + 20. * x, 50. + 20. * y);
    }

    let mut data = Data::new().move_to(map(0.));
    let nb = 50000;
    for i in 1..nb {
        let t = (i as f64) / (nb as f64);
        data = data.line_to(map(t));
    }
    data = data.close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.1)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 100, 100)).add(path);

    svg::save("image.svg", &document).unwrap();
}
