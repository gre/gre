use std::f32::consts::PI;

use self::{
  arrowtrail::ArrowTrail,
  attack::{
    extract_attack_pos, map_progress, resolve_trajectory_path, AttackOrigin,
    DefenseTarget,
  },
  ball::Ball,
  fireballtrail::FireballTrail,
  laser::Laser,
};
use super::army::{arrow::Arrow, ladder::Ladder, rope::Rope};
use crate::{
  algo::{
    clipping::regular_clip,
    math2d::{distance_angles, euclidian_dist},
    paintmask::PaintMask,
    polylines::Polylines,
  },
  global::GlobalCtx,
};
use noise::*;
use rand::prelude::*;

pub mod arrowtrail;
pub mod attack;
pub mod ball;
pub mod fireballtrail;
pub mod laser;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
#[derive(Clone)]
pub struct Projectiles {
  // store all the attack/def intent to be resolved later as projectiles
  attacks: Vec<AttackOrigin>,
  defenses: Vec<DefenseTarget>,

  // once the projectiles are resolved, they get added there
  routes: Vec<(usize, Vec<(f32, f32)>)>,
  arrows: Vec<Arrow>,
  balls: Vec<Ball>,
  fireballtrails: Vec<FireballTrail>,
  arrowtrails: Vec<ArrowTrail>,
  lasers: Vec<Laser>,
  ropes: Vec<Rope>,
  ladders: Vec<Ladder>,
}

fn matches_attack_defense(
  ctx: &GlobalCtx,
  origin: &AttackOrigin,
  target: &DefenseTarget,
) -> bool {
  let s = ctx.width / 210.;
  match (origin, target) {
    (AttackOrigin::Cannon(a), DefenseTarget::Building(d)) => {
      let dx = a.0 - d.0;
      let dy = a.1 - d.1;
      dx.abs() / (1.0 + dy.abs()) > 3.0
    }
    (AttackOrigin::Catapult(_), DefenseTarget::Building(_)) => true,
    (AttackOrigin::Trebuchet(_), DefenseTarget::Building(_)) => true,
    (AttackOrigin::Eye(_), DefenseTarget::Building(_)) => true,
    (AttackOrigin::Bow(_), DefenseTarget::Human(_)) => true,
    (AttackOrigin::Ladder(a), DefenseTarget::Ladder(d)) => {
      let minh = 4.0 * s;
      let maxl = 120.0 * s;
      let maxang = 0.4;
      if a.1 < d.1 + minh {
        return false;
      }
      let l = euclidian_dist(*a, *d);
      if l > maxl {
        return false;
      }
      let dx = a.0 - d.0;
      let dy = a.1 - d.1;
      let angle = dy.atan2(dx);
      let angled = distance_angles(angle, PI / 2.0);
      if angled > maxang {
        return false;
      }
      true
    }
    (AttackOrigin::Rope(a, _), DefenseTarget::Rope(d)) => {
      let dx = a.0 - d.0;
      let dy = a.1 - d.1;
      let l = dx.abs() + 2.0 * dy.abs();
      if l > ctx.rope_len_base * s {
        return false;
      }
      true
    }
    _ => false,
  }
}

impl Projectiles {
  pub fn new() -> Self {
    Self {
      attacks: vec![],
      defenses: vec![],
      routes: vec![],
      arrows: vec![],
      balls: vec![],
      fireballtrails: vec![],
      arrowtrails: vec![],
      lasers: vec![],
      ropes: vec![],
      ladders: vec![],
    }
  }

  pub fn add_attack(&mut self, origin: AttackOrigin) {
    self.attacks.push(origin);
  }

  pub fn add_defense(&mut self, target: DefenseTarget) {
    self.defenses.push(target);
  }

  pub fn resolve_and_render<R: Rng>(
    &mut self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &PaintMask,
    existing_routes: &mut Polylines,
    preserve_area: &PaintMask,
  ) {
    self.resolve(rng, ctx, paint);
    self.render(rng, ctx, existing_routes, preserve_area);
  }

