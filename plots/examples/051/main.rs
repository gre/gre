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

fn rotate(p: (f64, f64), a: f64) -> (f64, f64) {
    (
        p.0 * a.cos() - p.1 * a.sin(),
        p.0 * a.sin() + p.1 * a.cos(),
    )
}

fn triangle_spiral(
    data: Data,
    c: (f64, f64),
    r: f64,
    initial_a: f64,
    d_length: f64,
) -> Data {
    let mut d = data;
    let mut a: f64 = initial_a;
    let length = r * (3. as f64).sqrt();
    let delta = rotate((-length / 2., 1.5 * r / 3.), a);
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
        a -= PI * 2. / 3.;
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
    params: &mut Vec<((f64, f64), f64, f64)>,
    // 0: continues rec, 1: draw spiral, 2: empty
    decide: &dyn Fn(usize, (f64, f64), &Vec<u8>) -> u8,
    path: &Vec<u8>,
    c: (f64, f64),
    r: f64,
    angle: f64,
    depth: usize,
    pad: f64,
) {
    let decision = decide(depth, c, path);
    if decision > 0 {
        if decision == 1 {
            params.push((c, r - pad, angle));
        }
        return;
    }

    // middle
    rec(
        params,
        decide,
        &add(path, 0),
        c,
        r / 2.,
        angle + PI,
        depth - 1,
        pad,
    );
    for i in 0..3 {
        let base_a = angle + i as f64 * PI * 2. / 3.;
        let a = base_a + PI / 6.;
        let nc = (
            c.0 + 0.5 * r * a.cos(),
            c.1 + 0.5 * r * a.sin(),
        );
        rec(
            params,
            decide,
            &add(path, i + 1),
            nc,
            r / 2.,
            base_a,
            depth - 1,
            pad,
        );
    }
}

fn art(seed: f64) -> Vec<Group> {
    let perlin = Perlin::new();

    let mut groups = Vec::new();
    let mut data = Data::new();

    let d_length = 2.;

    let initial_c = (150., 130.);

    let mut params = Vec::new();
    let decide = |depth: usize,
                  (x, y): (f64, f64),
                  _path: &Vec<u8>| {
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

    rec(
        &mut params,
        &decide,
        &Vec::new(),
        initial_c,
        110.,
        0.,
        4,
        d_length,
    );

    for (c, r, a) in params {
        data = triangle_spiral(data, c, r, a, d_length);
    }

    data = render_polygon_stroke(
        data,
        triangle(initial_c, 120., 0.25),
    );

    let color = "black";
    groups.push(
        layer(color).add(base_path(color, 0.2, data)),
    );

    groups
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(17.0);
    let groups = art(seed);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (260.0, 190.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
