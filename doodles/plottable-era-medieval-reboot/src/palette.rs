use crate::objects::blazon::Blazon;
use crate::svgplot::Ink;
use crate::svgplot::Paper;
use rand::prelude::*;
use serde::Serialize;

pub static GOLD_GEL: Ink = Ink("Gold Gel", "#D8B240", "#FFE38C", 0.6);
pub static RED_GEL: Ink = Ink("Red Gel", "#BF738C", "#D880A6", 0.6);
pub static ORANGE_GEL: Ink = Ink("Orange Gel", "#B27333", "#E68C4D", 0.35);
pub static BLUE_GEL: Ink = Ink("Blue Gel", "#338CFF", "#4D8CFF", 0.35);
pub static GREEN_GEL: Ink = Ink("Green Gel", "#00B2A6", "#19CCBF", 0.35);
pub static SILVER_GEL: Ink = Ink("Silver Gel", "#CCCCCC", "#FFFFFF", 0.6);
pub static WHITE_GEL: Ink = Ink("White Gel", "#E5E5E5", "#FFFFFF", 0.35);
pub static BLACK: Ink = Ink("Black", "#1A1A1A", "#000000", 0.35);
pub static SEIBOKUBLUE: Ink =
  Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
pub static INAHO: Ink = Ink("iroshizuku ina-ho", "#ba6", "#7f6a33", 0.35);
pub static IMPERIAL_PURPLE: Ink =
  Ink("Imperial Purple", "#4D0066", "#260F33", 0.35);
pub static SHERWOOD_GREEN: Ink =
  Ink("Sherwood Green", "#337239", "#194D19", 0.35);
pub static EVERGREEN: Ink = Ink("Evergreen", "#4D6633", "#263319", 0.35);
pub static SOFT_MINT: Ink = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
pub static SPRING_GREEN: Ink = Ink("Spring Green", "#783", "#350", 0.35);
pub static MOONSTONE: Ink = Ink("Moonstone", "#bbb", "#ddd", 0.35);
pub static TURQUOISE: Ink = Ink("Turquoise", "#00B4E6", "#005A8C", 0.35);
pub static SARGASSO_SEA: Ink = Ink("Sargasso Sea", "#162695", "#111962", 0.35);
pub static INDIGO: Ink = Ink("Indigo", "#667599", "#334D66", 0.35);
pub static AURORA_BOREALIS: Ink =
  Ink("Aurora Borealis", "#009999", "#004D66", 0.35);
pub static PUMPKIN: Ink = Ink("Pumpkin", "#FF8033", "#E54D00", 0.35);
pub static PINK: Ink = Ink("Pink", "#fd728e", "#E5604D", 0.35);
pub static HOPE_PINK: Ink = Ink("Hope Pink", "#fc839b", "#E53399", 0.35);
pub static AMBER: Ink = Ink("Amber", "#FFC745", "#FF8000", 0.35);
pub static POPPY_RED: Ink = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
pub static RED_DRAGON: Ink = Ink("Red Dragon", "#9e061a", "#5b0a14", 0.35);
pub static FIRE_AND_ICE: Ink = Ink("Fire And Ice", "#00BEDE", "#006478", 0.35);
pub static BLOODY_BREXIT: Ink =
  Ink("Bloody Brexit", "#05206B", "#2E0033", 0.35);

