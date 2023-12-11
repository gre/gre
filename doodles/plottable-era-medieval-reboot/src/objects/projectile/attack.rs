use std::f32::consts::PI;

use crate::algo::{
  math2d::euclidian_dist,
  pathlookup::PathLookup,
  polylines::{step_polyline, Polyline},
};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

type Pos = (f32, f32);

#[derive(Clone, Copy)]
pub enum AttackOrigin {
  Cannon(Pos),
  Catapult(Pos),
  Trebuchet(Pos),
  Eye(Pos),
  Bow(Pos),
  Ladder(Pos),
  Rope(Pos, usize),
}

#[derive(Clone, Copy)]
pub enum DefenseTarget {
  Building(Pos),
  Human(Pos),
  Ladder(Pos),
  Rope(Pos),
}

pub fn extract_attack_pos(origin: AttackOrigin) -> Pos {
  match origin {
    AttackOrigin::Cannon(pos) => pos,
    AttackOrigin::Catapult(pos) => pos,
    AttackOrigin::Trebuchet(pos) => pos,
    AttackOrigin::Eye(pos) => pos,
    AttackOrigin::Bow(pos) => pos,
    AttackOrigin::Ladder(pos) => pos,
    AttackOrigin::Rope(pos, _) => pos,
  }
}

pub fn map_progress(origin: AttackOrigin, progress: f32) -> f32 {
  match origin {
    AttackOrigin::Cannon(_) => 0.4 * progress,
    _ => progress,
  }
}

pub fn resolve_trajectory_path(
  origin: AttackOrigin,
  target: DefenseTarget,
  progress: f32,
) -> Polyline {
  let o = extract_attack_pos(origin);
  let t = match target {
    DefenseTarget::Building(pos) => pos,
    DefenseTarget::Human(pos) => pos,
    DefenseTarget::Ladder(pos) => pos,
    DefenseTarget::Rope(pos) => pos,
  };
  let curvy_factor = match origin {
    AttackOrigin::Cannon(_) => 0.1,
    AttackOrigin::Catapult(_) => 0.5,
    AttackOrigin::Trebuchet(_) => 0.7,
    AttackOrigin::Eye(_) => 0.0,
    AttackOrigin::Bow(_) => 0.4,
    AttackOrigin::Ladder(_) => 0.0,
    AttackOrigin::Rope(_, _) => -0.1, // gravity
  };
  let p = match origin {
    AttackOrigin::Eye(_) => 1.0,
    AttackOrigin::Ladder(_) => 1.0,
    AttackOrigin::Rope(_, _) => 1.0,
    _ => progress,
  };

  let mut path = vec![];

  let l = euclidian_dist(o, t);
  path.push(o);
  path.push(t);

  if curvy_factor != 0.0 {
    path = step_polyline(&path, 5.0);
    let plen = path.len();
    path = path
      .iter()
      .enumerate()
      .map(|(i, p)| {
        let t = i as f32 / (plen - 1) as f32;
        let m = (t * PI).sin();
        (p.0, p.1 - curvy_factor * m * l)
      })
      .collect::<Vec<_>>();
  }

  if p < 1.0 {
    let lookup = PathLookup::init(path);
    path = lookup.slice_before(p * lookup.length());
  }

  path
}
