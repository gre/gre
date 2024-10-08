use std::f32::consts::PI;

use super::math1d::mix;
use rand::prelude::*;

pub fn euclidian_dist((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

pub fn lerp_point(a: (f32, f32), b: (f32, f32), m: f32) -> (f32, f32) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

pub fn p_r(p: (f32, f32), a: f32) -> (f32, f32) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

pub fn collides_segment(
  from_1: (f32, f32),
  to_1: (f32, f32),
  from_2: (f32, f32),
  to_2: (f32, f32),
) -> Option<(f32, f32)> {
  // see https://stackoverflow.com/a/565282
  let p = from_1;
  let q = from_2;
  let r = (to_1.0 - p.0, to_1.1 - p.1);
  let s = (to_2.0 - q.0, to_2.1 - q.1);

  let r_cross_s = cross(r, s);
  let q_minus_p = (q.0 - p.0, q.1 - p.1);
  let q_minus_p_cross_r = cross(q_minus_p, r);

  // are the lines are parallel?
  if r_cross_s == 0.0 {
    // are the lines collinear?
    if q_minus_p_cross_r == 0.0 {
      // the lines are collinear
      None
    } else {
      // the lines are parallel but not collinear
      None
    }
  } else {
    // the lines are not parallel
    let t = cross(q_minus_p, div(s, r_cross_s));
    let u = cross(q_minus_p, div(r, r_cross_s));

    // are the intersection coordinates both in range?
    let t_in_range = 0.0 <= t && t <= 1.0;
    let u_in_range = 0.0 <= u && u <= 1.0;

    if t_in_range && u_in_range {
      // there is an intersection
      Some((p.0 + t * r.0, p.1 + t * r.1))
    } else {
      // there is no intersection
      None
    }
  }
}

fn cross(a: (f32, f32), b: (f32, f32)) -> f32 {
  a.0 * b.1 - a.1 * b.0
}

fn div(a: (f32, f32), b: f32) -> (f32, f32) {
  (a.0 / b, a.1 / b)
}

pub fn same_point(a: (f32, f32), b: (f32, f32)) -> bool {
  (a.0 - b.0).abs() < 0.0001 && (a.1 - b.1).abs() < 0.0001
}

pub fn strictly_in_boundaries(
  p: (f32, f32),
  boundaries: (f32, f32, f32, f32),
) -> bool {
  p.0 > boundaries.0
    && p.0 < boundaries.2
    && p.1 > boundaries.1
    && p.1 < boundaries.3
}

// ridge is ordered on x
pub fn lookup_ridge(ridge: &Vec<(f32, f32)>, x: f32) -> f32 {
  let mut last = ridge[0];
  if x <= last.0 {
    return last.1;
  }
  // FIXME opportunity to rewrite this with dochotomic search
  for &p in ridge.iter() {
    if last.0 < x && x <= p.0 {
      let y = mix(last.1, p.1, (x - last.0) / (p.0 - last.0));
      return y;
    } else {
      last = p;
    }
  }
  return last.1;
}

pub fn sample_2d_candidates_f32<R: Rng>(
  rng: &mut R,
  f: &dyn Fn((f32, f32)) -> f32,
  dim: usize,
  samples: usize,
) -> Vec<(f32, f32)> {
  let mut candidates = Vec::new();
  for x in 0..dim {
    for y in 0..dim {
      let p = ((x as f32) / (dim as f32), (y as f32) / (dim as f32));
      if f(p) > rng.gen_range(0.0..1.0) {
        candidates.push(p);
      }
    }
  }
  candidates.shuffle(rng);
  candidates.truncate(samples);
  return candidates;
}

pub fn polar_sort_from_center(pts: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
  let mut sumx = 0.0;
  let mut sumy = 0.0;
  for &p in pts.iter() {
    sumx += p.0;
    sumy += p.1;
  }
  let l = pts.len();
  let center = (sumx / (l as f32), sumy / (l as f32));
  let mut points = pts
    .iter()
    .map(|p| {
      let dy = p.1 - center.1;
      let dx = p.0 - center.0;
      let a = dy.atan2(dx);
      (p, a)
    })
    .collect::<Vec<_>>();
  points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  points.iter().map(|p| *(p.0)).collect()
}

pub fn angle_between_0_2pi(a: f32) -> f32 {
  let mut a = a;
  while a < 0.0 {
    a += 2.0 * PI;
  }
  while a > 2.0 * PI {
    a -= 2.0 * PI;
  }
  a
}

pub fn angle_mirrored_on_x(a: f32) -> f32 {
  let acos = -a.cos();
  let asin = a.sin();
  asin.atan2(acos)
}

pub fn distance_angles(a1: f32, a2: f32) -> f32 {
  let norm_a1 = angle_between_0_2pi(a1);
  let norm_a2 = angle_between_0_2pi(a2);
  let diff = (norm_a1 - norm_a2).abs();
  if diff > PI {
    2.0 * PI - diff
  } else {
    diff
  }
}
