use self::{
  belfry::Belfry,
  belier::Belier,
  body::HumanPosture,
  cage::Cage,
  cannon::Cannon,
  car4l::Renault4L,
  catapult::Catapult,
  convoywalk::ConvoyWalk,
  firecamp::Firecamp,
  flag::Flag,
  horse::Horse,
  human::{HeadShape, HoldableObject, Human},
  hut::Hut,
  monk::Monk,
  relic::Relic,
  rider::Rider,
  trebuchet::Trebuchet,
  tunnelstructure::TunnelStructure,
};
use super::{
  animals::{armadillo::Armadillo, dog::Dog, fowl::Fowl},
  blazon::Blazon,
  mountains::{battlefield::Area, wall::MountainWall, Mountain, MountainsV2},
  projectile::attack::{AttackOrigin, DefenseTarget},
  tree::Tree,
};
use crate::{
  algo::{
    math1d::mix,
    math2d::lookup_ridge,
    moving_average::moving_average_2d,
    packing::VCircle,
    paintmask::PaintMask,
    pathlookup::PathLookup,
    renderable::{Container, Renderable},
    shapes::circle_route,
  },
  global::{GlobalCtx, Special},
};
use rand::prelude::*;
use std::f32::consts::PI;

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
pub mod cannon;
pub mod car4l;
pub mod catapult;
pub mod club;
pub mod convoywalk;
pub mod dragonhead;
pub mod fire;
pub mod firecamp;
pub mod flag;
pub mod flyingdragon;
pub mod head;
pub mod helmet;
pub mod horse;
pub mod human;
pub mod hut;
pub mod ladder;
pub mod longbow;
pub mod monk;
pub mod paddle;
pub mod relic;
pub mod rider;
pub mod rope;
pub mod shield;
pub mod spear;
pub mod sword;
pub mod torch;
pub mod trebuchet;
pub mod trojanhorse;
pub mod tunnelstructure;
pub mod wheeledplatform;
pub struct ArmyOnMountain {
  pub debug: bool,
  pub blazon: Blazon,
}

