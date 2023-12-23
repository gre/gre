use std::f32::consts::PI;

use palette::Palette;
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – TEMPLATE
 */
use rand::prelude::*;
use serde::Serialize;
use serde_json::json;
use wasm_bindgen::prelude::*;
mod koda;
use koda::*;
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

  for _ in 0..40 {
    let s =
      pad + rng.gen_range(pad..(width / 2. - pad)) * rng.gen_range(0.1..0.9);
    routes.push((
      rng.gen_range(0..palette.inks.len()),
      spiral_optimized(
        rng.gen_range(s..(width - s)),
        rng.gen_range(s..(height - s)),
        s - pad,
        rng.gen_range(0.8..2.4),
        0.5,
      ),
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

fn euclidian_dist((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

fn spiral_optimized(
  x: f32,
  y: f32,
  radius: f32,
  dr: f32,
  approx: f32,
) -> Vec<(f32, f32)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut last = (0., 0.);
  let mut a = 0f32;
  loop {
    let p = (x + r * a.cos(), y + r * a.sin());
    if route.is_empty() || euclidian_dist(last, p) > approx {
      last = p;
      route.push(p);
    }
    let da = 0.2 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
}
