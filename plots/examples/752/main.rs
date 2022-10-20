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
  #[clap(short, long, default_value = "148.0")]
  pub width: f64,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let seed = opts.seed;
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  // Prepare all the random values
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let passage_threshold = 9;

  let circleang = if rng.gen_bool(0.5) { 0.5 } else { 0.0 };

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -10.;

  let mut height_map: Vec<f64> = Vec::new();
  let mut height_map_stop: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let precision = rng.gen_range(0.1, 0.5);
  let count = rng.gen_range(3, 7);
  for j in 0..count {
    let stopy = rng.gen_range(0.2, 0.8) * height;
    let xfreq = rng.gen_range(0.002, 0.01);
    let h = rng.gen_range(1.0, 5.0);
    let peakfactor = rng.gen_range(-0.003, 0.005) * rng.gen_range(0.0, 1.0);
    let ampfactor = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
    let ynoisefactor = rng.gen_range(0.02, 0.5);
    let yincr = 0.5 + rng.gen_range(-1f64, 8.0).max(0.0);
    let amp2 = rng.gen_range(1.0, 8.0);
    let offsetstrategy = rng.gen_range(0, 5);

    let rounding =
      rng.gen_range(-50.0f64, 4.0).max(0.0) * rng.gen_range(0.1, 1.0);

    let stopy =
      mix(height, stopy, (j as f64 / ((count - 1) as f64)) * 0.7 + 0.3);

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height * 5.0;
    let mut miny = height;
    loop {
      if miny < stopy {
        break;
      }

      let mut route = Vec::new();
      let mut x = mountainpadding;
      let mut was_outside = true;
      loop {
        if x > width - mountainpadding {
          break;
        }
        let xv = (h - base_y / height) * (x - width / 2.);

        let amp = height * ampfactor;
        let mut y = base_y;

        if offsetstrategy == 0 {
          y += amp * peakfactor * xv;
        }

        y += -amp
          * perlin
            .get([
              //
              xv * xfreq + 19.9,
              y * 0.0211 + 30.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  55. + seed * 7.3,
                  80.3 + xv * 0.015,
                  y * 0.1 + 111.3,
                ]),
            ])
            .abs();

        if offsetstrategy == 1 {
          y += amp * peakfactor * xv;
        }

        y += amp2
          * amp
          * perlin.get([
            //
            8.311 + xv * 0.00811,
            88.1 + y * ynoisefactor,
            seed * 97.311,
          ]);

        if offsetstrategy == 2 {
          y += amp * peakfactor * xv;
        }

        y += amp
          * perlin.get([
            //
            seed * 9.31 + 7.77,
            xv * 0.081 + 9.33,
            y * 0.5,
          ])
          * perlin
            .get([
              //
              xv * 0.0112 + 8.33,
              8.1 + y * 0.217,
              seed / 7.7 + 6.66,
            ])
            .min(0.0);

        if offsetstrategy == 3 {
          y += amp * peakfactor * xv;
        }

        y += 0.1
          * amp
          * (1.0 - miny / height)
          * perlin.get([
            //
            66.6 + seed * 1.3,
            18.3 + xv * 0.501,
            88.1 + y * 0.503,
          ]);

        if offsetstrategy == 4 {
          y += amp * peakfactor * xv;
        }

        if rounding > 0.1 {
          y = (y / rounding).round() * rounding;
        }

        if y < miny {
          miny = y;
        }
        let mut collides = false;
        let xi = ((x - mountainpadding) / precision).round() as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let inside = !collides && strictly_in_boundaries((x, y), bound);
        if inside && passage.get((x, y)) < passage_threshold {
          if was_outside {
            if route.len() > min_route {
              routes.push((1, route));
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
        routes.push((1, route));
      }

      base_y -= yincr;
    }

    if height_map_stop.len() == 0 || rng.gen_bool(0.5) {
      height_map_stop = height_map.clone();
    }
  }

  let radius = 0.5 + rng.gen_range(1.0, 2.0) * rng.gen_range(0.0, 1.0);
  passage.grow_passage(radius);

  let p = pad + 2.0;
  let extrabound = (p, p, width - p, height - p);

  let overlap = |p| {
    passage.get(p) == 0
      && strictly_in_boundaries(p, extrabound)
      && p.1
        < height_map_stop[((p.0 - mountainpadding) / precision) as usize
          % height_map_stop.len()]
  };

  let does_overlap = |c: (f64, f64, f64)| {
    overlap((c.0, c.1))
      && circle_route((c.0, c.1), c.2, 8, circleang)
        .iter()
        .all(|&p| overlap(p))
  };

  let freq = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
  let count = 1 + (rng.gen_range(1.0, 3.5) * rng.gen_range(0.0, 1.0)) as usize;
  for i in 0..count {
    let ppad = rng.gen_range(0.4, 1.2);
    let total_pad = radius + pad + ppad;
    let min = ppad + rng.gen_range(0.4, 0.8);
    let max = min
      + rng.gen_range(0.0, 50.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
    let circles = packing(
      seed + i as f64 * 1.6,
      500000,
      10000,
      rng.gen_range(0, 4),
      ppad,
      (total_pad, total_pad, width - total_pad, height - total_pad),
      &does_overlap,
      min,
      max,
    );

    for c in circles {
      let ang = 2.
        * perlin.get([
          //
          c.x * freq,
          c.y * freq,
          3333. + 7.7 * seed,
        ]);
      let a = (c.x + c.r * ang.cos(), c.y + c.r * ang.sin());
      let b = (c.x - c.r * ang.cos(), c.y - c.r * ang.sin());
      routes.push((2, vec![a, b]));
    }
  }

  // Border around the postcard
  let border_size = 8;
  let border_dist = 0.3;
  let mut route = Vec::new();
  for i in 0..border_size {
    let d = i as f64 * border_dist;
    route.push((pad + d, pad + d));
    route.push((pad + d, height - pad - d));
    route.push((width - pad - d, height - pad - d));
    route.push((width - pad - d, pad + d));
    route.push((pad + d, pad + d));
  }
  routes.push((1, route));

  // Make the SVG
  let colors = vec!["white", "white", "white"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (c, route) in routes.clone() {
        if c == ci {
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
  let mut document = base_document("black", opts.width, opts.height);
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

fn circle_route(
  center: (f64, f64),
  r: f64,
  count: usize,
  ang: f64,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * (i as f64 + ang) / (count as f64);
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
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y, size)) && !circles.iter().any(|other| c.collides(other))
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
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
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
