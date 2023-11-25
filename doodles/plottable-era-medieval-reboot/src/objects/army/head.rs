use crate::algo::polylines::{
  path_subdivide_to_curve_it, route_translate_rotate,
};
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub fn head_square(
  clr: usize,
  origin: (f32, f32),
  angle: f32,
  size: f32,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = 0.4 * size;
  routes.push(path_subdivide_to_curve_it(
    &vec![(-dx, 0.0), (-dx, -h), (dx, -h), (dx, 0.0), (-dx, 0.0)],
    0.8,
  ));

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  routes
    .iter()
    .map(|route| (clr, route_translate_rotate(route, origin, ang)))
    .collect()
}

pub fn head_cyclope(
  clr: usize,
  origin: (f32, f32),
  angle: f32,
  size: f32,
  xflip: bool,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = Vec::new();
  let dx = 0.15 * size;
  let h = 0.5 * size;
  routes.push(path_subdivide_to_curve_it(
    &vec![(-dx, 0.0), (-dx, -h), (dx, -h), (dx, 0.0), (-dx, 0.0)],
    0.8,
  ));

  let xv = if xflip { 1.0 } else { -1.0 };

  let w2 = 0.15 * size;
  let h2 = 0.2 * size;
  let x1 = -dx * xv;
  let x2 = (-dx + w2) * xv;
  let y1 = -h / 2.0 - h2 / 2.0;
  let y2 = -h / 2.0 + h2 / 2.0;
  routes.push(vec![(x1, y1), (x2, y1), (x2, y2), (x1, y2), (x1, y1)]);

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  routes
    .iter()
    .map(|route| (clr, route_translate_rotate(route, origin, ang)))
    .collect()
}
