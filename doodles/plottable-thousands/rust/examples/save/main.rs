use main::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let doc = art(&Opts {
        seed: rng.gen_range(0.0, 100.0),
        hash: String::from(""),
        primary_name: String::from("P"),
        secondary_name: String::from("S"),
    });
    svg::save("image.svg", &doc).unwrap();
}
