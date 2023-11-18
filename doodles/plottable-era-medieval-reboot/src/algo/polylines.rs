use std::f32::consts::PI;

use super::math2d::*;
use rand::prelude::*;

pub type Polyline = Vec<(f32, f32)>;
pub type Polylines = Vec<(usize, Polyline)>;

pub fn path_subdivide_to_curve_it(
  path: Vec<(f32, f32)>,
  interpolation: f32,
) -> Vec<(f32, f32)> {
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

pub fn path_subdivide_to_curve(
  path: Vec<(f32, f32)>,
  n: usize,
  interpolation: f32,
) -> Vec<(f32, f32)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}

pub fn shake<R: Rng>(
  path: Vec<(f32, f32)>,
  scale: f32,
  rng: &mut R,
) -> Vec<(f32, f32)> {
  path
    .iter()
    .map(|&(x, y)| {
      let dx = rng.gen_range(-scale..scale);
      let dy = rng.gen_range(-scale..scale);
      (x + dx, y + dy)
    })
    .collect()
}

pub fn route_rotate(route: &Vec<(f32, f32)>, angle: f32) -> Vec<(f32, f32)> {
  let acos = angle.cos();
  let asin = angle.sin();
  route
    .iter()
    .map(|&(x, y)| (x * acos + y * asin, y * acos - x * asin))
    .collect()
}

pub fn route_translate_rotate(
  route: &Vec<(f32, f32)>,
  origin: (f32, f32),
  angle: f32,
) -> Vec<(f32, f32)> {
  let acos = angle.cos();
  let asin = angle.sin();
  route
    .iter()
    .map(|&(x, y)| {
      (
        x * acos + y * asin + origin.0,
        y * acos - x * asin + origin.1,
      )
    })
    .collect()
}

pub fn translate_rotate(
  (x, y): (f32, f32),
  origin: (f32, f32),
  angle: f32,
) -> (f32, f32) {
  let acos = angle.cos();
  let asin = angle.sin();
  (
    x * acos + y * asin + origin.0,
    y * acos - x * asin + origin.1,
  )
}

pub fn route_xreverse_translate_rotate(
  route: &Vec<(f32, f32)>,
  xreverse: bool,
  origin: (f32, f32),
  angle: f32,
) -> Vec<(f32, f32)> {
  let acos = angle.cos();
  let asin = angle.sin();
  route
    .iter()
    .map(|&(x, y)| {
      let x = if xreverse { -x } else { x };
      (
        x * acos + y * asin + origin.0,
        y * acos - x * asin + origin.1,
      )
    })
    .collect()
}

pub fn route_scale_translate_rotate(
  route: &Vec<(f32, f32)>,
  scale: (f32, f32),
  origin: (f32, f32),
  angle: f32,
) -> Vec<(f32, f32)> {
  let acos = angle.cos();
  let asin = angle.sin();
  route
    .iter()
    .map(|&(x, y)| {
      let (x, y) = (x * acos + y * asin, y * acos - x * asin);
      (x * scale.0 + origin.0, y * scale.1 + origin.1)
    })
    .collect()
}

pub fn grow_path_zigzag(
  path: Vec<(f32, f32)>,
  angle: f32,
  width: f32,
  line_dist: f32,
) -> Vec<(f32, f32)> {
  let mut route: Vec<(f32, f32)> = Vec::new();
  let dx = angle.cos();
  let dy = angle.sin();
  let incr_dx = -dy;
  let incr_dy = dx;

  let mut route = Vec::new();
  let count = (width / line_dist).ceil() as usize;
  let delta_i = if count < 2 { 0.0 } else { count as f32 / 2.0 };
  let mut rev = false;
  for i in 0..count {
    let mul = (i as f32 - delta_i) / (count as f32);
    let w = width * mul;
    let it: Vec<&(f32, f32)> = if rev {
      path.iter().rev().collect()
    } else {
      path.iter().collect()
    };
    for p in it {
      let (x, y) = p;
      let a = (x + incr_dx * w, y + incr_dy * w);
      route.push(a);
    }
    rev = !rev;
  }

  route
}

