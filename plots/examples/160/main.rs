use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "85.0")]
  seed: f64,
  #[clap(short, long, default_value = "200")]
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

  let f = |(x, y): (f64, f64)| {
    let c = ((x - 0.5) * width / height, y - 0.5);
    let f1 = 2.;
    let f2 = 4.;
    let f3 = 8.;
    let a1 = 0.2;
    let a2 = 0.4;
    let a3 = 1.2;
    let n1 = a1
      * perlin.get([
        f1 * c.0,
        f1 * c.1,
        opts.seed
          + a2
            * perlin.get([
              opts.seed - 10.,
              f2 * c.0 + a3 * perlin.get([f3 * c.0, f3 * c.1, 20. + opts.seed]),
              f2 * c.1 + a3 * perlin.get([f3 * c.0, f3 * c.1, 30. + opts.seed]),
            ]),
      ]);
    0.4 + 0.5 * sdf_box2(c, (0.2, 0.2)) + n1
  };
  let samples = opts.samples;

  colors
    .iter()
    .enumerate()
    .map(|(ci, &color)| {
      let pattern = (3., 10.);
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
      let radius = 80.;
      let in_circle = |p| euclidian_dist(p, (width / 2., height / 2.)) < radius;
      let mut circle: Vec<(f64, f64)> = (0..csamples)
        .map(|i| {
          let a = PI * 2. * i as f64 / csamples as f64;
          (
            width / 2. + radius * a.cos(),
            height / 2. + radius * a.sin(),
          )
        })
        .collect();
      circle.push(circle[0]);
      let collider = |a, b| {
        if !in_circle(a) && !in_circle(b) {
          return Some(a);
        }
        collide_route_segment(&circle, a, b)
      };
      for route in routes {
        data = render_route_collide(data, route, &collider);
      }
      if ci == 0 {
        data = render_route(data, circle);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l = l.add(signature(1.0, (200., 180.), color));
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
