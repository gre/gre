use super::math2d::*;
use rand::prelude::*;

pub fn path_subdivide_to_curve_it(
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

pub fn path_subdivide_to_curve(
  path: Vec<(f64, f64)>,
  n: usize,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}

pub fn shake<R: Rng>(
  path: Vec<(f64, f64)>,
  scale: f64,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  path
    .iter()
    .map(|&(x, y)| {
      let dx = rng.gen_range(-scale..scale);
      let dy = rng.gen_range(-scale..scale);
      (x + dx, y + dy)
    })
    .collect()
}

pub fn route_translate_rotate(
  route: &Vec<(f64, f64)>,
  origin: (f64, f64),
  angle: f64,
) -> Vec<(f64, f64)> {
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

pub fn route_xreverse_translate_rotate(
  route: &Vec<(f64, f64)>,
  xreverse: bool,
  origin: (f64, f64),
  angle: f64,
) -> Vec<(f64, f64)> {
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
  route: &Vec<(f64, f64)>,
  scale: (f64, f64),
  origin: (f64, f64),
  angle: f64,
) -> Vec<(f64, f64)> {
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
  path: Vec<(f64, f64)>,
  angle: f64,
  width: f64,
  line_dist: f64,
) -> Vec<(f64, f64)> {
  let mut route: Vec<(f64, f64)> = Vec::new();
  let dx = angle.cos();
  let dy = angle.sin();
  let incr_dx = -dy;
  let incr_dy = dx;

  let mut route = Vec::new();
  let count = (width / line_dist).ceil() as usize;
  let delta_i = if count < 2 { 0.0 } else { count as f64 / 2.0 };
  let mut rev = false;
  for i in 0..count {
    let mul = (i as f64 - delta_i) / (count as f64);
    let w = width * mul;
    let it: Vec<&(f64, f64)> = if rev {
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

pub fn grow_stroke_zigzag(
  from: (f64, f64),
  to: (f64, f64),
  width: f64,
  line_dist: f64,
) -> Vec<(f64, f64)> {
  let (x0, y0) = from;
  let (x1, y1) = to;
  let (dx, dy) = (x1 - x0, y1 - y0);
  let len = (dx * dx + dy * dy).sqrt();
  let incr_dx = -dy / len;
  let incr_dy = dx / len;

  let mut route = Vec::new();
  let count = (width / line_dist).ceil() as usize;
  let delta_i = if count < 2 { 0.0 } else { count as f64 / 2.0 };
  let mut rev = false;
  for i in 0..count {
    let mul = (i as f64 - delta_i) / (count as f64);
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
  route: &Vec<(f64, f64)>,
  segment_length: f64,
) -> Vec<Vec<(f64, f64)>> {
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
