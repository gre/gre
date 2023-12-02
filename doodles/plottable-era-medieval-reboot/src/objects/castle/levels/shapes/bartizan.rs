use super::super::{Floor, Level, LevelParams, RenderItem};
use super::{
  roof::{Roof, RoofParams},
  wall::{Wall, WallParams},
};
use crate::{
  algo::{
    math1d::{mix, smoothstep},
    paintmask::PaintMask,
  },
  global::GlobalCtx,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Bartizan {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
}

impl Bartizan {
  pub fn max_allowed_width(scale: f32) -> f32 {
    21.0 * scale
  }
  pub fn init<R: Rng>(
    rng: &mut R,
    ctx: &GlobalCtx,
    paintref: &PaintMask,
    params: &LevelParams,
  ) -> Self {
    let wallratio = rng.gen_range(0.5..0.7);

    let roofparams =
      RoofParams::from_reference(rng, ctx, &params.reference_roof_params);

    let mut items = vec![];
    let zorder = params.level_zorder + 100.5; // in front of next level (many in advance to secure it)
    let o = params.floor.pos;
    let width = params.floor.width;
    let count = rng.gen_range(2..4);
    let w = rng.gen_range(0.6..0.8) * width / count as f32;
    let h = rng.gen_range(1.0..3.0) * w;
    for i in 0..count {
      let xf = i as f32 / (count - 1) as f32;
      let x = mix(o.0 - width / 2.0, o.0 + width / 2.0, xf);
      items.extend(make_bartizan(
        rng,
        paintref,
        &params,
        &roofparams,
        (x, o.1 + h / 2.0),
        w,
        h,
        zorder,
        wallratio,
        xf,
      ));
      // TODO spawn flag
    }

    // only renders as is, rest is unchanged
    let roof_base = Some(params.floor.clone());

    Self { items, roof_base }
  }
}

impl Level for Bartizan {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }
}

fn make_bartizan<R: Rng>(
  rng: &mut R,
  paintref: &PaintMask,
  baseparams: &LevelParams,
  roofparams: &RoofParams,
  o: (f32, f32),
  w: f32,
  h: f32,
  zorder: f32,
  wallratio: f32,
  xf: f32,
) -> Vec<RenderItem> {
  let mut items = vec![];

  let mut params = baseparams.clone();
  params.level = 0;
  params.scaleref *= 0.5;
  params.floor = Floor::new(o, w, vec![], false);
  params.max_height = h;
  params.preferrable_height = wallratio * h;
  params.level_zorder = zorder;

  let mut wallparams = WallParams::new();
  // TODO force the presence of windows. wallparams to include the rng of these
  let push = 0.5 * h;
  wallparams.push_right_down = push * smoothstep(0.5, 0.0, xf);
  wallparams.push_left_down = push * smoothstep(0.5, 1.0, xf);
  let wall = Wall::init(rng, paintref, &params, &wallparams);

  params.level += 1;
  params.level_zorder += 1.0;
  params.max_height = h;
  params.preferrable_height = (1.0 - wallratio) * h;
  if let Some(floor) = wall.roof_base() {
    params.floor = floor;
  }
  let roof = Roof::init(&params, &roofparams);

  items.extend(wall.render());
  items.extend(roof.render());

  items
}
