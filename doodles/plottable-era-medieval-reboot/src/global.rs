use crate::{
  algo::{
    clipping::regular_clip, paintmask::PaintMask, polylines::Polylines,
    wormsfilling::WeightMap,
  },
  objects::projectile::{ball::Ball, trail::Trail},
  palette::{self, Palette, AMBER, GOLD_GEL},
  svgplot::inks_stats,
};
use rand::prelude::*;
use serde::Serialize;
use std::collections::HashSet;

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

#[derive(PartialEq, Eq, Hash)]
pub enum Special {
  TrojanHorse,
  Lockness, // TODO
  Excalibur,
  Ghuls, // TODO
  Giant, // TODO
  Montmirail,
}

pub struct GlobalCtx {
  pub palette: Palette,
  pub width: f32,
  pub height: f32,
  pub precision: f32,
  pub specials: HashSet<Special>,
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

  // projectile management
  pub balls: Vec<Ball>,
  pub trails: Vec<Trail>,
  pub projectilesclr: usize,
}

impl GlobalCtx {
  pub fn rand<R: Rng>(
    rng: &mut R,
    width: f32,
    height: f32,
    precision: f32,
    palette: Palette,
  ) -> Self {
    let mut specials = HashSet::new();
    if rng.gen_bool(0.01) {
      specials.insert(Special::TrojanHorse);
    } else if rng.gen_bool(0.01) {
      let c = palette.inks[1];
      if c == GOLD_GEL || c == AMBER {
        specials.insert(Special::Montmirail);
      }
    }

    let paper = palette.paper;
    let colors = &palette.inks;

    let destruction_map = WeightMap::new(width, height, precision, 0.0);
    let mut night_time = paper.2;
    if colors[0] == colors[1] {
      // in monochrome, we allow the night_time to get disabled
      if night_time {
        night_time = rng.gen_bool(0.5);
      }
    }

    Self {
      palette,
      width,
      height,
      precision,
      specials,
      night_time,
      destruction_map,
      balls: vec![],
      trails: vec![],
      projectilesclr: 1, // FIXME IDK YET.. if rng.gen_bool(0.5) { 1 } else { 0 },
    }
  }

  pub fn overrides_city_name(&self) -> Option<String> {
    if self.specials.contains(&Special::TrojanHorse) {
      return Some("Troy".to_string());
    } else if self.specials.contains(&Special::Montmirail) {
      return Some("Montmirail".to_string());
    }
    None
  }

  pub fn to_feature(&self, routes: &Polylines) -> Feature {
    let palette = &self.palette;
    let inks = inks_stats(&routes, &palette.inks);

    let feature = Feature {
      inks: inks.join(", "),
      inks_count: inks.len(),
      paper: palette.paper.0.to_string(),
    };

    feature
  }

  // TODO move this into a sub object "projectiles"
  pub fn throw_ball(&mut self, ball: Ball, trail: Trail) {
    self.balls.push(ball);
    self.trails.push(trail);
  }

  pub fn render_projectiles<R: Rng>(
    &self,
    rng: &mut R,
    existing_routes: &mut Polylines,
    preserve_area: &PaintMask,
  ) {
    let mut removing_area = preserve_area.clone();

    let mut routes = vec![];

    let clr = self.projectilesclr;

    for ball in self.balls.iter() {
      routes.extend(ball.render(rng, &mut removing_area, clr));
    }
    for trail in self.trails.iter() {
      routes.extend(trail.render(rng, &mut removing_area, clr));
    }

    routes = regular_clip(&routes, preserve_area);

    let mut mask = preserve_area.clone();
    mask.reverse();
    mask.intersects(&removing_area);
    *existing_routes = regular_clip(existing_routes, &mask);

    existing_routes.extend(routes);
  }
}
