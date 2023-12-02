use self::{
  arrowtrail::ArrowTrail,
  attack::{
    extract_attack_pos, resolve_trajectory_path, AttackOrigin, DefenseTarget,
  },
  ball::Ball,
  fireballtrail::FireballTrail,
  laser::Laser,
};
use super::army::arrow::Arrow;
use crate::{
  algo::{clipping::regular_clip, paintmask::PaintMask, polylines::Polylines},
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
    }
  }

  pub fn add_attack(&mut self, origin: AttackOrigin) {
    self.attacks.push(origin);
  }

  pub fn add_defense(&mut self, target: DefenseTarget) {
    self.defenses.push(target);
  }

  pub fn resolve<R: Rng>(&mut self, rng: &mut R, referencemask: &PaintMask) {
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
      let target = defs.pop().unwrap();

      let o = extract_attack_pos(origin);
      let progress =
        0.5 + 0.4 * perlin.get([o.0 as f64 * wf, o.1 as f64 * wf, 0.0]) as f32;
      let path = resolve_trajectory_path(origin, target, progress);

      if path.len() < 2 {
        continue;
      }

      let bulletpos = path[path.len() - 1];
      let bulletposprev = path[path.len() - 2];
      let angle =
        -(bulletpos.1 - bulletposprev.1).atan2(bulletpos.0 - bulletposprev.0);

      let trailpercent = 1.0
        - rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0);

      match origin {
        AttackOrigin::Fireball(_) => {
          let clr = 2;
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
          );
          self.balls.push(ball);
          self.fireballtrails.push(trail);
        }
        AttackOrigin::Arrow(_) => {
          let size = rng.gen_range(4.0..5.0);
          self.arrows.push(Arrow::init(0, bulletpos, size, angle));
          if rng.gen_bool(0.3) {
            self
              .arrowtrails
              .push(ArrowTrail::init(0, path, trailpercent));
          }
        }
        AttackOrigin::Laser(_) => {
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

    routes = regular_clip(&routes, preserve_area);

    let mut mask = preserve_area.clone();
    mask.reverse();
    mask.intersects(&removing_area);
    *existing_routes = regular_clip(existing_routes, &mask);

    existing_routes.extend(routes);
  }
}
