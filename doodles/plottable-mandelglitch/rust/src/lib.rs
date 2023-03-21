/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Mandelglitch
 */
mod utils;
use contour::ContourBuilder;
use noise::*;
use num_complex::Complex;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::ops::RangeInclusive;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use wasm_bindgen::prelude::*;

// Function called from JS to get the SVG document
#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
  let opts = val.into_serde().unwrap();
  let (doc, _) = art(&opts, true);
  let str = doc.to_string();
  return str;
}

// Input to the art function
#[derive(Deserialize)]
pub struct Opts {
  pub hash: String,
  pub width: f64,
  pub height: f64,
  pub pad: f64,
  // Properties from user
  pub lightness: f64,
  pub color_cutoff: usize,
  pub color_offset: usize,
  pub layers_count: usize,
  pub noise_effect: f64,
  pub kaleidoscope: bool,
  pub kaleidoscope_mod: usize,
}

// Feature tells caracteristics of a given art variant
// It is returned in the .SVG file
#[derive(Clone, Serialize)]
pub struct Feature {
  // which inks are used
  pub inks: String,
  // how much inks are used
  pub inks_count: usize,
  // which paper is used
  pub paper: String,
  // symmetry style
  pub symmetry: String,
  // density style
  pub density: String,
}

#[derive(Clone, Copy, Serialize)]
pub struct Ink(&'static str, &'static str, &'static str, f64);
#[derive(Clone, Copy, Serialize)]
pub struct Paper(&'static str, &'static str, bool);

// This is also returned in the SVG to have more metadata for the JS side to render a digital version
#[derive(Clone, Serialize)]
pub struct Palette {
  pub primary: Ink,
  pub secondary: Ink,
  pub third: Ink,
  pub paper: Paper,
}

// This is the main art function that will render the generative art piece
pub fn art(opts: &Opts, mask_mode: bool) -> (svg::Document, Feature) {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;
  let lightness = opts.lightness;
  let color_cutoff = opts.color_cutoff;
  let color_offset = opts.color_offset;
  let layers_count = opts.layers_count;
  let noise_effect = opts.noise_effect;
  let kaleidoscope = opts.kaleidoscope;
  let kaleidoscope_mod = opts.kaleidoscope_mod;

  // Prepare all the colors
  let mut rng = rng_from_fxhash(&opts.hash);

  /*
  let red_gel = Ink("Red Gel", "#BF738C", "#D880A6", 0.35);
  let orange_gel = Ink("Orange Gel", "#B27333", "#E68C4D", 0.35);
  let blue_gel = Ink("Blue Gel", "#338CFF", "#4D8CFF", 0.35);
  let green_gel = Ink("Green Gel", "#00B2A6", "#19CCBF", 0.35);
  */

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
  let turquoise = Ink("Turquoise Ink", "#00B4E6", "#005A8C", 0.35);
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

  let white_paper = Paper("White", "#fff", false);
  let red_paper = Paper("Red", "#aa0000", true);
  let darkblue_paper = Paper("Dark Blue", "#202040", true);
  let black_paper = Paper("Black", "#202020", true);
  let grey_paper = Paper("Grey", "#959fa8", true);

  let mut can_shuffle = true;

  let prob = 0.12;
  let (mut colors, paper) = if rng.gen_bool(prob) {
    (vec![pink, soft_mint, amber], white_paper)
  } else if rng.gen_bool(0.1 + prob) {
    if rng.gen_bool(0.5) {
      can_shuffle = false;
      (vec![amber, black], white_paper)
    } else {
      (vec![black], white_paper)
    }
  } else if rng.gen_bool(prob) {
    (vec![indigo, pink], white_paper)
  } else if rng.gen_bool(prob) {
    (vec![bloody_brexit, poppy_red], white_paper)
  } else if rng.gen_bool(prob) {
    can_shuffle = false;
    (vec![silver_gel, gold_gel], black_paper)
  } else if rng.gen_bool(prob) {
    (vec![poppy_red, amber, black], white_paper)
  } else if rng.gen_bool(prob) {
    (vec![imperial_purple, hope_pink], white_paper)
  } else if rng.gen_bool(prob) {
    (vec![white_gel], red_paper)
  } else if rng.gen_bool(prob) {
    (vec![aurora_borealis, red_dragon], white_paper)
  } else if rng.gen_bool(prob) {
    (vec![bloody_brexit, turquoise, pink], white_paper)
  } else if rng.gen_bool(prob) {
    (vec![seibokublue, inaho], white_paper)
  } else if rng.gen_bool(prob) {
    (vec![amber, hope_pink], white_paper)
  } else if rng.gen_bool(prob) {
    can_shuffle = false;
    (vec![black, white_gel], grey_paper)
  } else if rng.gen_bool(prob) {
    (vec![fire_and_ice, bloody_brexit], white_paper)
  } else if rng.gen_bool(prob) {
    (vec![sargasso_sea, pumpkin], white_paper)
  } else if rng.gen_bool(0.5) {
    can_shuffle = false;
    (vec![white_gel, gold_gel], black_paper)
  } else if rng.gen_bool(0.5) {
    can_shuffle = false;
    (vec![white_gel, gold_gel], darkblue_paper)
  } else {
    (vec![sherwood_green, evergreen], white_paper)
  };

  if can_shuffle && rng.gen_bool(0.4) {
    rng.shuffle(&mut colors);
  }

  // color_offset rotation on colors
  if color_offset > 0 {
    let mut new_colors = vec![];
    for i in 0..colors.len() {
      let color = colors[(i + color_offset) % colors.len()];
      new_colors.push(color);
    }
    colors = new_colors;
  }

  if color_cutoff < colors.len() {
    colors = colors[0..color_cutoff].to_vec();
  }

  // Prepare the generative code

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = vec![];

  let layers = (0..layers_count)
    .map(|i| (i % colors.len(), rng.gen::<[u8; 32]>()))
    .collect::<Vec<(usize, [u8; 32])>>();

  let mut sym_count = 0;
  let mut points_count = 0usize;

  for (clrindex, seed) in layers {
    let mut rng = rng_from_seed(seed);
    let s1 = rng.gen_range(0.0, 1.0);
    let s2 = rng.gen_range(0.0, 1.0);
    let s3 = rng.gen_range(0.0, 1.0);
    let s4 = rng.gen_range(0.0, 1.0);
    let s5 = rng.gen_range(0.0, 1.0);
    let s6 = rng.gen_range(0.0, 1.0);
    let s7 = rng.gen_range(0.0, 1.0);
    let s8 = rng.gen_range(0.0, 1.0);
    let s9 = rng.gen_range(0.0, 1.0);
    let mod1 = rng.gen_range(0.0, 1.0);

    let vignette_effect = rng.gen_range(-1f64, 4.0).max(0.0);
    let linear_effect = rng.gen_range(-5.0, 5.0) * rng.gen_range(0.0, 1.0);

    let mut map = WeightMap::new(width, height, 0.5);

    let perlin = Perlin::new();

    let seed = rng.gen_range(0.0, 1000.0);

    let f1 = 0.5 + rng.gen_range(0.5, 6.0) * rng.gen_range(0.1, 1.0);
    let f2 = 0.5 + rng.gen_range(0.5, 6.0) * rng.gen_range(0.1, 1.0);
    let f3 = 0.5 + rng.gen_range(0.5, 6.0) * rng.gen_range(0.1, 1.0);
    let noise_amp = noise_effect * rng.gen_range(0.2, 1.0);
    let warping = rng.gen_range(0.0, 4.0);

    let xsym = rng.gen_bool(0.5);

    if xsym {
      sym_count += 1;
    }

    let density_r = rng.gen_range(0.0, 0.6) * rng.gen_range(0.0, 1.0);

    map.fill_fn(|(x, y)| {
      if x < pad || x > width - pad || y < pad || y > height - pad {
        return 0.0;
      }

      let (x, y) = if kaleidoscope {
        kaleidoscope_project(
          (x, y),
          (width / 2.0, height / 2.0),
          2 * kaleidoscope_mod,
        )
      } else {
        (if xsym { (width as f64 - x).min(x) } else { x }, y)
      };

      // dist with center
      let d = (x - width / 2.0).hypot(y - height / 2.0);
      let d = d / (width / 2.0).max(height / 2.0);
      let d = d.powf(0.5) - 0.5;

      let dl = y / height - 0.5;

      let n = perlin.get([
        x / height as f64 * f1,
        y / height as f64 * f1,
        seed
          + warping
            * perlin.get([
              x / height as f64 * f2,
              y / height as f64 * f2,
              7.7 * seed
                + perlin.get([
                  77. + seed / 0.3,
                  x / height as f64 * f3,
                  y / height as f64 * f3,
                ]),
            ]),
      ]);

      let ratio = width / height;
      let mut p = ((ratio - 1.0) / 2.0 + x / width, y / height);

      p.0 += 0.2 * noise_amp.max(0.0) * (n * 10.0).cos();
      p.1 += 0.2 * noise_amp.max(0.0) * (n * 10.0).sin();

      (density_r + 3.0 / (layers_count as f64))
        * (7. * shade(p, s1, s2, s3, s4, s5, s6, s7, s8, s9, mod1)
          + vignette_effect * d
          + linear_effect * dl
          - lightness)
    });

    let step = 0.2
      + colors[clrindex].3
      + rng.gen_range(0.0, 0.7) * rng.gen_range(0.0, 1.0);
    let rot = PI / rng.gen_range(1.0, 4.0);
    let straight = rng.gen_range(-0.2, 0.8) * rng.gen_range(0.0, 1.0);
    let min_l = 5;
    let max_l = rng.gen_range(10, 50);
    let decrease_value = 1.0;
    let min_weight = 1.0;

    let count = (if paper.2 { 8000 } else { 16000 }) / layers_count;
    let search_max = 500;
    let mut bail_out = 0;
    for _i in 0..count {
      let top = map.search_weight_top(&mut rng, search_max, min_weight);
      if top.is_none() {
        bail_out += 1;
        if bail_out > 10 {
          break;
        }
      }
      if let Some(o) = top {
        let angle = perlin.get([
          2. * o.0 / height as f64,
          2. * o.1 / height as f64,
          seed,
        ]);

        if let Some(a) = map.best_direction(o, step, angle, PI, PI / 4.0, 0.0) {
          let route = map.dig_random_route(
            o,
            a,
            step,
            rot,
            straight,
            max_l,
            decrease_value,
          );
          if route.len() >= min_l {
            let rt = rdp(&route, 0.05);
            points_count += rt.len();
            routes.push((clrindex, rt));
          }
        }
      }
    }
  }

  // Infer the features from the generated pieces

  let colors_count = colors.len();
  let mut color_presence = vec![false; colors_count];
  for (i, _) in routes.iter() {
    color_presence[*i] = true;
  }
  let mut inks = vec![];
  for (i, &present) in color_presence.iter().enumerate() {
    if present && !inks.contains(&colors[i].0) {
      inks.push(colors[i].0);
    }
  }

  inks.sort();
  let inks_length = inks.len();

  let feature = Feature {
    inks: inks.join(", "),
    inks_count: inks_length,
    paper: paper.0.to_string(),
    symmetry: if sym_count == 0 {
      "None".to_string()
    } else if sym_count == layers_count {
      "Full".to_string()
    } else {
      "Partial".to_string()
    },
    density: if points_count < 30000 {
      "Very Low".to_string()
    } else if points_count < 60000 {
      "Low".to_string()
    } else if points_count < 130000 {
      "Medium".to_string()
    } else if points_count < 200000 {
      "High".to_string()
    } else {
      "Very High".to_string()
    },
  };

  let feature_json = serde_json::to_string(&feature).unwrap();
  let palette_json = serde_json::to_string(&Palette {
    paper,
    primary: colors[0],
    secondary: colors[1 % colors.len()],
    third: colors[2 % colors.len()],
  })
  .unwrap();

  let mask_colors = vec!["#0FF", "#F0F", "#FF0"];

  let layers = make_layers(
    colors
      .iter()
      .enumerate()
      .map(|(i, c)| {
        (
          if mask_mode { mask_colors[i] } else { c.1 },
          c.0.to_string(),
          c.3,
          routes
            .iter()
            .filter_map(
              |(ci, routes)| {
                if *ci == i {
                  Some(routes.clone())
                } else {
                  None
                }
              },
            )
            .collect(),
        )
      })
      .collect(),
  );

  let mut document = svg::Document::new()
    .set(
      "data-credits",
      "@greweb - 2023 - Plottable Mandelglitch".to_string(),
    )
    .set("data-hash", opts.hash.to_string())
    .set("data-traits", feature_json)
    .set("data-palette", palette_json)
    .set("viewBox", (0, 0, width, height))
    .set("width", format!("{}mm", width))
    .set("height", format!("{}mm", height))
    .set(
      "style",
      if mask_mode {
        "background:white".to_string()
      } else {
        format!("background:{}", paper.1)
      },
    )
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("xmlns", "http://www.w3.org/2000/svg");
  for l in layers {
    document = document.add(l);
  }

  (document, feature)
}

struct WeightMap {
  weights: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl WeightMap {
  fn new(width: f64, height: f64, precision: f64) -> WeightMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let weights = vec![0.0; w * h];
    WeightMap {
      weights,
      w,
      h,
      width,
      height,
      precision,
    }
  }
  fn fill_fn(&mut self, f: impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.weights[y * self.w + x] = v;
      }
    }
  }

  // do a simple bilinear interpolation
  fn get_weight(&self, p: (f64, f64)) -> f64 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let w00 = self.weights[y0 * self.w + x0];
    let w01 = self.weights[y0 * self.w + x1];
    let w10 = self.weights[y1 * self.w + x0];
    let w11 = self.weights[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the weights around the point p with a given radius
  fn decrease_weight_gaussian(
    &mut self,
    p: (f64, f64),
    radius: f64,
    value: f64,
  ) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = (x - radius).floor() as usize;
    let y0 = (y - radius).floor() as usize;
    let x1 = (x + radius).ceil() as usize;
    let y1 = (y + radius).ceil() as usize;
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let d = (p.0 - p.0).hypot(p.1 - p.1);
        if d < radius {
          let w = self.weights[y * self.w + x];
          let v = value * (1.0 - d / radius);
          self.weights[y * self.w + x] = w - v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  fn best_direction(
    &self,
    p: (f64, f64),
    step: f64,
    angle: f64,
    max_ang_rotation: f64,
    angle_precision: f64,
    straight_factor: f64,
  ) -> Option<f64> {
    let mut best_ang = None;
    let mut best_weight = 0.0;
    let mut a = -max_ang_rotation;
    while a < max_ang_rotation {
      let ang = a + angle;
      let dx = step * ang.cos();
      let dy = step * ang.sin();
      let np = (p.0 + dx, p.1 + dy);
      if np.0 < 0.0 || np.0 > self.width || np.1 < 0.0 || np.1 > self.height {
        a += angle_precision;
        continue;
      }
      // more important when a is near 0.0 depending on straight factor
      let wmul = (1.0 - straight_factor)
        + (1.0 - a.abs() / max_ang_rotation) * straight_factor;
      let weight = self.get_weight(np) * wmul;
      if weight > best_weight {
        best_weight = weight;
        best_ang = Some(ang);
      }
      a += angle_precision;
    }
    return best_ang;
  }

  fn search_weight_top<R: Rng>(
    &mut self,
    rng: &mut R,
    search_max: usize,
    min_weight: f64,
  ) -> Option<(f64, f64)> {
    let mut best_w = min_weight;
    let mut best_p = None;
    for _i in 0..search_max {
      let x = rng.gen_range(0.0, self.width);
      let y = rng.gen_range(0.0, self.height);
      let p = (x, y);
      let w = self.get_weight(p);
      if w > best_w {
        best_w = w;
        best_p = Some(p);
      }
    }
    return best_p;
  }

  fn dig_random_route(
    &mut self,
    origin: (f64, f64),
    initial_angle: f64,
    step: f64,
    max_ang_rotation: f64,
    straight_factor: f64,
    max_length: usize,
    decrease_value: f64,
  ) -> Vec<(f64, f64)> {
    let mut route = Vec::new();
    let mut p = origin;
    let mut angle = initial_angle;
    for _i in 0..max_length {
      if let Some(ang) = self.best_direction(
        p,
        step,
        angle,
        max_ang_rotation,
        0.2 * max_ang_rotation,
        straight_factor,
      ) {
        angle = ang;
        let prev = p;
        p = (p.0 + step * angle.cos(), p.1 + step * angle.sin());
        route.push(p);
        self.decrease_weight_gaussian(prev, step, decrease_value);
      } else {
        break;
      }
    }

    route
  }
}

#[inline]
fn significant_str(f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}
fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
  if route.len() == 0 {
    return data;
  }
  let first_p = route[0];
  let mut d =
    data.move_to((significant_str(first_p.0), significant_str(first_p.1)));
  for p in route {
    d = d.line_to((significant_str(p.0), significant_str(p.1)));
  }
  return d;
}