  pub fn resolve<R: Rng>(
    &mut self,
    rng: &mut R,
    ctx: &GlobalCtx,
    referencemask: &PaintMask,
  ) {
    let perlin = Perlin::new(rng.gen());

    let wf = 2.0 / referencemask.width as f64;

    // let progress_range = 0.1..0.9;
    if self.defenses.len() == 0 {
      return;
    }
    let mut defs = vec![];
    for origin in self.attacks.drain(..) {
      // TODO: we could pick a less random attack & favor a range (min..max) which depends on each kind of attack too
      // also arrows want to hit on humans, not on buildings
      if defs.len() == 0 {
        defs = self.defenses.clone();
        defs.shuffle(rng);
      }
      let index = defs
        .iter()
        .position(|t| matches_attack_defense(ctx, &origin, t));
      if index.is_none() {
        // no match
        continue;
      }
      let index = index.unwrap();
      let target = defs.remove(index);

      let o = extract_attack_pos(origin);
      let progress = map_progress(
        origin,
        0.5 + 0.4 * perlin.get([o.0 as f64 * wf, o.1 as f64 * wf, 0.0]) as f32,
      );
      let path = resolve_trajectory_path(origin, target, progress);

      if path.len() < 2 {
        continue;
      }

      let bulletpos = path[path.len() - 1];
      let bulletposprev = path[path.len() - 2];
      let angle =
        -(bulletpos.1 - bulletposprev.1).atan2(bulletpos.0 - bulletposprev.0);

      let trailpercent: f32 = 1.0
        - rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0);

      match origin {
        AttackOrigin::Ladder(_) => {
          let humansize = rng.gen_range(0.03..0.05) * ctx.width;
          let o = Ladder::init(rng, ctx, &path, humansize);
          self.ladders.push(o);
        }
        AttackOrigin::Rope(_, clr) => {
          let humansize = rng.gen_range(0.03..0.05) * ctx.width;
          let rope = Rope::init(rng, clr, ctx, &path, humansize);
          self.ropes.push(rope);
        }
        AttackOrigin::Cannon(_) => {
          let clr = ctx.fireball_color;
          let size = rng.gen_range(1.0..2.0);
          let particles = rng.gen_range(10..20);
          let strokes = rng.gen_range(1..4);
          let ball = Ball::init(rng, bulletpos, size, clr);
          let trail = FireballTrail::init(
            rng,
            &referencemask,
            path,
            size,
            trailpercent.min(0.96),
            particles,
            strokes,
            clr,
            0.5,
          );
          self.balls.push(ball);
          self.fireballtrails.push(trail);
        }
        AttackOrigin::Catapult(_) => {
          let clr = ctx.fireball_color;
          let size = rng.gen_range(1.0..2.0);
          let particles = rng.gen_range(10..20);
          let strokes = rng.gen_range(1..4);
          let ball = Ball::init(rng, bulletpos, size, clr);
          let trail = FireballTrail::init(
            rng,
            &referencemask,
            path,
            size,
            0.8 * trailpercent,
            particles,
            strokes,
            clr,
            1.0,
          );
          self.balls.push(ball);
          self.fireballtrails.push(trail);
        }
        AttackOrigin::Trebuchet(_) => {
          let clr = ctx.fireball_color;
          let size = rng.gen_range(1.0..3.0);
          let particles = rng.gen_range(10..100);
          let strokes = rng.gen_range(1..6);
          let ball = Ball::init(rng, bulletpos, size, clr);
          let trail = FireballTrail::init(
            rng,
            &referencemask,
            path,
            size,
            trailpercent,
            particles,
            strokes,
            clr,
            1.5,
          );
          self.balls.push(ball);
          self.fireballtrails.push(trail);
        }
        AttackOrigin::Bow(_) => {
          let size = rng.gen_range(4.0..5.0);
          self.arrows.push(Arrow::init(0, bulletpos, size, angle));
          if rng.gen_bool(0.3) {
            self
              .arrowtrails
              .push(ArrowTrail::init(0, path, trailpercent));
          }
        }
        AttackOrigin::Eye(_) => {
          let size = rng.gen_range(1.0..2.5);
          let laser = Laser::init(1, path, size);
          self.lasers.push(laser);
        }
      }
    }
    self.defenses = vec![];
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    existing_routes: &mut Polylines,
    preserve_area: &PaintMask,
  ) {
    let mut removing_area = preserve_area.clone();

    let mut routes = self.routes.clone();

    for laser in self.lasers.iter() {
      routes.extend(laser.render(&mut removing_area));
    }
    for ball in self.balls.iter() {
      routes.extend(ball.render(rng, &mut removing_area));
    }
    for arrow in self.arrows.iter() {
      routes.extend(arrow.render(&mut removing_area));
    }

    for trail in self.fireballtrails.iter() {
      routes.extend(trail.render(rng, ctx, &mut removing_area));
    }
    for trail in self.arrowtrails.iter() {
      routes.extend(trail.render(&mut removing_area));
    }

    for rope in self.ropes.iter() {
      routes.extend(rope.render(rng, &mut removing_area));
    }
    for ladder in self.ladders.iter() {
      routes.extend(ladder.render(rng, &mut removing_area));
    }

    routes = regular_clip(&routes, preserve_area);

    let mut mask = preserve_area.clone();
    mask.reverse();
    mask.intersects(&removing_area);
    *existing_routes = regular_clip(existing_routes, &mask);

    existing_routes.extend(routes);
  }
}
