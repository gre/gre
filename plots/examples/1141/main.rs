use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes(
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

  for (c, input_route) in input_routes.iter() {
    if input_route.len() < 2 {
      continue;
    }
    let clr = *c;

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
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if euclidian_dist(point, (circle.x, circle.y)) < circle.r {
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

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "400.0")]
  pub width: f64,
  #[clap(short, long, default_value = "600.0")]
  pub height: f64,
  #[clap(short, long, default_value = "30.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "200.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);

  let mut mask = PaintMask::new(0.1, width, height);

  let perlin = Perlin::new();
  let mut routes = Vec::new(); // all the paths to draw are stored here

  let max_clouds = 8.0;
  let count = (rng.gen_range(0.0, max_clouds)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0)) as usize;

  let in_shape = |p: (f64, f64)| -> bool {
    !mask.is_painted(p) && strictly_in_boundaries(p, bound)
  };

  let does_overlap = |c: &VCircle| {
    in_shape((c.x, c.y))
      && circle_route((c.x, c.y), c.r, 8)
        .iter()
        .all(|&p| in_shape(p))
  };

  let count = rng.gen_range(0, 100);
  let min = rng.gen_range(8.0, 20.0);
  let max = min + rng.gen_range(0.0, 50.0);

  let circles = packing(
    &mut rng,
    50000,
    count,
    1,
    0.0,
    bound,
    &does_overlap,
    min,
    max,
  );

  let count = rng.gen_range(0, 200);
  let min = rng.gen_range(8.0, 20.0);
  let max = min + rng.gen_range(0.0, 40.0);

  let circles2 = packing(
    &mut rng,
    500000,
    count,
    1,
    0.0,
    bound,
    &does_overlap,
    min,
    max,
  );

  let clouds: Vec<VCircle> = circles
    .iter()
    .flat_map(|c| {
      let (rts, circles) = cloud_in_circle(&mut rng, &c);
      let rts = clip_routes(
        &rts.iter().map(|r| (0, r.clone())).collect(),
        &|p| mask.is_painted(p),
        0.3,
        7,
      );
      routes.extend(rts);
      for c in circles.clone() {
        mask.paint_circle(&c);
      }
      circles
    })
    .collect();

  let clouds2: Vec<VCircle> = circles2
    .iter()
    .flat_map(|c| {
      let (rts, circles) = cloud_in_circle(&mut rng, &c);
      let rts = clip_routes(
        &rts.iter().map(|r| (0, r.clone())).collect(),
        &|p| mask.is_painted(p),
        0.3,
        7,
      );
      routes.extend(rts);
      for c in circles.clone() {
        mask.paint_circle(&c);
      }
      circles
    })
    .collect();

  let routes_copy = clip_routes(
    &routes
      .iter()
      .map(|route| (0, route.1.iter().map(|&(x, y)| (x, height - y)).collect()))
      .collect(),
    &|p| mask.is_painted(p),
    0.1,
    10,
  );

  routes.extend(routes_copy);

  vec!["black"]
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
      l = l.add(base_path(color, 0.3, data));
      l
    })
    .collect()
}

fn cloud_in_circle<R: Rng>(
  rng: &mut R,
  circle: &VCircle,
) -> (Vec<Vec<(f64, f64)>>, Vec<VCircle>) {
  // FIXME the clouds have a weird issue on the fact we don't always see the edges

  let mut routes = vec![];

  let mut circles: Vec<VCircle> = vec![];

  let stretchy = rng.gen_range(0.2, 1.0);

  let count = rng.gen_range(16, 40);
  for _i in 0..count {
    let radius = circle.r * rng.gen_range(0.3, 0.5) * rng.gen_range(0.2, 1.0);
    let angle = rng.gen_range(0.0, 2.0 * PI);
    let x = circle.x + angle.cos() * (circle.r - radius);
    let y = circle.y
      + angle.sin() * (circle.r - radius) * rng.gen_range(0.5, 1.0) * stretchy;
    let circle = VCircle::new(x, y, radius);

    let should_crop = |p| circles.iter().any(|c| c.includes(p));

    let mut input_routes = vec![];
    let mut r = radius;
    let dr = rng.gen_range(1.0, 3.0);
    loop {
      if r < 1.0 {
        break;
      }
      let count = (r * 2.0 + 10.0) as usize;
      let amp = rng.gen_range(0.5 * PI, 1.2 * PI);
      let ang = angle
        + PI
          * rng.gen_range(-1.0, 1.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0);
      let start = ang - amp / 2.0;
      let end = ang + amp / 2.0;
      input_routes.push(arc((x, y), r, start, end, count));
      r -= dr;
    }

    routes.extend(crop_routes_with_predicate_rng(
      rng,
      0.0,
      input_routes,
      &should_crop,
      &mut vec![],
    ));

    circles.push(circle);
  }

  (routes, circles)
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

fn arc(
  center: (f64, f64),
  r: f64,
  start: f64,
  end: f64,
  count: usize,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = start + (end - start) * i as f64 / (count as f64);
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
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist(p, (self.x, self.y)) < self.r
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
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
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

// TODO rework with clip_routes
fn crop_routes_with_predicate_rng<R: Rng>(
  rng: &mut R,
  proba_skip: f64,
  input_routes: Vec<Vec<(f64, f64)>>,
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
    if proba_skip > 0.0 && rng.gen_bool(proba_skip) {
      routes.push(input_route);
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