pub static WHITE_PAPER: Paper = Paper("White", "#fff", false);
pub static BLACK_PAPER: Paper = Paper("Black", "#202020", true);
pub static GREY_PAPER: Paper = Paper("Grey", "#959fa8", true);
pub static BLUE_PAPER: Paper = Paper("Blue", "#4cbadc", true);
pub static DARK_BLUE_PAPER: Paper = Paper("Blue", "#191932", true);
pub static RED_PAPER: Paper = Paper("Red", "#aa0000", true);

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct Palette {
  pub inks: Vec<Ink>,
  pub paper: Paper,
  pub monochrome: bool,
}
impl Palette {
  pub fn init<R: Rng>(rng: &mut R, blazon: Blazon) -> Self {
    // TODO FIXME to figure out:
    // is black paper necessarily night time? therefore do we make the sky night?
    // maybe the sun is the moon then, and it's white?
    // but then what would gold be? the attackers and castle lights? could work
    // we could try some nice combination as shared before too.
    // love the idea to have a monochrome white, even if it's light time

    // lights in attackers and castle if it's night time

    // TODO blue paper
    // TODO red paper??

    // colors
    // 0 : base color for most of the things
    // 1 : sun and lights
    // 2 : attacker color

    // TODO rework with reprensenting things as array and pow of probability for the distribution so we really master the ordering.

    let papers_choices = 6;
    let i = (rng.gen_range(0.0..papers_choices as f32)
      * rng.gen_range(0.0..1.0)) as usize;

    let (mut inks, paper) = match i {
      0 => {
        let mut base = WHITE_GEL;
        if rng.gen_bool(1. / 200.) {
          base = GOLD_GEL;
        }
        let sun = if rng.gen_bool(0.8) {
          GOLD_GEL
        } else {
          WHITE_GEL
        };
        let blazon_color = match blazon {
          // gels
          Blazon::Lys => {
            if rng.gen_bool(0.6) {
              GOLD_GEL
            } else if rng.gen_bool(0.7) {
              WHITE_GEL
            } else {
              BLUE_GEL
            }
          }
          Blazon::Dragon => {
            if rng.gen_bool(0.9) {
              RED_GEL
            } else {
              ORANGE_GEL
            }
          }
          Blazon::Falcon => {
            if rng.gen_bool(0.9) {
              SILVER_GEL
            } else {
              GREEN_GEL
            }
          }
        };
        let colors = vec![base, sun, blazon_color];
        (colors, BLACK_PAPER)
      }
      1 => {
        let base = if rng.gen_bool(0.8) {
          BLACK
        } else if rng.gen_bool(0.4) {
          MOONSTONE
        } else if rng.gen_bool(0.4) {
          INDIGO
        } else if rng.gen_bool(0.4) {
          SEIBOKUBLUE
        } else if rng.gen_bool(0.4) {
          BLOODY_BREXIT
        } else if rng.gen_bool(0.4) {
          INAHO
        } else {
          IMPERIAL_PURPLE
        };
        let sun = if rng.gen_bool(0.8) {
          AMBER
        } else if rng.gen_bool(0.4) {
          POPPY_RED
        } else if rng.gen_bool(0.4) {
          INAHO
        } else if rng.gen_bool(0.4) {
          PINK
        } else {
          HOPE_PINK
        };
        let blazon_color = match blazon {
          Blazon::Lys => {
            if rng.gen_bool(0.66) {
              SEIBOKUBLUE
            } else if rng.gen_bool(0.4) {
              INAHO
            } else if rng.gen_bool(0.4) {
              FIRE_AND_ICE
            } else if rng.gen_bool(0.4) {
              TURQUOISE
            } else {
              SARGASSO_SEA
            }
          }
          Blazon::Dragon => {
            if rng.gen_bool(0.66) {
              POPPY_RED
            } else if rng.gen_bool(0.66) {
              PUMPKIN
            } else {
              RED_DRAGON
            }
          }
          Blazon::Falcon => {
            if rng.gen_bool(0.66) {
              SPRING_GREEN
            } else if rng.gen_bool(0.4) {
              SOFT_MINT
            } else if rng.gen_bool(0.4) {
              EVERGREEN
            } else if rng.gen_bool(0.4) {
              EVERGREEN
            } else if rng.gen_bool(0.66) {
              AURORA_BOREALIS
            } else {
              SHERWOOD_GREEN
            }
          }
        };
        let colors = vec![base, sun, blazon_color];
        (colors, WHITE_PAPER)
      }
      2 => {
        let blazon_color = match blazon {
          Blazon::Lys => {
            if rng.gen_bool(0.66) {
              GOLD_GEL
            } else {
              WHITE_GEL
            }
          }
          Blazon::Dragon => WHITE_GEL,
          Blazon::Falcon => WHITE_GEL,
        };
        let colors = vec![BLACK, WHITE_GEL, blazon_color];
        (colors, GREY_PAPER)
      }
      3 => {
        let blazon_color = match blazon {
          Blazon::Lys => GOLD_GEL,
          _ => WHITE_GEL,
        };
        let colors = vec![WHITE_GEL, WHITE_GEL, blazon_color];
        (colors, DARK_BLUE_PAPER)
      }
      4 => {
        let blazon_color = match blazon {
          Blazon::Lys => WHITE_GEL,
          _ => BLACK,
        };
        let colors = vec![BLACK, WHITE_GEL, blazon_color];
        (colors, BLUE_PAPER)
      }
      _ => {
        let sun = if rng.gen_bool(0.3) {
          GOLD_GEL
        } else {
          WHITE_GEL
        };

        let blazon_color = match blazon {
          Blazon::Lys => GOLD_GEL,
          _ => WHITE_GEL,
        };
        let colors = vec![BLACK, sun, blazon_color];
        (colors, RED_PAPER)
      }
    };

    // going full monochrome
    let monochrome = if rng.gen_bool(0.05) {
      let r = inks[if rng.gen_bool(0.8) {
        0
      } else if rng.gen_bool(0.6) {
        1
      } else {
        2
      }];
      inks[1] = r;
      inks[2] = r;
      true
    } else {
      false
    };

    Self {
      inks,
      paper,
      monochrome,
    }
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
