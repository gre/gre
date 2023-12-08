use self::{
  belfry::Belfry,
  belier::Belier,
  body::HumanPosture,
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
pub mod firecamp;
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
    if has_wall {
      let xc = rng.gen_range(0.3..0.7);
      let w = rng.gen_range(0.3..2.0);
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

      if path.len() > 1 && (path[path.len() - 1].0 - path[0].0).abs() > minw {
        let path = moving_average_2d(&path, path.len() / 10);
        let baseh = rng.gen_range(0.05..0.08) * width;
        let wall =
          MountainWall::init(ctx, rng, blazonclr, mainclr, &path, baseh);
        renderables.extend(wall.container);
      }
    }

    let yallowunder = width * 0.02;
    let clr = mainclr;

    let mut exclusion_mask = paint.clone_rescaled(1.0).clone_empty();
    let mut should_spawn_leader = mountain.will_have_the_leader;
    let mut remaining_trebuchets =
      (rng.gen_range(-4.0f32..4.0) * rng.gen_range(0.0..1.0)).max(0.0) as usize
        + if ctx.specials.contains(&Special::Trebuchets) {
          10
        } else {
          0
        };

    // TODO global counters to stop when reaching enough? alternative it's the goal of exclusion mask

    let sampling = 2000; // it could be interesting to vary it
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
        continue;
      }

      let area = mountains.battlefield.get(x, y);
      let destruction = ctx.destruction_map.get_weight((x, y));
      // TODO take destruction into account

      // TODO work on convoy location

      let origin = (x, y);
      let mut angle = mountain.slope_for_x(x);
      if angle > PI {
        angle -= 2.0 * PI;
      }
      let maxa = 0.8;
      angle = angle.max(-maxa).min(maxa);
      // FIXME i think angle is fucked up. eg on catapulte

      match area {
        Area::Empty => {}

        Area::Animal => {
          let proximity = 2.0;
          if rng.gen_bool(0.2) {
            let size = rng.gen_range(0.5..1.0) * 0.01 * width;
            let obj = Armadillo::init(rng, clr, origin, size, angle);
            renderables.add(obj);
            let circle = VCircle::new(x, y - 0.5 * size, proximity * size);
            debug_circle.push(circle);
            exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
          } else if rng.gen_bool(0.4) {
            let size = rng.gen_range(0.01..0.03) * width;
            let obj = Fowl::init(rng, clr, origin, size, angle);
            renderables.add(obj);
            let circle = VCircle::new(x, y - 0.5 * size, proximity * size);
            debug_circle.push(circle);
            exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
          } else {
            let size = rng.gen_range(0.015..0.035) * width;
            let reversex = rng.gen_bool(0.5);
            let barking = rng.gen_bool(0.5);
            let obj = Dog::init(rng, clr, origin, size, reversex, barking);
            renderables.add(obj);
            let circle = VCircle::new(x, y - 0.5 * size, proximity * size);
            debug_circle.push(circle);
            exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
          };
        }

        Area::Defender(props) => {
          let blazonclr = ctx.defendersclr;
          let size = 0.04 * width;
          let xflip = props.oriented_left;

          ctx.projectiles.add_defense(DefenseTarget::Human(origin));

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

          // TODO in destruction case, we will only have the objects, possibly the horse

          if props.on_horse {
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
              human,
            );
            renderables.add(rider);
          } else {
            renderables.add(human);
          }

          let proximity = props.proximity * size;
          let circle = VCircle::new(x, y - 0.5 * size, proximity);
          debug_circle.push(circle);
          exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
        }

        Area::Attacker(props) => {
          let is_leader = should_spawn_leader
            && yposfactor < 0.8
            && (xposfactor - 0.5).abs() < 0.4;
          let mainclr = if is_leader { blazonclr } else { clr };
          let size = if is_leader { 0.06 } else { 0.04 } * width;
          let xflip = props.oriented_left;

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

          if ctx.archers_should_shoot
            && !mountain.is_behind
            && (matches!(props.leftobj, Some(HoldableObject::LongBow(_)))
              || matches!(props.rightobj, Some(HoldableObject::LongBow(_))))
          {
            ctx.projectiles.add_attack(AttackOrigin::Bow(
              human.body.hand_right_pos_angle().0,
            ));
          }

          // TODO in destruction case, we will only have the objects, possibly the horse

          if props.on_horse {
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
              human,
            );
            renderables.add(rider);
          } else {
            renderables.add(human);
          }

          let proximity = (if is_leader { 2.0 } else { 1.0 })
            //* (if props.on_horse { 2.0 } else { 1.0 })
            * props.proximity
            * size;
          let circle = VCircle::new(x, y - 0.5 * size, proximity);
          debug_circle.push(circle);
          exclusion_mask.paint_circle(circle.x, circle.y, circle.r);

          if is_leader {
            should_spawn_leader = false;
          }
        }

        Area::DirectionalSiegeMachine(props) => {
          // TODO spawn people
          let xflip = props.oriented_left;
          if rng.gen_bool(0.5) {
            let progress = rng.gen_range(0.0..1.0);
            let size = rng.gen_range(0.04..0.06) * width;
            let catapult =
              Catapult::init(rng, clr, origin, size, angle, xflip, progress);
            renderables.add(catapult);

            let circle = VCircle::new(x, y - 0.5 * size, 2.0 * size);
            debug_circle.push(circle);
            exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
          } else {
            let size = rng.gen_range(0.01..0.015) * width;
            let cannon = Cannon::init(rng, clr, origin, size, angle, xflip);
            renderables.add(cannon);

            let circle = VCircle::new(x, y - 0.5 * size, 5.0 * size);
            debug_circle.push(circle);
            exclusion_mask.paint_circle(circle.x, circle.y, circle.r);
          }
        }

        Area::Cyclope => {
          // TODO only if visible
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
          let size = rng.gen_range(0.02..0.03) * width;
          let smokel = size * rng.gen_range(4.0..12.0);
          let camp = Firecamp::init(rng, ctx, mainclr, origin, size, smokel);
          renderables.add(camp);
          let area = VCircle::new(x, y - 0.3 * size, size);
          debug_circle.push(area);
          exclusion_mask.paint_circle(area.x, area.y, area.r);
        }

        Area::Tree(foliage_ratio, bush_width_ratio) => {
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

          // TODO spawn people
          // FIXME terrible positioning atm
          // TODO check for terrain to be compatible
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
          // TODO spawn people
          // TODO verify terrain compatibility

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

          trebuchet.possibly_throw_projectiles(ctx);

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
