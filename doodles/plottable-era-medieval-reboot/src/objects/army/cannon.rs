use crate::{
  algo::{
    clipping::{regular_clip, regular_clip_polys},
    math2d::lerp_point,
    paintmask::PaintMask,
    polygon::make_wireframe_from_vertexes,
    polylines::{grow_as_rectangle, path_to_fibers, Polylines},
    renderable::Renderable,
    shapes::{circle_route, spiral_optimized},
  },
  global::GlobalCtx,
  objects::projectile::attack::AttackOrigin,
};
use rand::prelude::*;

use super::dragonhead::DragonHead;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Cannon {
  pub size: f32,
  pub origin: (f32, f32),
  pub clr: usize,
  pub w: f32,
  pub headp: (f32, f32),
  pub tailp: (f32, f32),
}

impl Cannon {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
  ) -> Self {
    let w = size;
    let headp = (origin.0 - rng.gen_range(1.0..2.0) * size, origin.1 - size);
    let tailp = (
      origin.0 + rng.gen_range(2.0..3.0) * size,
      origin.1 + size - w * 0.2,
    );
    Self {
      size,
      origin,
      clr,
      w,
      headp,
      tailp,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = vec![];

    let size = self.size;
    let clr = self.clr;
    let center = self.origin;
    let r = size;
    routes.extend(wheel(paint, clr, center, r, 8));

    let w = self.w;
    let headp = self.headp;
    let tailp = self.tailp;

    routes.extend(body(rng, paint, clr, w, headp, tailp));

    routes
  }

  pub fn throw_projectiles(&self, ctx: &mut GlobalCtx) {
    let o = self.headp;
    ctx.projectiles.add_attack(AttackOrigin::Cannon(o));
  }
}

impl<R: Rng> Renderable<R> for Cannon {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(rng, paint)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}

fn body<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  clr: usize,
  w: f32,
  headp: (f32, f32),
  tailp: (f32, f32),
) -> Polylines {
  let mut polys = vec![];
  let mut routes = vec![];

  let cannonw = rng.gen_range(1.1..1.5) * w;

  let h = (tailp.1 - headp.1).abs();
  let mut p1 = lerp_point(headp, tailp, rng.gen_range(0.4..0.6));
  p1.1 -= rng.gen_range(0.2..0.3) * h;
  let mut p2 = lerp_point(headp, tailp, rng.gen_range(0.7..0.8));
  p2.1 = tailp.1;

  let widths = vec![cannonw, cannonw, 1.3 * w, 0.7 * w];
  let count = 2 + (w / 2.0) as usize;
  let fibers = path_to_fibers(&vec![headp, p1, p2, tailp], &widths, count);

  for fiber in fibers.iter().skip(1).take(fibers.len() - 2) {
    let rt = fiber.iter().skip(1).cloned().collect::<Vec<_>>();
    routes.push((clr, rt));
  }

  let wireframe =
    make_wireframe_from_vertexes(&fibers[0], &fibers[fibers.len() - 1]);

  polys.extend(wireframe.clone());

  for wire in wireframe.iter() {
    let mut wire = wire.clone();
    wire.push(wire[0]);
    routes.push((clr, wire));
  }

  regular_clip_polys(&routes, paint, &polys)
}

fn wheel(
  paint: &mut PaintMask,
  clr: usize,
  center: (f32, f32),
  r: f32,
  divs: usize,
) -> Polylines {
  let mut polys = vec![];
  let mut routes = vec![];

  let r1 = 0.2 * r;
  let r2 = 0.8 * r;
  let r3 = r;

  let count = 32;

  let c3 = circle_route(center, r3, count);
  let c2 = circle_route(center, r2, count);

  polys.push(c3.clone());

  routes.push((clr, c3.clone()));
  routes.push((clr, c2.clone()));
  routes.push((clr, spiral_optimized(center.0, center.1, r1, 0.5, 0.1)));

  routes.extend(
    circle_route(center, r1, divs)
      .iter()
      .zip(circle_route(center, r2, divs).iter())
      .map(|(p1, p2)| (clr, vec![*p1, *p2])),
  );

  regular_clip_polys(&routes, paint, &polys)
}
