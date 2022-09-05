use clap::*;
use core::f64;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::{path::Data, Group};

fn art(opts: Opts) -> Vec<Group> {
  let stroke_width = 0.5;
  let w = 297.;
  let h = 210.;
  let colors = opts.colors.split(",").collect::<Vec<&str>>();
  let mut rng = rng_from_seed(opts.seed);

  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut l = layer(color);
      let mut pts = (0..opts.points)
        .map(|i| {
          let a = PI / 2.
            + (2. * PI * ci as f64 / colors.len() as f64)
            + PI * (2. * PI * i as f64 / opts.points as f64).sin();
          (
            w / 2. + opts.radius * a.cos(),
            h / 2. + opts.radius * a.sin(),
          )
        })
        .collect::<Vec<(f64, f64)>>();
      rng.shuffle(&mut pts);
      let data = render_route_curve(Data::new(), pts);
      l = l.add(base_path(color, stroke_width, data));
      if ci == colors.len() - 1 {
        l = l.add(signature(1., (w - 110., h - 25.), color));
      }
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "400")]
  points: usize,
  #[clap(short, long, default_value = "0.")]
  seed: f64,
  #[clap(short, long, default_value = "90.")]
  radius: f64,
  #[clap(short, long, default_value = "hotpink,deepskyblue")]
  colors: String,
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
