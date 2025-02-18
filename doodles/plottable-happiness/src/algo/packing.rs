use crate::algo::math2d::*;
use rand::prelude::*;
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
#[derive(Clone, Copy, Debug)]
pub struct VCircle {
  pub x: f32,
  pub y: f32,
  pub r: f32,
}
impl VCircle {
  pub fn new(x: f32, y: f32, r: f32) -> Self {
    VCircle { x, y, r }
  }
  pub fn pos(self: &Self) -> (f32, f32) {
    (self.x, self.y)
  }
  pub fn includes(self: &Self, p: (f32, f32)) -> bool {
    euclidian_dist(p, (self.x, self.y)) < self.r
  }
  pub fn dist(self: &Self, c: &VCircle) -> f32 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  pub fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

pub fn scaling_search<F: FnMut(f32) -> bool>(
  mut f: F,
  min_scale: f32,
  max_scale: f32,
) -> Option<f32> {
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

pub fn search_circle_radius(
  does_overlap: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f32,
  y: f32,
  min_scale: f32,
  max_scale: f32,
) -> Option<f32> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

pub fn packing<R: Rng>(
  rng: &mut R,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f32,
  bound: (f32, f32, f32, f32),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f32,
  max_scale: f32,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  for _i in 0..iterations {
    let x: f32 = rng.gen_range(bound.0..bound.2);
    let y: f32 = rng.gen_range(bound.1..bound.3);
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
