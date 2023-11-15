mod algo;
mod frame;
mod fxhash;
mod objects;
mod performance;
mod svgplot;

use algo::math1d::mix;
use algo::packing::VCircle;
use fxhash::*;
use objects::army::house::get_duel_houses;
use objects::army::ArmyOnMountain;
use objects::castle::Castle;
use objects::mountains::*;
use objects::sea::Sea;
use objects::sky::MedievalSky;
use rand::prelude::*;
use serde::Serialize;
use svgplot::*;
use wasm_bindgen::prelude::*;
mod epictitle;
use algo::clipping::regular_clip;
use algo::paintmask::*;
use algo::text::*;
use epictitle::*;
use frame::medieval_frame;
use performance::*;

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

#[wasm_bindgen]
pub fn render(
  hash: String,
  width: f64,
  height: f64,
  pad: f64,
  precision: f64,
  fontdata: Vec<u8>,
  mask_mode: bool,
  debug: bool,
) -> String {
  let mut perf = PerfRecords::start(debug);

  perf.span("init");
  let mut rng = rng_from_hash(&hash);
  let mut font = load_font(&fontdata);
  perf.span_end("init");

  // Colors

  let red_gel = Ink("Red Gel", "#BF738C", "#D880A6", 0.35);
  let orange_gel = Ink("Orange Gel", "#B27333", "#E68C4D", 0.35);
  let blue_gel = Ink("Blue Gel", "#338CFF", "#4D8CFF", 0.35);
  let green_gel = Ink("Green Gel", "#00B2A6", "#19CCBF", 0.35);

  let gold_gel = Ink("Gold Gel", "#D8B240", "#FFE38C", 0.6);
  let silver_gel = Ink("Silver Gel", "#CCCCCC", "#FFFFFF", 0.6);
  let white_gel = Ink("White Gel", "#E5E5E5", "#FFFFFF", 0.35);
  let black = Ink("Black", "#1A1A1A", "#000000", 0.35);
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let inaho = Ink("iroshizuku ina-ho", "#ba6", "#7f6a33", 0.35);
  let imperial_purple = Ink("Imperial Purple", "#4D0066", "#260F33", 0.35);
  let sherwood_green = Ink("Sherwood Green", "#337239", "#194D19", 0.35);
  let evergreen = Ink("Evergreen", "#4D6633", "#263319", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let turquoise = Ink("Turquoise", "#00B4E6", "#005A8C", 0.35);
  let sargasso_sea = Ink("Sargasso Sea", "#162695", "#111962", 0.35);
  let indigo = Ink("Indigo", "#667599", "#334D66", 0.35);
  let aurora_borealis = Ink("Aurora Borealis", "#009999", "#004D66", 0.35);
  let pumpkin = Ink("Pumpkin", "#FF8033", "#E54D00", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);
  let hope_pink = Ink("Hope Pink", "#fc839b", "#E53399", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let poppy_red = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
  let red_dragon = Ink("Red Dragon", "#9e061a", "#5b0a14", 0.35);
  let fire_and_ice = Ink("Fire And Ice", "#00BEDE", "#006478", 0.35);
  let bloody_brexit = Ink("Bloody Brexit", "#05206B", "#2E0033", 0.35);

  // ideas for color:
  // monochrome => any color
  // bicolor => 0 is mostly always black
  // bicolor => maybe seilor seiboku + ina ho
  // bicolor => maybe grey + ina ho
  // bicolor => black + grey

  // FIXME to figure out:
  // is black paper necessarily night time? therefore do we make the sky night?
  // maybe the sun is the moon then, and it's white?
  // but then what would gold be? the attackers and castle lights? could work
  // we could try some nice combination as shared before too.
  // love the idea to have a monochrome white, even if it's light time???

  let white_paper = Paper("White", "#fff", false);
  let black_paper = Paper("Black", "#202020", true);
  let grey_paper = Paper("Grey", "#959fa8", true);
  // TODO blue paper
  // TODO red paper??

  // colors
  // 0 : mountains & objects
  // 1 : sun
  // 2 : human lights / fire -> MAYBE IT'S THE SAME COLOR!
  let (mut colors, mut paper) = (vec![black, amber, poppy_red], white_paper);

  if rng.gen_bool(0.7) {
    colors = vec![white_gel, gold_gel, gold_gel];
    paper = black_paper;
  }

  if rng.gen_bool(0.1) {
    colors = vec![black, white_gel, black];
    paper = grey_paper;
  }

  // Make the scene
  perf.span("scene");
  let (attacker_house, defender_house) = get_duel_houses(&mut rng);

  let mut routes = vec![];
  let mut paint = PaintMask::new(precision, width, height);

  let mut decoration_routes = vec![];
  let framingw = 0.05 * width;

  perf.span("epic title");
  let txt = epic_title(&mut rng);
  let fontsize = width / 26.0;
  let iterations = 5000;
  let density = 4.0;
  let growpad = 2.0;
  decoration_routes.extend(draw_font_with_worms_filling(
    &mut rng,
    &mut font,
    &mut paint,
    fontsize,
    (
      pad + framingw + 0.6 * fontsize,
      height - pad - framingw - 1.8 * fontsize,
    ),
    txt.as_str(),
    0,
    iterations,
    density,
    growpad,
  ));
  perf.span_end("epic title");

  perf.span("framing");
  let golden_frame = colors[1] == gold_gel && rng.gen_bool(0.3);
  let clr = if golden_frame { 1 } else { 0 };
  decoration_routes.extend(medieval_frame(
    &mut rng, &mut paint, width, height, pad, framingw, clr,
  ));
  perf.span_end("framing");

  //  mountains
  perf.span("mountains_front");
  // TODO better peaky shapes. not too annoying. can sometimes disappear
  let ystart = rng.gen_range(0.8..0.95) * height;
  let ybase = height;
  let clr = 0;
  let mut mountains = Mountains::init(clr, ybase, ystart, width);
  routes.extend(mountains.render(&mut rng, &mut paint));
  perf.span_end("mountains_front");

  // TODO: here opportunity to have front facing attackers

  perf.span("sea");
  let yhorizon = rng.gen_range(0.6..0.8) * height;
  let boat_color = 0;
  let sea = Sea::from(&paint, yhorizon, boat_color);
  let sea_routes = sea.render(&mut rng, &mut paint);
  perf.span_end("sea");

  perf.span("mountains");
  // TODO rework mountains...
  let ystart = yhorizon - rng.gen_range(0.05..0.15) * height;
  let ybase = yhorizon;
  let clr = 0;
  let mut mountains = Mountains::init(clr, ybase, ystart, width);
  routes.extend(mountains.render(&mut rng, &mut paint));
  perf.span_end("mountains");

  // TODO calculate where the castle will be based on mountains
  // we will use it to orient attackers

  // FIXME: ohno the attackers are in front of the mountains, but i need the ridge before...
  // we need to split the render logic (dont call it render) and build the mountain in the init
  // so we can ridge at any time

  perf.span("attackers");
  let ridge = mountains.ridge();
  let army = ArmyOnMountain {
    yhorizon,
    ridge,
    width,
    house: attacker_house,
  };
  routes.extend(army.render(&mut rng, &mut paint));
  perf.span_end("attackers");

  perf.span("castle");
  let castle = Castle {
    pos: (
      width / 2.0,
      mix(
        ystart,
        ybase,
        rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
      ),
    ),
    width: width * rng.gen_range(0.3..0.5),
    scale: 1.0,
    clr: 0,
    ybase: yhorizon,
    wallh: rng.gen_range(0.1..0.15) * height,
    wall: true,
    left_tower: true,
    right_tower: true,
    dark_wall: rng.gen_bool(0.5),
    chapel: rng.gen_bool(0.5),
    dark_chapel: rng.gen_bool(0.5),
    destructed_wall: rng.gen_bool(0.5),
    portcullis: rng.gen_bool(0.5),
  };
  routes.extend(castle.render(&mut rng, &mut paint));
  perf.span_end("castle");

  perf.span("sky");
  let sky = MedievalSky::rand(&mut rng, width, height, pad);
  routes.extend(sky.render(&mut rng, &mut paint));

  perf.span_end("sky");

  perf.span("reflect_shapes");
  let probability_par_color = vec![0.05, 0.1, 0.12];

  routes.extend(sea.reflect_shapes(
    &mut rng,
    &mut paint,
    &routes,
    probability_par_color,
  ));
  perf.span_end("reflect_shapes");

  routes.extend(sea_routes);
  routes.extend(decoration_routes);

  perf.span_end("scene");

  let inks = inks_stats(&routes, &colors);

  let feature = Feature {
    inks: inks.join(", "),
    inks_count: inks.len(),
    paper: paper.0.to_string(),
  };

  let feature_json = serde_json::to_string(&feature).unwrap();

  let palette_json = serde_json::to_string(&Palette {
    paper,
    primary: colors[0 % colors.len()],
    secondary: colors[1 % colors.len()],
    third: colors[2 % colors.len()],
  })
  .unwrap();

  let layers = make_layers_from_routes_colors(&routes, &colors, mask_mode);

  let svg = make_document(
    hash.as_str(),
    feature_json,
    palette_json,
    width,
    height,
    mask_mode,
    paper.1,
    &layers,
    Some(perf),
  );

  svg
}
