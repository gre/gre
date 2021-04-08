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
        .unwrap_or(15.0);
    let samples = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(500);
    let default_color1 = &String::from("gold");
    let default_color2 = &String::from("white");
    let color1 = args.get(3).unwrap_or(default_color1).as_str();
    let color2 = args.get(4).unwrap_or(default_color2).as_str();

    let perlin = Perlin::new();

    let golden_angle = PI * (3.0 - (5.0 as f64).sqrt());

    let field = |_i: f64, j: f64, (x, y): (f64, f64)| {
        let angle = (y - 0.5).atan2(x - 0.5);
        angle
            + 1.5 * perlin.get([4.0 * x, 4.0 * y, 1.0 + seed])
            + 1.5 * perlin.get([9.0 * x, 9.0 * y, 2.0 + seed])
            + 0.5 * j * perlin.get([19.0 * x, 19.0 * y, 3.0 + seed])
    };

    let mut layer1 = layer(color1);
    let mut layer2 = layer(color2);
    let boundaries = (0.0, 0.0, 297.0, 210.0);
    let precision = 0.5;
    let radius_from = 0.0;
    let radius_to = 50.0;
    let length_base = 70.0;
    let length_unify = 0.5;
    for i in 0..samples {
        let mut data = Data::new();
        let a = golden_angle * (i as f64);
        let amp =
            radius_from + (radius_to - radius_from) * ((i as f64) / (samples as f64)).powf(0.6);
        let mut p = (
            boundaries.0 + (boundaries.2 - boundaries.0) * 0.5 + a.cos() * amp,
            boundaries.1 + (boundaries.3 - boundaries.1) * 0.5 + a.sin() * amp,
        );
        let length = length_base - length_unify * amp;
        let iterations = (length / precision) as usize;
        let mut first = true;
        for j in 0..iterations {
            let normalized = (
                (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
                (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
            );
            let angle = field(
                (i as f64) / (samples as f64),
                (j as f64) / (iterations as f64),
                normalized,
            );
            p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
            if p.0 < boundaries.0 || p.0 > boundaries.2 || p.1 < boundaries.1 || p.1 > boundaries.3
            {
                break;
            }
            if first {
                first = false;
                data = data.move_to(p);
            } else {
                data = data.line_to(p);
            }
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke-width", 0.5)
            .set("d", data);

        if i % 8 == 0 {
            layer2 = layer2.add(path.set("stroke", color2));
        } else {
            layer1 = layer1.add(path.set("stroke", color1));
        }
    }

    layer1 = layer1.add(signature(1.0, (260.0, 196.0), "gold"));

    let groups = Group::new().add(layer1).add(layer2);

    let document = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("style", "background: #111")
        .set("viewBox", (0, 0, 297, 210))
        .set("width", "297mm")
        .set("height", "210mm")
        .add(groups);

    svg::save("image.svg", &document).unwrap();
}
