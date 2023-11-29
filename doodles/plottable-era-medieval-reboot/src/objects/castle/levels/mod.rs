pub mod bartizan;
pub mod bell;
pub mod builder;
pub mod merlon;
pub mod pillars;
pub mod poles;
pub mod roof;
pub mod wall;
pub mod wallshadows;
pub mod walltexture;
pub mod walltransition;
pub mod windows;
pub mod zigzaggrid;

use crate::algo::{
  clipping::regular_clip_polys, paintmask::PaintMask, polylines::Polylines,
};
use std::cmp::Ordering;

use self::poles::SpawnablePole;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

// it's like a Renderable item, but we need to work with polygon for the destruction logic.
#[derive(Clone)]
pub struct RenderItem {
  pub routes: Polylines,
  pub polygons: Vec<Vec<(f32, f32)>>,
  pub zorder: f32,
  // hack in order to return other things that aren't fitting in here
  pub foreign_id: Option<usize>,
}

impl PartialEq for RenderItem {
  fn eq(&self, other: &Self) -> bool {
    self.zorder == other.zorder
  }
}

impl Eq for RenderItem {}

impl PartialOrd for RenderItem {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.zorder.partial_cmp(&self.zorder)
  }
}

impl Ord for RenderItem {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap_or(Ordering::Equal)
  }
}

impl RenderItem {
  pub fn new(
    routes: Polylines,
    polygons: Vec<Vec<(f32, f32)>>,
    zorder: f32,
  ) -> Self {
    Self {
      routes,
      polygons,
      zorder,
      foreign_id: None,
    }
  }

  pub fn from_foreign(foreign_id: usize, zorder: f32) -> Self {
    Self {
      routes: vec![],
      polygons: vec![],
      zorder,
      foreign_id: Some(foreign_id),
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let rts = regular_clip_polys(&self.routes, paint, &self.polygons);
    rts
  }
}

#[derive(Clone)]
pub struct LevelParams {
  // seed global for the tower.
  pub tower_seed: u32,
  // index of the level
  pub level: usize,
  // a reference scale that is shared on the castle
  pub scaleref: f32,
  // color for the castle team
  pub blazonclr: usize,
  // general color
  pub clr: usize,
  // location of the level
  pub floor: Floor,
  // the max height the level is allowed to do. it's only an indication, some level won't reach that height.
  pub max_height: f32,
  // a convenient height to plan
  pub preferrable_height: f32,
  // the zorder for the level under the new one to build. chose a value direction!
  pub level_zorder: f32,
  // below this y, it's forbidden to put some shapes
  pub lowest_y_allowed: f32,
  // lightness direction. -1.0 is left, 1.0 is right. 0.0 would lower the effect.
  pub light_x_direction: f32,
}

#[derive(Clone)]
pub struct Floor {
  // center of the level
  pub pos: (f32, f32),
  // width of the level from that center
  pub width: f32,
  // possible splits in the level to take into account. in % of x
  pub splits: Vec<f32>,
  // is the level under closed the roof?
  pub is_closed: bool,
}
impl Floor {
  pub fn new(
    pos: (f32, f32),
    width: f32,
    splits: Vec<f32>,
    is_closed: bool,
  ) -> Self {
    Self {
      pos,
      width,
      splits,
      is_closed,
    }
  }
}

#[derive(Clone)]
pub struct SpawnableHuman {
  pub pos: (f32, f32),
  pub zorder: f32,
}
impl SpawnableHuman {
  pub fn new(pos: (f32, f32), zorder: f32) -> Self {
    Self { pos, zorder }
  }
}

pub trait Level {
  // tells if a level can be built on top of the current one
  // if so, it will return what should be considered as base for the next level
  // note that it's possible for shapes to already overlap above that base and clipping is required
  fn roof_base(&self) -> Option<Floor>;

  // all the renderables to use for this level
  // we will use conventional zordering that aren't compatible with the other objects of this project
  // the levels will be solved together for each tower.
  fn render(&self) -> Vec<RenderItem>;

  // iterate where we can spawn humans. they would be just behind the level in term of zindex.
  fn possible_background_human_positions(&self) -> Vec<SpawnableHuman> {
    vec![]
  }

  // iter where the poles are and where we can spawn
  fn possible_pole_positions(&self) -> Vec<SpawnablePole> {
    vec![]
  }

  fn condamn_build_belowy(&self) -> Option<f32> {
    None
  }
}
