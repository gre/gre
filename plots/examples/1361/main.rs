use std::f64::consts::PI;

use clap::*;
use gre::*;
use rand::prelude::*;
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn path_subdivide_to_curve_it(
  path: &Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path.clone();
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

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();

  let mut paint = PaintMask::new(0.8, width, height);

  let s1 = rng.gen_range(8, 16);
  let s2 = rng.gen_range(5, 10);
  let maxt = 3 + (rng.gen_range(8., 15.) * rng.gen_range(0.0, 1.)) as usize;
  let p = rng.gen_range(1.8, 3.0);
  let min = rng.gen_range(0.6, 2.0);
  let max = rng.gen_range(5.0, 12.0);
  let mut circles = packing(
    opts.seed,
    200000,
    5000,
    1,
    p,
    (pad, pad, width - pad, height - pad),
    &VCircle::new(width / 2., height / 2., height / 2.0 - pad),
    p + min,
    p + max,
  );
  let diff = rng.gen_range(10.0, 80.0);
  let mut orders: Vec<(usize, f64)> = circles
    .iter()
    .map(|c| c.y + rng.gen_range(-diff, diff))
    .enumerate()
    .collect();
  orders.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  circles = orders.iter().map(|(i, _)| circles[*i].clone()).collect();

  let mut group1 = vec![];
  let mut group2 = vec![];
  let proba2 = rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);
  let pgroup1 = 1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

  for c in circles {
    if c.r > mix(min, max, 0.8) && rng.gen_bool(pgroup1) {
      group1.push(c);
    } else if rng.gen_bool(proba2) {
      group2.push(c);
    }
  }

  let psym = 1.0
    - rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
  let gdispy = rng.gen_range(0.0, 1.0);
  let mut i = 0;
  while i < group1.len() {
    let ct = rng.gen_range(2, maxt);
    if i + ct > group1.len() {
      break;
    }
    let mut pts = vec![];
    for j in i..(i + ct) {
      let dispy = rng.gen_range(-10.0, 20.0) * gdispy;
      let c = &group1[j];
      if pts.len() > 0 {
        let b: (f64, f64) = pts[pts.len() - 1];
        let p = (
          mix(c.x, b.0, rng.gen_range(0.0, 1.0)),
          mix(c.x, b.1, 0.5) + dispy,
        );
        pts.push(p);
      }
      pts.push((c.x, c.y));
    }
    pts = path_subdivide_to_curve_it(&pts, rng.gen_range(0.6, 0.8));
    pts = path_subdivide_to_curve_it(&pts, 0.75);
    pts = path_subdivide_to_curve_it(&pts, 0.8);
    pts = path_subdivide_to_curve_it(&pts, 0.8);
    pts = path_subdivide_to_curve_it(&pts, 0.8);
    routes.push((s1, pts.clone()));
    if rng.gen_bool(psym) {
      let pts = pts.iter().map(|&(x, y)| (width - x, y)).collect();
      routes.push((s1, pts));
    }
    i += ct;
  }

  for c in group2 {
    let mut pts = vec![];
    for i in 0..rng.gen_range(2, 5) {
      let a = (i % 2) as f64 * PI
        + rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0);
      let r = c.r;
      pts.push((c.x + r * a.cos(), c.y + r * a.sin()));
    }
    pts = path_subdivide_to_curve_it(&pts, 0.9);
    routes.push((rng.gen_range(s2 / 2, s2), pts));
  }

  let mut rts = vec![];
  let dy = rng.gen_range(0.4, 0.6);
  for (count, route) in routes {
    for i in 0..count {
      let rt = route.iter().map(|&(x, y)| (x, y + i as f64 * dy)).collect();
      paint.paint_polyline(&rt, 1.0);
      rts.push(rt);
    }
  }

  let mut rts2 = vec![];
  let base_r = rng.gen_range(3.0, 10.0);
  for _ in 0..100000 {
    let a = rng.gen_range(-PI, PI);
    let r = rng.gen_range(0.0, 1.0);
    let x = width / 2. + r * (height / 2. - pad) * a.cos();
    let y = height / 2. + r * (height / 2. - pad) * a.sin();
    let r = base_r * rng.gen_range(0.5, 1.0);
    let s = spiral_optimized(x, y, r, 1.0, 0.1);
    if s.iter().all(|p| !paint.is_painted(*p)) {
      let radius = rng.gen_range(1.0, r.max(1.1));
      paint.paint_circle(x, y, radius + p);
      if rng.gen_bool(0.5) {
        rts2.push(spiral_optimized(x, y, radius, 0.5, 0.01));
      }
      rts2.push(circle_route((x, y), radius, 50));
    }
  }

  vec![(rts, "black"), (rts2, "#c90")]
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
pub struct PaintMask {
  mask: Vec<bool>,
  pub precision: f64,
  pub width: f64,
  pub height: f64,
  pub wi: usize,
  pub hi: usize,
}

