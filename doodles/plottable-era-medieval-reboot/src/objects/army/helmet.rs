use crate::algo::{
  clipping::regular_clip, math2d::p_r, paintmask::PaintMask,
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
          route_xreverse_translate_rotate(&route, xreverse, origin, ang),
        )
      })
      .collect(),
    paint,
  )
}

pub fn helmet(
  origin: (f64, f64),
  angle: f64,
  size: f64,
  xreverse: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = 0.4 * size;

  // head
  routes.push(vec![(-dx, 0.0), (-dx, -h), (dx, -h), (dx, 0.0), (-dx, 0.0)]);

  routes.push(vec![
    (-dx, -h * 0.7),
    (-dx, -h * 0.8),
    (dx, -h * 0.8),
    (dx, -h * 0.7),
    (-dx, -h * 0.7),
  ]);

  // TODO implement

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  routes
    .iter()
    .map(|route| {
      (
        clr,
        route
          .iter()
          .map(|&(x, y)| {
            let x = if xreverse { -x } else { x };
            let (x, y) = p_r((x, y), ang);
            (x + origin.0, y + origin.1)
          })
          .collect(),
      )
    })
    .collect()
}
