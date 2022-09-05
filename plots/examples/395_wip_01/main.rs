use clap::*;
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
      // OFF TODO(8) dedup points
      // OFF TODO(2) center the general shape
      // OFF TODO(9) avoid any point outside the boundaries
      // WIP IDEA(6) one axis (e.g. distance to bottom / center) can lead the "blurryness"
      // OFF TODO(3) group points by chunk of <path> (optim for inkscape plugin)
      let xdivider = rng.gen_range(1000.0, 3000.0);
      let ydivider = 0.9 * xdivider;
      let cidiv = ci as f64 / 99.;

      let f = |i, ampx, ampy| {
        let x = width / 2.0
          + r
            * perlin.get([
              i as f64 / xdivider,
              9.7
                + ampx * perlin.get([5.7 + 3.4 * opts.seed, i as f64 / 7.2934]),
              opts.seed / 3. + 4.4 + cidiv,
            ]);
        let y = height / 2.0
          + r
            * perlin.get([
              i as f64 / ydivider,
              7.3
                + ampy
                  * perlin.get([5.7 + 3.4 * opts.seed, i as f64 / 88.2934]),
              7.7 - opts.seed / 11. + cidiv,
            ]);
        (x, y)
      };

      for i in 0..40000 {
        let p = f(i, 0.0, 0.0);
        let dist = euclidian_dist(p, (width / 2., height / 2.));
        let amp = 0.5 + 1.5 * (2. * dist / width).powf(2.0);
        let (x, y) = f(i, amp, 0.1 * amp);
        data = data.move_to((x, y));
        let angle =
          (x - width / 2.).atan2(y - height / 2.) + rng.gen_range(-0.5, 0.5);
        let amp = rng.gen_range(1.0, 2.0);
        data = data.line_to((x + amp * angle.cos(), y + amp * angle.sin()));
      }
      let mut l = layer(color);
      l = l.add(base_path(color, stroke_width, data));
      l
    })
    .collect()
}

#[derive(Parser)]
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
