use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
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
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn heart_function(t: f64) -> (f64, f64) {
  let x = 16.0 * f64::sin(t).powi(3);
  let y = -13.0 * f64::cos(t)
    + 5.0 * f64::cos(2.0 * t)
    + 2.0 * f64::cos(3.0 * t)
    + f64::cos(4.0 * t);
  (x * 0.059, y * 0.059)
}

fn heart_spiral(ox: f64, oy: f64, radius: f64, dr: f64) -> Vec<(f64, f64)> {
  let mut points = Vec::new();
  let mut t = 0.0;
  let mut r = 0.0;
  let end_r = radius + 2.0 * PI * dr;
  while r < end_r {
    let da = 1.0 / (r + 8.0);
    t += da;
    r += 0.2 * dr * da;
    let (x, y) = heart_function(t);
    let v = r.min(radius);
    let dy = 0.1 * radius * (1. - v / radius);
    let p = (x * v + ox, y * v + oy + dy);
    points.push(p);
  }
  // points.extend(circle_route((ox, oy), radius, 100));
  points
}

fn heart_nested(
  ox: f64,
  oy: f64,
  radius: f64,
  dr: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let mut r = radius;
  while r > 0.1 {
    let mut route = vec![];
    let count = (2.0 * PI * r / 0.5).floor() as usize;
    if count > 3 {
      for i in 0..count {
        let a = i as f64 * 2.0 * PI / (count as f64);
        let (x, y) = heart_function(a);
        let p = (x * r + ox, y * r + oy);
        route.push(p);
      }
      route.push(route[0]);
      routes.push(route);
    }
    r -= dr;
  }
  routes
}

fn heart(ox: f64, oy: f64, r: f64, ang: f64) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  let count = (2.0 * PI * r / 0.5).floor() as usize;
  for i in 0..count {
    let a = i as f64 * 2.0 * PI / (count as f64);
    let (x, y) = heart_function(a);
    let (x, y) = p_r((x, y), ang);
    let p = (x * r + ox, y * r + oy);
    route.push(p);
  }
  route.push(route[0]);
  route
}

fn heart_nested_rotating<R: Rng>(
  rng: &mut R,
  ox: f64,
  oy: f64,
  radius: f64,
  extra_radius: f64,
  dr: f64,
  stopr: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let mut r = extra_radius;
  let perlin = Perlin::new();
  let seed = rng.gen_range(-555., 555.);
  let f = rng.gen_range(0.05, 0.1) * rng.gen_range(0.2, 1.0);
  let amp = rng.gen_range(0.03, 0.08) / f;
  let basen = perlin.get([seed, f * r]);
  while r > stopr {
    let actualr = r.min(radius);
    let count = (2.0 * PI * r / 0.5).floor() as usize;
    if count > 3 {
      let n = perlin.get([seed, f * r]) - basen;
      let offr = n * amp;
      let route = heart(ox, oy, actualr, offr);
      routes.push(route);
    }
    r -= dr;
  }
  routes
}

