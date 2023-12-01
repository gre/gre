use self::levels::builder::{build_castle, GlobalCastleProperties};
use super::mountains::CastleGrounding;
use crate::{algo::paintmask::PaintMask, global::GlobalCtx};
use rand::prelude::*;

pub mod chinesedoor;
pub mod chineseroof;
pub mod levels;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Castle {
  pub castleprops: GlobalCastleProperties,
}

impl Castle {
  pub fn init<R: Rng>(
    ctx: &mut GlobalCtx,
    rng: &mut R,
    castle: &CastleGrounding,
    ybase: f32,
    ymax: f32,
    extra_towers: usize,
  ) -> Self {
    let castleprops = GlobalCastleProperties::rand(
      rng,
      ctx,
      &castle,
      ybase,
      ymax,
      extra_towers,
    );
    Self { castleprops }
  }

  pub fn render<R: Rng>(
    &self,
    ctx: &mut GlobalCtx,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    build_castle(rng, ctx, paint, &self.castleprops)
  }
}
