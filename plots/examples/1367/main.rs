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
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "148.5")]
  pub width: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn wireframe(
  clr: usize,
  poly: &Vec<(f64, f64)>,
  xsplits: usize,
  ysplits: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut grids = vec![];
  for (splits, p1, p2, p3, p4) in vec![
    (ysplits, poly[0], poly[1], poly[3], poly[2]),
    (xsplits, poly[1], poly[2], poly[0], poly[3]),
  ] {
    for i in 0..splits + 1 {
      let p = i as f64 / splits as f64;
      let a = lerp_point(p1, p3, p);
      let b = lerp_point(p2, p4, p);
      grids.push((clr, vec![a, b]));
    }
  }
  grids
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  let mut paint = PaintMask::new(0.4, width, height);

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = Vec::new();

  let scale = 1.0;
  let w = 50.0;
  let h = 20.0;
  let zh = 45.0;
  let dx = rng.gen_range(0.2, 0.25) * w;
  let divs = 8;

  let project_polyline = |l: &Vec<(f64, f64)>| -> Vec<(f64, f64)> {
    l.iter()
      .map(|(x, y)| {
        (scale * x + width / 2.0, scale * y + height / 2. - zh / 2.0)
      })
      .collect()
  };

  let faces = vec![
    project_polyline(&vec![
      (-w / 2.0 - dx, -h / 2.0),
      (w / 2.0 - dx, -h / 2.0),
      (w / 2.0 + dx, h / 2.0),
      (-w / 2.0 + dx, h / 2.0),
    ]),
    project_polyline(&vec![
      (w / 2.0 + dx, h / 2.0),
      (-w / 2.0 + dx, h / 2.0),
      (-w / 2.0 + dx, h / 2.0 + zh),
      (w / 2.0 + dx, h / 2.0 + zh),
    ]),
    project_polyline(&vec![
      (-w / 2.0 - dx, h / 2.0 - h),
      (-w / 2.0 + dx, h / 2.0),
      (-w / 2.0 + dx, h / 2.0 + zh),
      (-w / 2.0 - dx, h / 2.0 + zh - h),
    ]),
  ];
  for poly in faces {
    routes.extend(wireframe(0, &poly, divs, divs));
    paint.paint_polygon(&poly);
    let mut poly = poly.clone();
    poly.push(poly[0]);
    paint.paint_polyline(&poly, 0.8);
  }

  let mut rts = vec![];
  let mut y = pad;
  let dy = rng.gen_range(3.0, 4.0);
  while y < height - pad {
    rts.push((1, vec![(pad, y), (width - pad, y)]));
    y += dy;
  }
  let rts = clip_routes_with_colors(&rts, &|p| paint.is_painted(p), 1.0, 4);
  routes.extend(rts);

  let mut clrs = vec!["#09F", "#F90"];
  rng.shuffle(&mut clrs);

  clrs
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if i == ci && is_plottable_polyline(&route, 0.5) {
          data = render_route(data, route);
        }
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

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
}

fn polygon_bounds(polygon: &Vec<(f64, f64)>) -> (f64, f64, f64, f64) {
  let mut minx = f64::MAX;
  let mut miny = f64::MAX;
  let mut maxx = f64::MIN;
  let mut maxy = f64::MIN;
  for &(x, y) in polygon {
    minx = minx.min(x);
    miny = miny.min(y);
    maxx = maxx.max(x);
    maxy = maxy.max(y);
  }
  (minx, miny, maxx, maxy)
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

fn clip_routes_with_colors(
  input_routes: &Vec<(usize, Vec<(f64, f64)>)>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  // locate the intersection where inside and outside cross
  let search = |inside: (f64, f64), outside: (f64, f64), n| {
    let mut a = inside;
    let mut b = outside;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if is_outside(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for (clrp, input_route) in input_routes.iter() {
    let clr = *clrp;
    if input_route.len() < 2 {
      continue;
    }

    let mut prev = input_route[0];
    let mut prev_is_outside = is_outside(prev);
    let mut route = vec![];
    if !prev_is_outside {
      // prev is not to crop. we can start with it
      route.push(prev);
    }

    for &p in input_route.iter().skip(1) {
      // we iterate in small steps to detect any interruption
      let static_prev = prev;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let d = (dx * dx + dy * dy).sqrt();
      let vx = dx / d;
      let vy = dy / d;
      let iterations = (d / stepping).ceil() as usize;
      let mut v = 0.0;
      for _i in 0..iterations {
        v = (v + stepping).min(d);
        let q = (static_prev.0 + vx * v, static_prev.1 + vy * v);
        let q_is_outside = is_outside(q);
        if prev_is_outside != q_is_outside {
          // we have a crossing. we search it precisely
          let intersection = if prev_is_outside {
            search(q, prev, dichotomic_iterations)
          } else {
            search(prev, q, dichotomic_iterations)
          };

          if q_is_outside {
            // we close the path
            route.push(intersection);
            if route.len() > 1 {
              // we have a valid route to accumulate
              routes.push((clr, route));
            }
            route = vec![];
          } else {
            // we open the path
            route.push(intersection);
          }
          prev_is_outside = q_is_outside;
        }

        prev = q;
      }

      // prev should be == p
      if !prev_is_outside {
        // prev is not to crop. we can start with it
        route.push(p);
      }
    }

    if route.len() > 1 {
      // we have a valid route to accumulate
      routes.push((clr, route));
    }
  }

  routes
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn is_plottable_polyline(
  polyline: &Vec<(f64, f64)>,
  manhattan_skip_threshold: f64,
) -> bool {
  let mut last = polyline[0];
  let mut d = 0.0;
  for &p in &polyline[1..] {
    d += (p.0 - last.0).abs() + (p.1 - last.1).abs();
    if d > manhattan_skip_threshold {
      return true;
    }
    last = p;
  }
  return false;
}
