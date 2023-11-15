pub mod bandpattern;
pub mod framing;

use crate::algo::paintmask::PaintMask;
use bandpattern::*;
use framing::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn medieval_frame<R: Rng>(
  rng: &mut R,
  mask: &mut PaintMask,
  width: f32,
  height: f32,
  pad: f32,
  innerp: f32,
  clr: usize,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = vec![];

  let p = innerp;
  let m = pad;
  let (pattern, strokew): (Box<dyn BandPattern>, f32) =
    match rng.gen_range(0..5) {
      0 => (Box::new(lrect::MedievalBandLRectPattern::new()), 0.08 * p),
      1 => (
        Box::new(feather::MedievalBandFeatherTrianglePattern::new()),
        0.06 * p,
      ),
      2 => (Box::new(fork::MedievalBandForkPattern::new()), 0.06 * p),
      3 => (Box::new(comb::MedievalBandComb::new()), 0.04 * p),
      4 => (Box::new(curve::MedievalBandCurvePattern::new()), 0.04 * p),
      _ => (
        Box::new(concentric::MedievalBandConcentric::new(2)),
        0.08 * p,
      ),
    };
  routes.extend(framing(
    rng,
    mask,
    clr,
    (pad, pad, width - pad, height - pad),
    pattern.as_ref(),
    p,
    m,
    strokew,
    3.0,
    6000,
  ));

  routes
}
