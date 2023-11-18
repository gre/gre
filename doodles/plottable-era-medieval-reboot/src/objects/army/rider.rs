use crate::{algo::paintmask::PaintMask, objects::blazon::traits::Blazon};
use rand::prelude::*;
use std::f32::consts::PI;

use super::{horse::Horse, sword::Sword, warrior::Warrior};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Rider {
  pub horse: Horse,
  pub warrior: Warrior,
  pub sword: Sword,
}

impl Rider {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    blazon: Blazon,
    mainclr: usize,
    blazonclr: usize,
    decorationratio: f32,
    foot_offset: f32,
  ) -> Self {
    let warrior = Warrior::init(
      rng, origin, size, angle, xflip, blazon, mainclr, blazonclr, true,
    );

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

    let xdir = if xflip { -1.0 } else { 1.0 };
    let swordang = PI / 2.0 - xdir * rng.gen_range(0.0..2.0);
    let sword = Sword::init(warrior.human.elbow_left, size, swordang, mainclr);

    Self {
      warrior,
      horse,
      sword,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    mask: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let warrior = &self.warrior;
    let sword = &self.sword;
    let horse = &self.horse;

    let mut routes = vec![];

    routes.extend(warrior.render_foreground_only(mask));
    routes.extend(sword.render(rng, mask));
    routes.extend(horse.render(rng, mask));
    routes.extend(warrior.render_background_only(mask));

    // add halo around
    for (_, route) in routes.iter() {
      mask.paint_polyline(route, 1.0);
    }
    routes
  }
}
