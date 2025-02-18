use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let height = 297f64;
  let width = 210f64;
  let w = 180.;
  let h = 260.;
  let granularity = 2f64;
  let counts = [4000];
  let max_count = 4000;
  let colors = vec!["orange"];
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();

  let sx = (width - w) / 2.;
  let sy = (height - h) / 2.;
  let candidates: Vec<Vec<(f64, f64)>> = (0..max_count)
    .map(|i| {
      let mut route = Vec::new();
      if i % 2 == 0 {
        let x = sx + rng.gen_range(0., w);
        let y = sy + rng.gen_range(-0.3 * h, h);
        let len = rng.gen_range(0.1 * h, 0.8 * h);
        let y_from = y.max(sy);
        let y_to = (y + len).min(sy + h);
        let mut yp = y_from;
        loop {
          if yp > y_to {
            break;
          }
          let xp =
            x + 0.5 * perlin.get([0.03 * x, 0.03 * yp, opts.seed + i as f64]);
          route.push((xp, yp));
          yp += granularity;
        }
      } else {
        let y = sy + rng.gen_range(0., h);
        let x = sx + rng.gen_range(-0.3 * w, w);
        let len = rng.gen_range(0.1 * w, 0.8 * w);
        let x_from = x.max(sx);
        let x_to = (x + len).min(sx + w);
        let mut xp = x_from;
        loop {
          if xp > x_to {
            break;
          }
          let yp =
            y + 0.5 * perlin.get([0.03 * xp, 0.03 * y, opts.seed + i as f64]);
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
        l = l.add(signature(1.0, (170.0, 280.0), color));
      }
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
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
