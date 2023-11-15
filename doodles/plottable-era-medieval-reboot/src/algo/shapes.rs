use std::f32::consts::PI;

use super::math2d::euclidian_dist;

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
  let mut a = 0f32;
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
