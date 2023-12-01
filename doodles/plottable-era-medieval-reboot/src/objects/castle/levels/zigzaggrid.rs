use super::{Floor, Level, LevelParams, RenderItem};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct ZigZagGrid {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
  forbiddeny: f32,
}

impl ZigZagGrid {
  pub fn max_allowed_width(scale: f32) -> f32 {
    20.0 * scale
  }
  pub fn init<R: Rng>(rng: &mut R, params: &LevelParams) -> Self {
    let mut items = vec![];
    let zorder = params.level_zorder + 100.5; // in front of next level (many in advance to secure it)
    let w = params.floor.width;
    let scale = params.scaleref;
    let h = scale * rng.gen_range(0.8..1.6);
    let o = params.floor.pos;
    let clr = params.clr;

    let mut routes = vec![];
    let polygons = vec![];

    let mut route = vec![];

    let xincr = rng.gen_range(0.5..1.0) * h;
    let divs = (w / xincr) as usize;
    let xincr = w / divs as f32;
    for xi in 0..(divs + 1) {
      let x = o.0 - w / 2. + xi as f32 * xincr;
      route.push((x, o.1 - if xi % 2 == 0 { 0.0 } else { h }));
    }
    routes.push((clr, route));

    routes.push((
      clr,
      vec![
        (o.0 - w / 2., o.1 - h),
        (o.0 - w / 2., o.1),
        (o.0 + w / 2., o.1),
        (o.0 + w / 2., o.1 - h),
        (o.0 - w / 2., o.1 - h),
      ],
    ));

    items.push(RenderItem::new(routes, polygons, zorder));

    // ZigZagGrid only renders as is, rest is unchanged
    let roof_base = Some(params.floor.clone());

    let forbiddeny = o.1;

    Self {
      items,
      roof_base,
      forbiddeny,
    }
  }
}

impl Level for ZigZagGrid {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }

  fn condamn_build_belowy(&self) -> Option<f32> {
    Some(self.forbiddeny)
  }
}
