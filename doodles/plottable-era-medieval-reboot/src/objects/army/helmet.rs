use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask,
  polylines::route_xreverse_translate_rotate,
};
use std::f32::consts::PI;

// TODO implement different helmet types

pub struct Helmet {
  pub origin: (f32, f32),
  pub angle: f32,
  pub size: f32,
  pub xreverse: bool,
}
impl Helmet {
  pub fn init(
    origin: (f32, f32),
    angle: f32,
    size: f32,
    xreverse: bool,
  ) -> Self {
    Self {
      origin,
      angle,
      size,
      xreverse,
    }
  }

  pub fn render(
    &self,
    paint: &mut PaintMask,
    clr: usize,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    full_helmet(
      paint,
      clr,
      self.origin,
      self.angle,
      self.size,
      self.xreverse,
    )
  }
}

fn full_helmet(
  paint: &mut PaintMask,
  clr: usize,
  origin: (f32, f32),
  angle: f32,
  size: f32,
  xreverse: bool,
) -> Vec<(usize, Vec<(f32, f32)>)> {
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
  let routes = regular_clip(
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
  );

  // consider routes to be polygon for now.
  for (_clr, route) in &routes {
    paint.paint_polygon(&route);
  }

  routes
}
