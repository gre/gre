use super::{
  wallshadows::wall_shadow,
  walltexture::wall_texture,
  windows::{wall_windows, WallWindowParams},
  Floor, Level, LevelParams, RenderItem,
};
use crate::algo::{math2d::lerp_point, paintmask::PaintMask};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct WallParams {
  pub fill_to_lowest_y_allowed: bool,
  pub push_left_down: f32,
  pub push_right_down: f32,
  // TODO stairs with a door entrance in castle
  // TODO door
}

impl WallParams {
  pub fn new() -> Self {
    Self {
      fill_to_lowest_y_allowed: false,
      push_left_down: 0.0,
      push_right_down: 0.0,
    }
  }
}

pub struct Wall {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
}

impl Wall {
  pub fn max_allowed_width(_scale: f32) -> f32 {
    f32::INFINITY
  }
  pub fn init<R: Rng>(
    rng: &mut R,
    paintref: &PaintMask,
    params: &LevelParams,
    wallparams: &WallParams,
  ) -> Self {
    let mut items = vec![];
    let zorder = params.level_zorder;
    let w = params.floor.width;
    let h = params.preferrable_height;
    let o = params.floor.pos;
    let scale = params.scaleref;
    let splits = params.floor.splits.clone();
    let should_draw_base = !params.floor.is_closed || rng.gen_bool(0.3);

    let mut routes = vec![];
    let mut polygons = vec![];
    let x1 = o.0 - w / 2.;
    let x2 = o.0 + w / 2.;
    let y1 = if wallparams.fill_to_lowest_y_allowed {
      params.lowest_y_allowed
    } else {
      o.1
    };
    let y2 = o.1 - h;
    let roof_base = Some(Floor::new((o.0, y2), w, splits, false));
    let p0 = (x1, y2);
    let p1 = (x1, y1 + wallparams.push_left_down);
    let p2 = (x2, y1 + wallparams.push_right_down);
    let p3: (f32, f32) = (x2, y2);
    if should_draw_base {
      routes.push((params.clr, vec![p0, p1, p2, p3]));
    } else {
      routes.push((params.clr, vec![p0, p1]));
      routes.push((params.clr, vec![p2, p3]));
    }

    let mut ranges = vec![];
    let mut areas = vec![];
    let mut preva = p0;
    let mut prevb = p1;
    let mut prevsplit = 0.0;
    for &split in params.floor.splits.iter() {
      let a = lerp_point(p0, p3, split);
      let b = lerp_point(p1, p2, split);
      areas.push(vec![preva, prevb, b, a]);
      ranges.push(prevsplit..split);
      preva = a;
      prevb = b;
      prevsplit = split;
      routes.push((params.clr, vec![a, b]));
    }
    ranges.push(prevsplit..1.0);
    areas.push(vec![preva, prevb, p2, p3]);

    let poly = vec![p0, p1, p2, p3];
    polygons.push(poly.clone());

    let index_with_shadows = if params.light_x_direction < 0.0 {
      0
    } else {
      areas.len() - 1
    };

    let index_with_windows = if areas.len() == 1 {
      0
    } else {
      let possibles = (0..areas.len())
        .filter(|&i| i != index_with_shadows)
        .collect::<Vec<_>>();
      let p = rng.gen_range(0..possibles.len());
      possibles[p]
    };

    let wall_textured = w * rng.gen_range(0.0..1.0) > 8.0;

    if let Some(poly) = areas.get(index_with_windows) {
      let range = ranges[index_with_windows].clone();
      let ratio = range.end - range.start;
      let windowparams = WallWindowParams::init(rng, scale, ratio * w);
      items.extend(wall_windows(
        &windowparams,
        params.clr,
        zorder + 0.1,
        poly,
        ratio,
        wall_textured,
      ));
    }

    if wall_textured {
      routes.extend(wall_texture(
        rng,
        paintref,
        params.tower_seed,
        params.clr,
        &poly,
        scale,
      ));
    } else if w < 30.0 * scale {
      if let Some(poly) = areas.get(index_with_shadows) {
        let light_x_direction = params.light_x_direction;
        routes.extend(wall_shadow(
          params.tower_seed,
          params.clr,
          &poly,
          light_x_direction,
          scale,
          0.33,
        ));
      }
    }

    items.push(RenderItem::new(routes, polygons, zorder));

    Self { items, roof_base }
  }
}

impl Level for Wall {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }
}
