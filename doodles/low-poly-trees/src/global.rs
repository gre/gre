use rand::prelude::*;
use serde::Serialize;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone, Serialize)]
// Feature tells characteristics of a given art variant.
pub struct Feature {}

impl Feature {
  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

pub struct GlobalCtx {
  pub palette: Vec<[f32; 3]>,
}

impl GlobalCtx {
  pub fn rand(_rng: &mut StdRng) -> Self {
    Self {
      // TODO generative
      palette: vec![
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
      ],
    }
  }

  pub fn to_feature(&self) -> Feature {
    let feature = Feature {};
    feature
  }
}
