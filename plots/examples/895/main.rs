use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn sd_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
  manhattan: bool,
) -> f64 {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;

  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let h = (dot_pa_ba / dot_ba_ba).max(0.0).min(1.0);

  let h_x = ba_x * h;
  let h_y = ba_y * h;

  if manhattan {
    return (pa_x - h_x).abs().max((pa_y - h_y).abs());
  } else {
    ((pa_x - h_x) * (pa_x - h_x) + (pa_y - h_y) * (pa_y - h_y)).sqrt()
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let precision = 0.4;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);
  let mut passage = Passage::new(0.5, width, height);

  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;

  let samples = 100;
  let pattern = (1.0, 10.0);

  let divisor = (samples as f64 * (pattern.0 + pattern.1) / pattern.0).floor();
  let thresholds: Vec<f64> = (0..samples)
    .map(|i| (i as f64 + pattern.1 * (i as f64 / pattern.0).floor()) / divisor)
    .collect();

  let balance = 0.8;
  let offsetmax = 0.1;
  let count = rng.gen_range(16, 32);
  let segments: Vec<((f64, f64), (f64, f64), bool, f64)> = (0..count)
    .map(|_| {
      let a = (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));
      let b = if rng.gen_bool(0.5) {
        a
      } else {
        (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))
      };
      (
        a,
        b,
        if balance >= 1.0 {
          true
        } else if balance <= 0.0 {
          false
        } else {
          rng.gen_bool(balance)
        },
        rng.gen_range(0.0, offsetmax) * rng.gen_range(0.0, 1.0),
      )
    })
    .collect();

  let distortion = rng.gen_range(-0.05f64, 0.02).max(0.0);
  let ratio = width / height;

  let xflip = true;
  let yflip = true;

  let perlin = Perlin::new();

  let noiseamp = rng.gen_range(0.0, 0.01);
  let noisemod = rng.gen_range(2.0, 40.0);
  let noisefreq = rng.gen_range(60.0, 140.0);

  let off = rng.gen_range(0.0, 0.25);

  let f = |p: (f64, f64)| {
    let mut p = p;

    p = (
      p.0,
      p.1 + distortion * (p.0 - 0.5).abs() * (p.0 * 40.0 + p.1.sin()).cos(),
    );

    if xflip {
      p.0 = p.0.min(1.0 - p.0);
    }
    if yflip {
      p.1 = p.1.min(1.0 - p.1);
    }

    let mut s = 9999.0f64;

    for &(from, to, manhattan, offset) in segments.iter() {
      s = s.min(
        sd_segment(
          (p.0 * ratio, p.1),
          (from.0 * ratio, from.1),
          (to.0 * ratio, to.1),
          manhattan,
        ) - offset,
      );
    }
    let dedge = (p.0.min(1.0 - p.0) * ratio).min(p.1.min(1.0 - p.1));

    s += noiseamp
      * (if (dedge * noisemod) % 2.0 > 1.0 {
        0.
      } else {
        1.
      })
      * perlin.get([noisefreq * p.0, noisefreq * p.1, opts.seed]);

    s = s.min(dedge - off);

    s
  };

  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, bound);

  let should_crop = |p| !strictly_in_boundaries(p, bound);
  let mut cutted_points = vec![];
  routes =
    crop_routes_with_predicate(&routes, &should_crop, &mut cutted_points);

  let offset = 0.3;
  let mul = 1.5;
  let mut frame = vec![];
  for i in 0..5 {
    let l = offset + i as f64 * mul;
    frame.push(vec![
      (pad - l, pad - l),
      (width - pad + l, pad - l),
      (width - pad + l, height - pad + l),
      (pad - l, height - pad + l),
      (pad - l, pad - l),
    ]);
  }

  let mut gold_routes = vec![];
  for route in routes.clone() {
    for &p in route.iter() {
      passage.count(p);
    }
    let simplified = rdp(&route, 0.1);
    if route_length(&simplified) > 2.0 {
      gold_routes.push(simplified);
    }
  }
  for route in frame.clone() {
    gold_routes.push(route);
  }

  let grow = rng.gen_range(2., 5.);
  passage.grow_passage(grow);

  let cpad = rng.gen_range(0.5, 1.0);
  let s = cpad + rng.gen_range(0.5, 1.5);
  let maxadd = rng.gen_range(0.0, 40.0) * rng.gen_range(0.0, 1.0);
  let p = pad + grow;
  let bound = (p, p, width - p, height - p);
  let count = 4;
  let ang = rng.gen_range(0, 2) as f64 / 2.0;

  let overlap = |p| passage.get(p) == 0 && strictly_in_boundaries(p, bound);
  let does_overlap = |(x, y, r)| {
    overlap((x, y))
      && circle_route((x, y), r, count, ang)
        .iter()
        .all(|&p| overlap(p))
  };
  let circles = packing(
    &vec![],
    &mut rng,
    500000,
    10000,
    1,
    cpad,
    bound,
    &does_overlap,
    s,
    s + maxadd,
  );

  let mut red_routes = vec![];
  for c in circles {
    red_routes.push(circle_route((c.x, c.y), c.r, count, ang));
  }

  vec![("orange", gold_routes), ("grey", red_routes)]
    .iter()
    .enumerate()
    .map(|(i, (color, routes))| {
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn route_length(route: &Vec<(f64, f64)>) -> f64 {
  let mut length = 0.0;
  for i in 0..route.len() - 1 {
    length += (route[i].0 - route[i + 1].0).powi(2)
      + (route[i].1 - route[i + 1].1).powi(2);
  }
  length.sqrt()
}

fn crop_routes_with_predicate(
  input_routes: &Vec<Vec<(f64, f64)>>,
  should_crop: &dyn Fn((f64, f64)) -> bool,
  cutted_points: &mut Vec<(f64, f64)>,
) -> Vec<Vec<(f64, f64)>> {
  let search = |a_, b_, n| {
    let mut a = a_;
    let mut b = b_;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if should_crop(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes {
    if input_route.len() < 2 {
      continue;
    }
    let mut prev = input_route[0];
    let mut route = vec![];
    if !should_crop(prev) {
      // prev is not to crop. we can start with it
      route.push(prev);
    } else {
      if !should_crop(input_route[1]) {
        // prev is to crop, but [1] is outside. we start with the exact intersection
        let intersection = search(input_route[1], prev, 7);
        prev = intersection;
        cutted_points.push(intersection);
        route.push(intersection);
      } else {
        cutted_points.push(prev);
      }
    }
    // cut segments with crop logic
    for &p in input_route.iter().skip(1) {
      // TODO here, we must do step by step to detect crop inside the segment (prev, p)

      if should_crop(p) {
        if route.len() > 0 {
          // prev is outside, p is to crop
          let intersection = search(prev, p, 7);
          cutted_points.push(intersection);
          route.push(intersection);
          routes.push(route);
          route = vec![];
        } else {
          cutted_points.push(p);
        }
      } else {
        // nothing to crop
        route.push(p);
      }
      prev = p;
    }
    if route.len() >= 2 {
      routes.push(route);
    }
  }

  routes
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

fn packing<R: Rng>(
  first_circles: &Vec<VCircle>,
  rng: &mut R,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = first_circles.clone();
  let mut tries = Vec::new();
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

#[inline]
fn strictly_in_boundaries(
  p: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> bool {
  p.0 > boundaries.0
    && p.0 < boundaries.2
    && p.1 > boundaries.1
    && p.1 < boundaries.3
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

pub fn spiral_optimized(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = (x + r * a.cos(), y + r * a.sin());
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 0.2 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
}
