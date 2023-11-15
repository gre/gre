use crate::algo::{math2d::p_r, polylines::grow_stroke_zigzag};
use rand::prelude::*;

pub fn spear<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  size: f64,
  angle: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let spear_len = rng.gen_range(1.8..2.2) * size;
  let spear_w = 0.06 * size;

  let blade_w = 0.15 * size;
  let blade_len = 0.3 * size;

  let line_dist = 0.3;

  routes.push(grow_stroke_zigzag(
    (-spear_len / 2.0, 0.0),
    (spear_len / 2.0, 0.0),
    spear_w,
    line_dist,
  ));

  let mut route = Vec::new();
  route.push((spear_len / 2.0, -blade_w / 2.0));
  route.push((spear_len / 2.0 + blade_len, 0.0));
  route.push((spear_len / 2.0, blade_w / 2.0));
  route.push(route[0]);
  routes.push(route);

  // translate routes
  routes
    .iter()
    .map(|route| {
      (
        clr,
        route
          .iter()
          .map(|&(x, y)| {
            let (x, y) = p_r((x, y), angle);
            (x + origin.0, y + origin.1)
          })
          .collect(),
      )
    })
    .collect()
}
