use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let height = 210f64;
  let width = 297f64;
  let h = height - 40.;
  let w = width - 40.;
  let granularity = 5f64;
  let counts = [2000];
  let max_count = 2000;
  let colors = vec!["darkblue"];
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();

  let sx = (width - w) / 2.;
  let sy = (height - h) / 2.;

  let f = |t: f64| {
    // quad
    if t < 0.5 {
      2.0 * t * t
    } else {
      -1.0 + (4.0 - 2.0 * t) * t
    }
  };

  let candidates: Vec<Vec<(f64, f64)>> = (0..max_count)
    .map(|i| {
      let mut route = Vec::new();
      let s1 = f(rng.gen_range(0., 1.0));
      let s2 = 1.3 * f(rng.gen_range(0.0, 1.0)) - 0.3;
      let l1 = rng.gen_range(0.1, 0.8);
      if i % 2 == 0 {
        let x = sx + w * s1;
        let y = sy + h * s2;
        let len = h * l1;
        let y_from = y.max(sy);
        let y_to = (y + len).min(sy + h);
        let mut yp = y_from;
        loop {
          if yp > y_to {
            break;
          }
          let xp = x + perlin.get([0.05 * x, 0.05 * yp, opts.seed + i as f64]);
          route.push((xp, yp));
          yp += granularity;
        }
      } else {
        let x = sx + w * s2;
        let y = sy + h * s1;
        let len = w * l1;
        let x_from = x.max(sx);
        let x_to = (x + len).min(sx + w);
        let mut xp = x_from;
        loop {
          if xp > x_to {
            break;
          }
          let yp = y + perlin.get([0.05 * xp, 0.05 * y, opts.seed + i as f64]);
          route.push((xp, yp));
          xp += granularity;
        }
      }
      route
    })
    .filter(|r| r.len() >= 2)
    .collect();

  colors
    .iter()
    .enumerate()
    .map(|(g, color)| {
      let count = counts[g];
      let mut routes = candidates.clone();
      rng.shuffle(&mut routes);
      routes.truncate(count);
      let data = routes
        .iter()
        .fold(Data::new(), |data, route| render_route(data, route.clone()));
      let mut l = layer(color);
      l = l.add(base_path(color, 0.3, data));
      if g == colors.len() - 1 {
        l = l.add(signature(1.0, (250.0, 192.0), color));
      }
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "2.0")]
  seed: f64,
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
