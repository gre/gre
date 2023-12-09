use std::f64::consts::PI;

use super::{CastleGrounding, Mountain};
use crate::{
  algo::{
    math1d::mix,
    math2d::{distance_angles, euclidian_dist},
  },
  global::{GlobalCtx, Special},
  objects::{
    army::human::{HeadShape, HoldableObject},
    blazon::Blazon,
  },
};
use noise::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone, Copy, PartialEq)]
pub struct HumanProps {
  pub proximity: f32, // multiplicator that controls density units can be
  pub oriented_left: bool,
  pub on_horse: bool,
  pub headshape: HeadShape,
  pub leftobj: Option<HoldableObject>,
  pub rightobj: Option<HoldableObject>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CastleSiegeMachineProps {
  pub oriented_left: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub struct DistantSiegeMachineProps {
  pub oriented_left: bool,
  pub action_percent: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CannonProps {
  pub oriented_left: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Area {
  Defender(HumanProps),
  Empty,
  Animal,
  Tree(f32, f32),
  Attacker(HumanProps),
  DirectionalSiegeMachine(CannonProps),
  CastleSiegeMachine(CastleSiegeMachineProps),
  DistantSiegeMachine(DistantSiegeMachineProps), // spawn machines + their humans
  Cyclope,
  Hut,
  Firecamp,
  // Relic?
}

pub struct BattlefieldArea {
  cells: Vec<Area>,
  precision: f32,
  wi: usize,
  hi: usize,
  // + determine paths of travels?
  // + determine the palisade?
}

impl BattlefieldArea {
  pub fn rand<R: Rng>(
    rng: &mut R,
    ctx: &GlobalCtx,
    width: f32,
    height: f32,
    yhorizon: f32,
    mountains: &Vec<Mountain>,
  ) -> Self {
    let precision = 2.0;
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    let mut cells = vec![Area::Empty; wi * hi];

    let castle_grounding = mountains.iter().find_map(|m| m.castle.clone());

    let last_mountain = mountains.iter().last();

    let castle_center = castle_grounding
      .clone()
      .map(|c: CastleGrounding| c.position)
      .unwrap_or_else(|| {
        last_mountain
          .map(|m| m.ridge_pos_for_x(width / 2.))
          .unwrap_or((width / 2., yhorizon))
      });
    let estimated_h = castle_grounding.map(|c| c.width * 0.3).unwrap_or(0.);
    let castle_center = (castle_center.0, castle_center.1 - estimated_h / 2.0);
    let dscale = (castle_center.1 - yhorizon).abs() as f64;

    let has_cyclope = ctx.specials.contains(&Special::Cyclopes);
    let no_attackers = ctx.specials.contains(&Special::TrojanHorse);

    let perlin = Perlin::new(rng.gen());

    let scaleref = (width / 210.0) as f64;

    let global_dist_delta = rng.gen_range(-20.0..20.0);
    let noise1_dist_amp = rng.gen_range(5.0..30.0);

    let attacker_cutoff = rng.gen_range(-0.5..0.5);

    let range_defenders = 0.0..rng.gen_range(0.0..0.2);
    let range_attackers = rng.gen_range(0.1..0.3)
      ..0.3
        + rng.gen_range(-1.0f64..1.0).max(0.0)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0);
    let range_archers =
      rng.gen_range(-0.05..0.0) + 0.5..0.55 + rng.gen_range(0.0..0.1);
    let range_distance_machines =
      rng.gen_range(-0.2..0.0) + 0.8..0.9 + rng.gen_range(0.0..0.2);

    let grid1f = rng.gen_range(3.0..10.0);
    let gridoffx = rng.gen_range(0.0..1.0);
    let gridoffy = rng.gen_range(0.0..1.0);
    let unitmod = (rng.gen_range(2..8), rng.gen_range(2..8));
    let grid1 = |x: f64, y: f64| {
      let xp = gridoffx + x as f32 / width;
      let yp = 2.0 * (gridoffy + y as f32 / width);
      (
        (xp * grid1f).floor() as usize,
        (yp * grid1f).floor() as usize,
      )
    };

    // TODO shield formation case
    // TODO more crazy cases

    let s0 = rng.gen();
    let s1 = rng.gen();
    let s2 = rng.gen();
    let s3 = rng.gen();
    let s4 = rng.gen();
    let s5 = rng.gen();
    let s6 = rng.gen();
    let s7 = rng.gen();
    let s8 = rng.gen();
    let s9 = rng.gen();

    let oscillation_freq = rng.gen_range(-0.2..(PI * 1.2)).max(0.0).min(PI);
    let oscillation_phase =
      rng.gen_range(0.0..PI) * rng.gen_range(-0.2f64..1.0).max(0.0);
    let oscillation_amp = rng.gen_range(20.0..100.0) * rng.gen_range(0.0..1.0);

    for xi in 0..wi {
      for yi in 0..hi {
        let i = xi + yi * wi;
        let x = xi as f32 * precision;
        let y = yi as f32 * precision;

        let xratio = x / width;

        let mut mountain_index = 0;
        let mut lastyridge = yhorizon;
        let mut nextyridge = yhorizon;
        loop {
          if mountain_index == mountains.len() {
            if mountain_index > 0 {
              mountain_index -= 1;
            }
            break;
          }
          let mountain = &mountains[mountain_index];
          nextyridge = mountain.ridge_pos_for_x(x).1;
          if y > nextyridge {
            break;
          }
          mountain_index += 1;
          lastyridge = nextyridge;
        }
        let yratio = if lastyridge == nextyridge {
          0.5
        } else {
          ((y - lastyridge) / (nextyridge - lastyridge)) as f64
        };

        let mountainxdiff = oscillation_amp
          * (yratio * oscillation_freq + oscillation_phase).sin()
          * ((mountain_index % 2) as f64 - 0.5);
        let xf = (x as f64 + mountainxdiff) * scaleref;
        let yf = y as f64 * scaleref;

        let grid1p = grid1(xf, yf);

        let xmulratio = 0.5;
        let noise800 = perlin.get([xmulratio * xf / 800., yf / 800., s0]);
        let noise400 = perlin.get([xmulratio * xf / 400., yf / 400., s1]);
        let noise200 = perlin.get([xmulratio * xf / 200., yf / 200., s2]);
        let noise100 = perlin.get([xmulratio * xf / 100., yf / 100., s3]);
        let noise50 = perlin.get([xmulratio * xf / 50., yf / 50., s4]);
        let noise25 = perlin.get([xmulratio * xf / 25., yf / 25., s5]);
        let noise12 = perlin.get([xmulratio * xf / 12., yf / 12., s6]);
        let noise6 = perlin.get([xmulratio * xf / 6., yf / 6., s7]);
        let noise100stretched =
          perlin.get([xmulratio * xf / 100., yf / 50., s8]);
        let noise100alt = perlin.get([xmulratio * xf / 100., yf / 100., s9]);

        let cdx = x - castle_center.0;
        let cdy = y - castle_center.1;
        let castle_angle = cdy.atan2(cdx);
        let deucl = euclidian_dist((x, y), castle_center);
        let dcastle = mix(deucl, cdy.abs(), rng.gen_range(0.0..1.0)) as f64;
        let sum =
          (global_dist_delta + noise1_dist_amp * noise100stretched + dcastle)
            / scaleref;

        let oriented_left = mix(noise100 as f32, x / 210.0 - 0.5, 0.3) > 0.0;

        let r = sum / dscale;

        let mut candidates = vec![];
        if range_defenders.contains(&r) {
          candidates.push(Area::Defender(HumanProps {
            proximity: 1.0,
            oriented_left,
            on_horse: rng.gen(),
            headshape: HeadShape::HELMET,
            leftobj: Some(HoldableObject::Shield),
            rightobj: Some(HoldableObject::Sword),
          }));
        }

        if range_attackers.contains(&r) {
          let mut rightobj = match ctx.attackers {
            Blazon::Lys => Some(HoldableObject::Sword),
            Blazon::Falcon => Some(HoldableObject::Club),
            Blazon::Dragon => Some(HoldableObject::Axe),
          };
          if rng.gen_bool(0.05) {
            rightobj = Some(HoldableObject::Flag);
          }
          candidates.push(Area::Attacker(HumanProps {
            proximity: 1.0,
            oriented_left,
            on_horse: noise12 > 0.0,
            headshape: HeadShape::HELMET,
            leftobj: Some(HoldableObject::Shield),
            rightobj,
          }));
        }

        if range_archers.contains(&r) {
          let mut rightobj =
            Some(HoldableObject::LongBow(rng.gen_range(0.0..1.0)));
          if rng.gen_bool(0.05) {
            rightobj = Some(HoldableObject::Flag);
          }
          candidates.push(Area::Attacker(HumanProps {
            proximity: rng.gen_range(0.5..0.8),
            oriented_left,
            on_horse: false,
            headshape: HeadShape::HELMET,
            leftobj: None,
            rightobj,
          }));
        }
        if range_distance_machines.contains(&r) {
          candidates.push(Area::DistantSiegeMachine(
            DistantSiegeMachineProps {
              oriented_left,
              action_percent: rng.gen_range(0.0..1.0),
            },
          ));
        }

        let mut area = Area::Empty;
        if !candidates.is_empty() {
          area = candidates[rng.gen_range(0..candidates.len())];
        }

        if noise200 < -0.2 && noise50 > 0.0 && dcastle < 20.0 {
          area = Area::CastleSiegeMachine(CastleSiegeMachineProps {
            oriented_left: rng.gen(),
          });
        }

        if noise200 > 0.0
          && noise50 < -0.2
          && matches!(area, Area::Attacker(_))
          && (0.1..0.9).contains(&xratio)
        {
          let accepted_dist = 0.2;
          let climb_angle = 0.1;
          for (a, oriented_left) in
            vec![(climb_angle, false), (PI - climb_angle, true)]
          {
            if distance_angles(a as f32, castle_angle) < accepted_dist {
              area =
                Area::DirectionalSiegeMachine(CannonProps { oriented_left });
            }
          }
        }

        // spawn organized units
        if grid1p.0 % unitmod.0 == 0 && grid1p.1 % unitmod.1 == 0 {
          area = Area::Attacker(HumanProps {
            proximity: 0.25,
            oriented_left,
            on_horse: false,
            headshape: HeadShape::HELMET,
            leftobj: Some(HoldableObject::Shield),
            rightobj: if rng.gen_bool(0.05) {
              Some(HoldableObject::Flag)
            } else {
              Some(HoldableObject::Sword)
            },
          });
        }

        if no_attackers
          && (matches!(area, Area::Attacker(_))
            || matches!(area, Area::DistantSiegeMachine(_))
            || matches!(area, Area::CastleSiegeMachine(_))
            || matches!(area, Area::DirectionalSiegeMachine(_)))
        {
          area = Area::Defender(HumanProps {
            proximity: 2.0,
            oriented_left,
            on_horse: noise12 > 0.0,
            headshape: HeadShape::NAKED,
            leftobj: None,
            rightobj: None,
          });
        }

        if area == Area::Empty && noise50 > 0.3 && noise6 > 0.49 {
          area = Area::Animal;
        }

        if matches!(area, Area::Attacker(_)) && noise50 < attacker_cutoff {
          area = Area::Empty;
        }

        // TODO fire in middle of firecamp with people around it?
        if sum > 0.8 * dscale && noise100alt > 0.0 && noise25 > 0.4 {
          area = Area::Hut;
        }
        if area == Area::Hut
          && noise100alt > 0.495
          && (0.1..0.9).contains(&xratio)
        {
          area = Area::Firecamp;
        }

        if noise200 > 0.3 && noise800 > 0.3 {
          let foliage_ratio =
            (0.5 + 0.6 * (noise400 + noise12)).max(0.4).min(0.8) as f32;
          let bush_width_ratio = mix(0.3, 1.0, 0.5 + 0.5 * noise25 as f32);
          area = Area::Tree(foliage_ratio, bush_width_ratio);
        }

        if has_cyclope
          && noise200 > 0.0
          && noise50 > 0.2
          && matches!(area, Area::Empty)
          && (0.12..0.88).contains(&xratio)
        {
          area = Area::Cyclope;
        }

        /*
        area = if (xf / 210. - 0.5).abs() < 0.05 {
          let foliage_ratio =
            (0.5 + 0.6 * (noise400 + noise12)).max(0.4).min(0.8) as f32;
          let bush_width_ratio = mix(0.3, 1.0, 0.5 + 0.5 * noise25 as f32);
          Area::Tree(foliage_ratio, bush_width_ratio)
        } else {
          Area::Empty
        };
        */

        cells[i] = area;
      }
    }

    Self {
      cells,
      precision,
      wi,
      hi,
    }
  }

  pub fn get(&self, x: f32, y: f32) -> Area {
    let x = (x / self.precision).max(0.) as usize;
    let y = (y / self.precision).max(0.) as usize;
    if x < self.wi && y < self.hi {
      self.cells[x + y * self.wi]
    } else {
      Area::Empty
    }
  }
}
