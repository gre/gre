use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(_seed: f64) -> Vec<Group> {
    let mut groups = Vec::new();

    let mut data = Data::new();

    let color = "white";
    let mut data = Data::new();
    let c = (297. / 2., 210. / 2.);
    let end = 95.;
    let samples = 10000;
    let spin = 30;
    data = data.move_to(c);
    for i in 0..samples {
        let a = i as f64 * 2. * PI * (spin as f64)
            / (samples as f64);
        let l = end * i as f64 / (samples as f64);
        data = data.line_to((
            c.0 + l * a.cos(),
            c.1 + l * a.sin(),
        ));
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
    let mut document = base_a4_landscape("black");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
