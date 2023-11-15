use crate::algo::paintmask::PaintMask;

use super::traits::Blazon;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct FlagPattern {}

impl FlagPattern {
  fn render<R: rand::Rng>(
    &self,
    house: Blazon,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];

    routes
  }
}
