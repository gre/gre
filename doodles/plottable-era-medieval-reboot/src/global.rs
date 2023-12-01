use crate::{
  algo::{math1d::smoothstep, polylines::Polylines, wormsfilling::WeightMap},
  objects::{blazon::Blazon, projectile::Projectiles},
  palette::{
    Palette, AMBER, BLACK_PAPER, BLUE_PAPER, DARK_BLUE_PAPER, GOLD_GEL,
    GREY_PAPER, RED_PAPER,
  },
  svgplot::inks_stats,
};
use noise::*;
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
  pub specials: String,  // which specials are used
  pub day_time: String,
  pub castle: String,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Special {
  TrojanHorse,
  // Lockness, // TODO
  Excalibur,
  Montmirail,
  Dragon(usize),
  Chinese,
  Sauroned,
  Barricades,
  EaglesAttack,
  Trebuchets,
  Cyclopes,
}

pub struct GlobalCtx {
  pub palette: Palette,
  pub width: f32,
  pub height: f32,
  pub precision: f32,
  pub specials: HashSet<Special>,
  pub night_time: bool,
  pub full_castle: bool,

  pub sun_xpercentage_pos: f32,

  pub destruction_map: WeightMap,

  // -1.0..0.0: defenders
  // 0.0..1.0: neutral
  // 1.0..2.0: attackers
  pub battlefield_map: WeightMap,

  pub attackers: Blazon,
  pub defenders: Blazon,
  pub defendersclr: usize,
  pub attackersclr: usize,

  pub projectiles: Projectiles,

  // stats that we need to cleanup at the end & for the features
  pub nb_cyclopes: usize,
}

