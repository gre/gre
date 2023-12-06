use super::{horse::Horse, human::Human};
use crate::{
  algo::{paintmask::PaintMask, polylines::Polylines, renderable::Renderable},
  global::GlobalCtx,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Rider {
  pub horse: Horse,
  pub warrior: Human,
}

impl Rider {
  pub fn init<R: Rng>(
    _rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    mainclr: usize,
    blazonclr: usize,
    decorationratio: f32,
    foot_offset: f32,
    warrior: Human,
  ) -> Self {
    let horse = Horse::init(
      origin,
      size,
      angle,
      xflip,
      mainclr,
      blazonclr,
      decorationratio,
      foot_offset,
    );

    Self { warrior, horse }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    mask: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let warrior = &self.warrior;
    let horse = &self.horse;

    let mut routes = vec![];

    routes.extend(warrior.render_foreground_only(rng, mask));
    routes.extend(horse.render(rng, mask));
    routes.extend(warrior.render_background_only(rng, mask));

    // add halo around
    for (_, route) in routes.iter() {
      mask.paint_polyline(route, 1.0);
    }
    routes
  }
}

impl<R: Rng> Renderable<R> for Rider {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(rng, paint)
  }

  fn zorder(&self) -> f32 {
    self.horse.origin.1
  }
}
