use crate::{palette::Palette, svgplot::inks_stats};
use rand::prelude::*;
use serde::Serialize;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone, Serialize)]
// Feature tells characteristics of a given art variant. It is returned in the .SVG file
pub struct Feature {
  pub inks: String,      // which inks are used
  pub inks_count: usize, // how much inks are used
  pub paper: String,     // which paper is used
}

impl Feature {
  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

pub struct GlobalCtx {
  pub palette: Palette,
  pub width: f32,
  pub height: f32,
  pub precision: f32,
}

impl GlobalCtx {
  pub fn rand(
    _rng: &mut StdRng,
    width: f32,
    height: f32,
    precision: f32,
    palette: &Palette,
  ) -> Self {
    Self {
      palette: palette.clone(),
      width,
      height,
      precision,
    }
  }

  pub fn to_feature(&self, routes: &Vec<(usize, Vec<(f32, f32)>)>) -> Feature {
    let palette = &self.palette;
    let inks = inks_stats(&routes, &palette.inks);

    let feature = Feature {
      inks: inks.join(", "),
      inks_count: inks.len(),
      paper: palette.paper.0.to_string(),
    };

    feature
  }
}
