use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{
    route_rotate, route_translate_rotate, translate_rotate, Polylines,
  },
  shapes::{circle_route, spiral_optimized},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct WheeledPlatform {
  pub origin: (f32, f32),
  pub h: f32,
  pub w: f32,
  pub angle: f32,
  pub plank_ratio: f32,
  pub wheel_pad: f32,
  pub wheel_count: usize,
  pub plank_filling: f32,
  pub wheel_filling: f32,
}

impl WheeledPlatform {
  pub fn init(
    origin: (f32, f32),
    h: f32,
    w: f32,
    angle: f32,
    wheel_pad: f32,
    wheel_count: usize,
  ) -> Self {
    let plank_ratio = 0.5;
    let plank_filling = 1.0;
    let wheel_filling = 0.7;
    Self {
      origin,
      h,
      w,
      angle,
      plank_ratio,
      wheel_pad,
      wheel_count,
      plank_filling,
      wheel_filling,
    }
  }

  pub fn render(&self, paint: &mut PaintMask, clr: usize) -> Polylines {
    let mut routes = Polylines::new();
    let origin = self.origin;
    let h = self.h;
    let w = self.w;
    let angle = self.angle;
    let plank_ratio = self.plank_ratio;
    let wheel_pad = self.wheel_pad;
    let wheel_count = self.wheel_count;

    // Wheels
    let r = h / 2.0;
    let mut p = (-w / 2. + wheel_pad, 0.0);
    let incrx = if wheel_count > 1 {
      (w - 2. * wheel_pad) / (wheel_count as f32 - 1.0)
    } else {
      0.0
    };
    for _i in 0..wheel_count {
      let o = translate_rotate(p, origin, angle);
      routes.extend(wheel(paint, o, r, self.wheel_filling, clr));
      p.0 += incrx;
    }

    // Plank
    let mut plankrts = Polylines::new();
    let hw = w / 2.0;
    let rh = plank_ratio * h / 2.0;
    let mut rect = vec![(-hw, -rh), (hw, -rh), (hw, rh), (-hw, rh)];
    rect = route_translate_rotate(&rect, origin, angle);
    rect.push(rect[0]);
    plankrts.push((clr, rect.clone()));
    let incr = self.plank_filling;
    let mut dy = -rh + incr;
    while dy <= rh - incr {
      let rt =
        route_translate_rotate(&vec![(-hw, dy), (hw, dy)], origin, angle);
      plankrts.push((clr, rt));
      dy += incr;
    }
    plankrts = regular_clip(&plankrts, paint);
    routes.extend(plankrts);
    paint.paint_polygon(&rect);

    routes
  }
}

fn wheel(
  paint: &mut PaintMask,
  p: (f32, f32),
  r: f32,
  dr: f32,
  clr: usize,
) -> Polylines {
  let mut routes = vec![];
  routes.push((clr, circle_route(p, r, 16)));
  routes.push((clr, spiral_optimized(p.0, p.1, r, dr, 0.1)));
  routes = regular_clip(&routes, paint);
  paint.paint_circle(p.0, p.1, r);
  routes
}
