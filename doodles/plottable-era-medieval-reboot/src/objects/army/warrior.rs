use crate::algo::{clipping::regular_clip, paintmask::PaintMask};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn warrior<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  clr: usize,
  origin: (f32, f32),
  angle: f32,
  size: f32, // reference size (height of the boat)
  xflip: bool,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = vec![];

  // placeholder

  let w = size * 0.2;
  routes.push((
    clr,
    vec![
      (origin.0 - w, origin.1),
      (origin.0 + w, origin.1),
      (origin.0 + w, origin.1 - size),
      (origin.0 - w, origin.1 - size),
      (origin.0 - w, origin.1),
    ],
  ));

  routes = regular_clip(&routes, paint);

  paint.paint_rectangle(origin.0 - w, origin.1 - size, origin.0 + w, origin.1);

  routes
}
