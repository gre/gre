use super::super::{
  wallshadows::wall_shadow,
  walltexture::wall_texture,
  windows::{wall_windows, WallWindowParams},
  Floor, Level, LevelParams, RenderItem,
};
use crate::{
  algo::{
    clipping::clip_routes_with_colors,
    math1d::mix,
    math2d::lerp_point,
    paintmask::PaintMask,
    polygon::polygon_includes_point,
    polylines::{
      grow_as_rectangle, grow_stroke_zigzag, route_translate_rotate,
    },
    shapes::{arc, circle_route, spiral_optimized_with_initial_angle},
  },
  objects::{castle::levels::SpawnableFire, mountains::Moat},
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct WallParams {
  pub with_door: Option<(f32, f32)>,
  pub moats: Vec<Moat>,
  pub fill_to_lowest_y_allowed: bool,
  pub push_left_down: f32,
  pub push_right_down: f32,
  // TODO stairs with a door entrance in castle
}

impl WallParams {
  pub fn new() -> Self {
    Self {
      fill_to_lowest_y_allowed: false,
      push_left_down: 0.0,
      push_right_down: 0.0,
      with_door: None,
      moats: vec![],
    }
  }
}

pub struct Wall {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
  possible_ladder_positions: Vec<(f32, f32)>,
  fire_start_positions: Vec<SpawnableFire>,
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
    let clr = params.clr;
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
      o.1.min(params.lowest_y_allowed)
    };
    let y2 = o.1 - h;
    let roof_base = Some(Floor::new((o.0, y2), w, splits, false));
    let p0 = (x1, y2);
    let p1 = (x1, y1 + wallparams.push_left_down);
    let p2 = (x2, y1 + wallparams.push_right_down);
    let p3: (f32, f32) = (x2, y2);
    if should_draw_base {
      routes.push((clr, vec![p0, p1, p2, p3]));
    } else {
      routes.push((clr, vec![p0, p1]));
      routes.push((clr, vec![p2, p3]));
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
      routes.push((clr, vec![a, b]));
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
        rng,
        &windowparams,
        clr,
        params.blazonclr,
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
        clr,
        &poly,
        scale,
      ));
    } else if w < 30.0 * scale {
      if let Some(poly) = areas.get(index_with_shadows) {
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
    }

    items.push(RenderItem::new(routes, polygons, zorder));

    if let Some(pos) = wallparams.with_door {
      let h = h * rng.gen_range(0.5..0.6);
      let w = (0.2 * w).min(h);
      let closing = 1.0 - rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
      items.push(door(rng, clr, zorder + 0.2, pos, y1, w, h, scale, closing));
    }

    for m in &wallparams.moats {
      let p1 = m.from;
      let p2 = m.to;
      let closing = m.closing;
      let ropeh = (h * rng.gen_range(0.5..0.7)).min(2. * (p2.0 - p1.0).abs());
      let rope_top_p = (p1.0, p1.1 - ropeh);
      items.push(drawbridge(
        rng,
        clr,
        zorder + 0.2,
        rope_top_p,
        p1,
        p2,
        scale,
        closing,
      ));
    }

    let mut possible_ladder_positions = vec![];
    if params.level == 0
    // && params.rec_level == 0
    {
      let samples = rng.gen_range(1..10);
      for _ in 0..samples {
        let x = rng.gen_range(x1..x2);
        let y = mix(o.1, y2, rng.gen_range(0.5..1.0));
        possible_ladder_positions.push((x, y));
      }
    }

    let mut fire_start_positions = vec![];

    let count =
      (w / (4. + rng.gen_range(0.0..50.0) * rng.gen_range(0.0..1.0))) as usize;
    if count > 0 {
      for _ in 0..count {
        let x = mix(o.0 - w / 2.0, o.0 + w / 2.0, rng.gen_range(0.1..0.9));
        let y = mix(o.1, y2, rng.gen_range(0.1..0.9));
        let radius = rng.gen_range(0.1..0.2) * w;
        fire_start_positions.push(SpawnableFire {
          pos: (x, y),
          radius,
          zorder: zorder + 1000.,
        });
      }
    }

    Self {
      items,
      roof_base,
      possible_ladder_positions,
      fire_start_positions,
    }
  }
}

