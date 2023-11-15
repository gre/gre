use crate::algo::paintmask::PaintMask;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub enum House {
  Lys,
}

pub trait ArmyHouse {
  fn render_flag_pattern<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f64, f64)>)>;
}
