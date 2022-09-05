use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "8.0")]
  seed: f64,
  #[clap(short, long, default_value = "60")]
  samples: usize,
}

fn art(opts: Opts) -> Vec<Group> {
  let (width, height) = (297., 210.);
  let precision = 0.2;
  let pad = 20.;
  let k = 0.32;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  let bounds = (pad, pad, width - pad, height - pad);

  let colors = vec!["black"];

  let f = |(x, y): (f64, f64)| {
    let c = ((x - 0.5) * width / height, y - 0.5);
    let r = 0.15;
    let off = 0.45;
    let offy = 0.0;
    let c1 = (c.0 - off, c.1 + offy);
    let c2 = (c.0, c.1 - offy);
    let c3 = (c.0 + off, c.1 + offy);
    let f1 = 4.;
    let f2 = 5.;
    let f3 = 8.;
    let a1 = 0.35;
    let a2 = 0.8;
    let a3 = 0.9;
    let n1 = a1
      * perlin.get([
        f1 * c.0,
        f1 * c.1,
        opts.seed
          + a2
            * perlin.get([
              opts.seed - 10.
                + a3 * perlin.get([f3 * c.0, f3 * c.1, 20. + opts.seed]),
              f2 * c.0,
              f2 * c.1,
            ]),
      ]);
    -0.4
      + f_op_union_round(
        length(c1),
        f_op_union_round(length(c2), length(c3), k),
        k,
      ) / r
      + n1
  };
  let samples = opts.samples;

  colors
    .iter()
    .enumerate()
    .map(|(_ci, &color)| {
      let pattern = (2., 10.);
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
      for route in routes {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l = l.add(signature(1.0, (180., 180.), color));
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
