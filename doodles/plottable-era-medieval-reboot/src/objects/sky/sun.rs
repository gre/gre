use crate::algo::{
  clipping::clip_routes_with_colors,
  paintmask::PaintMask,
  shapes::{circle_route, spiral_optimized},
};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub fn sun(
  paint: &mut PaintMask,
  clr: usize,
  c: (f64, f64),
  r: f64,
  dr: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let routes = vec![
    (clr, spiral_optimized(c.0, c.1, r, dr, 0.1)),
    (clr, circle_route(c, r, (r * 2. + 8.) as usize)),
  ];
  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  paint.paint_circle(c.0, c.1, r);
  routes
}
