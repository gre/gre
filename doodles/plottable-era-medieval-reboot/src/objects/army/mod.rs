use self::{
  archer::bowman, belfry::belfry, horseman::horse_with_rider,
  trebuchet::trebuchet, warrior::warrior,
};
use crate::algo::{
  math1d::mix,
  math2d::{euclidian_dist, lookup_ridge},
  packing::VCircle,
  paintmask::PaintMask,
  shapes::circle_route,
};
use noise::*;
use rand::prelude::*;

use super::{
  blazon::traits::Blazon,
  castle,
  mountains::{Mountain, MountainsV2},
};

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
pub mod warrior;

// we could use this multiple times if we have multiple mountains
pub struct ArmyOnMountain {
  pub debug: bool,
  // kind of army
  pub house: Blazon,
  // TODO idea: we could split the area in 2 parts for attackers defenders.

  // TODO we need to build a camp area for the attackers. or maybe it's in the front mountain?
  // if the castle is in one side it make sense it would be on other side
}

impl ArmyOnMountain {
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    mountain: &Mountain,
    mountains: &MountainsV2,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let house = self.house;
    let mut routes = vec![];
    let perlin = Perlin::new(rng.gen());

    let norm = paint.width as f64;
    let fnoise = |x: f32, y: f32| -> f32 {
      let x = x as f64 / norm;
      let y = y as f64 / norm;
      perlin.get([2.0 * x, 2.0 * y]) as f32
    };

    let archer_noise_range = 0.1f32..0.5;
    let warriors_noise_range = -0.2f32..0.2;
    let riders_noise_range: std::ops::Range<f32> = -0.5f32..-0.1;

    let riders_sampling =
      (rng.gen_range(0.0f64..100.) * rng.gen_range(0.0..1.0)) as usize;
    let warriors_sampling =
      (rng.gen_range(0.0f64..200.) * rng.gen_range(0.0..1.0)) as usize;
    let archers_sampling =
      (rng.gen_range(0.0f64..500.) * rng.gen_range(0.0..1.0)) as usize;

    // TODO orient the bodies depending on the terrain

    // we track a bunch of circle to avoid spawning people too close to each other
    let mut debug_circle: Vec<VCircle> = vec![];
    let mut exclusion_mask = paint.clone_empty();

    let first_castle = mountains.mountains.iter().find_map(|m| m.castle);

    if let Some(castle) = first_castle {
      let trebuchet_tries = (rng.gen_range(-4.0f32..4.0)
        * rng.gen_range(0.0..1.0))
      .max(0.0) as usize;
      let ridge = mountain.ridge.clone();
      let width = mountain.width;
      let yhorizon = mountain.yhorizon;

      let first = ridge[0];
      let last = ridge[ridge.len() - 1];

      for _t in 0..trebuchet_tries {
        let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
        let y = mix(
          lookup_ridge(&ridge, x),
          yhorizon,
          rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
        );
        let origin = (x, y);
        if y < yhorizon && !exclusion_mask.is_painted(origin) {
          let too_close_to_castle =
            euclidian_dist((x, y), castle.position) < 0.4 * castle.width;
          let dist_to_border = x.min(width - x);
          if !too_close_to_castle && dist_to_border > 0.1 * width {
            let height = 0.1 * width;
            let action_percent = rng.gen_range(0.0..1.0);
            let xflip = x > castle.position.0;
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

            let area = VCircle::new(x, y - 0.3 * height, height);
            debug_circle.push(area);
            exclusion_mask.paint_circle(area.x, area.y, area.r);

            // TODO people managing the trebuchet
          }
        }
      }

      if let Some(castle) = &mountain.castle {
        let height = rng.gen_range(0.05..0.1) * width;
        let dx = castle.width / 2.0 + 0.3 * height;
        let (x, xflip) = if castle.position.0 < width / 2.0 {
          (castle.position.0 + dx, true)
        } else {
          (castle.position.0 - dx, false)
        };
        let y = lookup_ridge(&ridge, x);
        if y < yhorizon {
          let y1 = lookup_ridge(&ridge, x + 0.2 * height);
          let y2 = lookup_ridge(&ridge, x - 0.2 * height);
          let y = y.max(y1).max(y2);
          let origin = (x, y);
          let clr = 0;
          let bridge_width = rng.gen_range(0.3..0.6) * height;
          let bridge_opening = rng.gen_range(0.0f32..2.0).min(1.0);
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

          // TODO army of warriors behind the belfry
        }
      }

      for _i in 0..riders_sampling {
        let angle = 0.0;
        let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
        let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..0.5));
        if y < yhorizon
          && !exclusion_mask.is_painted((x, y))
          && riders_noise_range.contains(&fnoise(x, y))
        {
          let origin = (x, y);
          let size = 0.04 * width;
          let mainclr = 0;
          let skinclr = 0;
          let xflip = x > castle.position.0;
          let is_leader = rng.gen_bool(0.1);
          routes.extend(horse_with_rider(
            rng, paint, origin, angle, size, xflip, mainclr, skinclr,
            is_leader, house,
          ));

          let area = VCircle::new(x, y - 0.3 * size, 0.5 * size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }
      }

      let clr = 0;
      for _i in 0..warriors_sampling {
        let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
        let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..1.0));
        if y < yhorizon
          && !exclusion_mask.is_painted((x, y))
          && warriors_noise_range.contains(&fnoise(x, y))
        {
          let origin = (x, y);
          let size = 0.04 * width;
          let xflip = x > castle.position.0;
          routes.extend(warrior(rng, paint, clr, origin, 0.0, size, xflip));

          let area = VCircle::new(x, y - 0.3 * size, 0.5 * size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }
      }

      let clr = 0;
      for _i in 0..archers_sampling {
        let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
        let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..1.0));
        if y < yhorizon
          && !exclusion_mask.is_painted((x, y))
          && archer_noise_range.contains(&fnoise(x, y))
        {
          let origin = (x, y);
          let size = 0.04 * width;
          let xflip = x > castle.position.0;
          routes.extend(bowman(rng, paint, clr, origin, size, xflip));

          let area = VCircle::new(x, y - 0.3 * size, 0.5 * size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }
      }
    }
    // TODO we also are going to spawn some projectiles.

    if self.debug {
      for c in debug_circle.iter() {
        routes.push((2, circle_route((c.x, c.y), c.r, 64)));
      }
    }

    routes
  }
}
