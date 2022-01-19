use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
    let stroke_width = 0.35;
    let colors = vec!["black", "red"];
    let width = 297.0;
    let height = 210.0;
    let r = 130.0;
    colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
        let mut data = Data::new();
        let perlin = Perlin::new();
        let mut rng = rng_from_seed(opts.seed);
        // TODO(9) avoid any point outside the boundaries
        // TODO(8) dedup points
        // TODO(4) colors?
        // TODO(2) center the general shape
        // IDEA(6) one axis (e.g. distance to bottom) can lead the "blurryness"
        // IDEA(6) feedback loop
        // IDEA(6) reverse the randomness to have negative version
        // IDEA(6) use diff color on distance to diff noise. color 0 near 0, color 1 near 1
        // IDEA(6) orient the stroke following a path.
        // IDEA(6) try different stroke lines. (variable on diff places possibly)
        // TODO(3) group points by chunk of <path> (optim for inkscape plugin)
        let xdivider = rng.gen_range(1000.0, 3000.0);
        let ydivider = 0.9 * xdivider;
        let cidiv = ci as f64 / 77.;
        
        for i in 0..50000 {
          let x = width / 2.0 + r * perlin.get([
            i as f64 / xdivider,
            9.7 + perlin.get([
              5.7 + 3.4 * opts.seed,
              i as f64 / 7.2934
            ]),
            opts.seed / 3. + 4.4 + cidiv
          ]);
          let y = height / 2.0 + r * perlin.get([
            i as f64 / ydivider,
            7.3 + 0.005 * perlin.get([
              5.7 + 3.4 * opts.seed,
              i as f64 / 88.2934
            ]),
            7.7 - opts.seed / 11. + cidiv
          ]);
          data = data.move_to((x,y));
          let angle = rng.gen_range(0.0, 2.0 * PI);
          let amp = 1.0;
          data = data.line_to((
            x + amp * angle.cos(),
            y + amp * angle.sin()
          ));
        }
        let mut l = layer(color);
        l = l.add(base_path(color, stroke_width, data));
        l
    })
    .collect()
}

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let groups = art(opts);
    let mut document = base_a4_landscape("white");
    for g in groups {
        document = document.add(g);
    }
    svg::save("image.svg", &document).unwrap();
}
