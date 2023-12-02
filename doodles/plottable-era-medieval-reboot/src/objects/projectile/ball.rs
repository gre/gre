use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  shapes::{circle_route, spiral_optimized},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct Ball {
  pub origin: (f32, f32),
  pub r: f32,
  pub clr: usize,
}

impl Ball {
  pub fn init<R: Rng>(
    _rng: &mut R,
    origin: (f32, f32),
    r: f32,
    clr: usize,
  ) -> Self {
    Self { origin, r, clr }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let clr = self.clr;
    let origin = self.origin;
    let r = self.r;
    let mut routes = vec![];
    let density = 2.0;
    let dr = rng.gen_range(0.4..0.7);

    routes.push((clr, circle_route(origin, r, (r * density) as usize + 10)));
    routes.push((clr, spiral_optimized(origin.0, origin.1, r, dr, 0.1)));

    // routes.push((clr, yarnballs(rng, origin, r, density)));
    //routes.push((0, yarnballs(rng, origin, r, density * 0.8)));
    //routes.push((2, yarnballs(rng, origin, r, density * 0.2)));
    routes = regular_clip(&routes, paint);
    paint.paint_circle(origin.0, origin.1, r);
    routes
  }
}