impl PaintMask {
  pub fn clone_empty(&self) -> Self {
    let wi = self.wi;
    let hi = self.hi;
    Self {
      mask: vec![false; wi * hi],
      width: self.width,
      height: self.height,
      precision: self.precision,
      wi,
      hi,
    }
  }

  pub fn clone_empty_rescaled(&self, precision: f64) -> Self {
    if precision == self.precision {
      return self.clone();
    }
    let width = self.width;
    let height = self.height;
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    let next = Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
      wi,
      hi,
    };
    next
  }

  pub fn clone_rescaled(&self, precision: f64) -> Self {
    if precision == self.precision {
      return self.clone();
    }
    let width = self.width;
    let height = self.height;
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    let mut next = Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
      wi,
      hi,
    };
    for x in 0..wi {
      for y in 0..hi {
        let j = x + y * wi;
        let xf = x as f64 * precision;
        let yf = y as f64 * precision;
        next.mask[j] = self.is_painted((xf, yf));
      }
    }
    next
  }

  pub fn new(precision: f64, width: f64, height: f64) -> Self {
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

  pub fn is_painted(&self, (x, y): (f64, f64)) -> bool {
    let precision = self.precision;
    let wi = self.wi;
    let hi = self.hi;
    let xi = ((x / precision) as usize).min(wi - 1);
    let yi = ((y / precision) as usize).min(hi - 1);
    self.mask[xi + yi * wi]
  }

  pub fn manhattan_distance(&self) -> Vec<usize> {
    let width = self.wi;
    let height = self.hi;
    let mut distances = vec![usize::MAX / 2; self.mask.len()];
    // Forward pass
    for y in 0..height {
      for x in 0..width {
        let idx = x + y * width;
        if self.mask[idx] {
          distances[idx] = 0;
        } else {
          if x > 0 {
            let i = x - 1 + y * width;
            distances[idx] = distances[idx].min(distances[i] + 1);
          }
          if y > 0 {
            let i = x + (y - 1) * width;
            distances[idx] = distances[idx].min(distances[i] + 1);
          }
        }
      }
    }
    // Backward pass
    for y in (0..height).rev() {
      for x in (0..width).rev() {
        let idx = x + y * width;
        if x < width - 1 {
          let i = x + 1 + y * width;
          distances[idx] = distances[idx].min(distances[i] + 1);
        }
        if y < height - 1 {
          let i = x + (y + 1) * width;
          distances[idx] = distances[idx].min(distances[i] + 1);
        }
      }
    }
    distances
  }

  pub fn assign_data_lower_than_threshold(
    &mut self,
    data: &Vec<usize>,
    radius: f64,
  ) {
    let threshold = (radius / self.precision) as usize;
    let wi = self.wi;
    let hi = self.hi;
    for y in 0..hi {
      for x in 0..wi {
        let i = x + y * wi;
        if data[i] <= threshold {
          self.mask[i] = true;
        }
      }
    }
  }

  pub fn paint(&mut self, other: &Self) {
    if other.width != self.width
      || other.height != self.height
      || other.precision != self.precision
    {
      // alternative less efficient way when the sizes are different
      let wi = self.wi;
      let hi = self.hi;
      for x in 0..wi {
        let xf = x as f64 * self.precision;
        for y in 0..hi {
          let yf = y as f64 * self.precision;
          if other.is_painted((xf, yf)) {
            let i: usize = x + y * wi;
            self.mask[i] = true;
          }
        }
      }
    } else {
      // regular way
      for (i, &v) in other.mask.iter().enumerate() {
        if v {
          self.mask[i] = true;
        }
      }
    }
  }

  pub fn paint_fn<F: Fn((f64, f64)) -> bool>(&mut self, f: F) {
    let precision = self.precision;
    let wi = self.wi;
    let hi = self.hi;
    for x in 0..wi {
      for y in 0..hi {
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let point = (x as f64 * precision, y as f64 * precision);
        if f(point) {
          self.mask[j] = true;
        }
      }
    }
  }

  pub fn reverse(&mut self) {
    for v in self.mask.iter_mut() {
      *v = !*v;
    }
  }

  pub fn intersects(&mut self, other: &Self) {
    if other.width != self.width
      || other.height != self.height
      || other.precision != self.precision
    {
      panic!("PaintMask::paint: incompatible sizes");
    }

    let len = self.mask.len();
    let mut i = 0;
    while i < len {
      if !other.mask[i] {
        self.mask[i] = false;
      }
      i += 1;
    }
  }

  pub fn painted_boundaries(&self) -> (f64, f64, f64, f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let wi = self.wi;
    let hi = self.hi;

    let mut minx = width;
    let mut miny = height;
    let mut maxx = 0.0f64;
    let mut maxy = 0.0f64;
    for x in 0..wi {
      for y in 0..hi {
        if self.mask[x + y * wi] {
          let xf = x as f64 * precision;
          let yf = y as f64 * precision;
          minx = minx.min(xf);
          miny = miny.min(yf);
          maxx = maxx.max(xf);
          maxy = maxy.max(yf);
        }
      }
    }

    if minx > maxx || miny > maxy {
      minx = 0.0;
      maxx = 0.0;
      miny = 0.0;
      maxy = 0.0;
    }

    (minx, miny, maxx, maxy)
  }

  pub fn paint_columns_left_to_right<F: Fn(f64) -> std::ops::Range<f64>>(
    &mut self,
    f: F,
  ) {
    let precision = self.precision;
    let wi = self.wi;
    let hi = self.hi;
    for x in 0..wi {
      let range = f(x as f64 * precision);
      let miny = (range.start.max(0.) / precision) as usize;
      let maxy = ((range.end / precision) as usize).min(hi);
      for y in miny..maxy {
        self.mask[x + y * wi] = true;
      }
    }
  }

  pub fn paint_circle(&mut self, cx: f64, cy: f64, cr: f64) {
    let (minx, miny, maxx, maxy) = (
      (cx - cr).max(0.),
      (cy - cr).max(0.),
      (cx + cr).min(self.width),
      (cy + cr).min(self.height),
    );
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    let cr2 = cr * cr;
    for x in minx..maxx {
      for y in miny..maxy {
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let point = (x as f64 * precision, y as f64 * precision);
        let dx = point.0 - cx;
        let dy = point.1 - cy;
        if dx * dx + dy * dy < cr2 {
          self.mask[j] = true;
        }
      }
    }
  }

  pub fn paint_pixels(
    &mut self,
    topleft: (f64, f64),
    data: &Vec<u8>,
    datawidth: usize,
  ) {
    let ox = (topleft.0 / self.precision).max(0.0) as usize;
    let oy = (topleft.1 / self.precision).max(0.0) as usize;
    let wi = self.wi;
    let hi = self.hi;
    for (i, &v) in data.iter().enumerate() {
      if v > 0 {
        let dx = i % datawidth;
        let dy = i / datawidth;
        let x = ox + dx;
        let y = oy + dy;
        if x < wi && y < hi {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  pub fn paint_rectangle(
    &mut self,
    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
  ) {
    self.paint_rectangle_v(minx, miny, maxx, maxy, true);
  }

  pub fn paint_rectangle_v(
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

  pub fn paint_borders(&mut self, pad: f64) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
  }

  pub fn unpaint_borders(&mut self, pad: f64) {
    self.paint_rectangle_v(0., 0., self.width, pad, false);
    self.paint_rectangle_v(0., 0., pad, self.height, false);
    self.paint_rectangle_v(
      0.,
      self.height - pad,
      self.width,
      self.height,
      false,
    );
    self.paint_rectangle_v(
      self.width - pad,
      0.,
      self.width,
      self.height,
      false,
    );
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

pub fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  p: (f64, f64),
) -> bool {
  let mut inside = false;
  let mut j = polygon.len() - 1;
  for i in 0..polygon.len() {
    let pi = polygon[i];
    let pj = polygon[j];
    if (pi.1 > p.1) != (pj.1 > p.1)
      && p.0 < (pj.0 - pi.0) * (p.1 - pi.1) / (pj.1 - pi.1) + pi.0
    {
      inside = !inside;
    }
    j = i;
  }
  inside
}
