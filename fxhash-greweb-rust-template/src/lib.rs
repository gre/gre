/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – TEMPLATE
 */
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;
mod fxhash;
use fxhash::*;
mod svgplot;
use svgplot::*;

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
  mask_mode: bool,
) -> String {
  let mut rng = rng_from_hash(&hash);

  let white = Ink("White", "#fff", "#eee", 0.35);
  let black = Ink("Black", "#111", "#000", 0.35);
  let turquoise = Ink("Turquoise", "#0AD", "#058", 0.35);
  let amber = Ink("Amber", "#FB2", "#F80", 0.35);
  let white_paper = Paper("White", "#fff", false);
  let black_paper = Paper("Black", "#222", true);

  let dark_mode = rng.gen_bool(0.5);

  let paper = if dark_mode { black_paper } else { white_paper };
  let colors = if dark_mode {
    vec![white]
  } else {
    vec![black, turquoise, amber]
  };

  let mut routes = vec![];

  for _i in 0..50 {
    routes.push((
      rng.gen_range(0..colors.len()),
      vec![
        (
          rng.gen_range(pad..(width - pad)),
          rng.gen_range(pad..(height - pad)),
        ),
        (
          rng.gen_range(pad..(width - pad)),
          rng.gen_range(pad..(height - pad)),
        ),
      ],
    ));
  }

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
  );

  svg
}
