use crate::algo::{
  clipping::regular_clip_polys,
  paintmask::PaintMask,
  polylines::{grow_as_rectangle, grow_stroke_zigzag, route_translate_rotate},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub struct TODO {
  pub origin: (f32, f32),
  pub size: f32,
  pub angle: f32,
  pub clr: usize,
}

impl TODO {
  pub fn init(origin: (f32, f32), size: f32, angle: f32, clr: usize) -> Self {
    Self {
      origin,
      size,
      angle,
      clr,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let size = self.size;
    let origin = self.origin;
    let angle = self.angle;
    let clr = self.clr;
    let mut routes: Vec<Vec<(f32, f32)>> = vec![];

    routes
  }
}
