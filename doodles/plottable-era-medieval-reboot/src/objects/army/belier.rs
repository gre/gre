use super::belierhead::BelierHead;
use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{route_translate_rotate, Polylines},
  renderable::Renderable,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub struct Belier {
  pub routesbg: Polylines,
  pub routes: Polylines,
  pub polysbg: Vec<Vec<(f32, f32)>>,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub head: BelierHead,
  pub size: f32,
  pub origin: (f32, f32),
  pub xflip: bool,
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
      origin,
      xflip,
    }
  }

  pub fn human_positions(&self) -> Vec<(f32, f32)> {
    let mut out = vec![];
    let n = 6;
    let dx = if self.xflip { 0.1 } else { -0.1 };
    let w = 0.7;
    for i in 0..n {
      let f = i as f32 / (n as f32 - 1.) - 0.5;
      let x = self.origin.0 + (f * w + dx) * self.size;
      let y = self.origin.1 + self.size * 0.2 + ((i % 2) as f32 - 0.5) * 2.;
      out.push((x, y));
    }
    out
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

impl<R: Rng> Renderable<R> for Belier {
  fn render(&self, _rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(paint)
  }
  fn zorder(&self) -> f32 {
    self.origin.1 + self.size * 0.2
  }
}
