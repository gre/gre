use std::f32::consts::PI;

use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    polylines::{step_polyline, Polylines},
    renderable::{Container, Renderable},
  },
  global::{GlobalCtx, Special},
  objects::{army::flag::Flag, castle::chinesedoor::ChineseDoor},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

struct PalisadeWoodTrunk {
  pub pos: (f32, f32),
  pub width: f32,
  pub height: f32,
  pub clr: usize,
}

impl<R: Rng> Renderable<R> for PalisadeWoodTrunk {
  fn render(
    &self,
    _rng: &mut R,
    _ctx: &mut crate::global::GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = vec![];
    let pos = self.pos;
    let width = self.width;
    let height = self.height;
    let clr = self.clr;
    let left = (pos.0 - width / 2., pos.1);
    let right = (pos.0 + width / 2., pos.1);
    let wallheighty = pos.1 - height;

    let mut route = Vec::new();
    route.push(left);
    route.push((left.0, wallheighty));
    route.push((pos.0, wallheighty - width));
    route.push((right.0, wallheighty));
    route.push(right);
    routes.push((clr, route.clone()));

    routes = regular_clip(&routes, paint);

    paint.paint_polygon(&route);
    paint.paint_polyline(&route, 0.5);

    routes
  }

  fn zorder(&self) -> f32 {
    self.pos.1
  }
}

pub struct MountainWall<R: Rng> {
  pub container: Container<R>,
}
impl<R: Rng> MountainWall<R> {
  pub fn init(
    ctx: &mut GlobalCtx,
    rng: &mut R,
    blazonclr: usize,
    clr: usize,
    path: &Vec<(f32, f32)>,
    baseh: f32,
  ) -> Self {
    let mut container = Container::new();
    let trunkw = baseh * rng.gen_range(0.15..0.2);
    let step = rng.gen_range(0.8..1.0) * trunkw;
    let all = step_polyline(path, step);

    let mut parts = vec![];

    let l = all.len();
    let gap = rng.gen_range(3..12);
    // let entrance_door_range = 4..8;
    if gap * 6 < l {
      let splitcenter = l / 2;
      let i = splitcenter - gap;
      let j = splitcenter + gap;
      let a = all[i];
      let b = all[j];
      let center = all[splitcenter];
      let w = 2.0 * (b.0 - a.0).abs();
      let h = baseh * 2.0;
      let y = a.1.max(b.1) + 0.5;
      let origin = (center.0, y);
      if ctx.specials.contains(&Special::Chinese) {
        let door = ChineseDoor::init(rng, clr, origin, w, h, 0.);
        container.add(door);
      }

      parts.push(all[..i].to_vec());
      parts.push(all[j..].to_vec());
    } else {
      parts.push(all);
    }

    for part in parts {
      let l = part.len();
      for (i, pos) in part.iter().enumerate() {
        let extremity = i == 0 || i == l - 1;
        let dy = if extremity { -0.4 } else { 0.0 };
        let pos = (pos.0, pos.1 + dy);
        let width = trunkw * rng.gen_range(0.8..1.2)
          + if extremity { 0.5 * trunkw } else { 0.0 };
        let height = baseh
          * (1.0 + rng.gen_range(-0.2..0.2) * rng.gen_range(0.0..1.0))
          + if extremity { 0.5 * baseh } else { 0.0 };
        if extremity {
          let right = false;
          let close_height_factor = 0.5;
          let close_len_factor = 0.7;
          let spike = true;
          let flag = Flag::init(
            rng,
            clr,
            blazonclr,
            (pos.0, pos.1 - height),
            baseh,
            -PI / 2.0,
            right,
            close_height_factor,
            close_len_factor,
            spike,
          );
          container.add(flag);
        }
        let trunk = PalisadeWoodTrunk {
          pos,
          width,
          height,
          clr,
        };
        container.add(trunk);
      }
    }
    Self { container }
  }
}
