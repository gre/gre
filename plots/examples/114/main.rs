use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let width = 210f64;
  let height = 297f64;
  let size = 90.;
  let count = 100;
  let mut rng = rng_from_seed(opts.seed);
  let samples = (opts.seed * 80.) as usize;
  let mut passage = Passage2DCounter::new(0.3, width, height);

  let parametric = |t: f64, p: f64| {
    (
      (0.2 + 0.7 * p) * (2. * PI * t).sin() + 0.1 * (14. * PI * t).sin(),
      (0.2 + 0.8 * p) * (2. * PI * t).cos() + 0.2 * (20. * PI * t).cos(),
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
      let count = passage.count(p);
      if count > 5 {
        if route.len() > 1 {
          routes.push(route);
        }
        route = Vec::new();
      } else {
        route.push(p);
      }
    }
    routes.push(route);
  }

  let color = "white";
  let data = routes
    .iter()
    .fold(Data::new(), |data, route| render_route(data, route.clone()));
  let mut l = layer(color);
  l = l.add(base_path(color, 0.1, data));
  l = l.add(signature(1.0, (160.0, 250.0), color));
  vec![l]
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "7.0")]
  seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_portrait("black");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
