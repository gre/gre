use super::super::{
  wallshadows::wall_shadow, walltexture::wall_texture, Floor, Level,
  LevelParams, RenderItem,
};
use crate::algo::{
  math1d::mix, paintmask::PaintMask, polylines::path_subdivide_to_curve,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct BridgesParams {
  pub fill_to_lowest_y_allowed: bool,
  pub pillar_w: f32,
  pub curve_stop: f32,
  pub pad_top: f32,
  pub curvy: f32,
  pub maxh: f32,
}

impl BridgesParams {
  pub fn new<R: Rng>(rng: &mut R) -> Self {
    let curve_stop = 0.5;
    let pad_top = 0.2;
    let pillar_w = rng.gen_range(0.1..0.2);
    let curvy = rng.gen_range(0.0..1.0);
    let maxh = rng.gen_range(8.0..24.0);
    Self {
      fill_to_lowest_y_allowed: false,
      pillar_w,
      curve_stop,
      pad_top,
      curvy,
      maxh,
    }
  }
}

pub struct Bridges {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
  pub forbidden_y: f32,
}

impl Bridges {
  pub fn max_allowed_width(_scale: f32) -> f32 {
    f32::INFINITY
  }
  pub fn init<R: Rng>(
    rng: &mut R,
    paintref: &PaintMask,
    params: &LevelParams,
    bridgeparams: &BridgesParams,
  ) -> Self {
    let mut items = vec![];
    let scale = params.scaleref;
    let clr = params.clr;
    let zorder = params.level_zorder;
    let w = params.floor.width;
    let h = params.preferrable_height.min(bridgeparams.maxh * scale);
    let o = params.floor.pos;

    let mut routes = vec![];
    let mut polygons = vec![];
    let x1 = o.0 - w / 2.;
    let x2 = o.0 + w / 2.;
    let y1 = if bridgeparams.fill_to_lowest_y_allowed {
      params.lowest_y_allowed
    } else {
      o.1.min(params.lowest_y_allowed)
    };
    let y2 = o.1 - h;
    let roof_base = Some(Floor::new((o.0, y2), w, vec![], false));

    // we make the bridges shape as a polygon that we will also draw. the top edge is not closed and it's the next level that will continues from it.

    let pad_top = bridgeparams.pad_top;
    let pillar_w = bridgeparams.pillar_w;
    let curve_stop = bridgeparams.curve_stop;
    let curvy = bridgeparams.curvy;

    let count = (w / (h * (1.0 - pad_top))).ceil().max(1.0) as usize;
    let countf = count as f32;
    let incr = 1.0 / countf;

    let mut poly = vec![];
    poly.push((x1, y2));
    poly.push((x1, y1));
    for i in 0..count {
      let r = i as f32 / countf;
      let xleft = mix(x1, x2, r);
      let xright = mix(x1, x2, r + incr);
      let diff = xright - xleft;
      let mut rt = vec![
        ((xleft + diff * pillar_w / 2.0, y1)),
        ((xleft + diff * pillar_w / 2.0, y1 - curve_stop * h)),
        ((xleft + 0.5 * diff, y2 + pad_top * h)),
        ((xright - diff * pillar_w / 2.0, y1 - curve_stop * h)),
        ((xright - diff * pillar_w / 2.0, y1)),
      ];
      if curvy > 0.0 {
        rt = path_subdivide_to_curve(&rt, 2, 1.0 - 0.4 * curvy);
      }
      poly.extend(rt.clone());

      if !params.floor.is_closed {
        let mut rt = rt.clone();
        rt.push(rt[0]);
        routes.push((clr, rt));

        routes.push((
          clr,
          vec![
            (xleft + diff * pillar_w / 2.0, y1),
            (xright - diff * pillar_w / 2.0, y1),
          ],
        ));
      }
    }
    poly.push((x2, y1));
    poly.push((x2, y2));

    if params.floor.is_closed {
      routes.push((clr, poly.clone()));
    } else {
      routes.push((clr, vec![(x1, y1), (x1, y2)]));
      routes.push((clr, vec![(x2, y1), (x2, y2)]));
    }

    polygons.push(poly.clone());

    let wall_textured = w * rng.gen_range(0.0..1.0) > 8.0;

    if wall_textured {
      routes.extend(wall_texture(
        rng,
        paintref,
        params.tower_seed,
        clr,
        &poly,
        scale,
      ));
    } else if w < 30.0 * scale {
      let light_x_direction = params.light_x_direction;
      routes.extend(wall_shadow(
        params.tower_seed,
        clr,
        &poly,
        light_x_direction,
        scale,
        0.33,
      ));
    }

    items.push(RenderItem::new(routes, polygons, zorder));

    let forbidden_y = o.1;
    Self {
      items,
      roof_base,
      forbidden_y,
    }
  }
}

impl Level for Bridges {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }

  fn condamn_build_belowy(&self) -> Option<f32> {
    Some(self.forbidden_y)
  }
}
