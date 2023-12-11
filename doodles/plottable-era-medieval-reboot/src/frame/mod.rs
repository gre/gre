pub mod bandpattern;
pub mod framing;

use crate::{algo::paintmask::PaintMask, global::GlobalCtx};
use bandpattern::*;
use framing::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn medieval_frame<R: Rng>(
  rng: &mut R,
  ctx: &GlobalCtx,
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
  let wmul = rng.gen_range(0.9..1.2);
  let v: Option<(Box<dyn BandPattern>, f32)> =
    match rng.gen_range(0.0..6.5) as usize {
      0 => Some((
        Box::new(lrect::MedievalBandLRectPattern::new()),
        rng.gen_range(0.06..0.1) * p,
      )),
      1 => Some((
        Box::new(feather::MedievalBandFeatherTrianglePattern::new()),
        rng.gen_range(0.05..0.07) * p,
      )),
      2 => Some((
        Box::new(fork::MedievalBandForkPattern::new()),
        rng.gen_range(0.05..0.07) * p,
      )),
      3 => Some((
        Box::new(comb::MedievalBandComb::new()),
        rng.gen_range(0.03..0.06) * p,
      )),
      4 => Some((
        Box::new(curve::MedievalBandCurvePattern::new()),
        rng.gen_range(0.03..0.06) * p,
      )),
      5 => Some((
        Box::new(concentric::MedievalBandConcentric::new(rng.gen_range(1..4))),
        rng.gen_range(0.05..0.2) * p,
      )),
      _ => None,
    };
  let iterations = 6000;
  routes.extend(framing(
    rng,
    ctx,
    mask,
    clr,
    (pad, pad, width - pad, height - pad),
    v,
    p,
    m,
    wmul,
    3.0,
    iterations,
  ));

  // sort routes by angle with center
  let c = (width / 2., height / 2.);
  let mut routes = routes
    .iter()
    .map(|data| (data, (data.1[0].0 - c.0).atan2(data.1[0].1 - c.1)))
    .collect::<Vec<_>>();
  routes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

  let routes = routes.iter().map(|data| data.0.clone()).collect();

  routes
}
