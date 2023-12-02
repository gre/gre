use std::f32::consts::PI;

use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, polylines::Polylines,
  renderable::Renderable,
};
use rand::prelude::*;

use super::wheeledplatform::wheel;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct ConvoyWalk {
  pub routes: Polylines,
  pub bgroutes: Polylines,
  pub left: (f32, f32),
  pub right: (f32, f32),
  pub wheelp: (f32, f32),
  pub wheelr: f32,
  pub origin: (f32, f32),
}

impl ConvoyWalk {
  pub fn init<R: Rng>(
    _rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    w: f32,
    extraratio: f32,
  ) -> Self {
    let mut routes = vec![];

    let spread = (0.5 + extraratio) * size;
    let dx = angle.cos() / 2.0;
    let dy = angle.sin() / 2.0;
    let left = (origin.0 - spread * dx, origin.1 - spread * dy);
    let right = (origin.0 + spread * dx, origin.1 + spread * dy);

    let left2 = (origin.0 - 0.5 * size * dx, origin.1 - 0.5 * size * dy);
    let right2 = (origin.0 + 0.5 * size * dx, origin.1 + 0.5 * size * dy);

    let a = angle + PI / 2.0;
    let d2x = a.cos();
    let d2y = a.sin();
    let mut y = 0.0;
    while y < w {
      let p3 = (left.0 + y * d2x, left.1 + y * d2y);
      let p4 = (right.0 + y * d2x, right.1 + y * d2y);
      routes.push((clr, vec![p3, p4]));
      y += 0.4;
    }

    let dist = size * 0.2;
    let wheelr = size * 0.1;
    let wheelp = (origin.0 + dist * d2x, origin.1 + dist * d2y);

    let dx = d2x * w / 2.0;
    let dy = d2y * w / 2.0;
    let bgroutes = vec![
      (
        clr,
        vec![
          (left2.0 + dx, left2.1 + dy),
          (wheelp.0 + dx, wheelp.1 + dy),
          (wheelp.0 - dx, wheelp.1 - dy),
          (left2.0 - dx, left2.1 - dy),
          (left2.0 + dx, left2.1 + dy),
        ],
      ),
      (
        clr,
        vec![
          (right2.0 + dx, right2.1 + dy),
          (wheelp.0 + dx, wheelp.1 + dy),
          (wheelp.0 - dx, wheelp.1 - dy),
          (right2.0 - dx, right2.1 - dy),
          (right2.0 + dx, right2.1 + dy),
        ],
      ),
    ];

    Self {
      routes,
      bgroutes,
      left,
      right,
      wheelp,
      wheelr,
      origin,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut routes = regular_clip(&self.routes, paint);
    for (_, poly) in routes.iter() {
      paint.paint_polyline(poly, 0.6);
    }

    routes.extend(wheel(paint, self.wheelp, self.wheelr, 0.6, 0));

    routes.extend(regular_clip(&self.bgroutes, paint));
    for (_, poly) in self.bgroutes.iter() {
      paint.paint_polygon(poly);
      paint.paint_polyline(poly, 0.6);
    }

    routes
  }
}

impl<R: Rng> Renderable<R> for ConvoyWalk {
  fn render(
    &self,
    _rng: &mut R,
    _ctx: &mut crate::global::GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(paint)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
