use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "100.0")]
  pub width: f64,
  #[clap(short, long, default_value = "150.0")]
  pub height: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

fn branch(
  accpath: Vec<(f64, f64)>,
  n: usize,
  p: (f64, f64),
  a: f64,
  d: f64,
) -> Vec<Vec<(f64, f64)>> {
  let path = vec![accpath, vec![p]].concat();
  if n == 0 {
    return vec![path];
  }
  let q = (p.0 + d * a.cos(), p.1 + d * a.sin());
  let da = 0.3;
  let dist = d * 0.85;
  let left = branch(path.clone(), n - 1, q, a - da, dist);
  let right = branch(path.clone(), n - 1, q, a + da, dist);
  vec![left, right].concat()
}

fn art(opts: &Opts) -> Vec<Group> {
  let mut rng = rng_from_seed(opts.seed);
  let delta = 0.2;
  let n = 6;
  let d = 16.0;
  let origin = (opts.width / 2.0, opts.height * delta);
  let mut routes1 = branch(Vec::new(), n, origin, PI / 2.0, d);
  rng.shuffle(&mut routes1);
  let origin = (opts.width / 2.0, opts.height * (1.0 - delta));
  let mut routes2 = branch(Vec::new(), n, origin, -PI / 2.0, d);
  rng.shuffle(&mut routes2);

  vec![(routes1, "#c00"), (routes2, "#00c")]
    .iter()
    .map(|(routes, color)| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route.clone());
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
