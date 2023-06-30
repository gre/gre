use clap::*;
use gre::*;
use noise::*;
use rand::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "148.5")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "1.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = Vec::new();

  let h = 11.0;

  let mut y = pad + 0.5 * h;

  let mut paint = PaintMask::new(0.3, width, height);

  while y <= height - pad {
    routes.push((0, vec![(pad, y), (width - pad, y)]));

    let w = h * mix(0.5, 2.0, y / height);
    let carh = w * 0.45;
    let mut x = width - rng.gen_range(1.0, 2.5) * pad - 0.5 * w;
    while x > pad + 0.5 * w {
      let clr = rng.gen_range(0, 3);
      routes.extend(gpmoto(
        &mut rng,
        &mut paint,
        (x, y - 0.5),
        clr,
        carh,
        true,
      ));
      x -= w
        * (1.05
          + rng.gen_range(-1.0, 1.0)
            * rng.gen_range(-3.0f64, 2.0).max(0.0).min(1.0));
    }

    y += carh + pad;
  }

  let colors = vec!["black", "grey", "red"];

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.5, data));
      l
    })
    .collect()
}

fn gpmoto<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  (x, y): (f64, f64),
  clr: usize,
  h: f64,
  xrev: bool,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let xmul = if xrev { -1. } else { 1. };

  // pad for the paint mask
  let ppad = 0.15 * h;

  let w = 2. * h;
  let wheelr = 0.25 * h;
  let wheelrinner = 0.55 * wheelr;
  let wheelrextra = 0.2 * wheelr;

  let base = rng.gen_range(4, 12);
  let bodycoords = vec![
    (26, 48),
    (53, rng.gen_range(45, 52)),
    (52, 43),
    (23, 30),
    (18, base),
    (-21, base),
    (-23, 16),
    (-44, 36),
    (-26, 56),
    (-16, 52),
    (rng.gen_range(-30, -20), 39),
    (rng.gen_range(0, 22), 37),
  ];

  let scale = 0.017;
  let disp = 0.2 * h;

  let mut body = bodycoords
    .iter()
    .map(|&(dx, dy)| {
      let dx = dx as f64 * xmul
        + rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0) * disp;
      let dy =
        dy as f64 + rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0) * disp;
      (dx * h * scale + x, -dy * h * scale + y)
    })
    .collect::<Vec<(f64, f64)>>();
  body.push(body[0]);

  let bodypoly = body.clone();

  /*
  bodypoly.remove(9);
  bodypoly.remove(9);
  bodypoly.remove(9);
  let top = y - h * 1.3;
  bodypoly[9].1 = top;
  bodypoly[8].1 = top;
  bodypoly[0].1 = top;
  */

  if rng.gen_bool(0.5) {
    body = path_subdivide_to_curve_it(&body, rng.gen_range(0.85, 0.95));
  }

  // driver body

  let footx = rng.gen_range(-5, 10);
  let footy = rng.gen_range(10, 20);

  let handx = -rng.gen_range(4, 12);

  let drivercoords = vec![
    (footx, footy),
    (footx + 10, footy + 6),
    (-3, 37),
    (rng.gen_range(15, 20), rng.gen_range(45, 55)),
    (-3, 66),
    (handx, 42),
    (handx - rng.gen_range(5, 15), 42),
  ];

  let mut driver = drivercoords
    .iter()
    .map(|&(dx, dy)| {
      let dx = dx as f64 * xmul;
      let dy = dy as f64;
      (dx * h * scale + x, -dy * h * scale + y)
    })
    .collect::<Vec<(f64, f64)>>();
  let mut headc = driver[4].clone();

  driver = path_subdivide_to_curve_it(&driver, rng.gen_range(0.85, 0.95));
  driver = path_subdivide_to_curve_it(&driver, rng.gen_range(0.85, 0.95));

  let mut driverstrokes = vec![];
  for p in cordon(
    driver,
    0.18 * h,
    0.02 * h,
    0.2,
    (h * 0.3) as usize + 1,
    false,
    2.0,
    0.0,
  ) {
    driverstrokes.push((clr, p));
  }

  headc.0 += rng.gen_range(0.16, 0.2) * h;
  headc.1 += rng.gen_range(0.0, 0.05) * h;
  let dt = 0.3;
  let mut r = dt * 0.5;
  let maxr = r + 0.1 * h;
  let count = 20;
  let ang = rng.gen_range(-0.5, 0.5);
  let sx = rng.gen_range(1.5, 2.0);
  while r < maxr {
    let route = circle_route((0., 0.), r, count);
    let route = route
      .iter()
      .map(|&(x, y)| {
        let x = x * sx;
        let (x, y) = p_r((x, y), ang);
        (x + headc.0, y + headc.1)
      })
      .collect::<Vec<(f64, f64)>>();
    driverstrokes.push((clr, route));
    r += dt;
  }

  let is_outside = |p| paint.is_painted(p);
  routes.extend(clip_routes_with_colors(
    &vec![vec![(clr, body.clone())], driverstrokes.clone()].concat(),
    &is_outside,
    0.5,
    5,
  ));
  paint.paint_polygon(&bodypoly);

  let mut wheels = vec![];
  let mut wheelspoly = vec![];

  for side in 0..2 {
    let xc = x + xmul * (side as f64 - 0.5) * (w - 2. * wheelr);
    let c = (xc, y - wheelr);
    let incr = 0.3;
    let mut r = wheelr;
    let rays = (wheelr * 2. + 20.) as usize;

    wheelspoly.push(circle_route(c, r + ppad, rays));

    while r > wheelrinner {
      let route = circle_route(c, r, rays);
      wheels.push((clr, route));
      r -= incr;
    }

    let route = circle_route(c, wheelrextra, rays);
    wheels.push((clr, route));

    // connect the wheels
    let mut route = vec![];
    route.push(c);
    if side == 0 {
      route.push((x - 0.3 * xmul * w, y - 0.5 * h));
    } else {
      route.push((x, y - 0.5 * h));
      route.push(c);
      route.push((x + rng.gen_range(0.1, 0.2) * xmul * w, y - 0.7 * h));
    }
    wheels.push((clr, route));
  }

  let is_outside = |p| paint.is_painted(p);
  routes.extend(clip_routes_with_colors(&wheels, &is_outside, 0.5, 5));
  for poly in wheelspoly {
    paint.paint_polygon(&poly);
  }

  for (_, path) in driverstrokes {
    for p in path {
      paint.paint_circle(p, ppad);
    }
  }
  for p in path_subdivide_to_curve_it(&bodypoly, 0.75) {
    paint.paint_circle(p, ppad);
  }

  // debug rect
  // routes.push((2, bodypoly.clone()));

  routes
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

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  fn paint_point(&mut self, point: (f64, f64)) {
    let index = self.index(point);
    self.mask[index] = true;
  }

  fn fill_fn(&mut self, f: impl Fn((f64, f64)) -> bool) {
    let wi = (self.width / self.precision) as usize;
    let hi = (self.height / self.precision) as usize;
    for y in 0..wi {
      for x in 0..hi {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        if v {
          self.mask[y * wi + x] = v;
        }
      }
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

  pub fn grow(self: &mut Self, radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters: Vec<bool> = self.mask.iter().cloned().collect();
    let mut mask = Vec::new();
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
        if counters[index] {
          for &(dx, dy) in mask.iter() {
            self.paint_point((x + dx, y + dy));
          }
        }
        y += precision;
      }
      x += precision;
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

  fn paint_circle(&mut self, center: (f64, f64), radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let minx = ((center.0 - radius) / precision) as usize;
    let miny = ((center.1 - radius) / precision) as usize;
    let maxx = ((center.0 + radius) / precision) as usize;
    let maxy = ((center.1 + radius) / precision) as usize;
    let wi = (width / precision) as usize;
    let r2 = radius * radius;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        let dx = point.0 - center.0;
        let dy = point.1 - center.1;
        if dx * dx + dy * dy < r2 {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
}

fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  point: (f64, f64),
) -> bool {
  let mut c = false;
  for i in 0..polygon.len() {
    let j = (i + 1) % polygon.len();
    if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
      && (point.0
        < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
          / (polygon[j].1 - polygon[i].1)
          + polygon[i].0)
    {
      c = !c;
    }
  }
  c
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

fn cordon(
  path: Vec<(f64, f64)>,
  width: f64,
  noiseamp: f64,
  corner_pad: f64,
  tracks_count: usize,
  reconnect: bool,
  freq_mul: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let precision = 0.5;
  let r = precision;
  let mut pindex = 0;
  let mut p = path[pindex];
  let perlin = Perlin::new();
  let mut tracks = Vec::new();
  for _xi in 0..tracks_count {
    tracks.push(Vec::new());
  }
  for &next in path.iter().skip(1) {
    let dx = next.0 - p.0;
    let dy = next.1 - p.1;
    let a = dy.atan2(dx);
    let mut i = 0.0;
    let acos = a.cos();
    let asin = a.sin();
    let mut dist = (dx * dx + dy * dy).sqrt();
    if pindex != 0 {
      dist -= corner_pad;
      p.0 += corner_pad * acos;
      p.1 += corner_pad * asin;
    }
    if pindex == path.len() - 1 {
      dist -= corner_pad;
    }
    loop {
      if i >= dist {
        p = next;
        break;
      }
      p.0 += r * acos;
      p.1 += r * asin;
      for xi in 0..tracks_count {
        let variation = ((xi as f64 + (tracks_count as f64 * phase))
          % (tracks_count as f64)
          - ((tracks_count - 1) as f64 / 2.0))
          / (tracks_count as f64);
        let mut delta = variation * width;
        let noisefreq = freq_mul * (0.1 + 0.2 * (0.5 - variation.abs()));
        delta += noiseamp
          * perlin.get([
            //
            noisefreq * p.0,
            noisefreq * p.1,
            10.0 * xi as f64,
          ]);
        let a2 = a + PI / 2.0;
        let q = (p.0 + delta * a2.cos(), p.1 + delta * a2.sin());
        tracks[xi].push(q);
      }
      i += r;
    }
    pindex += 1;
  }
  for track in tracks {
    let mut track_copy = track.clone();
    if reconnect {
      track_copy.push(track[0]);
    }
    routes.push(track_copy);
  }
  routes
}
