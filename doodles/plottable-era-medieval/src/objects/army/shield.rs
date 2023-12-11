use std::f32::consts::PI;

use crate::{
  algo::{
    clipping::regular_clip_polys, paintmask::PaintMask,
    polylines::route_translate_rotate, renderable::Renderable,
  },
  objects::blazon::Blazon,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
#[derive(Clone)]
pub struct Shield {
  routes: Vec<(usize, Vec<(f32, f32)>)>,
  polygons: Vec<Vec<(f32, f32)>>,
  origin: (f32, f32),
}

impl Shield {
  pub fn init<R: Rng>(
    rng: &mut R,
    _clr: usize,
    blazonclr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    blazon: Blazon,
  ) -> Self {
    let mx = if xflip { -1.0 } else { 1.0 };
    let mut routes = Vec::new();
    let dx = 0.2 * size;
    let dy = 0.4 * size;
    let mut route = vec![];
    let mut route2 = vec![];

    let a = match blazon {
      Blazon::Dragon => 0.0,
      Blazon::Falcon => 2.0 * PI / 3.0,
      Blazon::Lys => -2.0 * PI / 3.0,
    } + rng.gen_range(-0.5..0.5);
    let s1 = 0.5 + 0.5 * a.cos();
    let s2 = 0.5 + 0.5 * a.sin();

    for v in vec![
      (0.0, -dy),
      (mx * 0.5 * dx, -dy),
      (mx * dx, -(1.0 - s1 * s1) * dy),
      (mx * dx, 0.0),
      (mx * dx, s2 * dy),
      (0.0, dy),
    ] {
      route.push(v);
      route2.push((-v.0, v.1));
    }
    route2.reverse();
    route.extend(route2);

    route = route_translate_rotate(&route, origin, angle);
    let polygons = vec![route.clone()];
    routes.push((blazonclr, route));

    let tick = rng.gen_range(0.2..0.3);
    let y = rng.gen_range(-0.2..0.0) * dy;
    routes.push((
      blazonclr,
      route_translate_rotate(
        &vec![
          (0.0, -tick * dy + y),
          (mx * tick * dx, y),
          (0.0, tick * dy + y),
        ],
        origin,
        angle,
      ),
    ));

    Self {
      routes,
      polygons,
      origin,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Vec<(usize, Vec<(f32, f32)>)> {
    regular_clip_polys(&self.routes, paint, &self.polygons)
  }
}

impl<R: Rng> Renderable<R> for Shield {
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
