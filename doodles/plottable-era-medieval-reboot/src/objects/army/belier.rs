use super::belierhead::BelierHead;
use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{route_translate_rotate, Polylines},
};
use rand::prelude::*;

pub struct Belier {
  pub routesbg: Polylines,
  pub routes: Polylines,
  pub polysbg: Vec<Vec<(f32, f32)>>,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub head: BelierHead,
  pub size: f32,
}

impl Belier {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    with_triangle_structure: bool,
  ) -> Self {
    let xdir = if xflip { -1. } else { 1. };
    let mut routes = vec![];
    let mut routesbg = vec![];
    let mut polysbg = vec![];
    let mut polys = vec![];

    if with_triangle_structure {
      let dy = size * 0.05;
      let topdy = -size * 0.4;
      let bottomdy = size * 0.3;
      let xdiff = size * 0.2;
      for i in 0..2 {
        let mul = (i as f32 - 0.5) * 2.0;
        let poly = vec![
          (-mul * xdiff, bottomdy - dy),
          (-mul * xdiff, bottomdy + dy),
          (0.0, topdy + dy),
          (0.0, topdy - dy),
        ];
        let mut poly = route_translate_rotate(&poly, origin, -angle);
        polysbg.push(poly.clone());
        poly.push(poly[0]);
        routesbg.push((clr, poly));
      }
    }

    // stick
    let dy = size * 0.05;
    let dx = size * 0.5;
    let rt = vec![(-dx, -dy), (-dx, dy), (dx, dy), (dx, -dy), (-dx, -dy)];
    let rt = route_translate_rotate(&rt, origin, -angle);
    routes.push((clr, rt.clone()));
    polys.push(rt);

    // head
    let acos = angle.cos();
    let asin = angle.sin();
    let s = 0.2 * size;
    let m = xdir * 0.45 * size;
    let o = (origin.0 + acos * m, origin.1 + asin * m);
    let head = BelierHead::init(rng, clr, o, s, angle, xflip);

    Self {
      routes,
      routesbg,
      polysbg,
      polys,
      head,
      size,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut out = vec![];
    out.extend(self.head.render(paint));
    out.extend(regular_clip(&self.routes, paint));
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }
    out.extend(regular_clip(&self.routesbg, paint));
    for poly in &self.polysbg {
      paint.paint_polygon(poly);
    }
    for (_clr, route) in &out {
      paint.paint_polyline(route, 0.02 * self.size);
    }
    out
  }
}
