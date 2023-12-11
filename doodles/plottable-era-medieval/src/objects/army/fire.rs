use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    polylines::{Polyline, Polylines},
    renderable::Renderable,
    shapes::{circle_route, spiral_optimized},
  },
  global::GlobalCtx,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct Fire {
  pub flame_polys: Vec<Polyline>,
  pub flame_routes: Polylines,
  pub origin: (f32, f32),
  pub size: f32,
}

impl Fire {
  pub fn init<R: Rng>(
    rng: &mut R,
    flameclr: usize,
    origin: (f32, f32),
    rad: f32,
  ) -> Self {
    let mut flame_polys = vec![];
    let mut flame_routes = vec![];

    // FLAME
    let (flamex, flamey) = origin;
    let dr = (rad / 6.0).max(0.5);
    let approx = (rad / 10.0).max(0.2);
    let mut rt = spiral_optimized(flamex, flamey, rad, dr, approx);
    let ydisp = rng.gen_range(0.5..1.0);
    for p in &mut rt {
      p.0 += 5.0 * rad * rng.gen_range(-0.05..0.05);
      p.1 += 5.0
        * rad
        * ydisp
        * rng.gen_range(-1.0..0.05)
        * rng.gen_range(0.0..1.0)
        * rng.gen_range(0.0..1.0);
    }
    // rt = step_polyline(&rt, 0.3);

    flame_routes.push((flameclr, rt));
    flame_polys.push(circle_route((flamex, flamey), rad + 0.5, 16));

    Self {
      flame_polys,
      flame_routes,
      origin,
      size: 4.0 * rad,
    }
  }

  pub fn render(
    &self,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = vec![];
    let rts = regular_clip(&self.flame_routes, paint);
    for (_, rt) in &rts {
      ctx.effects.hot.paint_polyline(rt, self.size.min(4.0));
      paint.paint_polyline(rt, 0.4);
    }
    for poly in &self.flame_polys {
      paint.paint_polygon(poly);
    }
    routes.extend(rts);
    routes
  }
}

impl<R: Rng> Renderable<R> for Fire {
  fn render(
    &self,
    _rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(ctx, paint)
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }

  fn apply_translation_rotation(&mut self, v: (f32, f32), _rot: f32) {
    for (_, rt) in &mut self.flame_routes {
      for p in rt {
        p.0 += v.0;
        p.1 += v.1;
      }
    }
    for poly in &mut self.flame_polys {
      for p in poly {
        p.0 += v.0;
        p.1 += v.1;
      }
    }
  }
}
