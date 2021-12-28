use main::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let doc = art(&Opts {
        primary_name: String::from("P"),
        secondary_name: String::from("S"),
        width: 297.0,
        height: 210.0,
        pad: 5.0,
        reverse_curve_x: false,
        reverse_curve_y: false,
        f1: 8.0,
        f2: 8.0,
        amp1: 0.4,
        amp2: 0.2,
        ricochets: 3,
        incr: 0.005,
        closing: true,
        colordelta: 0.0025,
        rad_start: 2.,
        rad_incr: 1.0,
        precision: 0.6,
        max_passage: 6
    });
    svg::save("image.svg", &doc).unwrap();
}