use crate::algo::polylines::route_translate_rotate;
use std::f64::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub fn head_square(
  clr: usize,
  origin: (f64, f64),
  angle: f64,
  size: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = 0.4 * size;
  routes.push(vec![(-dx, 0.0), (-dx, -h), (dx, -h), (dx, 0.0), (-dx, 0.0)]);

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  routes
    .iter()
    .map(|route| (clr, route_translate_rotate(route, origin, ang)))
    .collect()
}
