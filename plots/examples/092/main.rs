use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let width = 300.;
  let height = 240.;
  let pad = 10.;
  let bounds = (pad, pad, width - pad, height - pad);
  let mut rng = rng_from_seed(opts.seed);
  let colors = vec!["turquoise"];
  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();

      let parametric = |t: f64| {
        (
          0.1 * (2. * PI * t).cos() + 0.35 * (200. * 2. * PI * t).sin(),
          0.3 * (2. * PI * t).sin() + 0.001 * (200. * 2. * PI * t).cos(),
        )
      };

      let samples = 10000;
      let mut route: Vec<(f64, f64)> = (0..samples)
        .map(|i| {
          let mut p = parametric(i as f64 / (samples as f64));
          p.0 = p.0 + 0.5;
          p.1 = p.1 + 0.5;
          project_in_boundaries(p, bounds)
        })
        .collect();

      data = render_route(data, route);

      let mut l = layer(color);
      l = l.add(base_path(color, 0.2, data));
      if i == colors.len() - 1 {
        l = l.add(signature(1.0, (250.0, 200.0), color));
      }
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "10.0")]
  seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_24x30_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
