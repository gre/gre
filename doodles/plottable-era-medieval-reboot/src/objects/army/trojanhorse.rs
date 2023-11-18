use super::{horse::Horse, wheeledplatform::WheeledPlatform};
use crate::algo::paintmask::PaintMask;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn trojanhorse<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  (x, y): (f32, f32),
  size: f32,
  xflip: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = vec![];

  let h = 0.1 * size;
  let w = size;
  let wheel_pad = rng.gen_range(-0.1f32..0.1).max(0.0) * size;
  let wheel_count = rng.gen_range(2..8);
  let o = (x, y);
  let platform = WheeledPlatform::init(o, h, w, 0.0, wheel_pad, wheel_count);

  let o = (x, y - 0.2 * size);
  let horse = Horse::init(o, size, 0.0, xflip, clr, clr, 1.5, 0.0);

  routes.extend(platform.render(paint, clr));
  routes.extend(horse.render(rng, paint));

  // halo
  for (_, route) in &routes {
    paint.paint_polyline(route, 1.0);
  }

  routes
}
