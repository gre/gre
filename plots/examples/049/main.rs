use geo::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Group;

fn triangle(
    c: (f64, f64),
    r: f64,
    angle: f64,
) -> Polygon<f64> {
    let mut v: Vec<(f64, f64)> = Vec::new();
    for i in 0..3 {
        let a = (angle + i as f64) * PI * 2. / 3.;
        v.push((c.0 + r * a.cos(), c.1 + r * a.sin()));
    }
    Polygon::new(v.into(), vec![])
}

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

    let d_length = 5.;

    let poly = triangle((150., 120.), 100., 0.25);
    data = render_polygon_stroke(data, poly);

    data = triangle_spiral(
        data,
        (66.5, 165.0),
        0.,
        162.0,
        d_length,
    );

    let color = "deepskyblue";
    groups.push(
        layer(color).add(base_path(color, 0.5, data)),
    );

    let mut data = Data::new();

    data = triangle_spiral(
        data,
        (62.75, 167.25),
        2.5,
        169.5,
        d_length,
    );

    let color = "aquamarine";
    groups.push(
        layer(color).add(base_path(color, 0.5, data)),
    );

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
