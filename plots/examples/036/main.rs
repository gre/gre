use geo::algorithm::euclidean_distance::EuclideanDistance;
use geo::*;
use gre::*;
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
        .unwrap_or(0.0);
    let modulo = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(4);

    let perlin = Perlin::new();

    let color1 = "cyan";
    let color2 = "red";

    let mut group1 = layer(color1);
    let mut group2 = layer(color2);

    let field = |x: f64, y: f64, i: f64, l: f64| {
        let base = y.atan2(x);
        let dist = (x * x + y * y).sqrt();
        let a = base
            + PI / 2.0
            + -0.1
            + 1.0 * perlin.get([3.0 * x, 3.0 * y, l * l + seed])
            + 0.6 * perlin.get([5.0 * x, 5.0 * y, seed])
            + 0.2 * perlin.get([11.0 * x, 11.0 * y, seed]);
        let l = 1.0;
        Point::new(l * a.cos(), l * a.sin())
    };

    let lines = 500;
    let offseting = 50.0;
    let radius = 60.0;
    let center = Point::new(105.0, 105.0);
    let iterations = 220;
    for i in 0..lines {
        let mut data = Data::new();

        let angle = 2. * PI * (i as f64) / (lines as f64);
        let mut p = center + Point::new(radius * angle.cos(), radius * angle.sin());
        for j in 0..iterations {
            let rx = (p.x() - center.x()) / radius;
            let ry = (p.y() - center.y()) / radius;
            let ri = (i as f64) / (lines as f64);
            let rl = (j as f64) / (iterations as f64);
            let v = field(rx, ry, ri, rl);
            p = p + v;
            let d = p.euclidean_distance(&center);
            if d < 0.1 * radius {
                break;
            }
            let a = ry.atan2(rx);
            let offset = Point::new(a.cos(), a.sin()) * ri * offseting * (d / radius);
            let pp = (p - offset).x_y();
            if j == 0 {
                data = data.move_to(pp);
            } else {
                data = data.line_to(pp);
            }
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke-width", 0.1)
            .set("d", data);

        if i % modulo == 0 {
            group2 = group2.add(path.set("stroke", color2));
        } else {
            group1 = group1.add(path.set("stroke", color1));
        }
    }

    // Make svg
    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: white")
        .set("viewBox", (0, 0, 210, 210))
        .set("width", "210mm")
        .set("height", "210mm")
        .add(group1)
        .add(group2)
        .add(signature(1.0, (170.0, 190.0), "black"));
    svg::save("image.svg", &document).unwrap();
}
