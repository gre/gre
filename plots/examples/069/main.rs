use geo::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Group;

fn square(
    c: (f64, f64),
    r: f64,
    angle: f64,
) -> Polygon<f64> {
    let mut v: Vec<(f64, f64)> = Vec::new();
    for i in 0..4 {
        let a = (angle + 0.5 + i as f64) * PI * 0.5;
        v.push((c.0 + r * a.cos(), c.1 + r * a.sin()));
    }
    Polygon::new(v.into(), vec![])
}

fn rotate(p: (f64, f64), a: f64) -> (f64, f64) {
    (
        p.0 * a.cos() - p.1 * a.sin(),
        p.0 * a.sin() + p.1 * a.cos(),
    )
}

fn square_spiral(
    data: Data,
    c: (f64, f64),
    r: f64,
    initial_a: f64,
    d_length: f64,
) -> Data {
    let mut d = data;
    let mut a: f64 = initial_a;
    let length = r * 2. / (2. as f64).sqrt();
    let delta = rotate((-length / 2., length / 2.), a);
    let mut p = (c.0 + delta.0, c.1 + delta.1);
    let mut l = length;
    let mut i = 0;
    d = d.move_to((p.0, p.1));
    loop {
        if l < 0.0 {
            break;
        }
        p = (p.0 + l * a.cos(), p.1 + l * a.sin());
        d = d.line_to(p);
        a -= PI / 2.;
        if i > 0 {
            l -= d_length;
        }
        i += 1;
    }
    d
}

fn add<F: Clone>(v: &Vec<F>, item: F) -> Vec<F> {
    let mut copy = v.clone();
    copy.push(item);
    copy
}

fn rec(
    params: &mut Vec<((f64, f64), f64, f64, f64, usize)>,
    // 0: continues rec, 1: draw spiral, 2: empty
    decide: &dyn Fn(usize, (f64, f64)) -> u8,
    c: (f64, f64),
    r: f64,
    angle: f64,
    depth: usize,
) {
    let pad = 1.0;
    let decision = decide(depth, c);
    if decision > 0 {
        if decision == 1 {
            params.push((c, r - pad, angle, pad, depth));
        }
        return;
    }

    for i in 0..4 {
        let base_a = angle + i as f64 * PI / 2.;
        let a = base_a + PI / 4.;
        let nc = (
            c.0 + 0.5 * r * a.cos(),
            c.1 + 0.5 * r * a.sin(),
        );
        rec(params, decide, nc, r / 2., base_a, depth - 1);
    }
}

fn art(seed: f64) -> Vec<Group> {
    let perlin = Perlin::new();

    let initial_c = (150., 105.);

    let mut params = Vec::new();
    let decide = |depth: usize, (x, y): (f64, f64)| {
        if depth > 0
            && perlin.get([0.05 * x, 0.05 * y, seed]) < 0.1
        {
            return 0;
        }
        if perlin.get([0.5 * x, 0.5 * y, seed + 1.]) > 0.5 {
            return 2;
        }
        return 1;
    };

    rec(&mut params, &decide, initial_c, 115., 0., 6);

    let colors = vec!["turquoise", "skyblue"];

    let mut groups = Vec::new();

    for (i, color) in colors.iter().enumerate() {
        let mut data = Data::new();

        for (c, r, a, d_length, depth) in params.clone() {
            if depth % colors.len() == i {
                data =
                    square_spiral(data, c, r, a, d_length);
            }
        }

        if i == 0 {
            data = render_polygon_stroke(
                data,
                square(initial_c, 120., 0.0),
            );
        }

        groups.push(
            layer(color).add(base_path(color, 0.4, data)),
        );
    }

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(56.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "skyblue",
    ));
    svg::save("image.svg", &document).unwrap();
}
