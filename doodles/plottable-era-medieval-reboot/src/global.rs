use rand::prelude::*;

use crate::{
  algo::wormsfilling::WeightMap,
  svgplot::{Ink, Paper},
};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(PartialEq)]
pub enum Special {
  TrojanHorse,
  Lockness,  // TODO
  Excalibur, // TODO
  Ghuls,     // TODO
  Giant,     // TODO
}

pub struct GlobalCtx {
  pub width: f32,
  pub height: f32,
  pub precision: f32,
  pub specials: Vec<Special>,
  pub night_time: bool,

  /*
    TODO
    Global object: Destructed area weigh map.
    On the whole scene. Impacts all places.
    Slice the object and emit particles that can produce some fire.
    Dead battle field.
    Bushes, near castle.
    Rocks. In the sea.
    Axes.
  */
  pub destruction_map: WeightMap,
}

impl GlobalCtx {
  pub fn rand<R: Rng>(
    rng: &mut R,
    width: f32,
    height: f32,
    precision: f32,
    colors: &Vec<Ink>,
    paper: &Paper,
  ) -> Self {
    let mut specials = vec![];
    if rng.gen_bool(0.01) {
      specials.push(Special::TrojanHorse);
    }

    let destruction_map = WeightMap::new(width, height, precision, 0.0);
    let mut night_time = paper.2;
    if colors[0] == colors[1] {
      // in monochrome, we allow the night_time to get disabled
      if night_time {
        night_time = rng.gen_bool(0.5);
      }
    }

    Self {
      width,
      height,
      precision,
      specials,
      night_time,
      destruction_map,
    }
  }
}
