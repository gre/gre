mod algo;
mod frame;
mod fxhash;
mod objects;
mod palette;
mod performance;
mod svgplot;

use algo::clipping::clip_routes_with_colors;
use algo::math1d::mix;
use fxhash::*;
use objects::army::ArmyOnMountain;
use objects::blazon::get_duel_houses;
use objects::castle::Castle;
use objects::mountains::front::FrontMountains;
use objects::mountains::*;
use objects::sea::Sea;
use objects::sky::MedievalSky;
use palette::palette;
use palette::GOLD_GEL;
use rand::prelude::*;
use serde::Serialize;
use svgplot::*;
use wasm_bindgen::prelude::*;
mod epictitle;
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
  width: f32,
  height: f32,
  pad: f32,
  precision: f32,
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
  let (colors, paper) = palette(&mut rng);

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
  let golden_frame = colors[1] == GOLD_GEL && rng.gen_bool(0.3);
  let clr = if golden_frame { 1 } else { 0 };
  decoration_routes.extend(medieval_frame(
    &mut rng, &mut paint, width, height, pad, framingw, clr,
  ));
  perf.span_end("framing");

  let yhorizon = rng.gen_range(0.5..0.6) * height;

  //  mountains
  perf.span("mountains_front");
  // TODO better peaky shapes. not too annoying. can sometimes disappear
  let ystart = mix(yhorizon, height, rng.gen_range(0.0..1.0));
  let ybase = height;
  let clr = 0;
  let mut mountains = FrontMountains {
    clr,
    ybase,
    ystart,
    width,
  };
  routes.extend(mountains.render(&mut rng, &mut paint));
  perf.span_end("mountains_front");

  // TODO: here opportunity to have front facing attackers

  perf.span("sea");
  let boat_color = 0;
  let sea = Sea::from(&paint, yhorizon, boat_color);
  let sea_routes = sea.render(&mut rng, &mut paint);
  perf.span_end("sea");

  perf.span("mountains");
  let ymax = mix(0.0, yhorizon, 0.6);
  let count = rng.gen_range(2..8);
  let mountains =
    MountainsV2::rand(&mut rng, 0, width, height, yhorizon, ymax, count);
  perf.span_end("mountains");

  let army: ArmyOnMountain = ArmyOnMountain {
    house: attacker_house,
  };

  for mountain in mountains.mountains {
    perf.span("attackers");
    routes.extend(army.render(&mut rng, &mut paint, &mountain));
    perf.span_end("attackers");

    perf.span("mountains");
    routes.extend(mountain.render(&mut rng, &mut paint));
    perf.span_end("mountains");

    if let Some(castle) = &mountain.castle {
      perf.span("castle");
      let castle = Castle {
        pos: castle.position,
        width: castle.width,
        scale: 1.0,
        clr: 0,
        ybase: yhorizon,
        wallh: rng.gen_range(0.05..0.1) * height,
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
    }
  }

  perf.span("sky");
  let sky = MedievalSky::rand(&mut rng, width, height, pad);
  // prevent sky to glitch inside the sea
  let is_below_horizon = |(_x, y): (f32, f32)| y > yhorizon;
  let sky_routes = clip_routes_with_colors(
    &sky.render(&mut rng, &mut paint),
    &is_below_horizon,
    1.0,
    5,
  );
  routes.extend(sky_routes);
  perf.span_end("sky");

  perf.span("reflect_shapes");
  let probability_par_color = vec![0.08, 0.1, 0.2];

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
