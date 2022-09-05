use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

fn flower(seed: f64, center: (f64, f64), radius: f64) -> Vec<Vec<(f64, f64)>> {
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();
  let perlin = Perlin::new();
  let mut rng = rng_from_seed(seed);

  let count = (10. * radius) as usize;
  let golden_ratio = (1. + (5f64).sqrt()) / 2.;
  let d = radius / 40.0;
  for i in 0..count {
    let k = i as f64 / (count as f64);
    let a = 2. * PI * (i as f64) / (golden_ratio * golden_ratio);
    let r = 0.5 * radius * k.sqrt() - 0.5 * d;
    let ad = 0.4 + 0.4 * k;
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    let x2 = x + d * (a + ad).cos();
    let y2 = y + d * (a + ad).sin();
    let x3 = x + d * (a - ad).cos();
    let y3 = y + d * (a - ad).sin();
    let x2h = x + d * (a + 0.5 * ad).cos();
    let y2h = y + d * (a + 0.5 * ad).sin();
    let x3h = x + d * (a - 0.5 * ad).cos();
    let y3h = y + d * (a - 0.5 * ad).sin();
    let route = vec![
      (x3, y3),
      (x, y),
      (x2, y2),
      (x3, y3),
      (x3h, y3h),
      (x, y),
      (x2h, y2h),
      (x3h, y3h),
    ];
    routes.push(route);
  }

  let count = rng.gen_range(16, 30);
  for i in 0..count {
    let k = i as f64 / (count as f64);
    let main_a = 2. * PI * k;
    let init_a = main_a + rng.gen_range(-0.2, 0.2);

    let max = rng.gen_range(0.4, 0.6);
    let aamp = 0.8 - 0.4 * k;
    let lines = (0.6 * aamp * radius) as usize;
    for l in 0..lines {
      let kl = l as f64 / (lines as f64);
      let a = main_a + aamp * (kl - 0.5);
      let mut route = Vec::new();
      let r = 0.5 * radius;
      let mut x = center.0 + r * a.cos();
      let mut y = center.1 + r * a.sin();
      route.push((x, y));
      let m = max * (1. - 2. * (kl - 0.5).abs());
      let iterations = (radius * m) as usize;
      let mut a = init_a;
      let mut acc_diff = 0.0;
      let amp = 0.1;
      for j in 0..iterations {
        let mut d = amp
          * perlin.get([
            0.001 * x + seed - 70.7 * (k as f64),
            0.001 * y + 1.7 * (k as f64),
            0.1 * perlin.get([0.003 * x, 0.003 * y, seed]),
          ]);
        acc_diff += d;
        if j > iterations / 2 {
          if (acc_diff < 0.) == (d < 0.) {
            d *= -1.0;
          }
        }
        a += d;
        x += a.cos();
        y += a.sin();
        route.push((x, y));
      }
      routes.push(route);
    }
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = 297.;
  let height = 210.;

  let colors = vec!["#f90", "#09F"];

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      let mut l = layer(color);
      if i == 0 {
        let routes = flower(opts.seed, (width / 2., height / 2.), 80.0);
        for route in routes {
          data = render_route(data, route);
        }
      }
      l = l.add(base_path(color, 0.35, data));

      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
