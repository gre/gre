use std::f32::consts::PI;

use self::{
  belfry::belfry,
  belier::Belier,
  body::HumanPosture,
  car4l::Renault4L,
  convoywalk::ConvoyWalk,
  flag::Flag,
  human::{HeadShape, HoldableObject, Human},
  hut::Hut,
  monk::Monk,
  relic::Relic,
  rider::Rider,
  trebuchet::Trebuchet,
  tunnelstructure::TunnelStructure,
};
use super::{
  blazon::Blazon,
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
pub mod arrow;
pub mod axe;
pub mod belfry;
pub mod belier;
pub mod belierhead;
pub mod boat;
pub mod boatarmy;
pub mod body;
pub mod cage;
pub mod car4l;
pub mod catapult;
pub mod club;
pub mod convoywalk;
pub mod dragonhead;
pub mod flag;
pub mod flyingdragon;
pub mod head;
pub mod helmet;
pub mod horse;
pub mod human;
pub mod hut;
pub mod longbow;
pub mod monk;
pub mod paddle;
pub mod relic;
pub mod rider;
pub mod shield;
pub mod spear;
pub mod sword;
pub mod trebuchet;
pub mod trojanhorse;
pub mod tunnelstructure;
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
    index: usize,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let ridge = mountain.ridge.clone();
    let first = ridge[0];
    let last = ridge[ridge.len() - 1];
    let width = mountain.width;
    let yhorizon = mountain.yhorizon;
    let blazon = self.blazon;
    let mut routes = vec![];

    let mainclr = 0;
    let blazonclr = 2;

    let mut noarmy = false;

    if ctx.specials.contains(&Special::TrojanHorse) {
      noarmy = true;
    }

    if ctx.specials.contains(&Special::Montmirail) && index == 0 {
      let x = mix(first.0, last.0, rng.gen_range(0.2..0.8));
      let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..0.7));
      let origin = (x, y);
      let size = rng.gen_range(0.05..0.1) * width;
      let car = Renault4L::init(rng, 1, origin, size, 0.0);
      let leftobj = None;
      let rightobj = Some(HoldableObject::Club);
      let posture = HumanPosture::from_holding(rng, false, leftobj, rightobj);
      let jacquouille = Human::init(
        rng,
        (x - 1.5 * size, y),
        0.8 * size,
        0.0,
        false,
        blazon,
        mainclr,
        blazonclr,
        posture,
        HeadShape::NAKED,
        leftobj,
        rightobj,
      );
      routes.extend(jacquouille.render(paint));
      let rightobj = Some(HoldableObject::Sword);
      let posture = HumanPosture::from_holding(rng, false, leftobj, rightobj);
      let godefroy = Human::init(
        rng,
        (x - 1.2 * size, y),
        size,
        0.0,
        false,
        blazon,
        mainclr,
        blazonclr,
        posture,
        HeadShape::HELMET,
        None,
        Some(HoldableObject::Sword),
      );
      routes.extend(godefroy.render(paint));
      routes.extend(car.render(paint));
      noarmy = true;
    }

    if noarmy {
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
          let rts = tree.render(paint);
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
            let triangle_structure = rng.gen_bool(0.7);
            let s = height;
            let o = (x, y - 0.3 * height);
            let machine =
              Belier::init(rng, clr, o, s, 0.0, xflip, triangle_structure);
            routes.extend(machine.render(paint));
            // TODO army of warriors
          } else if rng.gen_bool(0.5) {
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
          let xflip = x > castle.position.0;
          let headshape = HeadShape::HELMET;
          let leftobj = Some(HoldableObject::Shield);
          let rightobj = Some(HoldableObject::LongSword);
          let posture =
            HumanPosture::from_holding(rng, false, leftobj, rightobj);
          let warrior = Human::init(
            rng, origin, size, angle, xflip, blazon, mainclr, blazonclr,
            posture, headshape, leftobj, rightobj,
          );

          let decorationratio = 0.3;
          let foot_offset = 1.0;
          let rider = Rider::init(
            rng,
            origin,
            size,
            angle,
            xflip,
            mainclr,
            blazonclr,
            decorationratio,
            foot_offset,
            warrior,
          );
          routes.extend(rider.render(rng, paint));

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
          let headshape = HeadShape::HELMET;
          let leftobj = Some(HoldableObject::Shield);
          let mut rightobj = match blazon {
            Blazon::Lys => Some(HoldableObject::Sword),
            Blazon::Falcon => Some(HoldableObject::Club),
            Blazon::Dragon => Some(HoldableObject::Axe),
          };
          if rng.gen_bool(0.1) {
            rightobj = Some(HoldableObject::Flag);
          }
          let posture =
            HumanPosture::from_holding(rng, false, leftobj, rightobj);
          let warrior = Human::init(
            rng, origin, size, 0.0, xflip, blazon, clr, blazonclr, posture,
            headshape, leftobj, rightobj,
          );

          // todo in one case it is riding a horse or it's not.

          routes.extend(warrior.render(paint));

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
          let phase = rng.gen_range(0.0..1.0);
          let headshape = HeadShape::NAKED;
          let leftobj = None;
          let rightobj = Some(HoldableObject::LongBow(phase));
          let posture =
            HumanPosture::from_holding(rng, xflip, leftobj, rightobj);
          let human = Human::init(
            rng, origin, size, 0.0, xflip, blazon, clr, blazonclr, posture,
            headshape, leftobj, rightobj,
          );
          routes.extend(human.render(paint));

          let area = VCircle::new(x, y - 0.3 * size, 0.5 * size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }
      }

      // huts

      // TODO sort by y order (as well as all other things)
      // TODO better spawning
      for _ in 0..rng.gen_range(0..4) {
        let x = mix(first.0, last.0, rng.gen_range(0.1..0.9));
        let y = mix(lookup_ridge(&ridge, x), yhorizon, rng.gen_range(0.0..1.0));
        let size = rng.gen_range(0.05..0.08) * width;
        let angle = 0.0;
        let hut = Hut::init(rng, mainclr, (x, y), size, angle);
        routes.extend(hut.render(paint));

        let flagtoright = rng.gen_bool(0.5);
        let cloth_height_factor = rng.gen_range(0.2..0.5);
        let cloth_len_factor = rng.gen_range(0.3..0.8);
        let dy = rng.gen_range(0.7..0.8) * size;
        let flag = Flag::init(
          rng,
          mainclr,
          blazonclr,
          (x, y - dy),
          size,
          angle - PI / 2.0,
          flagtoright,
          cloth_height_factor,
          cloth_len_factor,
          false,
        );
        let rt = flag.render(paint);
        for (_, r) in rt.iter() {
          paint.paint_polyline(&r, 0.8);
        }
        routes.extend(rt);
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
