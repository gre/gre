use self::{
  archer::bowman, belfry::belfry, convoywalk::ConvoyWalk, monk::Monk,
  rider::Rider, swordwarrior::SwordWarrior, trebuchet::Trebuchet,
  tunnelstructure::TunnelStructure,
};
use super::{
  blazon::Blazon,
  castle::relic::Relic,
  mountains::{Mountain, MountainsV2},
  tree::Tree,
};
use crate::{
  algo::{
    math1d::mix,
    math2d::{euclidian_dist, lookup_ridge},
    packing::VCircle,
    paintmask::PaintMask,
    shapes::circle_route,
  },
  global::{GlobalCtx, Special},
};
use noise::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub mod archer;
pub mod arrow;
pub mod axe;
pub mod belfry;
pub mod boat;
pub mod boatarmy;
pub mod body;
pub mod catapult;
pub mod convoywalk;
pub mod flag;
pub mod head;
pub mod helmet;
pub mod horse;
pub mod monk;
pub mod rider;
pub mod shield;
pub mod spear;
pub mod sword;
pub mod swordwarrior;
pub mod trebuchet;
pub mod trojanhorse;
pub mod tunnelstructure;
pub mod warrior;
pub mod wheeledplatform;

// we could use this multiple times if we have multiple mountains
pub struct ArmyOnMountain {
  pub debug: bool,
  pub blazon: Blazon,
  // TODO idea: we could split the area in 2 parts for attackers defenders.

  // TODO we need to build a camp area for the attackers. or maybe it's in the front mountain?
  // if the castle is in one side it make sense it would be on other side
}

impl ArmyOnMountain {
  pub fn init(blazon: Blazon) -> Self {
    Self {
      debug: false,
      blazon,
    }
  }

  pub fn render<R: Rng>(
    &self,
    ctx: &mut GlobalCtx,
    rng: &mut R,
    paint: &mut PaintMask,
    mountain: &Mountain,
    mountains: &MountainsV2,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let blazon = self.blazon;
    let mut routes = vec![];

    let mainclr = 0;
    let blazonclr = 2;

    if ctx.specials.contains(&Special::TrojanHorse) {
      return routes;
    }

    let perlin = Perlin::new(rng.gen());

    let norm = paint.width as f64;
    let fnoise = |x: f32, y: f32| -> f32 {
      let x = x as f64 / norm;
      let y = y as f64 / norm;
      perlin.get([2.0 * x, 2.0 * y]) as f32
    };

    let fnoisetree = |x: f32, y: f32| -> f32 {
      let x = x as f64 / norm;
      let y = y as f64 / norm;
      perlin.get([5.0, 0.4 * x, 0.4 * y]) as f32
    };

    // TODO intersection of two noises to make more collocated areas?

    let archer_noise_range = 0.1f32..0.5;
    let warriors_noise_range = -0.2f32..0.2;
    let riders_noise_range: std::ops::Range<f32> = -0.5f32..-0.1;

    let riders_sampling =
      (rng.gen_range(0.0f32..100.) * rng.gen_range(0.0..1.0)) as usize;
    let warriors_sampling =
      (rng.gen_range(0.0f32..200.) * rng.gen_range(0.0..1.0)) as usize;
    let archers_sampling =
      (rng.gen_range(0.0f32..500.) * rng.gen_range(0.0..1.0)) as usize;

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

      let trees_range: std::ops::Range<f32> = 0.35..0.5;
      let trees_sampling = if rng.gen_bool(0.3) {
        rng.gen_range(10..250)
      } else {
        0
      };
      let clr = 0;
      let foliage_ratio =
        0.5 + rng.gen_range(0.0..0.5) * rng.gen_range(0.0..0.5);
      let bush_width_ratio = mix(foliage_ratio, 0.8, 0.5);
      let mut tree_rts = vec![];
      for _i in 0..trees_sampling {
        let trunk_fill_each = rng.gen_range(1.0..10.0);
        let x = mix(first.0, last.0, rng.gen_range(0.0..1.0));
        let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..1.0));
        if y < yhorizon - 0.05 * paint.height // away enough from the beach
          && trees_range.contains(&fnoisetree(x, y))
        {
          let origin = (x, y);
          let size = rng.gen_range(0.07..0.13) * width;
          let tree = Tree::init(
            rng,
            origin,
            size,
            clr,
            foliage_ratio,
            bush_width_ratio,
            trunk_fill_each,
          );
          let rts = tree.render(rng, paint);
          tree_rts.extend(rts);

          let area = VCircle::new(x, y - 0.3 * size, 0.5 * size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }
      }
      // small halo around forest
      for (_, rt) in tree_rts.iter() {
        paint.paint_polyline(rt, 1.0);
      }
      routes.extend(tree_rts);

