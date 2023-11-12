use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask,
  polylines::route_xreverse_translate_rotate,
};
use std::f64::consts::PI;

pub fn full_helmet(
  paint: &mut PaintMask,
  clr: usize,
  origin: (f64, f64),
  angle: f64,
  size: f64,
  xreverse: bool,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = 0.4 * size;
  let extrax = 0.1 * size;
  routes.push(vec![
    (-dx, 0.0),
    (-dx, -h),
    (dx, -h),
    (dx + extrax, -0.5 * h),
    (dx, 0.0),
    (-dx, 0.0),
  ]);

  routes.push(vec![
    (dx + extrax, -0.5 * h),
    (0.2 * dx, -1.3 * h),
    (0.2 * dx, 0.3 * h),
  ]);
  routes.push(vec![(-dx, -0.5 * h), (dx + 0.6 * extrax, -0.4 * h)]);
  routes.push(vec![(-dx, -0.5 * h), (dx + 0.6 * extrax, -0.6 * h)]);

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  regular_clip(
    &routes
      .iter()
      .map(|route| {
        (
          clr,
          route_xreverse_translate_rotate(&route, xreverse, origin, angle),
        )
      })
      .collect(),
    paint,
  )
}
