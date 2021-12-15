use main::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let doc = art(&Opts {
        seed: rng.gen_range(0.0, 100.0),
        precision: 1.0,
        samples: 80,
        iterations: 20,
        f1: rng.gen_range(0.1, 5.0),
        f2x: rng.gen_range(1.0, 10.0),
        f2y: rng.gen_range(1.0, 10.0),
        f3: rng.gen_range(2.0, 6.0),
        a1: rng.gen_range(0.05, 0.4),
        a2: rng.gen_range(1.0, 3.0),
        a3: rng.gen_range(1.0, 3.0),
        k: rng.gen_range(0.0, 0.3),
        shapeamp: 0.8,
        offset: 0.1,
        overflowin: 0.1,
        overflowout: 0.1,
        vertical: rng.gen_range(0.0, 1.0) < 0.5,
        symmetry: rng.gen_range(0.0, 1.0) < 0.5,
        primary_name: String::from("P"),
        secondary_name: String::from("S"),
    });
    svg::save("image.svg", &doc).unwrap();
}