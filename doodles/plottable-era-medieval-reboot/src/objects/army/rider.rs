use super::{horse::Horse, warrior::Warrior};
use crate::{algo::paintmask::PaintMask, objects::blazon::Blazon};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Rider {
  pub horse: Horse,
  pub warrior: Warrior,
}

// TODO Warrior to be a param
impl Rider {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    mainclr: usize,
    blazonclr: usize,
    decorationratio: f32,
    foot_offset: f32,
    warrior: Warrior,
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

    routes.extend(warrior.render_foreground_only(mask));
    routes.extend(horse.render(rng, mask));
    routes.extend(warrior.render_background_only(mask));

    // add halo around
    for (_, route) in routes.iter() {
      mask.paint_polyline(route, 1.0);
    }
    routes
  }
}
