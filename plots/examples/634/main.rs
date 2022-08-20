/**
 * honk honk
 * if you ever see this Raph, I hope you survive the Rust code =)
 * I'll drive you through the code...
 */
use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

/**
 * Here, we're defining the INPUTS of the generator
 * It's basically a seed. but also we can configure a size (and an output file)
 * because yes, we are generating a .svg file for plotting purpose
 */
#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "90.0")]
  pub r: f64, // average radius
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "4.0")]
  pub amount: f64,
}

/**
 * that's the main entry function. We're building a SVG file and saving it
 */
fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

/**
 * And now the fun begins
 */
fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let r = opts.r;
  let colors = vec!["black"]; // we let ourself the possibility to do multi layers

  let mut rng = rng_from_seed(opts.seed);

  // global random values that drives the variation
  let a_delta = rng.gen_range(0.0, 2.0 * PI);
  let disp = rng.gen_range(0.1, 10.0) * rng.gen_range(0.3, 1.0);
  let adisp = rng.gen_range(0.1, 3.0) * rng.gen_range(0.0, 1.0);
  let dr = rng.gen_range(2.0, 60.0);
  let count = (opts.amount * (0.2 * dr + disp + adisp)) as usize;

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      let mut l = layer(color);

      for i in 0..count {
        // randomly offset of the position
        let x = width / 2.0 + rng.gen_range(-disp, disp);
        let y = height / 2.0 + rng.gen_range(-disp, disp);
        // randomly offset of the initial angle
        let start_a = a_delta + rng.gen_range(-adisp, adisp);
        let points = spiral(x, y, r, dr, start_a);
        data = render_route(data, points);
      }

      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

// this is our spiral helper
fn spiral(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  start_a: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = start_a;
  loop {
    route.push(round_point((x + r * a.cos(), y + r * a.sin()), 0.01));
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.1 {
      break;
    }
  }
  route
}
