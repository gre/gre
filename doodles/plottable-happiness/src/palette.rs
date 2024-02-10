use crate::svgplot::Ink;
use crate::svgplot::Paper;
use rand::prelude::*;
use serde::Serialize;

pub static GOLD_GEL: Ink = Ink("Gold Gel", "#D8B240", "#FFE38C", 0.6);
pub static SILVER_GEL: Ink = Ink("Silver Gel", "#CCCCCC", "#FFFFFF", 0.6);
pub static WHITE_GEL: Ink = Ink("White Gel", "#E5E5E5", "#FFFFFF", 0.35);
pub static BLACK_BOLD: Ink = Ink("Black (Bold)", "#1A1A1A", "#000000", 0.6);
pub static BLACK: Ink = Ink("Black", "#1A1A1A", "#000000", 0.35);

pub static RED_PAPER: Paper = Paper("Red", "#aa0000", true);
pub static BLACK_PAPER: Paper = Paper("Black", "#111", true);

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct Palette {
  pub inks: Vec<Ink>,
  pub paper: Paper,
}

//~~~ COLORS ~~~//

impl Palette {
  pub fn init(rng: &mut StdRng) -> Self {
    let black = rng.gen_bool(0.2);
    let paper = if black { BLACK_PAPER } else { RED_PAPER };

    let mut inks = if black {
      vec![SILVER_GEL, GOLD_GEL]
    } else if rng.gen_bool(0.5) {
      vec![SILVER_GEL, GOLD_GEL, BLACK_BOLD]
    } else {
      vec![WHITE_GEL, BLACK]
    };

    if rng.gen_bool(0.4) {
      inks.shuffle(rng);
    }
    inks.truncate(rng.gen_range(1..4));

    Self { inks, paper }
  }

  pub fn to_json(&self) -> String {
    let paper = self.paper;
    let inks = &self.inks;
    serde_json::to_string(&PaletteJson {
      paper,
      primary: inks[0 % inks.len()],
      secondary: inks[1 % inks.len()],
      third: inks[2 % inks.len()],
    })
    .unwrap()
  }
}

// This is also returned in the SVG to have more metadata for the JS side to render a digital version
#[derive(Clone, Serialize)]
struct PaletteJson {
  primary: Ink,
  secondary: Ink,
  third: Ink,
  paper: Paper,
}
