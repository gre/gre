use std::f32::consts::PI;

use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, polylines::Polylines,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Star {
  pub poly: Vec<(f32, f32)>,
  pub clr: usize,
}

impl Star {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    radius: f32,
    count: usize,
  ) -> Self {
    let mut poly = vec![];
    for i in 0..count {
      let a = (i as f32
        + rng.gen_range(-0.5..0.5)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0))
        / count as f32
        * 2.0
        * PI;
      let r2 = rng.gen_range(0.2..0.5) * radius;
      let r = if i % 2 == 0 { radius } else { r2 };
      poly.push((origin.0 + a.cos() * r, origin.1 + a.sin() * r));
    }
    poly.push(poly[0]);
    Self { poly, clr }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let poly = self.poly.clone();
    let clr = self.clr;
    let routes = vec![(clr, poly.clone())];
    let routes = regular_clip(&routes, paint);
    paint.paint_polygon(&poly);
    routes
  }
}