impl Level for Wall {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }

  fn possible_ladder_positions(&self) -> Vec<(f32, f32)> {
    self.possible_ladder_positions.clone()
  }

  fn possible_fire_start_positions(
    &self,
  ) -> Vec<crate::objects::castle::levels::SpawnableFire> {
    self.fire_start_positions.clone()
  }
}

fn door<R: Rng>(
  rng: &mut R,
  clr: usize,
  zorder: f32,
  pos: (f32, f32),
  ybase: f32,
  w: f32,
  h: f32,
  _scale: f32,
  closing: f32,
) -> RenderItem {
  let mut routes = vec![];
  let mut polygons = vec![];

  let (x, y) = pos;
  let r = w / 2.0;

  let door = vec![
    vec![
      (x + w / 2., ybase),
      (x - w / 2., ybase),
      (x - w / 2., y - h + r),
    ],
    arc((x, y - h + r), r, -PI, 0.0, 32),
    vec![(x + w / 2., y - h + r), (x + w / 2., ybase)],
  ]
  .concat();

  let mut grids = vec![];
  let r = rng.gen_range(0.08..0.14) * w;
  let ybottom = mix(y, ybase, closing);
  let mut xp = x - w / 2.0;
  let extra = 1.5;
  while xp < x + w / 2.0 {
    let grid = vec![(xp, ybottom + extra), (xp, y - h)];
    grids.push((clr, grid));
    xp += r;
  }
  let mut yp = y - h;
  while yp < ybottom {
    let grid = vec![(x - w / 2., yp), (x + w / 2., yp)];
    grids.push((clr, grid));
    yp += r;
  }

  polygons.push(door.clone());

  routes.push((clr, door.clone()));
  routes.extend(clip_routes_with_colors(
    &grids,
    &|p| !polygon_includes_point(&door, p),
    1.0,
    3,
  ));

  RenderItem::new(routes, polygons, zorder)
}

fn drawbridge<R: Rng>(
  rng: &mut R,
  clr: usize,
  zorder: f32,
  rope_top_p: (f32, f32),
  start_p: (f32, f32),
  end_p: (f32, f32),
  scale: f32,
  closing: f32,
) -> RenderItem {
  let mut routes = vec![];
  let mut polygons = vec![];

  // door
  let top = lerp_point(rope_top_p, start_p, 0.2);
  let w = scale;
  routes.push((clr, grow_stroke_zigzag(top, start_p, w, 0.4)));
  polygons.push(grow_as_rectangle(top, start_p, w));

  // bridge
  let pw = 0.5 * scale;
  let w = (end_p.0 - start_p.0).abs();
  let mut path = vec![(0.0, -pw), (0.0, pw), (w, pw), (w, -pw), (0.0, -pw)];
  let angle = (end_p.1 - start_p.1).atan2(end_p.0 - start_p.0);
  let angle = angle + (start_p.0 - end_p.0).signum() * closing * PI / 2.0;
  path = route_translate_rotate(&path, start_p, -angle);
  polygons.push(path.clone());
  routes.push((clr, path.clone()));

  let a = lerp_point(path[0], path[1], 0.5);
  let b = lerp_point(path[2], path[3], 0.5);
  routes.push((clr, vec![a, b]));

  // rope
  let rad = scale;
  let dr = 0.5;
  let attach_p = lerp_point(a, b, rng.gen_range(0.5..1.0));
  let ang = (attach_p.1 - rope_top_p.1).atan2(attach_p.0 - rope_top_p.0);
  let mut rope = spiral_optimized_with_initial_angle(
    rope_top_p.0,
    rope_top_p.1,
    rad,
    ang,
    dr,
    0.3,
    start_p.0 < end_p.0,
  );
  rope.push(attach_p);

  routes.push((clr, rope.clone()));
  polygons.push(circle_route(rope_top_p, rad * 1.5, 16));

  RenderItem::new(routes, polygons, zorder)
}
