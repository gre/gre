use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  paintmask::PaintMask,
  polylines::{route_translate_rotate, Polylines},
  renderable::Renderable,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Cage {
  pub routes: Polylines,
  pub origin: (f32, f32),
  pub size: f32,
}

impl Cage {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
  ) -> Self {
    let mut routes = vec![];

    let x1 = -0.5 * size;
    let x2 = 0.5 * size;
    let y1 = 0.0;
    let y2 = -size;

    let xsplits =
      2 + (size / (rng.gen_range(0.1..0.2) * size).max(2.0)) as usize;
    for xi in 0..xsplits {
      let x = mix(x1, x2, xi as f32 / (xsplits - 1) as f32);
      routes.push((
        clr,
        route_translate_rotate(&vec![(x, y1), (x, y2)], origin, -angle),
      ));
    }

    let mut y = y2;
    while y <= y1 {
      routes.push((
        clr,
        route_translate_rotate(&vec![(x1, y), (x2, y)], origin, -angle),
      ));
      y += (rng.gen_range(0.1..0.2) * size).max(2.0);
    }

    Self {
      routes,
      origin,
      size,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    if self.size > 15.0 {
      for (_, r) in &self.routes {
        paint.paint_polyline(r, 0.3);
      }
    }
    routes
  }
}

impl<R: Rng> Renderable<R> for Cage {
  fn render(
    &self,
    _rng: &mut R,
    _ctx: &mut crate::global::GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(paint)
  }

  fn zorder(&self) -> f32 {
    self.origin.1 + 0.2 * self.size
  }
}
