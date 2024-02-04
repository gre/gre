use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  height: f64,
  #[clap(short, long, default_value = "210.0")]
  width: f64,
  #[clap(short, long, default_value = "10.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn flower<R: Rng>(
  rng: &mut R,
  (ox, oy): (f64, f64),
  r: f64,
  ang: f64,
  count: usize,
  clr1: usize,
  clr2: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let s = r / 2.;
  routes.push((clr2, spiral_optimized(ox, oy, s, 0.4, 0.01)));
  routes.push((clr1, circle_route((ox, oy), s, 40)));
  for i in 0..count {
    let p = i as f64 / (count as f64) + ang;
    let a = p * 2. * PI;
    let s = rng.gen_range(0.65, 0.75) * s;
    let x = ox + 0.9 * r * a.cos();
    let y = oy + 0.9 * r * a.sin();
    routes.push((clr1, spiral_optimized(x, y, s, 0.5, 0.1)));
    routes.push((clr1, circle_route((x, y), s, 40)));
  }
  routes
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
  points
}

fn make_clover<R: Rng>(
  rng: &mut R,
  o: (f64, f64),
  r: f64,
  ang: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  let count = 4;
  for i in 0..count {
    let s = r * (0.41 + rng.gen_range(-0.03, 0.06) * rng.gen_range(0.0, 1.0));
    let yd = r * 0.5;
    let a = ang
      + i as f64 * PI / ((count as f64) / 2.)
      + rng.gen_range(-0.2, 0.2) * rng.gen_range(-2.0f64, 1.0).max(0.0);
    let dr = rng.gen_range(0.45, 0.6);
    let rt = heart_spiral(0., -yd, s, dr);
    let rt = rt
      .iter()
      .map(|&p| {
        let p = p_r(p, a);
        let p = (p.0 + o.0, p.1 + o.1);
        p
      })
      .collect();
    routes.push((clr, rt));
  }

  routes
}

fn polylines_smooth_union_filled(
  lines: &Vec<Vec<(f64, f64)>>,
  clr: usize,
  linew: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  for line in lines {
    for p in step_polyline(line, 0.65 * linew) {
      routes.push((clr, circle_route(p, linew / 2.0, 8)));
    }
  }
  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let precision = 0.3;
  let mut rng = rng_from_seed(opts.seed);
  let mut paint = PaintMask::new(precision, width, height);
  paint.paint_borders(pad);
  let mut routes = vec![];

  let circles = packing(
    opts.seed,
    1000000,
    500,
    1,
    0.0,
    (pad, pad, width - pad, height - pad),
    &VCircle::new(width / 2., height / 2., width.max(height)),
    &|c| true,
    4.0,
    10.0,
  );

  let perlin = Perlin::new();

  let clr1base = rng.gen_range(0.0, 4.0);
  let clr2base = rng.gen_range(0.0, 4.0);
  let clr1mul = rng.gen_range(4.0, 8.0);
  let clr2mul = rng.gen_range(4.0, 8.0);

  let rngbase = rng.gen_range(0.0, 0.4);

  let f1 = rng.gen_range(0.0, rngbase) * rng.gen_range(0.0, 1.0);
  let f2 = rng.gen_range(0.0, rngbase) * rng.gen_range(0.0, 1.0);
  let f3 = rng.gen_range(0.0, rngbase) * rng.gen_range(0.0, 1.0);
  let f4 = rng.gen_range(0.0, rngbase) * rng.gen_range(0.0, 1.0);

  let clover_proba = rng.gen_range(0.0, 1.0);

  for c in circles {
    let n = perlin.get([
      f1 * c.x,
      f1 * c.y,
      opts.seed / 0.0421,
    ]);
    let n1 = perlin.get([
      f2 * c.x,
      f2 * c.y,
      opts.seed / 0.07421,
    ]);
    let n2 = perlin.get([
      f3 * c.x,
      f3 * c.y,
      opts.seed / 0.04321,
    ]);
    let n3 = perlin.get([
      f4 * c.x,
      f4 * c.y,
      opts.seed / 0.014721,
    ]);
    let rot = PI * n1;
    if n > clover_proba {
      routes.extend(make_clover(&mut rng, (c.x, c.y), c.r, rot, 5));
    } else {
      let clr1 = (
        clr1base +
        clr1mul * (n2 + 1.0)) as usize % 5;
      let clr2 = (
        clr2base +
        clr2mul * (n3 + 1.0)) as usize % 5;
        let count = 6;
      routes.extend(flower(&mut rng, (c.x, c.y), 0.65 * c.r, rot, count, clr1, clr2));
    }
  }

  vec!["#e00", "#f0b", "#fc0", "#0da", "#469", "#2b2"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
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

#[derive(Clone)]
struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
  wi: usize,
  hi: usize,
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
      wi,
      hi,
    }
  }

  fn is_painted(&self, (x, y): (f64, f64)) -> bool {
    let precision = self.precision;
    let wi = self.wi;
    let hi = self.hi;
    let xi = ((x / precision) as usize).min(wi - 1);
    let yi = ((y / precision) as usize).min(hi - 1);
    self.mask[xi + yi * wi]
  }

  fn paint_rectangle(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
    self.paint_rectangle_v(minx, miny, maxx, maxy, true);
  }

  fn paint_rectangle_v(
    &mut self,
    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
    v: bool,
  ) {
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        self.mask[x + y * wi] = v;
      }
    }
  }

  fn paint_borders(&mut self, pad: f64) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
  }

  pub fn paint_polyline(&mut self, polyline: &Vec<(f64, f64)>, strokew: f64) {
    let len = polyline.len();
    if len < 1 {
      return;
    }
    let first = polyline[0];
    let mut minx = first.0;
    let mut miny = first.1;
    let mut maxx = first.0;
    let mut maxy = first.1;
    let mut i = 1;
    while i < len {
      let (x, y) = polyline[i];
      if x < minx {
        minx = x;
      }
      if x > maxx {
        maxx = x;
      }
      if y < miny {
        miny = y;
      }
      if y > maxy {
        maxy = y;
      }
      i += 1;
    }
    minx = (minx - strokew).max(0.0);
    miny = (miny - strokew).max(0.0);
    maxx = (maxx + strokew).min(self.width);
    maxy = (maxy + strokew).min(self.height);

    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      let xf = x as f64 * precision;
      for y in miny..maxy {
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let yf = y as f64 * precision;
        let point = (xf, yf);
        let mut i = 1;
        let mut prev = polyline[0];
        while i < len {
          let next = polyline[i];
          if point_in_segment(point, prev, next, strokew) {
            self.mask[j] = true;
            break;
          }
          i += 1;
          prev = next;
        }
      }
    }
  }
}

