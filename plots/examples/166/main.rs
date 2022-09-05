use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "3.0")]
  seed: f64,
  #[clap(short, long, default_value = "40")]
  samples: usize,
}

fn art(opts: Opts) -> Vec<Group> {
  let (width, height) = (297., 210.);
  let precision = 0.2;
  let pad = 10.;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  let bounds = (pad, pad, width - pad, height - pad);

  let colors = vec!["blue"];

  fn length(l: (f64, f64)) -> f64 {
    (l.0 * l.0 + l.1 * l.1).sqrt()
  }

  let f = |(x, y): (f64, f64)| {
    let f1 = 80.;
    let f2 = 3.;
    let f3 = 20.;
    let mut c = ((x - 0.5) * width / height, y - 0.5);
    c = p_r(c, PI / 4.);
    0.8
      + smoothstep(0.3, 0., length(c))
      + -2.5 * length(c)
      + 0.3 * (c.0 * f1).cos()
      + 0.3 * (0.8 + c.1 * f1).sin()
      + 0.3 * perlin.get([f2 * c.0, f2 * c.1, opts.seed])
      + 0.1 * perlin.get([f3 * c.0, f3 * c.1, opts.seed + 10.])
  };
  let samples = opts.samples;

  colors
    .iter()
    .enumerate()
    .map(|(_ci, &color)| {
      let pattern = (2., 6.);
      let thresholds: Vec<f64> = (0..samples)
        .map(|i| {
          (i as f64 + pattern.1 * (i as f64 / pattern.0).floor())
            / (samples as f64 * (pattern.0 + pattern.1) / pattern.0).floor()
        })
        .collect();

      let res = contour(w, h, f, &thresholds);
      let mut routes = features_to_routes(res, precision);
      routes = crop_routes(&routes, bounds);
      let mut data = Data::new();
      let csamples = 64;
      let collider = |a, b| {
        if !strictly_in_boundaries(a, bounds) {
          return Some(a);
        }
        return None;
      };
      for route in routes {
        data = render_route_collide(data, route, &collider);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l = l.add(signature(1.0, (220., 194.), color));
      l
    })
    .collect()
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
