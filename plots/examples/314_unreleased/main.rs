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
  #[clap(short, long, default_value = "100.0")]
  seed: f64,
  #[clap(short, long, default_value = "100")]
  samples: usize,
}

fn art(opts: Opts) -> Vec<Group> {
  let (width, height) = (210., 297.);
  let precision = 1.0;
  let pad = 14.;
  let w = ((width - 2. * pad) as f64 / precision) as u32;
  let h = ((height - 2. * pad) as f64 / precision) as u32;
  let perlin = Perlin::new();

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

  let samples = opts.samples;

  let colors = vec!["red", "cyan"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, &color)| {
      let mut l = layer(color);
      for r in vec![-1.0, 1.0] {
        let f = |(x, y): (f64, f64)| {
          let mut rng = rng_from_seed(700. + 8.738 * opts.seed);
          let mut c = ((x - 0.5) * width / height, y - 0.5);
          c = p_r(c, r * 2.0 * PI / 3.);
          if rng.gen_range(0.0, 1.0) < 0.3 {
            c.0 = c.0.abs();
          }
          if rng.gen_range(0.0, 1.0) < 0.3 {
            c.1 = c.1.abs();
          }
          let res = (rng.gen_range(2., 80.) * rng.gen_range(0.0, 1.0)) as usize;
          let mut s = 100f64;
          let k = rng.gen_range(0.0, 0.2);
          for _i in 0..res {
            let mut p = (c.0, c.1);
            let ang = rng.gen_range(0f64, PI);
            p.1 += rng.gen_range(-0.2, 0.2);
            p.0 += rng.gen_range(-0.1, 0.1);
            p = p_r(p, ang);
            let dim = (rng.gen_range(0.0, 0.2), rng.gen_range(0.0, 0.2));
            s = op_union_round(s, sdf_box2(p, dim), k);
          }
          let f1 = rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0);
          let f2 = rng.gen_range(0.0, 16.0) * rng.gen_range(0.0, 1.0);
          let f3 = rng.gen_range(0.0, 24.0) * rng.gen_range(0.0, 1.0);
          let a2 = 2.0 * rng.gen_range(0.0, 1f64).powf(2.0);
          let a3 = 2.0 * rng.gen_range(0.0, 1f64).powf(2.0);
          let n = 0.05
            * rng.gen_range(0.0, 1f64)
            * perlin.get([
              f1 * c.0,
              f1 * c.1,
              100.0
                + opts.seed * 7.3
                + a2
                  * perlin.get([
                    opts.seed
                      + rng.gen_range(0.0, 1.0f64).powf(4.0) * (ci as f64),
                    f2 * c.0
                      + a3
                        * perlin.get([
                          f3 * c.0,
                          f3 * c.1,
                          10. + 0.7 * opts.seed,
                        ]),
                    f2 * c.1
                      + a3
                        * perlin.get([
                          f3 * c.0,
                          f3 * c.1,
                          20. + 1.2 * opts.seed,
                        ]),
                  ]),
            ]);
          lerp(-0.5, 0.0, s) + n
        };

        let thresholds: Vec<f64> = (0..samples)
          .map(|i| {
            ((ci + colors.len() * i) as f64) / ((colors.len() * samples) as f64)
          })
          .collect();
        let res = contour(w, h, f, &thresholds);
        let routes = features_to_routes(res, precision);
        let mut data = Data::new();
        let inside = |from: (f64, f64), to| {
          let perlin = Perlin::new();
          let mut rng = rng_from_seed(2.38 * opts.seed);
          let nz = rng.gen_range(0.01, 0.05);
          let nz2 = rng.gen_range(0.0, 4.0) * nz;
          let nz3 = rng.gen_range(0.0, 8.0) * nz;
          strictly_in_boundaries(from, (pad, pad, width - pad, height - pad))
            && strictly_in_boundaries(to, (pad, pad, width - pad, height - pad))
            && perlin.get([
              nz * from.0,
              nz * from.1,
              5.5 * opts.seed
                + ci as f64 * rng.gen_range(0.0, 1f64).powf(3.0)
                + r * rng.gen_range(0.0, 1f64).powf(3.0)
                + rng.gen_range(0.0, 1.0)
                  * perlin.get([
                    0.7
                      + opts.seed * 6.3
                      + rng.gen_range(0.0, 1.0)
                        * perlin.get([opts.seed, nz3 * to.0, nz3 * to.1]),
                    nz2 * from.0,
                    nz3 * from.1,
                  ]),
            ]) < rng.gen_range(0.2, 0.5)
        };
        for route in routes.clone() {
          let r = route.iter().map(|&p| (p.0 + pad, p.1 + pad)).collect();
          data = render_route_when(data, r, inside);
        }
        l = l.add(base_path(color, 0.35, data));
      }
      // l = l.add(signature(1.0, (200., 180.), color));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_portrait("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
