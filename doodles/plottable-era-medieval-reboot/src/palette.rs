use rand::prelude::*;

use crate::svgplot::Ink;
use crate::svgplot::Paper;

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
// TODO add DARK_BLUE_PAPER
// TODO add BLUE_PAPER

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn palette<R: Rng>(rng: &mut R) -> (Vec<Ink>, Paper) {
  // ideas for color:
  // monochrome => any color
  // bicolor => 0 is mostly always black
  // bicolor => maybe seilor seiboku + ina ho
  // bicolor => maybe grey + ina ho
  // bicolor => black + grey

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
  // 0 : mountains & objects
  // 1 : sun
  // 2 : human lights / fire -> MAYBE IT'S THE SAME COLOR!
  let (mut colors, mut paper) = (vec![BLACK, AMBER, POPPY_RED], WHITE_PAPER);

  if rng.gen_bool(0.7) {
    colors = vec![WHITE_GEL, GOLD_GEL, RED_GEL];
    paper = BLACK_PAPER;
  }

  if rng.gen_bool(0.1) {
    colors = vec![BLACK, WHITE_GEL, WHITE_GEL];
    paper = GREY_PAPER;
  }

  (colors, paper)
}
