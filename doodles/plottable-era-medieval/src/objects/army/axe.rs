use crate::algo::{
  clipping::regular_clip,
  math2d::lerp_point,
  paintmask::PaintMask,
  polylines::{route_translate_rotate, Polylines},
  renderable::Renderable,
  shapes::arc,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
#[derive(Clone)]
pub struct Axe {
  pub routesbg: Polylines,
  pub routes: Polylines,
  pub polysbg: Vec<Vec<(f32, f32)>>,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub origin: (f32, f32),
}

impl Axe {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    (xflip, yflip): (bool, bool),
  ) -> Self {
    let mut routes = vec![];
    let mut routesbg = vec![];
    let mut polysbg = vec![];
    let mut polys = vec![];

    let xdir = if xflip { -1.0 } else { 1.0 };
    let ydir = if yflip { -1.0 } else { 1.0 };
    let dx = size * 0.05;
    let dy = size * 0.5;

    // stick
    let rt = vec![(-dx, -dy), (-dx, dy), (dx, dy), (dx, -dy), (-dx, -dy)];
    let rt = route_translate_rotate(&rt, origin, -angle);
    routes.push((clr, rt.clone()));
    polys.push(rt);

    // metal
    let c = (0.0, -0.35 * size);
    let r = 0.4 * size;
    let op = rng.gen_range(0.6..0.8);
    let res = 20;
    let mut rt = arc(c, r, -op, op, res);
    let m = rng.gen_range(0.3..0.6);
    let deform = rng.gen_range(0.0..0.03) * size;
    let mut a = lerp_point(c, rt[0], m);
    a.1 += deform;
    let mut b = lerp_point(c, rt[rt.len() - 1], m);
    b.1 -= deform;
    rt.push(b);
    rt.push((c.0, b.1));
    rt.push((c.0, a.1));
    rt.push(a);
    rt.push(rt[0]);
    let rt = rt
      .iter()
      .map(|o| (o.0 * xdir, o.1 * ydir))
      .collect::<Vec<_>>();
    let rt = route_translate_rotate(&rt, origin, -angle);
    routesbg.push((clr, rt.clone()));
    polysbg.push(rt);

    Self {
      routes,
      routesbg,
      polysbg,
      polys,
      origin,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut out = regular_clip(&self.routes, paint);
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }
    out.extend(regular_clip(&self.routesbg, paint));
    for poly in &self.polysbg {
      paint.paint_polygon(poly);
    }
    out
  }
}

impl<R: Rng> Renderable<R> for Axe {
  fn render(
    &self,
    _rng: &mut R,
    _ctx: &mut crate::global::GlobalCtx,
    paint: &mut crate::algo::paintmask::PaintMask,
  ) -> crate::algo::polylines::Polylines {
    let routes = self.render(paint);
    routes
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
