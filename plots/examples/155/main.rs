use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let (width, height, sign_pos) = (297., 210., (220.0, 176.));
  let precision = 0.2;
  let pad = 20.;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  let bounds = (pad, pad, width - pad, height - pad);

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
    let c = ((x - 0.5) * width / height, y - 0.5);
    let mut s = 100f64;
    let k = 0.18;
    s = op_union_round(s, sdf_box2((c.0, c.1), (0.2, 0.2)), k);
    s = op_union_round(s, length((c.0, c.1)) - 0.25, k);
    s = op_union_round(
      s,
      sdf_box2(p_r((c.0 - 0.25, c.1), PI / 4.), (0.1, 0.1)),
      k,
    );
    s = op_union_round(
      s,
      sdf_box2(p_r((c.0 + 0.25, c.1), PI / 4.), (0.1, 0.1)),
      k,
    );
    let f1 = 8.8;
    let f2 = 20.8;
    let f3 = 40.8;
    let a1 = 0.02;
    let a2 = 0.5;
    let a3 = 0.2;
    let n = a1
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
    lerp(-0.35, 0.2, s) + n
  };
  let samples = opts.samples;
  let thresholds: Vec<f64> =
    (0..samples).map(|i| i as f64 / (samples as f64)).collect();

  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, bounds);
  let mut data = Data::new();
  for route in routes {
    data = render_route(data, route);
  }
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
  #[clap(short, long, default_value = "200")]
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
