use crate::algo::{
  clipping::{clip_routes_with_colors, regular_clip},
  paintmask::PaintMask,
  polylines::Polylines,
  renderable::Renderable,
  shapes::{circle_route, spiral_optimized},
  wormsfilling::WormsFilling,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub struct SauronEye {
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub origin: (f32, f32),
}

impl SauronEye {
  pub fn init<R: Rng>(
    rng: &mut R,
    paint: &PaintMask,
    clr: usize,
    eyeclr: usize,
    origin: (f32, f32),
    size: f32,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];

    let rrock = 0.35 * size;
    let rbound = 0.4 * size;
    let c = (origin.0, origin.1 + 0.4 * size);
    let c2 = (c.0, c.1 - 0.2 * rrock);
    let r2 = 0.9 * rrock;
    let r2w = 1.2;
    let ceye = (c.0, c.1 - 0.3 * rrock);
    let reye = 0.18 * size;
    let c3 = ceye;
    let r3 = rng.gen_range(0.6..0.7) * reye;
    let r3w = rng.gen_range(2.0..3.0);

    let mut rock_routes = vec![];
    let spiral = spiral_optimized(c.0, c.1, rrock, 0.6, 0.2);
    rock_routes.push((clr, spiral.clone()));
    rock_routes.push((clr, circle_route(c, rrock, (rrock * 2. + 8.) as usize)));
    let exclude = |p: (f32, f32)| {
      let dx = r2w * (p.0 - c2.0);
      let dy = p.1 - c2.1;
      dx * dx + dy * dy < r2 * r2
    };
    let rock_routes = clip_routes_with_colors(&rock_routes, &exclude, 0.5, 3);
    routes.extend(rock_routes);

    polys.push(circle_route(c, rbound, 20));

    let mut eye_routes = vec![];
    for _ in 0..3 {
      let ym = rng.gen_range(0.0..1.0);
      eye_routes.push((
        eyeclr,
        vec![
          (ceye.0, ceye.1 - ym * reye),
          (ceye.0 + 1.4 * reye, ceye.1),
          (ceye.0, ceye.1 + ym * reye),
          (ceye.0 - 1.4 * reye, ceye.1),
          (ceye.0, ceye.1 - ym * reye),
        ],
      ));
    }
    let dr = rng.gen_range(0.5..1.6);
    eye_routes.push((eyeclr, spiral_optimized(ceye.0, ceye.1, reye, dr, 0.2)));

    let exclude = |p: (f32, f32)| {
      let dx = r3w * (p.0 - c3.0);
      let dy = p.1 - c3.1;
      dx * dx + dy * dy < r3 * r3
    };
    let eye_routes = clip_routes_with_colors(&eye_routes, &exclude, 0.5, 3);

    let filling = WormsFilling::rand(rng);
    let mut drawing = paint.clone_empty();
    for (_, rt) in eye_routes.iter() {
      drawing.paint_polyline(rt, 0.5);
    }
    let bound = drawing.painted_boundaries();
    let its = rng.gen_range(600..1000);
    let eye_routes =
      filling.fill_in_paint(rng, &drawing, eyeclr, 3.0, bound, its);

    routes.extend(eye_routes);

    Self {
      routes,
      polys,
      origin,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }
    routes
  }
}

impl<R: Rng> Renderable<R> for SauronEye {
  fn render(
    &self,
    _rng: &mut R,
    _ctx: &mut crate::global::GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(paint)
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
