use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    polylines::{
      path_subdivide_to_curve, route_scale_translate_rotate,
      scale_translate_rotate, Polylines,
    },
  },
  objects::blazon::Blazon,
};
use rand::prelude::*;
use std::f32::consts::PI;

use super::belierhead::BelierHead;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Boat {
  pub x1: f32,
  pub x2: f32,
  pub origin: (f32, f32),
  pub size: f32,
  pub angle: f32,
  pub w: f32,
  pub xflip: bool,
  pub blazon: Blazon,
  pub clr: usize,
}

impl Boat {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    w: f32,
    xflip: bool,
    blazon: Blazon,
    clr: usize,
  ) -> Self {
    let x1 = -w * rng.gen_range(0.3..0.45);
    let x2 = w * rng.gen_range(0.3..0.4);

    Self {
      x1,
      x2,
      origin,
      size,
      angle,
      w,
      xflip,
      blazon,
      clr,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    mask: &mut PaintMask,
    clr: usize,
  ) -> Polylines {
    let x1 = self.x1;
    let x2 = self.x2;

    let size = self.size;
    let origin = self.origin;
    let angle = self.angle;
    let w = self.w;
    let xflip = self.xflip;

    let mut out = vec![];
    let mut routes = vec![];

    let xdir = if xflip { -1.0 } else { 1.0 };

    let h = size;
    let yleft = -h * rng.gen_range(0.6..1.0);
    let yright = -h * rng.gen_range(0.8..1.0);

    let dy_edge = 0.3;
    // boat bottom
    let mut route = Vec::new();
    route.push((-w / 2.0 - dy_edge, yleft + dy_edge));
    route.push((x1, 0.0));
    route.push((x2, 0.0));
    route.push((w / 2.0 + dy_edge, yright + dy_edge));
    route = path_subdivide_to_curve(&route, 2, 0.8);
    routes.push((clr, route));

    // boat in between
    let mut route = Vec::new();
    let y = -0.15 * h;
    route.push((-w / 2.0, yleft));
    route.push((x1, y));
    route.push((x2, y));
    route.push((w / 2.0, yright));
    route = path_subdivide_to_curve(&route, 2, 0.8);
    // TODO route will be used to clip people
    routes.push((clr, route));

    // boat top
    let mut route = Vec::new();
    let y = -0.3 * h;
    route.push((-w / 2.0 + dy_edge, yleft - dy_edge));
    route.push((x1, y));
    route.push((x2, y));
    route.push((w / 2.0 - dy_edge, yright - dy_edge));
    route = path_subdivide_to_curve(&route, 2, 0.8);
    // TODO route will be used to clip people
    routes.push((clr, route.clone()));

    // make a boat head
    let o =
      scale_translate_rotate((w / 2.0, yright), (xdir, 1.), origin, angle);
    let s = 0.5 * size;
    let a = angle - PI / 4.0;

    let h = BelierHead::init(rng, clr, o, s, a, xflip);
    out.extend(h.render(mask));

    routes = routes
      .iter()
      .map(|(clr, route)| {
        (
          *clr,
          route_scale_translate_rotate(route, (xdir, 1.0), origin, angle),
        )
      })
      .collect();
    routes = regular_clip(&routes, mask);
    // FIXME probably better than this clipping. but good for now
    for (_clr, route) in &routes {
      mask.paint_polyline(route, 0.1 * size);
    }
    out.extend(routes);

    out
  }
}

impl<R: Rng> super::Renderable<R> for Boat {
  fn render(&self, rng: &mut R, mask: &mut PaintMask) -> Polylines {
    self.render(rng, mask, self.clr)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
