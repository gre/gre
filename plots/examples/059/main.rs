use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn triangle_spiral(
    data: Data,
    origin: (f64, f64),
    initial_offset: f64,
    length: f64,
    d_length: f64,
) -> Data {
    let mut d = data;
    let mut a: f64 = 0.0;
    let mut p = origin;
    let mut l = length;
    d = d.move_to((p.0 + initial_offset, p.1));
    loop {
        if l < 0.0 {
            break;
        }
        p = (p.0 + l * a.cos(), p.1 + l * a.sin());
        d = d.line_to(p);
        a -= PI * 2. / 3.;
        l -= d_length;
    }
    d
}

fn art(_seed: f64) -> Vec<Group> {
    let mut groups = Vec::new();

    let mut data = Data::new();

    data = triangle_spiral(data, (60., 180.), 0., 180., 2.);

    let color = "red";
    groups
        .push(layer(color).add(base_path(color, 1., data)));

    let color = "white";
    let mut data = Data::new();
    let c = (151., 127.5);
    let start = 10.;
    let end = 95.;
    for i in 0..3 {
        let a = 2. * PI * (i as f64 + 0.25) / 3.;
        data = data.move_to((
            c.0 + start * a.cos(),
            c.1 + start * a.sin(),
        ));
        data = data.line_to((
            c.0 + end * a.cos(),
            c.1 + end * a.sin(),
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
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "white",
    ));
    svg::save("image.svg", &document).unwrap();
}
