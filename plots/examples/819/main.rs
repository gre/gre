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
  #[clap(short, long, default_value = "420.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();

  let mut routes = Vec::new();
  let mut route = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let line_distance = 0.3;
  let precision = 0.3;
  let threshold_distance_center = 20.0;
  //let ang_divisions = 10000;
  //let mut height_map = vec![9999.0; ang_divisions];
  let divisions = 2000;
  let mut height_map = vec![9999.0; divisions * 4];
  let height_map_gap = 0.0;
  let margin = 100.0;
  let mut p = (-margin, -margin);
  let mut angle = 0.0;
  let mut horizontal = true;
  let mut distance_current_segment = 0.0;
  let mut progress_inside = 0.0;
  let mut i = 0;
  let yfactor = rng.gen_range(1.0, 3.0);
  loop {
    if euclidian_dist(p, (width / 2.0, height / 2.0))
      < threshold_distance_center
    {
      break;
    }

    let length = if horizontal {
      width + 2.0 * margin
    } else {
      height + 2.0 * margin
    };

    if distance_current_segment > length - progress_inside {
      // switch to next segment. turn 90°
      distance_current_segment = 0.0;
      progress_inside += line_distance;
      angle += PI / 2.0;
      horizontal = !horizontal;
      i = (i + 1) % 4;
    }

    // increment one step
    p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());

    let a = angle + PI / 2.0;
    let dx = a.cos();
    let dy = a.sin();
    let mut q = p;

    let s = if horizontal { q } else { (q.1, q.0) };
    let mut v = 0.0;
    let amp = 20.0;
    let f = 1. / amp;
    v += amp * perlin.get([f * s.0, yfactor * f * s.1, 77.4 + opts.seed / 7.7]);

    q.0 += v * dx;
    q.1 += v * dy;

    let s = if horizontal { q } else { (q.1, q.0) };
    let mut v = 0.0;
    let amp = 60.0;
    let f = 0.5 / amp;
    v += amp * perlin.get([f * s.0, f * s.1, 37.4 + opts.seed / 2.7]);

    q.0 += v * dx;
    q.1 += v * dy;

    let s = if horizontal { q } else { (q.1, q.0) };
    let mut v = 0.0;
    let amp = 10.0;
    let f = 0.3 / amp;
    v +=
      amp * perlin.get([f * s.0, yfactor * f * s.1, 977.4 + opts.seed / 17.7]);

    q.0 += v * dx;
    q.1 += v * dy;

    let s = if horizontal { q } else { (q.1, q.0) };
    let mut v = 0.0;
    let amp = 5.0;
    let f = 0.4;
    v += amp
      * perlin.get([f * s.0, f * s.1, 70.4 + opts.seed / 2.7])
      * perlin
        .get([0.05 * f * s.0, 0.05 * f * s.1, 97.4 + opts.seed / 72.7])
        .max(0.0);

    q.0 += v * dx;
    q.1 += v * dy;

    /*

    let a = angle + PI / 2.0;
    let dx = a.cos();
    let dy = a.sin();
    let mut q = p;

    for i in 0..3 {
      let s = if horizontal { q } else { (q.1, q.0) };
      let mut v = 0.0;
      let amp = rng.gen_range(5.0, 60.0);
      let f = 0.5 / amp;
      let f0 = rng.gen_range(0.5, 2.0);
      let f1 = 1.0 / f0;
      v += amp
        * perlin.get([
          f0 * f * s.0,
          f1 * f * s.1,
          i as f64 * 37.4 + opts.seed / (7.7 + i as f64),
        ]);
      q.0 += v * dx;
      q.1 += v * dy;
    }
    */

    /*
    let center_ang =
      ((q.1 - height / 2.0).atan2(q.0 - width / 2.0) + 2.0 * PI) % (2.0 * PI);
    let index =
      (ang_divisions as f64 * center_ang / (2.0 * PI)).floor() as usize;
    let d = euclidian_dist(q, (width / 2.0, height / 2.0));

    */

    let j = if horizontal {
      ((divisions as f64 * p.0 / width).round().max(0.0) as usize)
        .min(divisions - 1)
    } else {
      ((divisions as f64 * p.1 / height).round().max(0.0) as usize)
        .min(divisions - 1)
    };
    let d = if horizontal {
      (q.1 - height / 2.0).abs()
    } else {
      (q.0 - width / 2.0).abs()
    };
    let index = i * divisions + j;

    if
    // d > height_map_per_angle[index]
    d - height_map_gap > height_map[index]
      || !strictly_in_boundaries(q, bound)
      || passage.count(q) > 6
    {
      // stop the line
      if route.len() > 1 {
        routes.push(route);
        route = Vec::new();
      } else if route.len() > 0 {
        route = Vec::new();
      }
    } else {
      // drawing is allowed
      // height_map_per_angle[index] = d;
      height_map[index] = d;
      route.push(q);
    }

    distance_current_segment += precision;
  }

  if route.len() > 1 {
    routes.push(route);
  }

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }

  pub fn count_once(self: &mut Self, p: (f64, f64)) {
    let i = self.index(p);
    let v = self.counters[i];
    if v == 0 {
      self.counters[i] = 1;
    }
  }

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }

  pub fn add(self: &Self, other: &Self) -> Self {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters = self
      .counters
      .iter()
      .enumerate()
      .map(|(i, v)| v + other.counters[i])
      .collect();
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  pub fn prepare<F: Fn((f64, f64)) -> usize>(self: &mut Self, f: F) {
    let mut x = 0.0;
    loop {
      if x >= self.width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= self.height {
          break;
        }
        let index = self.index((x, y));
        self.counters[index] = f((x, y));
        y += self.precision;
      }
      x += self.precision;
    }
  }

  pub fn grow_passage(self: &mut Self, radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters: Vec<usize> = self.counters.iter().cloned().collect();
    let mut mask = Vec::new();
    // TODO, in future for even better perf, I will rewrite this
    // working directly with index integers instead of having to use index() / count_once()
    let mut x = -radius;
    loop {
      if x >= radius {
        break;
      }
      let mut y = -radius;
      loop {
        if y >= radius {
          break;
        }
        if x * x + y * y < radius * radius {
          mask.push((x, y));
        }
        y += precision;
      }
      x += precision;
    }

    let mut x = 0.0;
    loop {
      if x >= width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= height {
          break;
        }
        let index = self.index((x, y));
        if counters[index] > 0 {
          for &(dx, dy) in mask.iter() {
            self.count_once((x + dx, y + dy));
          }
        }
        y += precision;
      }
      x += precision;
    }
  }

  pub fn search_space<R: Rng>(
    self: &Self,
    rng: &mut R,
    min_r: f64,
    max_r: f64,
    pad: f64,
    max_search: usize,
  ) -> Option<(f64, f64)> {
    for j in 0..max_search {
      let optim_r = mix(min_r, max_r, 1.0 / (1.0 + j as f64 * 0.01));
      let minx = pad + optim_r;
      let miny = pad + optim_r;
      let maxx = self.width - pad - optim_r;
      let maxy = self.height - pad - optim_r;

      if minx >= maxx || miny >= maxy {
        break;
      }

      let p = (rng.gen_range(minx, maxx), rng.gen_range(miny, maxy));
      if self.get(p) == 0
        && self.get((p.0 - optim_r, p.1)) == 0
        && self.get((p.0 + optim_r, p.1)) == 0
        && self.get((p.0, p.1 - optim_r)) == 0
        && self.get((p.0, p.1 + optim_r)) == 0
      {
        return Some(p);
      }
    }

    None
  }
}
