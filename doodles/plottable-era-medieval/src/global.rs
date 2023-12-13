use crate::{
  algo::{
    math1d::smoothstep, paintmask::PaintMask, polylines::Polylines,
    wormsfilling::WeightMap,
  },
  effects::Effects,
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
  Excalibur,
  Montmirail,
  Dragon(usize),
  Chinese,
  Sauroned,
  Barricades,
  EaglesAttack,
  Trebuchets,
  Cyclopes,
  Sandbox,
}

pub struct GlobalCtx {
  pub palette: Palette,
  pub width: f32,
  pub height: f32,
  pub precision: f32,
  pub specials: HashSet<Special>,
  pub night_time: bool,
  pub full_castle: bool,
  pub castle_on_sea: bool,
  pub no_sea: bool,
  pub rope_len_base: f32,
  pub yhorizon: f32,
  pub trebuchets_should_shoot: bool,
  pub archers_should_shoot: bool,
  pub sun_xpercentage_pos: f32,
  pub destruction_map: WeightMap,
  pub attackers: Blazon,
  pub defenders: Blazon,
  pub defendersclr: usize,
  pub attackersclr: usize,
  pub fireball_color: usize,
  pub fire_proba: f64,
  pub arbitrary_convoys_proba: f64,
  pub projectiles: Projectiles,
  pub effects: Effects,
  pub nb_cyclopes: usize,
  pub is_sandbox: bool,
  pub has_leader_been_picked: bool,
}

impl GlobalCtx {
  pub fn rand<R: Rng>(
    rng: &mut R,
    paintref: &PaintMask,
    width: f32,
    height: f32,
    precision: f32,
    palette: Palette,
    defenders: &Blazon,
    attackers: &Blazon,
  ) -> Self {
    let paper = palette.paper;
    let colors = &palette.inks;

    let fireball_color = if rng.gen_bool(0.5) { 0 } else { 1 };

    let castle_on_sea = rng.gen_bool(0.2);
    let no_sea = !castle_on_sea && rng.gen_bool(0.15);

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

    let is_sandbox =
      night_time && rng.gen_bool(0.01) || !night_time && rng.gen_bool(0.001);

    let trebuchets_should_shoot = rng.gen_bool(0.3);
    let archers_should_shoot = rng.gen_bool(0.2);

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

    if is_sandbox {
      specials.insert(Special::Sandbox);
    } else {
      if rng.gen_bool(0.03) && !castle_on_sea {
        specials.insert(Special::Barricades);
      } else if rng.gen_bool(0.02) && matches!(attackers, Blazon::Falcon) {
        specials.insert(Special::EaglesAttack);
      } else if trebuchets_should_shoot && rng.gen_bool(0.06) && !castle_on_sea
      {
        specials.insert(Special::Trebuchets);
      } else if rng.gen_bool(0.02) && !castle_on_sea {
        specials.insert(Special::Cyclopes);
      }

      let dragon_proba_mul = if paper == RED_PAPER { 1.0 } else { 0.08 };

      if rng.gen_bool(0.5 * dragon_proba_mul)
        && matches!(attackers, Blazon::Dragon)
        || rng.gen_bool(0.15 * dragon_proba_mul)
          && matches!(attackers, Blazon::Lys)
      {
        specials.insert(Special::Dragon(rng.gen_range(1..4)));
      } else if rng.gen_bool(0.01)
        && !no_sea
        && !castle_on_sea
        && specials.is_empty()
      {
        specials.insert(Special::TrojanHorse);
      } else if rng.gen_bool(0.02) {
        let c = c[1];
        if (c == GOLD_GEL || c == AMBER)
          && !specials.contains(&Special::Barricades)
          && !no_sea
          && !castle_on_sea
        {
          specials.insert(Special::Montmirail);
        }
      } else if rng.gen_bool(0.05)
        && matches!(defenders, Blazon::Dragon)
        && !castle_on_sea
      {
        specials.insert(Special::Chinese);
      }
    }

    let destruction_map = gen_destruction_map(rng, width, height, 3.0);

    let sun_xpercentage_pos =
      0.5 + rng.gen_range(-0.3..0.3) * rng.gen_range(0.0..1.0);
    let full_castle = rng.gen_bool(0.04);

    let yhorizon = if no_sea {
      height
    } else {
      rng.gen_range(0.5..0.8) * height
    };

    let fire_proba = (rng.gen_range(-1.0f64..1.0) * rng.gen_range(0.0..1.0)
      + if specials.contains(&Special::Cyclopes) {
        1.
      } else {
        0.
      })
    .max(0.01)
    .min(0.9);

    let arbitrary_convoys_proba =
      rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);

    Self {
      is_sandbox,
      castle_on_sea,
      no_sea,
      palette,
      width,
      height,
      precision,
      full_castle,
      sun_xpercentage_pos,
      specials,
      night_time,
      destruction_map,
      projectiles: Projectiles::new(),
      effects: Effects::new(paintref),
      attackers: attackers.clone(),
      defenders: defenders.clone(),
      fireball_color,
      defendersclr,
      attackersclr,
      nb_cyclopes: 0,
      trebuchets_should_shoot,
      archers_should_shoot,
      has_leader_been_picked: false,
      rope_len_base: rng.gen_range(0.0..300.0),
      yhorizon,
      fire_proba,
      arbitrary_convoys_proba,
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

  pub fn render_projectiles<R: Rng>(
    &mut self,
    rng: &mut R,
    routes: &mut Polylines,
    paint: &PaintMask,
    mask_with_framing: &PaintMask,
  ) {
    let mut projectiles = self.projectiles.clone();
    projectiles.resolve_and_render(
      rng,
      self,
      &paint,
      routes,
      &mask_with_framing,
    );
    self.projectiles = projectiles;
  }

  pub fn finalize(&mut self) {
    self.effects.finalize();
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
      castle: if self.castle_on_sea {
        "On The Sea".to_string()
      } else if self.full_castle {
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
          Special::Sandbox => "Sandbox".to_string(),
        })
        .collect::<Vec<String>>()
        .join(", "),
    };

    feature
  }
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
  let fulldestruction =
    rng.gen_range(-5.0f32..2.0).max(0.0) * rng.gen_range(0.0..1.0);
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
