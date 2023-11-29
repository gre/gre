use super::{
  poles::PoleKind, Floor, Level, LevelParams, RenderItem, SpawnablePole,
};
use crate::{
  algo::{math1d::mix, math2d::lerp_point, polylines::path_subdivide_to_curve},
  global::GlobalCtx,
};
use rand::prelude::*;

pub struct RoofParams {
  // force color
  pub clr: Option<usize>,
  // how much it grows in width
  pub groww: f32,
  // how much it curves (negative or positive. zero is no curves)
  pub curvyfactor: f32,
  // push the roof down effect on edge
  pub pushdown: f32,
  // if Some, add a pole at the top of the roof
  pub pole_kind: Option<PoleKind>,
}

// TODO in Chinese mode, can we try to make the roof curved accordingly?

// TODO gargoyles?

impl RoofParams {
  pub fn rand<R: Rng>(rng: &mut R, ctx: &GlobalCtx) -> Self {
    Self {
      clr: if rng.gen_bool(0.02) {
        ctx.get_golden_color()
      } else {
        None
      },
      groww: rng.gen_range(-1.0f32..1.0).max(0.0),
      curvyfactor: rng.gen_range(0.0..1.0) * rng.gen_range(-0.5..0.5),
      pushdown: rng.gen_range(0.0..1.0) * rng.gen_range(-0.1..0.1),
      pole_kind: if rng.gen_bool(0.8) {
        Some(PoleKind::rand(rng))
      } else {
        None
      },
    }
  }
}

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Roof {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
  pole_positions: Vec<SpawnablePole>,
}

impl Roof {
  pub fn max_allowed_width(scale: f32) -> f32 {
    10.0 * scale
  }

  // TODO the shape of roof should be shared, so i think we need to move the rng into another param
  // and try to remove the Rng out of the renderer.
  pub fn init(params: &LevelParams, roofparams: &RoofParams) -> Self {
    let mut items = vec![];
    let zorder = params.level_zorder;
    let clr = roofparams.clr.unwrap_or(params.clr);
    let o = params.floor.pos;
    let s = params.scaleref;
    let groww = roofparams.groww;
    let w = params.floor.width + groww * 3.0 * s;
    let toph = 0.0; // flag etc..
    let h = (params.preferrable_height - toph)
      .max(3.0 * s)
      .min(10.0 * s);

    let mut routes = vec![];
    let mut polygons = vec![];
    let hw = w / 2.;
    let hw2 = 0.3;
    let y1 = o.1;
    let y2 = o.1 - h;
    let roof_base = None;
    let p1 = (o.0 - hw, y1);
    let p2 = (o.0 + hw, y1);
    let p3 = (o.0 + hw2, y2);
    let p4 = (o.0 - hw2, y2);
    //routes.push((clr, poly.clone()));
    let curvyfactor = roofparams.curvyfactor * w;
    let mut pushdown = roofparams.pushdown * w;
    if pushdown < 0.0 && groww < 0.5 {
      // protect a glitchy case
      pushdown = 0.0;
    }
    let mut bottom = vec![];
    let mut poly = vec![];
    let count = 2 + (hw / s) as usize;
    let d = -params.light_x_direction;
    let pow = if d > 0.0 {
      mix(1.0, 2.0, d.min(1.0))
    } else {
      mix(1.0, 0.5, -d.max(-1.0))
    };
    for i in 0..count {
      let f = (i as f32 / (count - 1) as f32).powf(pow);
      let dx = curvyfactor * (f - 0.5);
      let mut dy = pushdown * (2.0 * (f - 0.5).abs()).powf(2.0);
      if pushdown < 0.0 {
        dy -= pushdown;
      }
      let a: (f32, f32) = lerp_point(p1, p2, f);
      let b = lerp_point(p4, p3, f);
      let m = lerp_point(a, b, 0.5);
      let m = (m.0 + 0.8 * dx, m.1);
      let a = (a.0 - 0.4 * dx, (a.1 + dy).min(params.lowest_y_allowed));
      let path = vec![a, m, b];
      let path = path_subdivide_to_curve(&path, 2, 0.7);
      if i == 0 {
        let mut p = path.clone();
        p.reverse();
        poly.extend(p);
      } else if i == count - 1 {
        poly.extend(bottom.clone());
        poly.extend(path.clone());
      }
      routes.push((clr, path));
      bottom.push(a);
    }
    routes.push((clr, bottom.clone()));
    polygons.push(poly);
    items.push(RenderItem::new(routes, polygons, zorder));

    let mut pole_positions = vec![];
    if let Some(kind) = &roofparams.pole_kind {
      pole_positions.push(SpawnablePole {
        pos: (o.0, y2),
        zorder: zorder - 0.1,
        size: h / 5.0,
        kind: kind.clone(),
      });
    }

    Self {
      items,
      roof_base,
      pole_positions,
    }
  }
}

impl Level for Roof {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }

  fn possible_pole_positions(&self) -> Vec<super::SpawnablePole> {
    self.pole_positions.clone()
  }
}
