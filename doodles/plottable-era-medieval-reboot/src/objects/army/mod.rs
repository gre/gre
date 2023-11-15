use self::{
  archer::bowman, belfry::belfry, horseman::horse_with_rider,
  house::traits::House, trebuchet::trebuchet,
};
use crate::algo::{math1d::mix, paintmask::PaintMask};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub mod archer;
pub mod belfry;
pub mod boat;
pub mod body;
pub mod catapult;
pub mod head;
pub mod helmet;
pub mod horseman;
pub mod house;
pub mod shield;
pub mod spear;
pub mod sword;
pub mod trebuchet;

// we could use this multiple times if we have multiple mountains
pub struct ArmyOnMountain {
  // boundaries of where the army can be
  pub ridge: Vec<(f64, f64)>,
  pub yhorizon: f64,
  pub width: f64,
  // kind of army
  pub house: House,
}

impl ArmyOnMountain {
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];

    // TODO verify each of these correctly implement clipping...

    // hack because we're not placed correctly
    let paint = &mut paint.clone_empty();

    // TODO placement
    let ridge = self.ridge.clone();
    let first = ridge[0];
    let last = ridge[ridge.len() - 1];
    let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
    let y = self.yhorizon - 5.0; // todo lookup => need to put that lookup in algo
    let origin = (x, y);
    let height = 0.1 * self.width;
    let action_percent = rng.gen_range(0.0..1.0);
    let xflip = rng.gen_bool(0.5);
    let clr = 0;
    routes.extend(trebuchet(
      rng,
      paint,
      origin,
      height,
      action_percent,
      xflip,
      clr,
    ));

    // TODO placement
    let ridge = self.ridge.clone();
    let first = ridge[0];
    let last = ridge[ridge.len() - 1];
    let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
    let y = self.yhorizon - 5.0; // todo lookup => need to put that lookup in algo
    let origin = (x, y);
    let height = 0.1 * self.width;
    let clr = 0;
    let bridge_width = rng.gen_range(0.3..0.6) * height;
    let bridge_opening = rng.gen_range(0.0f64..1.5).min(1.0);
    let xflip = rng.gen_bool(0.5);
    routes.extend(belfry(
      rng,
      paint,
      clr,
      origin,
      height,
      bridge_width,
      bridge_opening,
      xflip,
    ));

    // TODO placement
    let angle = 0.0;
    let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
    let y = self.yhorizon - 5.0; // todo lookup => need to put that lookup in algo
    let origin = (x, y);
    let size = 0.04 * self.width;
    let mainclr = 0;
    let skinclr = 0;
    let xflip = rng.gen_bool(0.5);
    let is_leader = rng.gen_bool(0.1);
    routes.extend(horse_with_rider(
      rng, paint, origin, angle, size, xflip, mainclr, skinclr, is_leader,
    ));

    // TODO placement
    let clr = 0;
    for _i in 0..5 {
      let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
      let y = self.yhorizon - 5.0; // todo lookup => need to put that lookup in algo
      let origin = (x, y);
      let size = 0.04 * self.width;
      routes.extend(bowman(rng, paint, clr, origin, size));
    }

    routes
  }
}
