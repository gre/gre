use std::f32::consts::PI;

use rand::prelude::*;

use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  paintmask::PaintMask,
  polygon::make_tri_wireframe_from_vertexes,
  polylines::{route_translate_rotate, Polylines},
  renderable::Renderable,
};

use super::wheeledplatform::WheeledPlatform;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

/**
 * a triangular structure with warriors going in.
 */

pub struct TunnelStructure {
  pub routes: Polylines,
  pub wheelplat: WheeledPlatform,
  pub clr: usize,
  pub origin: (f32, f32),
}

impl TunnelStructure {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    angle: f32,
    h: f32,
    w: f32,
  ) -> Self {
    let triratio =
      mix(1.0, 0.5, rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0));
    let xincr = h * triratio;
    let mut x = -w / 2.0;
    let dx = -xincr / 2.0;
    let mut vert1 = vec![];
    let mut vert2 = vec![];
    vert2.push((x + dx, -h / 2.0));
    while x <= w / 2.0 {
      x += xincr;
      vert1.push((x - xincr / 2.0 + dx, h / 2.0));
      vert2.push((x + dx, -h / 2.0));
    }

    let mut polys = make_tri_wireframe_from_vertexes(&vert1, &vert2);

    if polys.len() > 1 {
      polys.remove(1);
    }

    let mut routes = Polylines::new();
    for poly in polys {
      let mut rt = route_translate_rotate(&poly, origin, angle);
      rt.push(rt[0]);
      routes.push((clr, rt));
    }

    let wh = 0.3 * h;
    let wheel_count =
      2 + mix(0.0, 0.5 * w / wh, rng.gen_range(0.0..1.0)) as usize;
    let wheel_pad = 0.5 * h * rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
    let mut o = origin;
    let a = angle + PI / 2.0;
    o.0 += a.cos() * h / 2.0;
    o.1 += a.sin() * h / 2.0;
    let wheelplat =
      WheeledPlatform::init(o, wh, w, angle, wheel_pad, wheel_count, clr);

    Self {
      routes,
      wheelplat,
      clr,
      origin,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut out = vec![];
    out.extend(self.wheelplat.render(paint));

    let rts = regular_clip(&self.routes, paint);
    out.extend(rts);
    for (_, route) in &self.routes {
      paint.paint_polygon(route);
      paint.paint_polyline(route, 1.0);
    }

    out
  }
}

impl<R: Rng> Renderable<R> for TunnelStructure {
  fn render(&self, rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(paint)
  }
  fn yorder(&self) -> f32 {
    self.origin.1
  }
}
