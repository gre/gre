use rand::prelude::*;
use std::f32::consts::PI;

use super::{
  math2d::{euclidian_dist, sample_2d_candidates_f32},
  polylines::{path_subdivide_to_curve, Polyline},
};

pub fn circle_route(
  center: (f32, f32),
  r: f32,
  count: usize,
) -> Vec<(f32, f32)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * (i as f32) / (count as f32);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

pub fn circle_route_angleoff(
  center: (f32, f32),
  r: f32,
  count: usize,
  angleoff: f32,
) -> Vec<(f32, f32)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = angleoff + 2. * PI * (i as f32) / (count as f32);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

pub fn arc(
  center: (f32, f32),
  r: f32,
  start: f32,
  end: f32,
  count: usize,
) -> Vec<(f32, f32)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = start + (end - start) * i as f32 / (count as f32);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

pub fn spiral_optimized(
  x: f32,
  y: f32,
  radius: f32,
  dr: f32,
  approx: f32,
) -> Vec<(f32, f32)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut last = (0., 0.);
  let mut a = 0f32;
  loop {
    let p = (x + r * a.cos(), y + r * a.sin());
    if route.is_empty() || euclidian_dist(last, p) > approx {
      last = p;
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

pub fn spiral_optimized_with_initial_angle(
  x: f32,
  y: f32,
  radius: f32,
  initial: f32,
  dr: f32,
  approx: f32,
  reverse: bool,
) -> Vec<(f32, f32)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = initial;
  let m = if reverse { -1. } else { 1. };
  loop {
    let p = (x + r * a.cos(), y + r * a.sin());
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 0.2 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (two_pi + a + da * m) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
}

pub fn yarnballs<R: Rng>(
  rng: &mut R,
  o: (f32, f32),
  r: f32,
  density: f32,
) -> Polyline {
  let pow = 1.8;
  let samples = sample_2d_candidates_f32(
    rng,
    &|p| {
      let dx = p.0 - 0.5;
      let dy = p.1 - 0.5;
      let d2 = dx * dx + dy * dy;
      if d2 > 0.25 {
        0.0
      } else {
        d2
      }
    },
    (6. * r) as usize,
    (8. + density * (r).powf(pow)) as usize,
  );
  let route = path_subdivide_to_curve(&samples, 2, 0.7);
  route
    .iter()
    .map(|(x, y)| (2.0 * r * (x - 0.5) + o.0, 2.0 * r * (y - 0.5) + o.1))
    .collect()
}