      // relic convoy
      if rng.gen_bool(0.05) {
        let x = mix(first.0, last.0, rng.gen_range(0.0..1.0));
        let y = lookup_ridge(&ridge, x);
        let relicsize = rng.gen_range(0.05..0.07) * width;
        let y = y - relicsize * 0.2;
        let ang = 0.0;
        let filling = 2.0;

        let extraratio = rng.gen_range(0.3..0.6);

        let convoy =
          ConvoyWalk::init(rng, (x, y), relicsize, ang, 1.0, extraratio);

        let mut monks = vec![];

        // TODO on top of these positions, add even more monks around the relic...
        for p in vec![convoy.left, convoy.right] {
          let p = (p.0, p.1 + relicsize * 0.4);
          let monk = Monk::init(rng, p, relicsize, 0.0, false, 0, true);
          monks.push(monk);
        }

        let relic = Relic::init(rng, (x, y), relicsize, ang, filling);

        let area = VCircle::new(x, y, 4.0 * relicsize);
        debug_circle.push(area);
        exclusion_mask.paint_circle(area.x, area.y, area.r);

        let mut rts = vec![];
        rts.extend(convoy.render(paint));
        rts.extend(relic.render(paint));
        for monk in monks {
          rts.extend(monk.render(paint));
        }
        // halo
        for (_, rt) in rts.iter() {
          paint.paint_polyline(rt, 1.0);
        }
        routes.extend(rts);
      }

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
            let trebuchet =
              Trebuchet::init(rng, origin, height, action_percent, xflip, clr);
            routes.extend(trebuchet.render(paint));

            let area = VCircle::new(x, y - 0.3 * height, height);
            debug_circle.push(area);
            exclusion_mask.paint_circle(area.x, area.y, area.r);

            // TODO people managing the trebuchet
          }
        }
      }

      if let Some(castle) = &mountain.castle {
        // we will try to find a spot for an attacking machine
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

          if rng.gen_bool(0.5) {
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
          } else {
            let h = width * 0.05;
            let w = width * rng.gen_range(0.1..0.25);
            let machine = TunnelStructure::init(rng, clr, origin, 0.0, h, w);
            routes.extend(machine.render(paint));
          }

          // TODO army of warriors behind the belfry
        }
      }

      // TODO we could figure out a global zone on which the teams are going to spawn. based on a noise.

      // TODO regroup riders, warriors, archers in a single loop
      // and we will just look what (x,y) can be and have a "max count" for each category of items.
      // we could also organize some sort of density map if we want.

      // TODO angle to follow a bit the terrain.

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
          let decorationratio = 0.3;
          let xflip = x > castle.position.0;
          let foot_offset = 1.0;
          let rider = Rider::init(
            rng,
            origin,
            size,
            angle,
            xflip,
            blazon,
            mainclr,
            blazonclr,
            decorationratio,
            foot_offset,
          );

          routes.extend(rider.render(rng, paint));

          /*
          let is_leader = rng.gen_bool(0.1);
          routes.extend(horse_with_rider(
            rng, paint, origin, angle, size, xflip, mainclr, blazonclr,
            is_leader, blazon,
          ));
          */

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
          let swordwarrior = SwordWarrior::init(
            rng, origin, size, 0.0, xflip, blazon, clr, blazonclr,
          );
          routes.extend(swordwarrior.render(rng, paint));

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
