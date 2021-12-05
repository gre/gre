use main::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let f1 = rng.gen_range(0.01, 0.1);
    let f2 = f1 * 2.0;
    let f3 = f1 * 4.0;
    let doc = art(&Opts {
        seed: rng.gen_range(0.0, 100.0),
        max_scale: rng.gen_range(20.0, 140.0) * rng.gen_range(0.0, 1.0),
        desired_count: 200,
        samples: 6000,
        particle_size: 35,
        fading: 40.0, // 20 to 60. centred
        gravity_dist: 40.0,//rng.gen_range(0.0, 140.0). center 40
        spiral_pad: rng.gen_range(0.0, 16.0),
        a1: rng.gen_range(0.0, 1.0),
        a2: rng.gen_range(0.1, 1.0),
        a3: rng.gen_range(0.1, 1.0),
        f1,
        f2,
        f3,
        yfactor: rng.gen_range(0f64, 1.0).powf(4.0),
        primary_name: String::from("P"),
        secondary_name: String::from("S"),
    });
    svg::save("image.svg", &doc).unwrap();
}
