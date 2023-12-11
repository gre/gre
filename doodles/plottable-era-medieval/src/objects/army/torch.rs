use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    polylines::{route_translate_rotate, Polyline, Polylines},
    renderable::Renderable,
    shapes::{circle_route, spiral_optimized},
  },
  global::GlobalCtx,
};
use rand::prelude::*;

use super::fire::Fire;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct Torch {
  pub stick_polys: Vec<Polyline>,
  pub stick_routes: Polylines,
  pub fire: Fire,
  pub origin: (f32, f32),
  pub size: f32,
}

impl Torch {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    flameclr: usize,
    origin: (f32, f32),
    angle: f32,
    size: f32,
  ) -> Self {
    let mut stick_polys = vec![];
    let mut stick_routes = vec![];

    // FLAME
    let rad = rng.gen_range(0.15..0.2) * size;
    let amp = size - rad / 2.0;
    let flamex = origin.0 + amp * (-angle).cos();
    let flamey = origin.1 + amp * (-angle).sin();

    let fire = Fire::init(rng, flameclr, (flamex, flamey), rad);

    stick_routes.push((clr, spiral_optimized(flamex, flamey, rad, 0.7, 0.2)));
    stick_routes.push((clr, circle_route((flamex, flamey), rad, 24)));

    // STICK
    let h1 = -0.05 * size;
    let h2 = size - rad;
    let w1 = rng.gen_range(0.4..0.5) * rad;
    let w2 = rng.gen_range(0.9..1.0) * rad;
    let mut stick = route_translate_rotate(
      &vec![(h1, -w1 / 2.), (h2, -w2 / 2.), (h2, w2 / 2.), (h1, w1 / 2.)],
      origin,
      angle,
    );
    stick_polys.push(stick.clone());
    stick.push(stick[0]);
    stick_routes.push((clr, stick));
    stick_routes.push((clr, vec![origin, (flamex, flamey)]));

    Self {
      fire,
      stick_polys,
      stick_routes,
      origin,
      size,
    }
  }

  pub fn render(
    &self,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = vec![];
    let rts = self.fire.render(ctx, paint);
    routes.extend(rts);

    let rts = regular_clip(&self.stick_routes, paint);
    for poly in &self.stick_polys {
      paint.paint_polygon(poly);
    }
    routes.extend(rts);

    routes
  }
}

impl<R: Rng> Renderable<R> for Torch {
  fn render(
    &self,
    _rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(ctx, paint)
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
