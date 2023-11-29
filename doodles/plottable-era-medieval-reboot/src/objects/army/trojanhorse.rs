use super::{horse::Horse, wheeledplatform::WheeledPlatform};
use crate::algo::{
  paintmask::PaintMask, polylines::Polylines, renderable::Renderable,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct TrojanHorse {
  pub platform: WheeledPlatform,
  pub horse: Horse,
}

impl TrojanHorse {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    xflip: bool,
    clr: usize,
  ) -> Self {
    let h = 0.1 * size;
    let w = size;
    let wheel_pad = rng.gen_range(-0.1f32..0.1).max(0.0) * size;
    let wheel_count = rng.gen_range(2..8);
    let platform =
      WheeledPlatform::init(origin, h, w, 0.0, wheel_pad, wheel_count, clr);

    let o = (origin.0, origin.1 - 0.2 * size);
    let horse = Horse::init(o, size, 0.0, xflip, clr, clr, 1.5, 0.0);

    Self { platform, horse }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = Polylines::new();

    let platform = self.platform.render(paint);
    let horse = self.horse.render(rng, paint);

    routes.extend(platform);
    routes.extend(horse);

    // halo
    for (_, route) in &routes {
      paint.paint_polyline(route, 1.0);
    }

    routes
  }
}

impl<R: Rng> Renderable<R> for TrojanHorse {
  fn render(&self, rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(rng, paint)
  }

  fn zorder(&self) -> f32 {
    self.platform.origin.1 + self.platform.h / 2.0
  }
}
