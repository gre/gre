use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{route_translate_rotate, Polylines},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Cage {
  pub routes: Polylines,
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

    let dx = rng.gen_range(0.15..0.2);
    let dy = rng.gen_range(0.2..0.4);

    let mut x = x1 - rng.gen_range(0.0..dx) * 0.5;
    while x <= x2 {
      routes.push((
        clr,
        route_translate_rotate(&vec![(x, y1), (x, y2)], origin, -angle),
      ));
      x += rng.gen_range(0.1..0.15) * size;
    }

    let mut y = y1 + rng.gen_range(0.0..dy) * 0.5;
    while y >= y2 {
      routes.push((
        clr,
        route_translate_rotate(&vec![(x1, y), (x2, y)], origin, -angle),
      ));
      y -= rng.gen_range(0.1..0.15) * size;
    }

    Self { routes }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for (_, r) in &self.routes {
      paint.paint_polyline(r, 0.5);
    }
    routes
  }
}
