use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, shapes::yarnballs,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Ball {
  pub origin: (f32, f32),
  pub r: f32,
}

impl Ball {
  pub fn init<R: Rng>(rng: &mut R, origin: (f32, f32), r: f32) -> Self {
    Self { origin, r }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    clr: usize,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let origin = self.origin;
    let r = self.r;
    let mut routes = vec![];
    let density = 2.0;
    // TODO do the random spiral instead?
    routes.push((clr, yarnballs(rng, origin, r, density)));
    //routes.push((0, yarnballs(rng, origin, r, density * 0.8)));
    //routes.push((2, yarnballs(rng, origin, r, density * 0.2)));
    routes = regular_clip(&routes, paint);
    paint.paint_circle(origin.0, origin.1, r);
    routes
  }
}
