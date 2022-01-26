use main::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let f1 = rng.gen_range(0.002, 0.05);
    let f2 = rng.gen_range(0.01, 0.1) * rng.gen_range(0.01, 1.0);
    let f3 = rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
    let a1 = rng.gen_range(0.4, 1.5) - rng.gen_range(0.0, 0.4);
    let a2 = rng.gen_range(0.0, 1.0);
    let a3 = rng.gen_range(0.0, 1.0);
    let opts = Opts {
        seed: rng.gen_range(0.0, 100.0),
        max_scale: rng.gen_range(20.0, 140.0)+1000.,
        desired_count: 200,
        a1,
        a2,
        a3,
        f1,
        f2,
        f3,
        base_pad: rng.gen_range(2.0, 10.0),
        base_min_scale: rng.gen_range(3.0, 10.0),
        wave_split_color: -10.0,
        base_offset: rng.gen_range(-1.5, 1.5) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
        xfactor: rng.gen_range(0.3, 0.6),
        primary_name: String::from("P"),
        secondary_name: String::from("S"),
        weights: vec![
            rng.gen_range(0.0, 11.0),
            rng.gen_range(0.0, 4.0),
            rng.gen_range(0.0, 4.0),
            rng.gen_range(0.0, 4.0),
            rng.gen_range(0.0, 4.0),
            rng.gen_range(0.0, 4.0),
            rng.gen_range(0.0, 4.0),
            rng.gen_range(0.0, 4.0),
            rng.gen_range(0.0, 4.0),
        ],
        diversity: rng.gen_range(0.0, 1.0),
        ribbons: 10.0 * (rng.gen_range(0f64, 20.0)-19.0).max(0.0),
        ribbons_freq: rng.gen_range(0.02, 0.2),
        ribbons_two_colors: rng.gen_range(0.0, 1.0) < 0.1
    };
    let doc = art(&opts);
    // println!("{:#?}", opts);
    svg::save("image.svg", &doc).unwrap();
}
