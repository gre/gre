use palette::Palette;
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – TEMPLATE
 */
use rand::prelude::*;
use serde::Serialize;
use serde_json::json;
use wasm_bindgen::prelude::*;
mod fxhash;
use fxhash::*;
mod svgplot;
use svgplot::*;
mod performance;
use performance::*;
mod global;
use global::*;
mod palette;

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
  mask_mode: bool,
  debug: bool,
) -> String {
  let mut perf = PerfRecords::start(debug);

  let mut rng = rng_from_hash(&hash);

  perf.span("all", &vec![]);

  let palette = Palette::init(&mut rng);
  let global = GlobalCtx::rand(&mut rng, width, height, precision, &palette);

  let mut routes = vec![];

  for _i in 0..50 {
    routes.push((
      rng.gen_range(0..palette.inks.len()),
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

  let feature = global.to_feature(&routes);
  let feature_json = feature.to_json();
  let palette_json = palette.to_json();

  let layers = make_layers_from_routes_colors(
    &routes,
    &palette.inks,
    mask_mode,
    2.0 * precision,
  );

  perf.span_end("all", &vec![]);

  let mut attributes = vec![];

  if debug {
    attributes.push(format!("data-perf='{}'", json!(perf.end()).to_string()));
  }

  let svg = make_document(
    hash.as_str(),
    feature_json,
    palette_json,
    width,
    height,
    mask_mode,
    palette.paper.1,
    &layers,
    &attributes,
  );

  svg
}