fn point_in_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
  strokew: f64,
) -> bool {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;
  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let mut h = dot_pa_ba / dot_ba_ba;
  if h < 0.0 {
    h = 0.0;
  } else if h > 1.0 {
    h = 1.0;
  }
  let h_x = ba_x * h;
  let h_y = ba_y * h;
  let dx = pa_x - h_x;
  let dy = pa_y - h_y;
  dx * dx + dy * dy < strokew * strokew
}

fn path_subdivide_to_curve_it(
  path: Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path;
  }
  let mut route = Vec::new();
  let mut first = path[0];
  let mut last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if looped {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for i in 1..(l - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if looped {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if looped {
    route.push(first);
  }
  route
}

fn step_polyline(path: &Vec<(f64, f64)>, step: f64) -> Vec<(f64, f64)> {
  let plen = path.len();
  let mut route = vec![];
  if plen < 1 {
    return route;
  }
  let mut lastp = path[0];
  route.push(lastp);
  let mut i = 0;
  while i < plen - 1 {
    let b = path[i + 1];
    let dist = euclidian_dist(lastp, b);
    if dist < step {
      i += 1;
    } else if dist >= step {
      let p = lerp_point(lastp, b, step / dist);
      route.push(p);
      lastp = p;
    }
  }
  route
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
  fn contains(self: &Self, c: &VCircle) -> bool {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - self.r + c.r < 0.0
  }
  fn inside_bounds(
    self: &Self,
    (x1, y1, x2, y2): (f64, f64, f64, f64),
  ) -> bool {
    x1 <= self.x - self.r
      && self.x + self.r <= x2
      && y1 <= self.y - self.r
      && self.y + self.r <= y2
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
  container_boundaries: (f64, f64, f64, f64),
  container_circle: &VCircle,
  is_valid: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    c.inside_bounds(container_boundaries)
      && container_circle.contains(&c)
      && is_valid(&c)
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  container_boundaries: (f64, f64, f64, f64),
  container: &VCircle,
  is_valid: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = container.x - container.r;
  let y1 = container.y - container.r;
  let x2 = container.x + container.r;
  let y2 = container.y + container.r;
  let max_scale = max_scale.min(container.r);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    if let Some(size) = search_circle_radius(
      container_boundaries,
      &container,
      is_valid,
      &circles,
      x,
      y,
      min_scale,
      max_scale,
    ) {
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
