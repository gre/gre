use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    polylines::{path_subdivide_to_curve, shake, Polylines},
    renderable::Renderable,
  },
  global::GlobalCtx,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone, Copy)]
pub struct PalmTree {
  pub origin: (f32, f32),
  pub size: f32,
  pub clr: usize,
}

impl PalmTree {
  pub fn init(clr: usize, origin: (f32, f32), size: f32) -> Self {
    Self { origin, size, clr }
  }
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    clr: usize,
  ) -> Polylines {
    let mut routes = Polylines::new();
    let p = self.origin;
    let h = self.size;
    let mut path = vec![
      (p.0, p.1),
      (
        p.0 - rng.gen_range(-1.0..1.0) * h * 0.2,
        p.1 - rng.gen_range(0.3..0.6) * h,
      ),
      (p.0 - rng.gen_range(-1.0..1.0) * h * 0.4, p.1 - h),
    ];
    path = path_subdivide_to_curve(&path, 2, 0.66);
    for _j in 0..5 {
      routes.push((clr, shake(path.clone(), 0.4, rng)));
    }
    for _j in 0..rng.gen_range(4..12) {
      let x = rng.gen_range(0.5 * h..h)
        * (if rng.gen_bool(0.5) { -1.0 } else { 1.0 });
      let mut path = vec![
        (p.0, p.1 - h * rng.gen_range(0.9..1.0)),
        (p.0 + x * 0.5, p.1 - h - rng.gen_range(-1.0..1.0) * h * 0.2),
        (p.0 + x, p.1 - h * rng.gen_range(0.3..1.1)),
      ];
      path = path_subdivide_to_curve(&path, 2, 0.66);
      routes.push((clr, path));
    }

    routes = regular_clip(&routes, paint);

    for (_clr, route) in &routes {
      paint.paint_polyline(route, 0.1 * h);
    }

    routes
  }
}

impl<R: Rng> Renderable<R> for PalmTree {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(rng, paint, self.clr)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
