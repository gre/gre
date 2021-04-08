use geo::*;
use gre::*;
use std::f64::consts::PI;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::{Group};

fn shape(
    c: (f64, f64),
    s: f64,
    variant: u8
) -> Polygon<f64> {
    let mut v: Vec<(f64, f64)> = Vec::new();
    let r = 0.4;
    if variant!=2 {
        v.push((c.0, c.1 - s));
    }
    v.push((c.0 + s * r, c.1));
    if variant!=1 {
        v.push((c.0, c.1 + s));
    }
    v.push((c.0 - s * r, c.1));
    Polygon::new(v.into(), vec![])
}

fn art(seed: u8, samples: usize, mul: f64, pow: f64) -> Vec<Group> {
    let pad = 10.;
    let width = 180.;
    let height = 270.;
    let size = 300;

    let project =
        |(x, y): (f64, f64)| (pad + x * width, pad + y * height);

    let mut rng = SmallRng::from_seed([
        seed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    let get_color =
        image_get_color("images/ethereum.png")
            .unwrap();

    let get = |p: (f64, f64)| {
        let r = width / height;
        let sp = ((p.0 + 0.25) * 0.66, p.1);
        let c = grayscale(get_color(sp));
        smoothstep(0.85, 0.0, c).powf(2.)
    };

    let candidates = sample_2d_candidates_f64(
        &get,
        size,
        samples,
        &mut rng,
    );
    

    let mut data = Data::new();

    for p in candidates {
        let u = rng.next_u32();
        let variant = if u % 8 == 0 { 1 } else if u % 8 == 1 { 2 } else { 0 };
        let t = shape(project(p), mul * (4. * (rng.gen_range(1., 4.) as f64 / 4.).powf(pow) + 3. * (p.1 - 1.).abs()), variant);
        data = render_polygon_stroke(data, t);
    }

    vec![Group::new().add(
        layer("black").add(base_path("black", 0.3, data)),
    )]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed = args
        .get(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(10);
    let samples = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1600);
    let mul = args
        .get(3)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.);
    let pow = args
        .get(4)
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.);
    let groups = art(seed, samples, mul, pow);
    let mut document = base_a4_portrait("white");
    for g in groups {
        document = document.add(g);
    }
    document = document.add(signature(
        1.0,
        (175.0, 280.0),
        "black",
    ));
    svg::save("image.svg", &document).unwrap();
}
