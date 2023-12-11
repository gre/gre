use super::{Floor, Level, LevelParams, RenderItem};
use crate::{algo::paintmask::PaintMask, global::GlobalCtx};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct NAME {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
}

impl NAME {
  pub fn init<R: Rng>(rng: &mut R, params: &LevelParams) -> Self {
    let mut items = vec![];
    let roof_base = None;
    Self { items, roof_base }
  }
}

impl Level for NAME {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }
}