pub fn grow_as_rectangle(
  from: (f32, f32),
  to: (f32, f32),
  width: f32,
) -> Vec<(f32, f32)> {
  let (x0, y0) = from;
  let (x1, y1) = to;
  let (dx, dy) = (x1 - x0, y1 - y0);
  let len = (dx * dx + dy * dy).sqrt();
  let incr_dx = -width * dy / len;
  let incr_dy = width * dx / len;
  let mut route = Vec::new();
  route.push((x0 + incr_dx, y0 + incr_dy));
  route.push((x1 + incr_dx, y1 + incr_dy));
  route.push((x1 - incr_dx, y1 - incr_dy));
  route.push((x0 - incr_dx, y0 - incr_dy));
  route.push((x0 + incr_dx, y0 + incr_dy));
  route
}

pub fn grow_stroke_zigzag(
  from: (f32, f32),
  to: (f32, f32),
  width: f32,
  line_dist: f32,
) -> Vec<(f32, f32)> {
  let (x0, y0) = from;
  let (x1, y1) = to;
  let (dx, dy) = (x1 - x0, y1 - y0);
  let len = (dx * dx + dy * dy).sqrt();
  let incr_dx = -dy / len;
  let incr_dy = dx / len;

  let mut route = Vec::new();
  let count = (width / line_dist).ceil() as usize;
  let delta_i = if count < 2 { 0.0 } else { count as f32 / 2.0 };
  let mut rev = false;
  for i in 0..count {
    let mul = (i as f32 - delta_i) / (count as f32);
    let w = width * mul;
    let a = (from.0 + incr_dx * w, from.1 + incr_dy * w);
    let b = (to.0 + incr_dx * w, to.1 + incr_dy * w);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    rev = !rev;
  }

  route
}

pub fn slice_polylines(
  route: &Vec<(f32, f32)>,
  segment_length: f32,
) -> Vec<Vec<(f32, f32)>> {
  let mut routes = vec![];
  let mut l = 0.0;
  let mut i = 1;
  let mut prev = route[0];
  let mut segment = vec![];
  segment.push(prev);
  loop {
    if i >= route.len() {
      break;
    }
    let mut next = route[i];
    let mut d = euclidian_dist(prev, next);
    while l + d < segment_length {
      segment.push(next);
      l += d;
      i += 1;
      if i >= route.len() {
        routes.push(segment);
        return routes;
      }
      prev = next;
      next = route[i];
      d = euclidian_dist(prev, next);
    }
    let current = lerp_point(prev, next, (segment_length - l) / d);
    segment.push(current);
    prev = current;
    if segment.len() > 1 {
      routes.push(segment);
      segment = vec![prev];
    }

    l = 0.0;
    prev = current;
    i += 1;
  }
  routes
}

// follow a path to build two polylines that expand along the path with some widths
pub fn path_to_fibers(
  path: Polyline,
  widths: Vec<f32>,
  count: usize,
) -> Vec<Polyline> {
  if count < 2 {
    return vec![path];
  }
  let mut fibers: Vec<Vec<(f32, f32)>> = vec![];
  for _ in 0..count {
    fibers.push(vec![]);
  }
  for i in 0..count {
    let df = (i as f32) / ((count - 1) as f32) - 0.5;
    for j in 0..path.len() {
      let p = path[j];
      let a = if j > 0 {
        let prev = path[j - 1];
        (p.1 - prev.1).atan2(p.0 - prev.0)
      } else {
        (path[1].1 - path[0].1).atan2(path[1].0 - path[0].0)
      };
      let orthogonal = a + PI / 2.0;
      let dist = widths[j];
      let d = df * dist * 0.5;
      let q = (p.0 + d * orthogonal.cos(), p.1 + d * orthogonal.sin());
      fibers[i].push(q);
    }
  }

  fibers
}
