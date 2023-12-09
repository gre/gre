use super::{
  body::HumanPosture,
  human::{HeadShape, HoldableObject, Human},
};
use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    pathlookup::PathLookup,
    polylines::{path_to_fibers, step_polyline, Polylines},
  },
  global::GlobalCtx,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct Ladder {
  pub clr: usize,
  pub ladder: Polylines,
  pub humans: Vec<Human>,
}

impl Ladder {
  pub fn init<R: Rng>(
    rng: &mut R,
    ctx: &GlobalCtx,
    path: &Vec<(f32, f32)>,
    humansize: f32,
  ) -> Self {
    let mut humans = vec![];

    let mut ladder = vec![];

    let stepy = 0.4 * humansize;
    let stepw = 0.5 * humansize;

    let clr = 0;
    let steps = step_polyline(path, stepy);
    let widths = vec![stepw; steps.len()];
    let all = path_to_fibers(&steps, &widths, 2);
    let first = &all[0];
    let last = &all[all.len() - 1];
    let from = 1;
    let to = first.len() - 1;
    if from < to {
      for i in from..to {
        ladder.push((clr, vec![first[i], last[i]]));
      }
    }
    ladder.push((clr, first.clone()));
    ladder.push((clr, last.clone()));

    let lookup = PathLookup::init(path.clone());

    let blazon = ctx.attackers;
    let mainclr = clr;
    let blazonclr = ctx.attackersclr;
    let pad = rng.gen_range(0.0..3.0) * humansize;
    let from = pad;
    let to = lookup.length() - pad;
    let mut l = from;
    let xflip = false;
    let diffbase = rng.gen_range(0.4..1.4);
    while l < to {
      let p = lookup.lookup_pos(l);
      let angle = lookup.lookup_angle(l);
      let posture = HumanPosture::climbing(rng, angle, 0.0);
      let headshape = if rng.gen_bool(0.7) {
        HeadShape::HELMET
      } else {
        HeadShape::NAKED
      };
      let lefthand = if rng.gen_bool(0.7) {
        None
      } else {
        Some(HoldableObject::Shield)
      };
      let righthand = if rng.gen_bool(0.2) {
        None
      } else {
        Some(HoldableObject::Sword)
      };
      let human = Human::init(
        rng, p, humansize, xflip, blazon, mainclr, blazonclr, posture,
        headshape, lefthand, righthand,
      );
      humans.push(human);
      l += humansize
        * (diffbase
          + rng.gen_range(0.0..4.0)
            * rng.gen_range(0.0..1.0)
            * rng.gen_range(0.0..1.0)
            * rng.gen_range(0.0..1.0));
    }

    Self {
      clr,
      ladder,
      humans,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = vec![];
    for human in &self.humans {
      routes.extend(human.render(rng, paint));
    }
    routes.extend(regular_clip(&self.ladder, paint));
    for (_, rt) in self.ladder.iter() {
      paint.paint_polyline(rt, 0.6);
    }
    routes
  }
}
