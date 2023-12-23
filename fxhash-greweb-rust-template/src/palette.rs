use crate::svgplot::Ink;
use crate::svgplot::Paper;
use rand::distributions::WeightedIndex;
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
pub static BLACK_BOLD: Ink = Ink("Black (Bold)", "#1A1A1A", "#000000", 0.6);
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
pub static DARK_BLUE_PAPER: Paper = Paper("Dark Blue", "#191932", true);
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

pub fn pick_weighted<T>(rng: &mut StdRng, choices: &Vec<(T, usize)>) -> T
where
  T: Clone,
{
  let weights = choices
    .iter()
    .map(|(_, weight)| *weight)
    .collect::<Vec<_>>();
  let dist = WeightedIndex::new(&weights).unwrap();
  choices[dist.sample(rng)].0.clone()
}

//~~~ COLORS ~~~//

impl Palette {
  pub fn init(rng: &mut StdRng) -> Self {
    let papers = vec![
      (WHITE_PAPER, 50),
      (BLACK_PAPER, 35),
      (GREY_PAPER, 8),
      (BLUE_PAPER, 4),
      (DARK_BLUE_PAPER, 2),
      (RED_PAPER, 1),
    ];
    let primary_gel_inks = vec![
      (WHITE_GEL, 70),
      (GOLD_GEL, 20),
      (RED_GEL, 10),
      (SILVER_GEL, 10),
      (ORANGE_GEL, 5),
      (BLUE_GEL, 5),
      (GREEN_GEL, 5),
    ];
    let primary_pen_inks = vec![
      (BLACK, 100),
      (AMBER, 30),
      (BLOODY_BREXIT, 20),
      (SEIBOKUBLUE, 20),
      (INAHO, 20),
      (SOFT_MINT, 20),
      (INDIGO, 20),
      (PINK, 20),
      (HOPE_PINK, 20),
      (POPPY_RED, 20),
      (SARGASSO_SEA, 10),
      (AURORA_BOREALIS, 10),
      (PUMPKIN, 10),
      (RED_DRAGON, 10),
      (FIRE_AND_ICE, 10),
      (MOONSTONE, 10),
      (TURQUOISE, 10),
      (IMPERIAL_PURPLE, 5),
      (SHERWOOD_GREEN, 5),
      (EVERGREEN, 5),
      (SPRING_GREEN, 5),
    ];

    let paper = pick_weighted(rng, &papers);
    let mut inks = if paper == BLUE_PAPER {
      vec![WHITE_GEL, BLACK_BOLD, BLACK_BOLD]
    } else if paper.is_dark() {
      vec![
        pick_weighted(rng, &primary_gel_inks),
        pick_weighted(rng, &primary_gel_inks),
        pick_weighted(rng, &primary_gel_inks),
      ]
    } else {
      vec![
        pick_weighted(rng, &primary_pen_inks),
        pick_weighted(rng, &primary_pen_inks),
        pick_weighted(rng, &primary_pen_inks),
      ]
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
