use std::f32::consts::PI;

use super::{wallshadows::wall_shadow, Floor, Level, LevelParams, RenderItem};
use crate::{
  algo::{
    math1d::mix,
    polylines::{path_subdivide_to_curve, Polyline, Polylines},
    shapes::{circle_route, spiral_optimized_with_initial_angle},
  },
  global::GlobalCtx,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Bell {
  roof_base: Option<Floor>,
  items: Vec<RenderItem>,
}

impl Bell {
  pub fn max_allowed_width(scale: f32) -> f32 {
    14.0 * scale
  }
  pub fn init<R: Rng>(
    _rng: &mut R,
    _ctx: &GlobalCtx,
    params: &LevelParams,
  ) -> Self {
    let mut items = vec![];
    let zorder = params.level_zorder;
    let w = params.floor.width;
    let h = params.preferrable_height.min(w);
    let o = params.floor.pos;
    let scale = params.scaleref;
    let clr = params.clr;

    let mut routes = vec![];
    let mut polygons = vec![];
    let x1 = o.0 - w / 2.;
    let x2 = o.0 + w / 2.;
    let y1 = o.1;
    let y2 = o.1 - h;
    let roof_base = Some(Floor::new((o.0, y2), w, vec![], true));
    let p0 = (x1, y2);
    let p1 = (x1, y1);

    let p2 = (x2, y1);
    let p3 = (x2, y2);

    let mut poly = vec![];
    let pad = 0.1 * w;
    let y3 = mix(y1, y2, 0.5);
    let y4 = y2 + pad;
    let mut route2 = vec![];
    route2.push((x1 + pad, y1));
    route2.push((x1 + pad, y3));
    route2.push((o.0, y4));
    route2.push((x2 - pad, y3));
    route2.push((x2 - pad, y1));
    route2 = path_subdivide_to_curve(&route2, 2, 0.7);
    poly.extend(route2.clone());
    let mut route3 = vec![];
    route3.push(p2);
    route3.push(p3);
    route3.push(p0);
    route3.push(p1);
    poly.extend(route3.clone());
    polygons.push(poly.clone());

    routes.push((clr, route2));
    routes.push((clr, route3));
    if !params.floor.is_closed {
      routes.push((clr, vec![(x1 + pad, y1), (x2 - pad, y1)]));
    }

    routes.extend(wall_shadow(
      params.tower_seed,
      clr,
      &poly,
      params.light_x_direction,
      scale,
      0.6,
    ));

    let y5 = y4 + 0.1 * h;
    let y6 = y5 + 0.14 * h;

    let p4 = (o.0, y6);
    routes.push((clr, vec![(o.0, y5), p4]));

    let size = 0.42 * h;
    let (rts, polys) = bell(params, p4, size, clr);
    routes.extend(rts);
    polygons.extend(polys);

    items.push(RenderItem::new(routes, polygons, zorder));

    Self { items, roof_base }
  }
}

impl Level for Bell {
  fn roof_base(&self) -> Option<Floor> {
    self.roof_base.clone()
  }

  fn render(&self) -> Vec<RenderItem> {
    self.items.clone()
  }
}

fn bell(
  params: &LevelParams,
  o: (f32, f32),
  size: f32,
  clr: usize,
) -> (Polylines, Vec<Polyline>) {
  let rad = 0.1;
  let xp = 0.2;
  let yp = 0.6;
  let wtop = 0.15;
  let interp = 0.8;
  let interpits = 2;

  let mut routes = vec![];
  let mut polygons = vec![];

  let y1 = o.1;
  let y2 = o.1 + yp * size;
  let y3 = o.1 + 0.9 * size;
  let y4 = o.1 + size;
  let x1 = o.0 - 0.5 * size;
  let x2 = o.0 - xp * size;
  let x3 = o.0 - wtop * size;
  let x4 = o.0 + wtop * size;
  let x5 = o.0 + xp * size;
  let x6 = o.0 + 0.5 * size;

  let mut route =
    vec![(x1, y3), (x2, y2), (x3, y1), (x4, y1), (x5, y2), (x6, y3)];
  route = path_subdivide_to_curve(&route, interpits, interp);

  let mut route2 = vec![(x1, y3), (o.0, y4), (x6, y3)];
  route2 = path_subdivide_to_curve(&route2, interpits, interp);
  route2.reverse();

  route.extend(route2);

  routes.push((clr, route.clone()));
  polygons.push(route.clone());

  routes.extend(wall_shadow(
    params.tower_seed,
    clr,
    &route,
    params.light_x_direction,
    params.scaleref / 2.0,
    0.6,
  ));

  let r = rad * size;
  let y5 = y4 + r;

  routes.push((
    clr,
    spiral_optimized_with_initial_angle(
      o.0,
      y5,
      r,
      -PI / 2.0,
      0.4,
      0.05,
      false,
    ),
  ));

  polygons.push(circle_route((o.0, y5), r, 20));

  (routes, polygons)
}