fn cell(
  seed: f64,
  origin: (f64, f64),
  width: f64,
  height: f64,
  pad: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let mut rng = rng_from_seed(seed);

  let dr = rng.gen_range(0.6, 1.0);

  let r = (width.min(height) / 2.0 - pad) * rng.gen_range(0.8, 1.0);
  let r2 = r
    * (1.0
      + rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(-1.0f64, 1.0).max(0.0));
  /*if rng.gen_bool(0.1) {
    routes.extend(heart_nested(
      origin.0 + width / 2.0,
      origin.1 + height / 2.0,
      r,
      dr,
    ));
  } else */
  if rng.gen_bool(0.1) {
    routes.push(heart_spiral(
      origin.0 + width / 2.0,
      origin.1 + height / 2.0,
      r,
      dr,
    ));
  } else {
    let stopr = if rng.gen_bool(0.5) {
      rng.gen_range(0.1, 0.7) * r
    } else {
      0.1
    };
    routes.extend(heart_nested_rotating(
      &mut rng,
      origin.0 + width / 2.0,
      origin.1 + height / 2.0,
      r,
      r2,
      dr,
      stopr,
    ));
  }

  let mut mask = PaintMask::new(0.2, width, height);
  let ppad = rng.gen_range(4.0, 6.0);

  // TODO use a inner heart step as mask to make a white?

  // to protect the paper from having too much passage, we will cut some lines based on a grid lookup.
  let prec = 0.5;
  let passage_limit = 10;
  let minlen = 3;
  let mut passage = Passage2DCounter::new(prec, width, height);
  let mut paths = vec![];
  for r in routes {
    let mut localpassage = Passage2DCounter::new(prec, width, height);
    let mut path: Vec<(f64, f64)> = vec![];
    for p in r {
      let localp = (p.0 - origin.0, p.1 - origin.1);
      if passage.get(localp) > passage_limit {
        if path.len() >= minlen {
          paths.push(path);
        }
        path = vec![];
      } else {
        path.push(p);
      }
      localpassage.count(localp);
      mask.paint_circle(&VCircle::new(p.0 - origin.0, p.1 - origin.1, ppad));
    }
    if path.len() >= minlen {
      paths.push(path);
    }
    passage.count_once_from(&localpassage);
  }
  routes = paths;

  let bounds = (pad, pad, width - pad, height - pad);

  let in_shape = |p: (f64, f64)| -> bool {
    !mask.is_painted(p) && strictly_in_boundaries(p, bounds)
  };

  let does_overlap = |c: &VCircle| {
    in_shape((c.x, c.y))
      && circle_route((c.x, c.y), c.r, 8)
        .iter()
        .all(|&p| in_shape(p))
  };

  let ppad = rng.gen_range(0.4, 0.8);
  let min = rng.gen_range(1.5, 2.0);
  let max = min + rng.gen_range(0.0, 5.0);
  let optim = rng.gen_range(1, 10);
  let count = 2000;
  let circles = packing(
    &mut rng,
    vec![],
    5000000,
    count,
    optim,
    ppad,
    bounds,
    &does_overlap,
    min,
    max,
  );

  let aligned = rng.gen_bool(0.3);

  for c in circles {
    let x = c.x + origin.0;
    let y = c.y + origin.1;
    let r = c.r;
    let ang = if aligned {
      0.
    } else {
      PI + (c.x - width / 2.0).atan2(c.y - height / 2.0)
    };
    routes.push(heart(x, y, r, ang));
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;

  let cw = width / 2.;
  let ch = height / 2.;
  let pad = 5.;

  let cols = (width / cw).floor() as usize;
  let rows = (height / ch).floor() as usize;

  let offsetx = 0.0;
  let offsety = 0.0;

  let routes = (0..rows)
    .into_par_iter()
    .flat_map(|j| {
      (0..cols).into_par_iter().flat_map(move |i| {
        cell(
          opts.seed / 7.7 + (i + j * cols) as f64 / 0.3,
          (offsetx + i as f64 * cw, offsety + j as f64 * ch),
          cw,
          ch,
          pad,
        )
      })
    })
    .collect::<Vec<Vec<(f64, f64)>>>();

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route_curve(data, route);
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

struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
}

impl PaintMask {
  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    }
  }

  fn is_painted(&self, point: (f64, f64)) -> bool {
    // check out of bounds
    if point.0 <= 0.0
      || point.0 >= self.width
      || point.1 <= 0.0
      || point.1 >= self.height
    {
      return false;
    }
    let precision = self.precision;
    let width = self.width;
    let x = (point.0 / precision) as usize;
    let y = (point.1 / precision) as usize;
    let wi = (width / precision) as usize;
    self.mask[x + y * wi]
  }

  fn paint_circle(&mut self, circle: &VCircle) {
    let (minx, miny, maxx, maxy) = (
      circle.x - circle.r,
      circle.y - circle.r,
      circle.x + circle.r,
      circle.y + circle.r,
    );
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    let hi = (self.height / precision) as usize;
    for x in minx..maxx {
      if x >= wi {
        continue;
      }
      for y in miny..maxy {
        if y >= hi {
          continue;
        }
        let point = (x as f64 * precision, y as f64 * precision);
        if euclidian_dist(point, (circle.x, circle.y)) < circle.r {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
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
  does_overlap: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing<R: Rng>(
  rng: &mut R,
  initial_circles: Vec<VCircle>,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = initial_circles.clone();
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

pub struct Passage2DCounter {
  granularity: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage2DCounter {
  pub fn new(granularity: f64, width: f64, height: f64) -> Self {
    let wi = (width / granularity).ceil() as usize;
    let hi = (height / granularity).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage2DCounter {
      granularity,
      width,
      height,
      counters,
    }
  }
  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.granularity).ceil() as usize;
    let hi = (self.height / self.granularity).ceil() as usize;
    let xi = ((x / self.granularity).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.granularity).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }
  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }
  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    self.counters[self.index(p)]
  }

  pub fn count_once_from(self: &mut Self, other: &Self) {
    for i in 0..self.counters.len() {
      self.counters[i] += if other.counters[i] > 0 { 1 } else { 0 };
    }
  }
}
