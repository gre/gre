use clap::*;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

// "Rain"

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "25.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

#[derive(Clone, Copy, Debug)]
struct Umbrella {
  x: f64,
  y: f64,
  width: f64,
}

impl Umbrella {
  fn new(x: f64, y: f64, width: f64) -> Self {
    Umbrella { x, y, width }
  }
  fn curve_y(self: &Self, x: f64) -> f64 {
    let dx = self.x;
    let dy = self.y;
    ((x - dx).abs() / (0.2 * self.width)).exp() + dy
  }
  fn rain(self: &Self, p: (f64, f64)) -> (f64, f64) {
    let x1 = self.x - self.width / 2.0;
    let x2 = self.x + self.width / 2.0;
    let h = 0.5 * self.width;
    if self.under_with_distance(p).is_some() && p.1 < self.y + h {
      (if p.0 < self.x { x1 } else { x2 }, self.y + h)
    } else {
      p
    }
  }
  fn just_above(self: &Self, p: (f64, f64), m: f64) -> bool {
    let x1 = self.x - self.width / 2.0;
    let x2 = self.x + self.width / 2.0;
    if x1 < p.0 && p.0 < x2 {
      let cy = self.curve_y(p.0);
      if cy - m < p.1 && p.1 < cy {
        return true;
      }
    }
    return false;
  }
  fn under_with_distance(self: &Self, p: (f64, f64)) -> Option<f64> {
    let x1 = self.x - self.width / 2.0;
    let x2 = self.x + self.width / 2.0;
    if x1 < p.0 && p.0 < x2 {
      let cy = self.curve_y(p.0);
      if p.1 > cy {
        return Some(p.1 - self.y);
      }
    }
    return None;
  }
  fn draw<R: Rng>(self: &Self, rng: &mut R) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();
    let y = self.y;
    let x1 = self.x - self.width / 2.0;
    let x2 = self.x + self.width / 2.0;
    let mut x = x1;
    let mut route = Vec::new();
    loop {
      if x > x2 {
        break;
      }
      let cy = self.curve_y(x);
      route.push((x, cy));
      x += 0.2;
    }
    routes.push(route.clone());
    routes.push(route.iter().map(|p| (p.0, p.1 + 0.3)).collect());
    routes.push(route.iter().map(|p| (p.0, p.1 + 0.6)).collect());
    routes.push(route.iter().map(|p| (p.0, p.1 + 0.9)).collect());
    let mut route = Vec::new();
    route.push((self.x, y - self.width * 0.02));
    let mut p = (self.x, y + 0.7 * self.width);
    for i in 0..rng.gen_range(9, 11) {
      route.push(p);
      let a = PI / 2.0 + i as f64 / 2.0;
      let amp = rng.gen_range(0.6, 0.7);
      p = (p.0 + amp * a.cos(), p.1 + amp * a.sin());
    }
    routes.push(route);
    routes
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let colors = vec!["white", "gold"];
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;
  let mut rng = rng_from_seed(opts.seed);
  let mut passage = Passage2DCounter::new(1.0, width, height);

  let raineffect = rng.gen_range(0.4, 2.0);
  let umbrellaystart = 0.05 * height;
  let umbrellaystop = 0.85 * height;
  let umbrellas_count = rng.gen_range(5, 12);
  let absorption = rng.gen_range(0.7, 1.0);
  let dispersion = rng.gen_range(0.2, 0.8);
  let density = 0.6;
  let bounce = 0.5;
  let protection_dist = rng.gen_range(30.0, 120.0);

  let umbrellas: Vec<Umbrella> = (0..umbrellas_count)
    .map(|i| {
      let width = rng.gen_range(0.17, 0.24) * opts.width;
      let dx = opts.width / 2.0 - pad - width / 2.0;
      let x =
        opts.width / 2.0 + rng.gen_range(-dx, dx) * rng.gen_range(0.5, 1.0);
      let h = (umbrellaystop - umbrellaystart) / (umbrellas_count as f64);
      let cy = umbrellaystart + (i as f64 + 0.5) * h;
      let y = cy
        + rng.gen_range(-h / 2.0, h / 2.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0);
      Umbrella::new(x, y, width)
    })
    .collect();

  let count =
    ((opts.width - 2.0 * pad) * (opts.height - 2.0 * pad) * density) as usize;

  let drops: Vec<((f64, f64), bool)> = (0..count)
    .map(|_i| {
      let x = rng.gen_range(pad, width - pad);
      let y = rng.gen_range(pad, height - pad);
      let mut p = (x, 0.0);
      loop {
        if p.1 > y {
          break;
        }
        for u in umbrellas.iter() {
          if rng.gen_bool(1.0 - absorption) && y - p.1 > 0.3 * u.width {
            p = u.rain(p);
          }
          if p.1 > y {
            break;
          }
        }
        if p.1 > y {
          break;
        }
        p.1 += 1.0;
        p.0 += dispersion * rng.gen_range(-1.0, 1.0);
      }
      p.1 = y;
      (p, rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))
    })
    .filter_map(|(p, v, v2)| {
      if p.0 < pad || p.0 > width - pad {
        return None;
      }
      let mut count = 0.0;
      let mut q = p;
      let mut rebounce = false;
      for (i, umbrella) in umbrellas.iter().enumerate() {
        if umbrella.just_above(p, bounce) {
          q.1 -= bounce * (1.0 + v2);
          rebounce = true;
          break;
        }
        if let Some(d) = umbrella.under_with_distance(p) {
          if v > (d / protection_dist).powf(2.0) {
            count += 2.0 / (i as f64 * 0.1 + 1.0);
          }
        }
      }
      if count * raineffect > v2 {
        return None;
      }
      if passage.count(q) > 3 {
        return None;
      }
      return Some((q, rebounce));
    })
    .collect();

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      if i == 0 {
        for &(p, rebounces) in drops.iter() {
          let mut dx = rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0);
          if rebounces {
            dx += rng.gen_range(-2.0, 2.0);
          }
          data = data.move_to((p.0 + dx, p.1 - rng.gen_range(1.0, 2.0)));
          data = data.line_to(p);
        }
      } else {
        for u in umbrellas.iter() {
          for route in u.draw(&mut rng) {
            data = render_route(data, route);
          }
        }
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.5, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