impl ArmyOnMountain {
  pub fn init(blazon: Blazon) -> Self {
    Self {
      // debug: true,
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
    let mut debug_circle: Vec<VCircle> = vec![];
    let mut exclusion_mask = paint.clone_empty_rescaled(1.0);

    let ridge = mountain.ridge.clone();
    let first = ridge[0];
    let last = ridge[ridge.len() - 1];
    let width = mountain.width;
    let yhorizon = mountain.yhorizon;
    let blazon = self.blazon;
    let mut renderables = Container::new();

    let mainclr = mountain.clr;
    let blazonclr = if mountain.is_behind { mountain.clr } else { 2 };

    if ctx.specials.contains(&Special::Montmirail) && index == 0 {
      montmirail(
        rng,
        &mut renderables,
        &ridge,
        yhorizon,
        width,
        blazon,
        mainclr,
        blazonclr,
      );
    }

    let ml = mountains.mountains.len();
    let prevridge = if index > 0 && ml > 2 {
      Some(mountains.mountains[index - 1].ridge.clone())
    } else {
      None
    };

    let has_wall = ctx.specials.contains(&Special::Barricades)
      || rng.gen_bool(0.5) && (ml < 2 || index == ml - 2);
    let xc = rng.gen_range(0.3..0.7);
    let w = rng.gen_range(0.3..1.0);
    let xfrom = (xc - w / 2.0) * width;
    let xto = (xc + w / 2.0) * width;
    let minw = 0.2 * width;
    let yfromp = rng.gen_range(0.1..0.9);
    let ytop = rng.gen_range(0.1..0.9);
    let path = mountain
      .ridge
      .iter()
      .enumerate()
      .filter_map(|(i, p)| {
        if !(xfrom < p.0 && p.0 < xto) {
          return None;
        }
        let bottomy = if let Some(prevridge) = &prevridge {
          prevridge[i].1
        } else {
          yhorizon
        };
        let m = mix(yfromp, ytop, (p.0 - xfrom) / (xto - xfrom));
        let y = mix(bottomy, p.1, m);
        if y < yhorizon {
          Some((p.0, y))
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

    if path.len() > 1 {
      if has_wall {
        if path.len() > 1 && (path[path.len() - 1].0 - path[0].0).abs() > minw {
          let path = moving_average_2d(&path, path.len() / 10);
          let baseh = rng.gen_range(0.05..0.08) * width;
          let wall =
            MountainWall::init(ctx, rng, blazonclr, mainclr, &path, baseh);
          renderables.extend(wall.container);
        }
      } else {
        // we use this path to make a convoy. but we need to check first if there are room for it.
        let len = path.len();
        let checks = (rng.gen_range(4..20)).min(len);
        let mut is_valid = true;

        for i in 0..checks {
          let j = i * (len / checks);
          let (x, y) = path[j];
          let cell = mountains.battlefield.get(x, y);
          if !matches!(cell, Area::Empty) {
            is_valid = false;
            break;
          }
        }

        if is_valid {
          let size = rng.gen_range(0.04..0.07) * width;
          make_random_convoy(
            rng,
            &mut renderables,
            &mut exclusion_mask,
            ctx.defenders,
            mainclr,
            ctx.defendersclr,
            size,
            &path,
          );
        }
      }
    }

    let yallowunder = width * 0.02;
    let clr = mainclr;

    let mut should_spawn_leader = mountain.will_have_the_leader;
    let mut remaining_trebuchets =
      (rng.gen_range(-4.0f32..4.0) * rng.gen_range(0.0..1.0)).max(0.0) as usize
        + if ctx.specials.contains(&Special::Trebuchets) {
          10
        } else {
          0
        };

    let catapult_instead_of_cannon_proba = rng.gen_range(0.0..1.0);
    let climb_attack_proba =
      rng.gen_range(-1.0f64..1.0).max(0.0001) * rng.gen_range(0.0..1.0);

    let should_skip = rng.gen_bool(0.05)
      || index % 2 == 1 && rng.gen_bool(mountains.skip_alt_factor);

    let sampling_skip_if_fails_more_than = 100;
    let mut fails = 0;
    let sampling = if should_skip { 1 } else { 2000 };
    for _ in 0..sampling {
      let xposfactor = rng.gen_range(0.0..1.0);
      let x = mix(first.0, last.0, xposfactor);
      let i = mountain.lookup_ridge_index(x);
      let bottomy = if let Some(prevridge) = &prevridge {
        (prevridge[i].1 + yallowunder).min(yhorizon)
      } else {
        yhorizon
      };
      let yposfactor = rng.gen_range(0.0..1.0);
      let y = mix(mountain.ridge_pos_for_x(x).1, bottomy, yposfactor);

      if exclusion_mask.is_painted((x, y)) {
        fails += 1;
        if fails > sampling_skip_if_fails_more_than {
          break;
        }
        continue;
      }
      fails = 0;

      let area = mountains.battlefield.get(x, y);
      let destruction = ctx.destruction_map.get_weight((x, y));

      let origin = (x, y);
      let mut angle = mountain.slope_for_x(x);
      if angle > PI {
        angle -= 2.0 * PI;
      }
      let maxa = 0.8;
      angle = angle.max(-maxa).min(maxa);
      // FIXME i think angle is fucked up. eg on catapulte

      let mut spawn_animal = |rng: &mut R| {
        let proximity = 2.0;
        if rng.gen_bool(0.2) {
          let size = rng.gen_range(0.7..1.0) * 0.015 * width;
          let obj = Armadillo::init(rng, clr, origin, size, -angle);
          renderables.add(obj);
          let circle = VCircle::new(x, y - 0.5 * size, proximity * size);
          debug_circle.push(circle);
          exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
        } else if rng.gen_bool(0.4) {
          let size = rng.gen_range(0.02..0.03) * width;
          let obj = Fowl::init(rng, clr, origin, size, angle);
          renderables.add(obj);
          let circle = VCircle::new(x, y - 0.5 * size, proximity * size);
          debug_circle.push(circle);
          exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
        } else {
          let size = rng.gen_range(0.02..0.035) * width;
          let reversex = rng.gen_bool(0.5);
          let barking = rng.gen_bool(0.5);
          let obj = Dog::init(rng, clr, origin, size, reversex, barking);
          renderables.add(obj);
          let circle = VCircle::new(x, y - 0.5 * size, proximity * size);
          debug_circle.push(circle);
          exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
        };
      };

      match area {
        Area::Empty => {}

        Area::Animal => {
          spawn_animal(rng);
        }

        Area::Defender(props) => {
          let blazonclr = if mountain.is_behind {
            mountain.clr
          } else {
            ctx.defendersclr
          };
          let size = 0.04 * width;
          let xflip = props.oriented_left;

          ctx.projectiles.add_defense(DefenseTarget::Human(origin));

          let decorationratio = 0.3;
          let foot_offset = 1.0;
          if destruction > 0.5 {
            if destruction < 1.0 && rng.gen_bool(1. - destruction as f64) {
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
              renderables.add(horse);
            }

            for obj in vec![props.leftobj, props.rightobj].iter().flatten() {
              if rng.gen_bool(0.5) {
                if let Some(o) = obj.as_destructed_renderable(
                  rng, origin, size, mainclr, blazonclr, blazon,
                ) {
                  renderables.push(o);
                }
              }
            }
          } else {
            let posture = HumanPosture::from_holding(
              rng,
              xflip,
              props.leftobj,
              props.rightobj,
            );

            let human = Human::init(
              rng,
              origin,
              size,
              xflip,
              blazon,
              mainclr,
              blazonclr,
              posture,
              props.headshape,
              props.leftobj,
              props.rightobj,
            );

            if props.on_horse {
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
                human,
              );
              renderables.add(rider);
            } else {
              renderables.add(human);
            }
          }

          let proximity = props.proximity * size;
          let circle = VCircle::new(x, y - 0.5 * size, proximity);
          debug_circle.push(circle);
          exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
        }

        Area::Attacker(props) => {
          if let Some(castle) = &mountain.castle {
            if rng.gen_bool(climb_attack_proba) {
              let dy = origin.1 - castle.position.1;
              if ctx.projectiles.get_attacks_count() < 5 {
                if dy < 0.05 * width && rng.gen_bool(0.7) {
                  ctx.projectiles.add_attack(AttackOrigin::Ladder(origin));
                } else {
                  ctx.projectiles.add_attack(AttackOrigin::Rope(origin, 0));
                }
              }
            }
          }

          let is_leader = should_spawn_leader
            && yposfactor < 0.8
            && (xposfactor - 0.5).abs() < 0.4;
          let mainclr = if is_leader { blazonclr } else { clr };
          let size = if is_leader { 0.06 } else { 0.04 } * width;
          let xflip = props.oriented_left;

          let decorationratio = 0.3;
          let foot_offset = 1.0;
          if destruction > 0.5 {
            if destruction < 1.0 && rng.gen_bool(1. - destruction as f64) {
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
              renderables.add(horse);
            }

            for obj in vec![props.leftobj, props.rightobj].iter().flatten() {
              if rng.gen_bool(0.5) {
                if let Some(o) = obj.as_destructed_renderable(
                  rng, origin, size, mainclr, blazonclr, blazon,
                ) {
                  renderables.push(o);
                }
              }
            }
          } else {
            let posture = HumanPosture::from_holding(
              rng,
              xflip,
              props.leftobj,
              props.rightobj,
            );

            let human = Human::init(
              rng,
              origin,
              size,
              xflip,
              blazon,
              mainclr,
              blazonclr,
              posture,
              props.headshape,
              props.leftobj,
              props.rightobj,
            );

            if !mountain.is_behind {
              human.throw_projectiles(ctx);
            }
            if props.on_horse {
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
                human,
              );
              renderables.add(rider);
            } else {
              renderables.add(human);
            }

            if is_leader {
              should_spawn_leader = false;
            }
          }

          let proximity = (if is_leader { 2.0 } else { 1.0 })
            //* (if props.on_horse { 2.0 } else { 1.0 })
            * props.proximity
            * size;
          let circle = VCircle::new(x, y - 0.5 * size, proximity);
          debug_circle.push(circle);
          exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
        }

        Area::DirectionalSiegeMachine(props) => {
          let stability = mountain.ground_stability(origin, 5.0);
          if stability < 0.5 {
            continue;
          }

          // TODO spawn people
          let angle = 0.3 * angle;
          let xflip = props.oriented_left;
          if rng.gen_bool(catapult_instead_of_cannon_proba) {
            let progress = rng.gen_range(0.0..1.0);
            let size = rng.gen_range(0.04..0.06) * width;
            let catapult =
              Catapult::init(rng, clr, origin, size, angle, xflip, progress);

            catapult.throw_projectiles(ctx);

            renderables.add(catapult);

            let circle = VCircle::new(x, y - 0.5 * size, 1.5 * size);
            debug_circle.push(circle);
            exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
          } else {
            let size = rng.gen_range(0.01..0.015) * width;
            let cannon = Cannon::init(rng, clr, origin, size, angle, xflip);

            cannon.throw_projectiles(ctx);

            renderables.add(cannon);

            let circle = VCircle::new(x, y - 0.5 * size, 3.0 * size);
            debug_circle.push(circle);
            exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
          }
        }

        Area::Cyclope => {
          if x < 0.1 * width || x > 0.9 * width {
            continue;
          }
          let size = rng.gen_range(0.1..0.2) * width;
          let xflip = rng.gen_bool(0.5);
          let lasering = rng.gen_bool(0.5);
          let headshape = HeadShape::CYCLOPE;
          let leftobj = None;
          let rightobj = if lasering {
            if rng.gen_bool(0.5) {
              None
            } else {
              Some(HoldableObject::Club)
            }
          } else {
            Some(HoldableObject::Club)
          };
          let clr = if mainclr == 0 {
            if rng.gen_bool(0.8) {
              0
            } else {
              1
            }
          } else {
            mainclr
          };
          let posture =
            HumanPosture::from_holding(rng, xflip, leftobj, rightobj);
          let human = Human::init(
            rng, origin, size, xflip, blazon, clr, clr, posture, headshape,
            leftobj, rightobj,
          )
          .with_worms_filling_defaults();

          ctx.nb_cyclopes += 1;

          if lasering {
            let o = human.eye_pos();
            ctx.projectiles.add_attack(AttackOrigin::Eye(o));
          }

          renderables.add(human);

          let area = VCircle::new(x, y - 0.3 * size, size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }

        Area::Firecamp => {
          let size = rng.gen_range(0.01..0.02) * width;
          let smokel = size * rng.gen_range(4.0..12.0);
          let camp = Firecamp::init(rng, ctx, mainclr, origin, size, smokel);
          renderables.add(camp);
          let area = VCircle::new(x, y - 0.3 * size, size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }

        Area::Tree(foliage_ratio, bush_width_ratio) => {
          let animal = rng.gen_bool(0.01);
          if animal {
            spawn_animal(rng);
          }

          let trunk_fill_each = rng.gen_range(1.0..10.0);
          let size = mix(
            0.05,
            0.15,
            mix(
              1.0 - foliage_ratio,
              rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
              0.5,
            ),
          ) * width;
          let tree = Tree::init(
            rng,
            origin,
            size,
            clr,
            foliage_ratio,
            bush_width_ratio,
            trunk_fill_each,
          );
          renderables.add(tree);

          let area = VCircle::new(x, y - 0.3 * size, 0.3 * size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }

        Area::Hut => {
          // TODO spawn people

          let size = rng.gen_range(0.05..0.08) * width;
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
          let hut = Hut::init(rng, mainclr, (x, y), size, angle, Some(flag));
          renderables.add(hut);

          let area = VCircle::new(x, y - 0.5 * size, size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }

        Area::CastleSiegeMachine(props) => {
          if ctx.nb_castle_siege_machine > 2 {
            continue;
          }

          let stability = mountain.ground_stability(origin, 5.0);
          if stability < 0.5 {
            continue;
          }

          // TODO spawn people
          let h = rng.gen_range(0.05..0.1) * width;
          let xflip = props.oriented_left;
          if rng.gen_bool(0.5) {
            let triangle_structure = rng.gen_bool(0.7);
            let s = rng.gen_range(0.1..0.13) * width;

            let o = (x, y - 0.3 * h);
            let machine =
              Belier::init(rng, clr, o, s, 0.0, xflip, triangle_structure);

            for pos in machine.human_positions() {
              let size = 0.5 * s;
              let headshape = HeadShape::NAKED;
              let leftobj = Some(HoldableObject::RaisingUnknown);
              let rightobj = Some(HoldableObject::RaisingUnknown);
              let posture =
                HumanPosture::from_holding(rng, false, leftobj, rightobj);
              let warrior = Human::init(
                rng, pos, size, xflip, blazon, mainclr, blazonclr, posture,
                headshape, leftobj, rightobj,
              );
              renderables.add(warrior);
            }
            renderables.add(machine);
          } else if rng.gen_bool(0.5) {
            let bridge_width = rng.gen_range(0.3..0.6) * h;
            let bridge_opening = rng.gen_range(0.0f32..2.0).min(1.0);
            renderables.add(Belfry::init(
              rng,
              clr,
              origin,
              h,
              bridge_width,
              bridge_opening,
              xflip,
            ));
          } else {
            let h = width * 0.05;
            let w = width * rng.gen_range(0.1..0.25);
            let machine = TunnelStructure::init(rng, clr, origin, 0.0, h, w);
            renderables.add(machine);
          }

          let area = VCircle::new(x, y - 0.3 * h, h);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);

          ctx.nb_castle_siege_machine += 1;
        }

        Area::DistantSiegeMachine(props) => {
          if remaining_trebuchets == 0 {
            continue;
          }

          let stability = mountain.ground_stability(origin, 5.0);
          if stability < 0.5 {
            continue;
          }

          // TODO spawn people

          let height = rng.gen_range(0.08..0.12) * width;
          let action_percent =
            if ctx.trebuchets_should_shoot && !mountain.is_behind {
              rng.gen_range(0.0..1.0)
            } else {
              0.0
            };
          let xflip = props.oriented_left;
          let clr = mainclr;

          let trebuchet =
            Trebuchet::init(rng, origin, height, action_percent, xflip, clr);

          trebuchet.throw_projectiles(ctx);

          renderables.add(trebuchet);

          let area = VCircle::new(x, y - 0.3 * height, height);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
          remaining_trebuchets -= 1;
        }
      }
    }

    let mut routes = vec![];

    routes.extend(renderables.render(rng, ctx, paint));

    if self.debug {
      for c in debug_circle.iter() {
        routes.push((2, circle_route((c.x, c.y), c.r, 64)));
      }
    }

    routes

    /*

      // relic convoy
      if rng.gen_bool(0.05) {
        let x = mix(first.0, last.0, rng.gen_range(0.0..1.0));
        let y = lookup_ridge(&ridge, x);
        let relicsize = rng.gen_range(0.05..0.07) * width;
        let y = y - relicsize * 0.2;
        let ang = 0.0;
        let filling = 2.0;

        let extraratio = rng.gen_range(0.3..0.6);

        let convoy = ConvoyWalk::init(
          rng,
          mainclr,
          (x, y),
          relicsize,
          ang,
          1.0,
          extraratio,
        );

        let mut monks = vec![];

        for p in vec![convoy.left, convoy.right] {
          let p = (p.0, p.1 + relicsize * 0.4);
          let monk = Monk::init(rng, p, relicsize, 0.0, false, 0, true);
          monks.push(monk);
        }

        let relic = Relic::init(rng, (x, y), relicsize, ang, filling);

        let area = VCircle::new(x, y, 4.0 * relicsize);
        debug_circle.push(area);
        exclusion_mask.paint_circle(area.x, area.y, area.r);

        renderables.add(convoy);
        renderables.add(relic);
        for monk in monks {
          renderables.add(monk);
        }
      }


    */
  }
}

pub fn make_random_convoy<R: Rng>(
  rng: &mut R,
  renderables: &mut Container<R>,
  exclusion_mask: &mut PaintMask,
  blazon: Blazon,
  mainclr: usize,
  blazonclr: usize,
  size: f32,
  path: &Vec<(f32, f32)>,
) {
  let lookup = PathLookup::init(path.clone());
  let (x, y) = lookup.lookup_percentage(rng.gen_range(0.2..0.8));
  let filling = rng.gen_range(1.5..2.5);
  let convoyp = (x, y - 0.3 * size);
  let totalw = (path[path.len() - 1].0 - path[0].0).abs();

  let xflip = rng.gen_bool(0.5);
  let is_relic = rng.gen_bool(0.5);
  let is_human_holders = is_relic && rng.gen_bool(0.7) || rng.gen_bool(0.03);
  let monk_proba = if is_relic {
    rng.gen_range(0.6..1.0)
  } else {
    rng.gen_range(0.0..0.3)
  };
  let horse_proba = rng.gen_range(0.0..1.0);

  let hold_diff = if is_human_holders {
    0.5 * size
  } else {
    0.9 * size
  };
  let xleft = x - hold_diff;
  let xright = x + hold_diff;
  let left = lookup.lookup_pos_at_x(xleft);
  let right = lookup.lookup_pos_at_x(xright);
  let slope = (right.1 - left.1).atan2(right.0 - left.0);

  let extraratio = if is_human_holders { 0.5 } else { 1.1 };
  let w = 1.0;
  let convoy =
    ConvoyWalk::init(rng, mainclr, convoyp, size, slope, w, extraratio);

  if is_relic {
    let relic = Relic::init(rng, convoyp, size, slope, filling);
    renderables.add(relic);
  } else {
    let cage = Cage::init(rng, mainclr, convoyp, 0.8 * size, slope);
    renderables.add(cage);

    let lefthand = None;
    let righthand = None;
    let headshape = HeadShape::NAKED;
    let posture = HumanPosture::sit(rng, slope);
    let p = (convoyp.0, convoyp.1 + 0.15 * size);
    let human = Human::init(
      rng, p, size, xflip, blazon, mainclr, blazonclr, posture, headshape,
      lefthand, righthand,
    );
    renderables.add(human);
  }
  let area = VCircle::new(x, y, 4.0 * size);
  exclusion_mask.paint_circle(area.x, area.y, area.r);

  let mut v = 0.0;
  let pad = 0.5 * size;
  let minincr = 0.2 * size / lookup.length();
  while v <= 1.0 {
    if rng.gen_bool(0.5) {
      v += minincr;
      continue;
    }
    let (x, y) = lookup.lookup_percentage(v);
    let convoy_loc = left.0 - pad < x && x < right.0 + pad;
    if !convoy_loc {
      let is_horse = rng.gen_bool(horse_proba);
      if is_horse {
        v += 2. * minincr;
      }
      let p = lookup.lookup_percentage(v);

      let distfactor = (1.0 - 3.0 * (p.1 - convoyp.1).abs() / totalw)
        .max(0.01)
        .min(0.99) as f64;

      let is_monk = rng.gen_bool(monk_proba * distfactor);
      if is_monk {
        let monk = Monk::init(rng, p, size, 0.0, xflip, 0, false);
        renderables.add(monk);
      } else {
        let lefthand = Some(HoldableObject::Shield);
        let righthand = Some(if rng.gen_bool(0.8) {
          HoldableObject::Sword
        } else {
          HoldableObject::Flag
        });
        let headshape = HeadShape::NAKED;
        let posture =
          HumanPosture::from_holding(rng, xflip, lefthand, righthand);
        let human = Human::init(
          rng, p, size, xflip, blazon, mainclr, blazonclr, posture, headshape,
          lefthand, righthand,
        );
        if is_horse {
          let decorationratio = 0.3;
          let foot_offset = 1.0;
          let rider = Rider::init(
            rng,
            p,
            size,
            slope,
            xflip,
            mainclr,
            blazonclr,
            decorationratio,
            foot_offset,
            human,
          );
          renderables.add(rider);
          v += 2. * minincr;
        } else {
          renderables.add(human);
        }
      }
      let area = VCircle::new(x, y, size);
      exclusion_mask.paint_circle(area.x, area.y, area.r);
    }
    v += rng.gen_range(0.8..1.4) * minincr;
  }

  if is_human_holders {
    for p in vec![left, right] {
      let monk = Monk::init(rng, p, size, 0.0, xflip, 0, true);
      renderables.add(monk);
      let area = VCircle::new(x, y, size);
      exclusion_mask.paint_circle(area.x, area.y, area.r);
    }
  } else {
    let origin = if xflip { left } else { right };
    let decorationratio = 0.3;
    let foot_offset = 1.0;
    let lefthand = Some(HoldableObject::RaisingUnknown);
    let righthand = Some(if rng.gen_bool(0.8) {
      HoldableObject::LongSword
    } else {
      HoldableObject::RaisingUnknown
    });
    let headshape = HeadShape::NAKED;
    let posture = HumanPosture::from_holding(rng, xflip, lefthand, righthand);
    let human = Human::init(
      rng, origin, size, xflip, blazon, mainclr, blazonclr, posture, headshape,
      lefthand, righthand,
    );
    let angle = 0.0;
    let horse = Rider::init(
      rng,
      origin,
      size,
      angle,
      xflip,
      mainclr,
      blazonclr,
      decorationratio,
      foot_offset,
      human,
    );
    renderables.add(horse);
  }

  renderables.add(convoy);
}

fn montmirail<R: Rng>(
  rng: &mut R,
  renderables: &mut Container<R>,
  ridge: &Vec<(f32, f32)>,
  yhorizon: f32,
  width: f32,
  blazon: Blazon,
  mainclr: usize,
  blazonclr: usize,
) {
  let first = ridge[0];
  let last = ridge[ridge.len() - 1];
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
    false,
    blazon,
    mainclr,
    blazonclr,
    posture,
    HeadShape::NAKED,
    leftobj,
    rightobj,
  );
  renderables.add(jacquouille);
  let rightobj = Some(HoldableObject::Sword);
  let posture = HumanPosture::from_holding(rng, false, leftobj, rightobj);
  let godefroy = Human::init(
    rng,
    (x - 1.2 * size, y),
    size,
    false,
    blazon,
    mainclr,
    blazonclr,
    posture,
    HeadShape::HELMET,
    None,
    Some(HoldableObject::Sword),
  );
  renderables.add(godefroy);
  renderables.add(car);
}
