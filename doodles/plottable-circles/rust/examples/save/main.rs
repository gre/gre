use main::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let doc = art(&Opts {
        seed: rng.gen_range(0.0, 100.0),
        primary_name: String::from("P"),
        secondary_name: String::from("S"),
        rings: rng.gen_range(0, 100),
        ringcenter: rng.gen_range(0.0, 100.0),
        ring_resolution_multiplier: rng.gen_range(0.0, 100.0),
        ring_w_lower: rng.gen_range(0.0, 100.0),
        ring_w_upper: rng.gen_range(0.0, 100.0),
        ring_max_width: rng.gen_range(0.0, 100.0),
        line_gap_max: rng.gen_range(0.0, 100.0),
        ring_1x: rng.gen_range(0.0, 100.0),
        ring_1y: rng.gen_range(0.0, 100.0),
        ring_1xf2x: rng.gen_range(0.0, 100.0),
        ring_1xf2y: rng.gen_range(0.0, 100.0),
        ring_1yf2x: rng.gen_range(0.0, 100.0),
        ring_1yf2y: rng.gen_range(0.0, 100.0),
        ring_1y3: rng.gen_range(0.0, 100.0),
        ring_1yf3x: rng.gen_range(0.0, 100.0),
        ring_1yf3y: rng.gen_range(0.0, 100.0),
        size: rng.gen_range(0.0, 100.0),
    });
    svg::save("image.svg", &doc).unwrap();
}