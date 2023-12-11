use std::f32::consts::PI;

use super::wheeledplatform::WheeledPlatform;
use crate::{
  algo::{
    clipping::regular_clip,
    math1d::mix,
    math2d::{angle_mirrored_on_x, lerp_point},
    paintmask::PaintMask,
    polylines::{grow_as_rectangle, route_translate_rotate, Polylines},
    renderable::Renderable,
    shapes::ovale_route,
  },
  global::GlobalCtx,
  objects::projectile::attack::AttackOrigin,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub struct Catapult {
  pub clr: usize,
  pub origin: (f32, f32),
  pub basket: (f32, f32),
  pub wheeledplatform: WheeledPlatform,
  pub woods: Vec<Vec<(f32, f32)>>,
  pub decoration_routes: Polylines,
  pub progress: f32,
}

impl Catapult {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    progress: f32,
  ) -> Self {
    let flipmul = if !xflip { -1.0 } else { 1.0 };

    let orient_angle = if !xflip { angle_mirrored_on_x } else { |a| a };

    let w = size;
    let h = rng.gen_range(0.45..0.55) * size;
    let wheeledplatform = WheeledPlatform::init(
      origin,
      rng.gen_range(0.15..0.2) * w,
      w,
      orient_angle(-angle),
      0.0,
      2,
      clr,
    );

    let w2 = rng.gen_range(0.55..0.65) * w;

    let a = orient_angle(angle);
    let acos = a.cos();
    let asin = a.sin();
    let angle2 = orient_angle(angle - PI / 2.0);
    let a2cos = angle2.cos();
    let a2sin = angle2.sin();
    let a = orient_angle(angle - PI * mix(0.9, 0.7, progress));
    let a3cos = a.cos();
    let a3sin = a.sin();

    let amp = (w - w2) / 2.0;
    let dx = acos * amp;
    let dy = asin * amp;
    let p = (origin.0 + dx, origin.1 + dy);
    let p1 = (p.0 - acos * w2 / 2., p.1 - asin * w2 / 2.);
    let p2 = (p.0 + acos * w2 / 2., p.1 + asin * w2 / 2.);

    let amp = h;
    let dx = a2cos * amp;
    let dy = a2sin * amp;
    let q = (p.0 + dx, p.1 + dy);

    let bastetl = size;
    let basketp = (p.0 + a3cos * bastetl, p.1 + a3sin * bastetl);

    let rope_end = lerp_point(basketp, p, 0.4);

    let mut woods = vec![];

    let ovale = ovale_route(
      (0.0, 0.0),
      (0.16 * size, rng.gen_range(0.09..0.13) * size),
      12,
    );

    let mw = 0.02 * size;
    let mw2 = 0.035 * size;
    let ovale = ovale
      .iter()
      .map(|p| (p.0, flipmul * p.1.min(1.7 * mw2)))
      .collect();
    let ovale = route_translate_rotate(&ovale, basketp, -a);
    woods.push(ovale);
    woods.push(grow_as_rectangle(p, q, mw2));
    woods.push(grow_as_rectangle(p1, q, mw));
    woods.push(grow_as_rectangle(p2, q, mw));
    woods.push(grow_as_rectangle(p, basketp, mw2));
    let mut decoration_routes = vec![];
    decoration_routes.push((
      clr,
      vec![lerp_point(rope_end, q, -0.02), lerp_point(p, q, 0.95)],
    ));

    Self {
      clr,
      origin,
      basket: basketp,
      wheeledplatform,
      woods,
      decoration_routes,
      progress,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut routes = vec![];
    let clr = self.clr;

    routes.extend(regular_clip(&self.decoration_routes, paint));

    routes.extend(self.wheeledplatform.render(paint));

    for plank in self.woods.iter() {
      routes.extend(regular_clip(&vec![(clr, plank.clone())], paint));
      paint.paint_polygon(plank);
    }

    routes
  }

  pub fn throw_projectiles(&self, ctx: &mut GlobalCtx) {
    if self.progress < 0.5 {
      return;
    }
    let o = self.basket;
    ctx.projectiles.add_attack(AttackOrigin::Catapult(o));
  }
}

impl<R: Rng> Renderable<R> for Catapult {
  fn render(
    &self,
    _rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(paint)
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
