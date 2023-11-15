use core::arch;

use self::{
  archer::bowman, belfry::belfry, horseman::horse_with_rider,
  trebuchet::trebuchet,
};
use crate::algo::{
  math1d::mix,
  math2d::{euclidian_dist, lookup_ridge},
  paintmask::PaintMask,
};
use rand::prelude::*;

use super::{blazon::traits::Blazon, castle, mountains::Mountain};

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
pub mod shield;
pub mod spear;
pub mod sword;
pub mod trebuchet;

// we could use this multiple times if we have multiple mountains
pub struct ArmyOnMountain {
  // kind of army
  pub house: Blazon,
}

impl ArmyOnMountain {
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    mountain: &Mountain,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let riders_count = rng.gen_range(0..20);
    let archers_count = rng.gen_range(0..50);

    let ridge = mountain.ridge.clone();
    let width = mountain.width;
    let yhorizon = mountain.yhorizon;

    let mut routes = vec![];

    let first = ridge[0];
    let last = ridge[ridge.len() - 1];
    let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
    let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..1.0));
    if y < yhorizon {
      let too_close_to_castle = if let Some(castle) = &mountain.castle {
        euclidian_dist((x, y), castle.position) < 0.2 * castle.width
      } else {
        false
      };
      if !too_close_to_castle {
        let origin = (x, y);
        let height = 0.1 * width;
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
      }
    }

    if let Some(_castle) = &mountain.castle {
      let first = ridge[0];
      let last = ridge[ridge.len() - 1];
      let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
      let y = lookup_ridge(&ridge, x);
      if y < yhorizon {
        let origin = (x, y);
        let height = 0.1 * width;
        let clr = 0;
        let bridge_width = rng.gen_range(0.3..0.6) * height;
        let bridge_opening = rng.gen_range(0.0f32..1.5).min(1.0);
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
      }
    }

    for _i in 0..riders_count {
      let angle = 0.0;
      let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
      let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..1.0));
      if y < yhorizon {
        let origin = (x, y);
        let size = 0.04 * width;
        let mainclr = 0;
        let skinclr = 0;
        let xflip = rng.gen_bool(0.5);
        let is_leader = rng.gen_bool(0.1);
        routes.extend(horse_with_rider(
          rng, paint, origin, angle, size, xflip, mainclr, skinclr, is_leader,
        ));
      }
    }

    let clr = 0;
    for _i in 0..archers_count {
      let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
      let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..1.0));
      if y < yhorizon {
        let origin = (x, y);
        let size = 0.04 * width;
        routes.extend(bowman(rng, paint, clr, origin, size));
      }
    }

    // TODO we also are going to spawn some projectiles.

    routes
  }
}