fn rng_from_seed(bs: [u8; 32]) -> impl Rng {
  let rng = StdRng::from_seed(bs);
  return rng;
}

fn rng_from_fxhash(hash: &String) -> impl Rng {
  let mut bs = [0; 32];
  bs58::decode(hash.chars().skip(2).take(43).collect::<String>())
    .into(&mut bs)
    .unwrap();
  let rng = StdRng::from_seed(bs);
  return rng;
}

fn make_layers(
  data: Vec<(&str, String, f64, Vec<Vec<(f64, f64)>>)>,
) -> Vec<Group> {
  let layers: Vec<Group> = data
    .iter()
    .filter(|(_color, _label, _stroke_width, routes)| routes.len() > 0)
    .enumerate()
    .map(|(ci, (color, label, stroke_width, routes))| {
      let mut l = Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", format!("{} {}", ci, label.clone()))
        .set("fill", "none")
        .set("stroke", color.clone())
        .set("stroke-linecap", "round")
        .set("stroke-width", *stroke_width);
      let opacity: f64 = 0.6;
      let opdiff = 0.15 / (routes.len() as f64);
      let mut trace = 0f64;
      for route in routes.clone() {
        trace += 1f64;
        let data = render_route(Data::new(), route);
        l = l.add(
          Path::new()
            .set(
              "opacity",
              (1000. * (opacity - trace * opdiff)).floor() / 1000.0,
            )
            .set("d", data),
        );
      }
      l
    })
    .collect();
  layers
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

#[inline]
pub fn strictly_in_boundaries(
  p: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> bool {
  p.0 > boundaries.0
    && p.0 < boundaries.2
    && p.1 > boundaries.1
    && p.1 < boundaries.3
}

fn mandelbrot_glitched(
  x: f64,
  y: f64,
  max_iterations: u32,
  s1: f64,
  s2: f64,
  s3: f64,
  s4: f64,
  s5: f64,
  s6: f64,
  s7: f64,
  s8: f64,
  s9: f64,
) -> f64 {
  let mut p = Complex::new(x, y);
  let init = p;
  let mut iterations = 0;

  for _ in 0..max_iterations {
    let x2 = p.re * p.re;
    let y2 = p.im * p.im;
    let xy = p.re * p.im;

    let a = 1.0 + (s1 - 0.5) * s7 * s9;
    let b = -1.0 + (s2 - 0.5) * s7;
    let c = 0.0 + (s3 - 0.5) * s7 * s9;
    let d = 0.0 + (s4 - 0.5) * s8;
    let e = 0.0 + (s5 - 0.5) * s8;
    let f = 2.0 + (s6 - 0.5) * s8 * s9;

    p.re = a * x2 + b * y2 + c * xy + d;
    p.im = f * xy + e;

    p += init;

    if p.norm_sqr() >= 4.0 {
      break;
    }

    iterations += 1;
  }

  iterations as f64 / max_iterations as f64
}

fn rotate_point(point: (f64, f64), angle: f64) -> (f64, f64) {
  let (x, y) = point;
  let cos_a = angle.cos();
  let sin_a = angle.sin();
  (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

fn shade(
  uv: (f64, f64),
  s1: f64,
  s2: f64,
  s3: f64,
  s4: f64,
  s5: f64,
  s6: f64,
  s7: f64,
  s8: f64,
  s9: f64,
  mod1: f64,
) -> f64 {
  let zoom = (0.3 + 6.0 * s7 * s8) * (1.0 + 3.0 * mod1);
  let focus_angle = 4.0 * mod1;
  let focus_amp = 0.4 * s7;
  let mut init = (2.0 * (uv.0 - 0.5) / zoom, 2.0 * (uv.1 - 0.5) / zoom);

  init =
    rotate_point(init, std::f64::consts::PI * (0.5 + 8.0 * s3).floor() / 4.0);
  init.0 -= 0.8;
  init.1 -= 0.0;
  init.0 += focus_amp * focus_angle.cos();
  init.1 += focus_amp * focus_angle.sin();

  let max_iterations = (50. + 500. * s7) as u32;

  let mandelbrot_value = mandelbrot_glitched(
    init.0,
    init.1,
    max_iterations,
    s1,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
  );

  mandelbrot_value
}

pub fn contour<F: FnMut((f64, f64)) -> f64>(
  width: u32,
  height: u32,
  mut f: F,
  thresholds: &Vec<f64>,
) -> Vec<geojson::Feature> {
  let c = ContourBuilder::new(width, height, true);
  let values = rasterize_1d(width, height, &mut f);
  c.contours(&values, &thresholds).unwrap_or(Vec::new())
}

pub fn rasterize_1d<F: FnMut((f64, f64)) -> f64>(
  width: u32,
  height: u32,
  mut f: F,
) -> Vec<f64> {
  (0..height)
    .flat_map(|y| {
      (0..width)
        .map(|x| f((x as f64 / width as f64, y as f64 / height as f64)))
        .collect::<Vec<f64>>()
    })
    .collect::<Vec<f64>>()
}

pub fn features_to_routes(
  features: Vec<geojson::Feature>,
  precision: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  for f in features {
    if let Some(g) = f.geometry {
      let value = g.value;
      match value {
        geojson::Value::MultiPolygon(all) => {
          for poly in all {
            for lines in poly {
              let mut points = lines
                .iter()
                .map(|p| (precision * p[0], precision * p[1]))
                .collect::<Vec<(f64, f64)>>();
              let len = points.len();
              if len < 3 {
                continue;
              }
              if euclidian_dist(points[0], points[len - 1]) <= precision {
                points.push(points[0]);
              }
              routes.push(points);
            }
          }
        }
        _ => {}
      }
    }
  }
  routes
}

// adapted from library "ramer_douglas_peucker"
/// Given a set of points and an epsilon, returns a list of indexes to keep.
/// If the first and last points are the same, then the points are treated as a closed polygon
pub fn rdp(points: &Vec<(f64, f64)>, epsilon: f64) -> Vec<(f64, f64)> {
  if points.len() < 3 {
    return points.clone();
  }
  let mut ranges = Vec::<RangeInclusive<usize>>::new();

  let mut results = Vec::new();
  results.push(0); // We always keep the starting point

  // Set of ranges to work through
  ranges.push(0..=points.len() - 1);

  while let Some(range) = ranges.pop() {
    let range_start = *range.start();
    let range_end = *range.end();

    let start = points[range_start];
    let end = points[range_end];

    // Caches a bit of the calculation to make the loop quicker
    let line = LineDistance::new(start, end);

    let (max_distance, max_index) =
      points[range_start + 1..range_end].iter().enumerate().fold(
        (0.0_f64, 0),
        move |(max_distance, max_index), (index, &point)| {
          let distance = match line.to(point) {
            Some(distance) => distance,
            None => {
              let base = point.0 - start.0;
              let height = point.1 - start.1;
              base.hypot(height)
            }
          };

          if distance > max_distance {
            // new max distance!
            // +1 to the index because we start enumerate()ing on the 1st element
            return (distance, index + 1);
          }

          // no new max, pass the previous through
          (max_distance, max_index)
        },
      );

    // If there is a point outside of epsilon, subdivide the range and try again
    if max_distance > epsilon {
      // We add range_start to max_index because the range needs to be in
      // the space of the whole vector and not the range
      let division_point = range_start + max_index;

      let first_section = range_start..=division_point;
      let second_section = division_point..=range_end;

      // Process the second one first to maintain the stack
      // The order of ranges and results are opposite, hence the awkwardness
      let should_keep_second_half = division_point - range_start > 2;
      if should_keep_second_half {
        ranges.push(second_section);
      }

      if division_point - range_start > 2 {
        ranges.push(first_section);
      } else {
        results.push(division_point);
      }

      if !should_keep_second_half {
        results.push(range_end);
      }
    } else {
      // Keep the end point for the results
      results.push(range_end);
    }
  }

  results.iter().map(|&i| points[i]).collect()
}

// adapted from "legasea_line"
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LineDistance {
  a: f64,
  b: f64,
  c: f64,
  pub length: f64,
}

impl LineDistance {
  pub fn new(p1: (f64, f64), p2: (f64, f64)) -> Self {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let a = y2 - y1;
    let b = x2 - x1;
    let c = (x2 * y1) - (y2 * x1);
    let length = euclidian_dist(p1, p2);
    Self { a, b, c, length }
  }
  pub fn to(&self, point: (f64, f64)) -> Option<f64> {
    let Self { a, b, c, length } = self;
    if 0.0 == *length {
      None
    } else {
      // https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
      Some(((a * point.0) - (b * point.1) + c).abs() / length)
    }
  }
}

fn kaleidoscope_project(
  p: (f64, f64),
  center: (f64, f64),
  n: usize,
) -> (f64, f64) {
  let (x, y) = p;
  let (cx, cy) = center;

  // Translate the point relative to the center
  let (x, y) = (x - cx, y - cy);

  let angle = 2.0 * PI / n as f64;
  let dist = (x * x + y * y).sqrt();
  let theta = PI + y.atan2(x);
  let sector = (theta / angle).floor();
  let theta_in_sector = theta - sector * angle;
  let new_theta = if sector as u32 % 2 == 0 {
    theta_in_sector
  } else {
    angle - theta_in_sector
  };
  let new_x = dist * new_theta.cos();
  let new_y = dist * new_theta.sin();

  // Translate the point back to its original position
  let (new_x, new_y) = (new_x + cx, new_y + cy);

  (new_x, new_y)
}
