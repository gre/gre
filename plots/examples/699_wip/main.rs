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
  #[clap(short, long, default_value = "105.0")]
  pub width: f64,
  #[clap(short, long, default_value = "148.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "221.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;

  // Prepare all the random values
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();
  let peakfactor = 0.001 * rng.gen_range(0.0, 1.0);
  let stopy = rng.gen_range(-0.1, 0.3) * height;
  let ampfactor = rng.gen_range(0.0, 0.25) * rng.gen_range(0.0, 1.0);
  let ynoisefactor = rng.gen_range(0.0, 0.2) * rng.gen_range(0.2, 1.0);
  let yincr = rng.gen_range(0.3, 0.4);
  let amp2 = rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0);
  let ymul = rng.gen_range(0.0, 0.01)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);
  let precision = rng.gen_range(0.08, 0.12);
  let min_route = 3;
  let offsetstrategy = 1; //rng.gen_range(0, 5);

  let mut passage = Passage::new(0.5, width, height);
  let passage_threshold = 6;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  // Build the mountains bottom-up, with bunch of perlin noises
  let mut base_y = height * 5.0;
  let mut miny = height;
  let mut height_map: Vec<f64> = Vec::new();
  loop {
    if miny < stopy {
      break;
    }

    let mut route = Vec::new();
    let mut x = pad;
    let mut was_outside = true;
    let mut clr: usize = 0;
    loop {
      if x > width - pad {
        break;
      }
      let xv = (4.0 - base_y / height) * (x - width / 2.);

      let amp = height * ampfactor;
      let mut y = base_y;

      if offsetstrategy == 0 {
        y += amp * peakfactor * xv * xv;
      }

      y += -amp
        * perlin
          .get([
            //
            xv * 0.004 + 9.9,
            y * ymul - 3.1,
            77.
              + opts.seed / 7.3
              + perlin.get([
                //
                -opts.seed * 7.3,
                8.3 + xv * 0.015,
                y * 0.1,
              ]),
          ])
          .abs();

      if offsetstrategy == 1 {
        y += amp * peakfactor * xv * xv;
      }

      y += amp2
        * amp
        * perlin.get([
          //
          18.3 + xv * 0.007777,
          88.1 + y * ynoisefactor,
          opts.seed * 97.3,
        ]);

      if offsetstrategy == 2 {
        y += amp * peakfactor * xv * xv;
      }

      y += amp
        * perlin.get([
          //
          opts.seed * 9.3 + 77.77,
          xv * 0.081 + 90.33,
          y * 0.5,
        ])
        * perlin
          .get([
            //
            xv * 0.015 - 8.33,
            88.1 + y * 0.2,
            -opts.seed / 7.7 - 6.66,
          ])
          .min(0.0);

      if offsetstrategy == 3 {
        y += amp * peakfactor * xv * xv;
      }

      y += 0.1
        * amp
        * (1.0 - miny / height)
        * perlin.get([
          //
          6666.6 + opts.seed * 1.3,
          18.3 + xv * 0.5,
          88.1 + y * 0.5,
        ]);

      if offsetstrategy == 4 {
        y += amp * peakfactor * xv * xv;
      }

      clr = (4.0
        * smoothstep(height, stopy, base_y).powf(rng.gen_range(0.2, 0.5)))
      .max(0.0)
      .min(4.0) as usize;

      if y < miny {
        miny = y;
      }
      let mut collides = false;
      let xi = (x / precision) as usize;
      if xi >= height_map.len() {
        height_map.push(y);
      } else {
        if y > height_map[xi] {
          collides = true;
        } else {
          height_map[xi] = y;
        }
      }
      let inside =
        !collides && pad < x && x < width - pad && pad < y && y < height - pad;
      if inside && passage.get((x, y)) < passage_threshold {
        if was_outside {
          if route.len() > min_route {
            routes.push((clr, route));
          }
          route = Vec::new();
        }
        was_outside = false;
        route.push((x, y));
        passage.count((x, y));
      } else {
        was_outside = true;
      }

      x += precision;
    }

    if route.len() > min_route {
      routes.push((clr, route));
    }

    base_y -= yincr;
  }

  // calculate a moving average to smooth the stick men positions
  let smooth = 8;
  let sf = smooth as f64;
  let mut sum = 0.0;
  let mut acc = Vec::new();
  let mut smooth_heights: Vec<(f64, f64, f64)> = Vec::new();
  for (i, h) in height_map.iter().enumerate() {
    if acc.len() == smooth {
      let avg = sum / sf;
      let xtheoric = (i as f64 - sf / 2.0) * precision;

      let l = smooth_heights.len();
      let b = (xtheoric, avg, 0.0);
      let a = if l > 2 { smooth_heights[l - 2] } else { b };
      let rot = -PI / 2.0 + (b.0 - a.0).atan2(b.1 - a.1);
      let p = (xtheoric, avg, rot);
      smooth_heights.push(p);
      let prev = acc.remove(0);
      sum -= prev;
    }
    acc.push(h);
    sum += h;
  }

  passage.grow_passage(2.0);

  let does_overlap = |p| passage.get(p) == 0;

  let circles = packing(
    opts.seed,
    1000000,
    2000,
    1,
    0.4,
    (pad * 2.0, pad * 2.0, width - pad * 2.0, height - pad * 2.0),
    &does_overlap,
    0.8,
    2.0,
  );
  for c in circles {
    let count = (c.r * 2.0 + 8.0) as usize;
    routes.push((3, circle_route((c.x, c.y), c.r, count)));
  }

  // External frame to around the whole piece
  let mut route = Vec::new();
  for i in 0..8 {
    let d = i as f64 * 0.3;
    route.push((pad + d, pad + d));
    route.push((pad + d, height - pad - d));
    route.push((width - pad - d, height - pad - d));
    route.push((width - pad - d, pad + d));
    route.push((pad + d, pad + d));
  }
  routes.push((0, route));

  // Make the SVG
  vec!["#800", "#f00", "#f60", "#fc0"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (clr, route) in routes.clone() {
        if ci == clr {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
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
}

fn circle_route(center: (f64, f64), r: f64, count: usize) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f64,
  y: f64,
  r: f64,
}
impl VCircle {
  fn new(x: f64, y: f64, r: f64) -> Self {
    VCircle { x, y, r }
  }
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

fn scaling_search<F: FnMut(f64) -> bool>(
  mut f: F,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let mut from = min_scale;
  let mut to = max_scale;
  loop {
    if !f(from) {
      return None;
    }
    if to - from < 0.1 {
      return Some(from);
    }
    let middle = (to + from) / 2.0;
    if !f(middle) {
      to = middle;
    } else {
      from = middle;
    }
  }
}

fn search_circle_radius(
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y)) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) =
      search_circle_radius(&does_overlap, &circles, x, y, min_scale, max_scale)
    {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
}
