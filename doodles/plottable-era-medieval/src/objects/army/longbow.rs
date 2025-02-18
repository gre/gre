use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  polylines::{
    grow_path_zigzag, path_subdivide_to_curve, route_translate_rotate,
  },
  renderable::Renderable,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn long_bow(
  clr: usize,
  origin: (f32, f32),
  size: f32,
  angle: f32,
  phase: f32,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes: Vec<Vec<(f32, f32)>> = Vec::new();

  // arc au repos
  let dy = 0.5 * size;
  let dx = 0.5 * dy;
  let bow_w = 0.1 * size;

  let max_allonge = 0.8 * size;
  let allonge = mix(dx, max_allonge, phase);

  let mut route = vec![];
  route.push((-dx, -dy));
  route.push((0.0, 0.0));
  route.push((-dx, dy));
  let bow = path_subdivide_to_curve(&route, 2, 0.8);

  routes.push(grow_path_zigzag(bow, angle, bow_w, 0.3));

  let string = vec![(-dx, -dy), (-allonge, 0.0), (-dx, dy)];

  routes.push(string);

  // translate routes
  let out = routes
    .iter()
    .map(|route| {
      let route = route_translate_rotate(route, origin, angle);
      (clr, route)
    })
    .collect();

  out
}

pub struct LongBow {
  pub origin: (f32, f32),
  pub size: f32,
  pub angle: f32,
  pub phase: f32,
  pub clr: usize,
}

impl LongBow {
  pub fn init(
    origin: (f32, f32),
    size: f32,
    angle: f32,
    phase: f32,
    clr: usize,
  ) -> Self {
    Self {
      origin,
      size,
      angle,
      phase,
      clr,
    }
  }

  pub fn render(&self) -> Vec<(usize, Vec<(f32, f32)>)> {
    long_bow(self.clr, self.origin, self.size, self.angle, self.phase)
  }
}

impl<R: Rng> Renderable<R> for LongBow {
  fn render(
    &self,
    _rng: &mut R,
    _ctx: &mut crate::global::GlobalCtx,
    paint: &mut crate::algo::paintmask::PaintMask,
  ) -> crate::algo::polylines::Polylines {
    let routes = self.render();
    let routes = regular_clip(&routes, paint);
    for (_, route) in routes.iter() {
      paint.paint_polyline(route, 0.5);
    }
    routes
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
