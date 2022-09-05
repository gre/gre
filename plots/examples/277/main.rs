use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "400")]
  samples: usize,
}

fn art(opts: Opts) -> Vec<Group> {
  let (width, height) = (297., 210.);
  let precision = 0.2;
  let pad = 20.;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  let bounds = (pad, pad, width - pad, height - pad);

  let colors = vec!["black"];

  fn length(l: (f64, f64)) -> f64 {
    (l.0 * l.0 + l.1 * l.1).sqrt()
  }
  fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
    (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
  }
  fn op_union_round(a: f64, b: f64, r: f64) -> f64 {
    r.max(a.min(b)) - length(((r - a).max(0.), (r - b).max(0.)))
  }
  let sdf_box2 = |(x, y): (f64, f64), (w, h): (f64, f64)| {
    let dx = x.abs() - w;
    let dy = y.abs() - h;
    length((dx.max(0.), dy.max(0.))) + dx.min(0.).max(dy.min(0.))
  };

  let f = |(x, y): (f64, f64)| {
    let mut rng = rng_from_seed(8.38 * opts.seed);
    let mut c = ((x - 0.5) * width / height, y - 0.5);
    if rng.gen_range(0.0, 1.0) < 0.3 {
      c.0 = c.0.abs();
    }
    if rng.gen_range(0.0, 1.0) < 0.3 {
      c.1 = c.1.abs();
    }
    let res =
      (1. + rng.gen_range(0.0, 24.0) * rng.gen_range(0.0, 1.0)) as usize;
    let mut s = 100f64;
    let k = rng.gen_range(0.0, 0.3);
    for _i in 0..res {
      let mut p = (c.0, c.1);
      let ang = rng.gen_range(0f64, PI);
      p.0 += rng.gen_range(-0.3, 0.3);
      p.1 += rng.gen_range(-0.1, 0.1);
      p = p_r(p, ang);
      let dim = (rng.gen_range(0.0, 0.25), rng.gen_range(0.0, 0.2));
      s = op_union_round(s, sdf_box2(p, dim), k);
    }
    let f1 = rng.gen_range(0.2, 3.0);
    let f2 = rng.gen_range(1.0, 8.0);
    let f3 = rng.gen_range(2.0, 12.0);
    let a1 = rng.gen_range(0.02, 0.1);
    let a2 = rng.gen_range(0.0, 2.0);
    let a3 = rng.gen_range(0.0, 2.0);
    let n = a1
      * perlin.get([
        f1 * c.0,
        f1 * c.1,
        opts.seed
          + a2
            * perlin.get([
              7. + opts.seed,
              f2 * c.0 + a3 * perlin.get([f3 * c.0, f3 * c.1, 1. + opts.seed]),
              f2 * c.1 + a3 * perlin.get([f3 * c.0, f3 * c.1, 2. + opts.seed]),
            ]),
      ]);
    lerp(-1.2, 0.1, s) + n
  };
  let samples = opts.samples;

  colors
    .iter()
    .enumerate()
    .map(|(_ci, &color)| {
      let pattern = (2., 2.);
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
      for route in routes {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      // l = l.add(signature(1.0, (200., 180.), color));
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
