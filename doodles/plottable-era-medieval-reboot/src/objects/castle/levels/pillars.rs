use super::{Floor, Level, LevelParams, RenderItem};
use crate::algo::math1d::mix;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Pillars {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
}

impl Pillars {
  pub fn max_allowed_width(scale: f32) -> f32 {
    10.0 * scale
  }
  pub fn init<R: Rng>(rng: &mut R, params: &LevelParams) -> Self {
    let mut items = vec![];
    let zorder = params.level_zorder;
    let clr = params.clr;
    let o = params.floor.pos;
    let splits = params.floor.splits.clone();
    let s = params.scaleref;
    let h = params.preferrable_height.max(6.0 * s);

    let wratio = 3.0;
    let pillarw = rng.gen_range(0.5..1.0) * s;
    let pad =
      pillarw + rng.gen_range(-0.1f32..0.2).max(0.0) * params.floor.width;
    let w = params.floor.width - pad * 2.0;
    let neww = mix(w, params.floor.width, rng.gen_range(0.0..1.0));
    let count = (w / (pillarw * wratio)).max(2.0) as usize;

    let y = o.1;
    let y2 = o.1 - h;

    let mut routes = vec![];
    let mut polygons = vec![];

    let dist = 0.35;
    let divs = (pillarw / dist).round().max(1.0) as usize;

    for i in 0..count {
      let f = i as f32 / (count - 1) as f32;
      let xc = o.0 + (f - 0.5) * w;
      let x1 = xc - pillarw / 2.;
      let x2 = xc + pillarw / 2.;
      for j in 0..divs {
        let f = if divs == 1 {
          0.5
        } else {
          j as f32 / (divs - 1) as f32
        };
        let x = mix(x1, x2, f);
        routes.push((clr, vec![(x, y), (x, y2)]));
      }
      polygons.push(vec![(x1, o.1), (x1, o.1 - h), (x2, o.1 - h), (x2, o.1)]);
    }

    // floor
    routes.push((
      clr,
      vec![
        (o.0 - params.floor.width / 2., y),
        (o.0 + params.floor.width / 2., y),
      ],
    ));

    // ceil
    routes.push((clr, vec![(o.0 - neww / 2., y2), (o.0 + neww / 2., y2)]));

    items.push(RenderItem::new(routes, polygons, zorder));

    let roof_base = Some(Floor::new((o.0, y2), neww, splits, false));

    Self { items, roof_base }
  }
}

impl Level for Pillars {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }
}
