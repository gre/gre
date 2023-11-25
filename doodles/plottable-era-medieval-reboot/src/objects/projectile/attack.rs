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
  Fireball(Pos),
  Laser(Pos),
  Arrow(Pos),
}

#[derive(Clone, Copy)]
pub enum DefenseTarget {
  Building(Pos),
  // Human(Pos),
}

pub fn extract_attack_pos(origin: AttackOrigin) -> Pos {
  match origin {
    AttackOrigin::Fireball(pos) => pos,
    AttackOrigin::Laser(pos) => pos,
    AttackOrigin::Arrow(pos) => pos,
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
    // DefenseTarget::Human(pos) => pos,
  };
  let curvy_factor = match origin {
    AttackOrigin::Fireball(_) => 0.7,
    AttackOrigin::Laser(_) => 0.0,
    AttackOrigin::Arrow(_) => 0.5,
  };
  let p = match origin {
    AttackOrigin::Laser(_) => 1.0,
    _ => progress,
  };

  let mut path = vec![];

  let l = euclidian_dist(o, t);
  path.push(o);
  path.push(t);

  if curvy_factor > 0.0 {
    path = step_polyline(&path, 1.0);
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