impl GlobalCtx {
  pub fn rand<R: Rng>(
    rng: &mut R,
    width: f32,
    height: f32,
    precision: f32,
    palette: Palette,
    defenders: &Blazon,
    attackers: &Blazon,
  ) -> Self {
    let paper = palette.paper;
    let colors = &palette.inks;

    let attackersclr = 2;
    let mut defendersclr = 0;
    let c = &palette.inks;
    if c[1] != c[2] {
      // there may be an opportunity to use sun color for the defenders if it makes sense...
      if palette.is_acceptable_color_for_blazon(c[1], defenders.clone()) {
        defendersclr = 1;
      }
    }

    let mut specials = HashSet::new();

    if rng.gen_bool(0.02) {
      specials.insert(Special::Barricades);
    } else if rng.gen_bool(0.01) && matches!(attackers, Blazon::Falcon) {
      specials.insert(Special::EaglesAttack);
    } else if rng.gen_bool(0.02) {
      specials.insert(Special::Trebuchets);
    } else if rng.gen_bool(0.02) {
      specials.insert(Special::Cyclopes);
    }

    let dragon_proba_mul = if paper == RED_PAPER { 1.0 } else { 0.08 };

    if rng.gen_bool(0.4 * dragon_proba_mul)
      && matches!(attackers, Blazon::Dragon)
      || rng.gen_bool(0.1 * dragon_proba_mul)
        && matches!(attackers, Blazon::Lys)
    {
      specials.insert(Special::Dragon(rng.gen_range(1..4)));
    } else if rng.gen_bool(0.01) {
      specials.insert(Special::TrojanHorse);
    } else if rng.gen_bool(0.01) {
      let c = c[1];
      if c == GOLD_GEL || c == AMBER {
        specials.insert(Special::Montmirail);
      }
    } else if rng.gen_bool(0.05) && matches!(defenders, Blazon::Dragon) {
      specials.insert(Special::Chinese);
    }

    let destruction_map = gen_destruction_map(rng, width, height, 3.0);
    let battlefield_map = gen_battlefield_map(rng, width, height, 3.0);

    let mut night_time = paper == DARK_BLUE_PAPER
      || rng.gen_bool(0.5) && paper == BLACK_PAPER
      || rng.gen_bool(0.4) && paper == RED_PAPER
      || rng.gen_bool(0.3) && paper == BLUE_PAPER
      || rng.gen_bool(0.2) && paper == GREY_PAPER;
    colors[1];
    if night_time && colors[0] == colors[1] {
      // in monochrome, we allow the night_time to get disabled
      if night_time {
        night_time = rng.gen_bool(0.5);
      }
    }

    let sun_xpercentage_pos =
      0.5 + rng.gen_range(-0.3..0.3) * rng.gen_range(0.0..1.0);
    let full_castle = rng.gen_bool(0.04);

    Self {
      palette,
      width,
      height,
      precision,
      full_castle,
      sun_xpercentage_pos,
      specials,
      night_time,
      destruction_map,
      battlefield_map,
      projectiles: Projectiles::new(),
      attackers: attackers.clone(),
      defenders: defenders.clone(),
      defendersclr,
      attackersclr,
      nb_cyclopes: 0,
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

  pub fn get_golden_color(&self) -> Option<usize> {
    self.palette.get_golden_color()
  }

  pub fn cleanup(&mut self) {
    if self.specials.contains(&Special::Cyclopes) && self.nb_cyclopes == 0 {
      self.specials.remove(&Special::Cyclopes);
    }
  }

  pub fn to_feature(&self, routes: &Polylines) -> Feature {
    let palette = &self.palette;
    let inks = inks_stats(&routes, &palette.inks);

    let feature = Feature {
      inks: inks.join(", "),
      inks_count: inks.len(),
      paper: palette.paper.0.to_string(),
      day_time: if self.night_time {
        "Night".to_string()
      } else {
        "Day".to_string()
      },
      castle: if self.full_castle {
        "Huge".to_string()
      } else {
        "Regular".to_string()
      },
      specials: self
        .specials
        .iter()
        .map(|s| match s {
          Special::TrojanHorse => "TrojanHorse".to_string(),
          Special::Montmirail => "Montmirail".to_string(),
          Special::Chinese => "Chinese".to_string(),
          Special::Dragon(_) => "Dragon".to_string(),
          Special::Excalibur => "Excalibur".to_string(),
          Special::Barricades => "Barricades".to_string(),
          Special::EaglesAttack => "EaglesAttack".to_string(),
          Special::Trebuchets => "Trebuchets".to_string(),
          Special::Cyclopes => "Cyclopes".to_string(),
          Special::Sauroned => "Sauroned".to_string(),
        })
        .collect::<Vec<String>>()
        .join(", "),
    };

    feature
  }
}

fn gen_battlefield_map<R: Rng>(
  rng: &mut R,
  width: f32,
  height: f32,
  precision: f32,
) -> WeightMap {
  let mut destruction_map = WeightMap::new(width, height, precision, 0.0);
  let _perlin = Perlin::new(rng.gen());

  destruction_map.fill_fn(&|(_x, _y)| 2.0);

  destruction_map
}

fn gen_destruction_map<R: Rng>(
  rng: &mut R,
  width: f32,
  height: f32,
  precision: f32,
) -> WeightMap {
  let mut destruction_map = WeightMap::new(width, height, precision, 0.0);
  let perlin = Perlin::new(rng.gen());

  let s1 = rng.gen_range(0.0..999.0);
  let s2 = rng.gen_range(0.0..999.0);
  let f1 = rng.gen_range(1.0..4.0) * rng.gen_range(0.1..1.0);
  let f2 = rng.gen_range(8.0..12.0);

  let thres = rng.gen_range(-1.0..1.0);
  let sat = rng.gen_range(1.5..2.0);
  let fulldestruction = rng.gen_range(-1.0f32..2.0).max(0.0);
  let warp = rng.gen_range(0.0..20.0) * rng.gen_range(0.0..1.0);

  let w = width as f64;
  destruction_map.fill_fn(&|(x, y)| {
    let xf = x as f64 / w;
    let yf = y as f64 / w;
    let x = f1 * xf;
    let y = f1 * yf;
    let n1 = perlin.get([x, y, s1]);
    let x = f2 * xf;
    let y = f2 * yf;
    let n2 = perlin.get([x, y, s2 + warp * n1]);
    let n1 = n1 as f32;
    let n2 = n2 as f32;
    let v = sat * smoothstep(-0.2, 0.2, n1 + thres) * n2
      + fulldestruction * smoothstep(0.5, 0.6, n1 + thres);
    v.max(0.0).min(1.0)
  });

  destruction_map
}
