use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let (width, height, sign_pos) = (297., 210., (250.0, 190.));
  let precision = 0.2;
  let pad = 20.;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  let bounds = (pad, pad, width - pad, height - pad);

  let f = |(x, y): (f64, f64)| {
    let c = ((x - 0.5) * width / height, y - 0.5);
    let f1 = 8.8;
    let f2 = 12.8;
    let f3 = 20.8;
    let a1 = 0.05;
    let a2 = 0.3;
    let a3 = 0.8;
    let b1 = 0.25;
    let n1 = a1
      * perlin.get([
        f1 * c.0,
        f1 * c.1,
        opts.seed
          + a2
            * perlin.get([
              4. + opts.seed,
              f2 * c.0 + a3 * perlin.get([f3 * c.0, f3 * c.1, 20. + opts.seed]),
              f2 * c.1 + a3 * perlin.get([f3 * c.0, f3 * c.1, 30. + opts.seed]),
            ]),
      ]);
    let n2 = b1
      * perlin
        .get([1.6 * c.0, 1.6 * c.1, opts.seed + 100.])
        .powf(2.);
    0.3 + 0.7 * x + n1 + n2
  };
  let samples = opts.samples;
  let thresholds: Vec<f64> =
    (0..samples).map(|i| i as f64 / (samples as f64)).collect();

  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, bounds);
  let mut data = Data::new();
  let collider = |a, b| collide_segment_boundaries(a, b, bounds);
  for route in routes {
    data = render_route_collide(data, route, &collider);
  }
  data = render_route(data, boundaries_route(bounds));
  let color = "black";
  let mut l = layer(color);
  l = l.add(base_path(color, 0.25, data));
  l = l.add(signature(0.8, sign_pos, color));
  vec![l]
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "2.0")]
  seed: f64,
  #[clap(short, long, default_value = "100")]
  samples: usize,
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
