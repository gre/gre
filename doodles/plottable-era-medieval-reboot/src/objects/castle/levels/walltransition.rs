use super::{wallshadows::wall_shadow, Floor, Level, LevelParams, RenderItem};
use crate::algo::math2d::lerp_point;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct WallTransition {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
}

impl WallTransition {
  pub fn max_allowed_width(scale: f32) -> f32 {
    21.0 * scale
  }
  pub fn init<R: Rng>(
    rng: &mut R,
    params: &LevelParams,
    is_transition_to_roof: bool,
  ) -> Self {
    let mut items = vec![];
    let zorder = params.level_zorder;
    let clr = params.clr;
    let w = params.floor.width;
    let o = params.floor.pos;
    let splits = params.floor.splits.clone();
    let s = params.scaleref;
    let should_draw_shadows = !is_transition_to_roof && rng.gen_bool(0.3);
    let should_draw_base = should_draw_shadows || rng.gen_bool(0.2);
    let reducing = rng.gen_bool(0.7);
    let h = params.preferrable_height.max(1.2 * s).min(
      if is_transition_to_roof || !reducing || rng.gen_bool(0.5) {
        3.0
      } else {
        14.0
      } * s,
    );
    let grow = if is_transition_to_roof {
      0.0
    } else {
      h.min(3. * s) * rng.gen_range(0.8..1.2) * if reducing { -2. } else { 2. }
    };

    let mut routes = vec![];
    let mut polygons = vec![];
    let newroofw = (w + grow).max(0.0);
    let hw = w / 2.;
    let hw2 = newroofw / 2.;
    let y1 = o.1;
    let y2 = o.1 - h;
    let roof_base = if newroofw > 0.0 {
      Some(Floor::new((o.0, y2), w + grow, splits, false))
    } else {
      None
    };
    let p1 = (o.0 - hw, y1);
    let p2 = (o.0 + hw, y1);
    let p3 = (o.0 + hw2, y2);
    let p4 = (o.0 - hw2, y2);
    let poly = vec![p4, p1, p2, p3];
    if should_draw_base {
      routes.push((clr, poly.clone()));
    } else {
      routes.push((clr, vec![p4, p1]));
      routes.push((clr, vec![p2, p3]));
    }
    polygons.push(poly);

    if should_draw_shadows {
      let count = newroofw as usize;
      for i in 0..count {
        let f = (i + 1) as f32 / (count + 1) as f32;
        let a = lerp_point(p1, p2, f);
        let b = lerp_point(p4, p3, f);
        routes.push((clr, vec![a, b]));
      }
    } else {
      let mut areas = vec![];
      let mut preva = p1;
      let mut prevb = p4;
      for &split in params.floor.splits.iter() {
        let a = lerp_point(p1, p2, split);
        let b = lerp_point(p4, p3, split);
        areas.push(vec![preva, prevb, b, a]);
        preva = a;
        prevb = b;
        routes.push((clr, vec![a, b]));
      }
      areas.push(vec![preva, prevb, p3, p2]);

      let i = if params.light_x_direction < 0.0 {
        0
      } else {
        areas.len() - 1
      };
      if let Some(poly) = areas.get(i) {
        let light_x_direction = params.light_x_direction;
        routes.extend(wall_shadow(
          params.tower_seed,
          params.clr,
          &poly,
          light_x_direction,
          s,
          0.33,
        ));
      }
    }

    items.push(RenderItem::new(routes, polygons, zorder));

    Self { items, roof_base }
  }
}

impl Level for WallTransition {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }
}
