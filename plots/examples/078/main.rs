use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(_seed: f64) -> Vec<Group> {
    let mut groups = Vec::new();

    let color = "black";
    let mut data = Data::new();
    let c = (20. / 2., 210. / 2.);
    let samples = 5000;
    let mut phase: f64 = 0.0;
    for i in 0..samples {
        let p = i as f64 / (samples as f64);
        let point = (
            c.0 + 280. * p,
            c.1 + 100. * (1. - p).powf(2.) * phase.sin(),
        );
        if i == 0 {
            data = data.move_to(point);
        } else {
            data = data.line_to(point);
        }
        phase += 0.2 * (1. - p);
    }
    groups
        .push(layer(color).add(base_path(color, 1., data)));

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        2.0,
        (240.0, 95.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
