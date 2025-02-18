use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let height = 210f64;
  let width = 297f64;
  let mut rng = rng_from_seed(opts.seed);

  let size = 90.;
  let f1 = (12., 12.);
  let f2 = (8., 8.);
  let amp1 = 0.9;
  let amp2 = 0.1;
  let count = 140;
  let samples = 1000;

  let parametric = |t: f64, p: f64| {
    let scale = 1. - 0.8 * p;
    (
      scale
        * amp1
        * ((2. * PI * t).cos()
          + amp2 * mix((f1.0 * PI * t).cos(), (f2.0 * PI * t).cos(), p)),
      scale
        * amp1
        * ((2. * PI * t).sin()
          + amp2 * mix((f1.1 * PI * t).sin(), (f2.1 * PI * t).sin(), p)),
    )
  };

  let mut routes = Vec::new();
  for pass in 0..count {
    let mut route = Vec::new();
    let r = rng.gen::<f64>();
    for i in 0..(samples + 1) {
      let sp = (r + i as f64 / (samples as f64)) % 1.0;
      let o = parametric(sp, pass as f64 / (count as f64));
      let p = (width * 0.5 + size * o.0, height * 0.5 + size * o.1);
      route.push(p);
    }
    routes.push(route);
  }

  let color = "black";
  let data = routes
    .iter()
    .fold(Data::new(), |data, route| render_route(data, route.clone()));
  let mut l = layer(color);
  l = l.add(base_path(color, 0.1, data));
  l = l.add(signature(1.5, (210.0, 180.0), color));
  vec![l]
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
