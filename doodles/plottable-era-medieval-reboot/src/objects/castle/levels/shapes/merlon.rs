use super::super::{Floor, Level, LevelParams, RenderItem, SpawnableHuman};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Merlon {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
  forbiddeny: f32,
  spawnable_humans: Vec<SpawnableHuman>,
}

// TODO merlon should draw under also to create a nice rendering of the machicoulis

impl Merlon {
  pub fn max_allowed_width(_scale: f32) -> f32 {
    f32::INFINITY
  }
  pub fn init<R: Rng>(rng: &mut R, params: &LevelParams) -> Self {
    let mut items = vec![];
    let zorder = params.level_zorder + 100.5; // in front of next level (many in advance to secure it)
    let w = params.floor.width;
    let scale = params.scaleref;
    let h = scale * rng.gen_range(1.0..2.0);
    let o = params.floor.pos;
    let clr = params.clr;

    let mut routes = vec![];
    let mut polygons = vec![];

    let mut route = vec![];
    build_merlon(
      &mut polygons,
      &mut route,
      o.0 - w / 2.,
      o.1 - h,
      o.0 + w / 2.,
      o.1 - h,
      h,
    );
    routes.push((clr, route));

    items.push(RenderItem::new(routes, polygons, zorder));

    // merlon only renders as is, rest is unchanged
    let roof_base = Some(params.floor.clone());

    let forbiddeny = o.1;

    let mut spawnable_humans = vec![];
    let pad = rng.gen_range(1.0..2.0) * scale;
    let mut x = pad;
    while x < w - pad {
      spawnable_humans
        .push(SpawnableHuman::new((o.0 - w / 2. + x, o.1), zorder - 0.1));
      x += rng.gen_range(2.0..4.0) * scale;
    }

    Self {
      items,
      roof_base,
      forbiddeny,
      spawnable_humans,
    }
  }
}

impl Level for Merlon {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }

  fn possible_background_human_positions(&self) -> Vec<SpawnableHuman> {
    self.spawnable_humans.clone()
  }

  fn condamn_build_belowy(&self) -> Option<f32> {
    Some(self.forbiddeny)
  }
}

fn build_merlon(
  polys: &mut Vec<Vec<(f32, f32)>>,
  route: &mut Vec<(f32, f32)>,
  leftx: f32,
  lefty: f32,
  rightx: f32,
  _righty: f32,
  h: f32,
) {
  let mut count = ((rightx - leftx) / h).ceil();
  count = (count / 2.0).floor() * 2.0 + 1.0;
  if count <= 0.0 {
    return;
  }
  let w = (rightx - leftx) / count;
  let mut x = leftx;
  let mut alt = false;
  route.push((x, lefty + h));
  route.push((x, lefty));
  loop {
    if x > rightx - w / 2.0 {
      break;
    }
    let y = lefty;
    x += w;
    if alt {
      route.push((x, y + h));
      route.push((x, y));
    } else {
      if route.len() > 1 {
        let last = route[route.len() - 1];
        let minx = last.0;
        let miny = last.1;
        let maxx = x;
        let maxy = y + h;
        polys.push(vec![
          (minx, miny),
          (maxx, miny),
          (maxx, maxy),
          (minx, maxy),
        ]);
      }
      route.push((x, y));
      route.push((x, y + h));
    }
    alt = !alt;
  }
}
