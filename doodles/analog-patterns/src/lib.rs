mod utils;
use hex::FromHex;
use noise::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::ops::RangeInclusive;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use wasm_bindgen::prelude::*;

// Input to the art function
#[derive(Deserialize)]
pub struct Opts {
  pub hash: String,
  pub width: f64,
  pub height: f64,
  pub pad: f64,
  pub images: Vec<ImageData>,
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

pub fn art(opts: &Opts, mask_mode: bool) -> (svg::Document, Feature) {
  let height = opts.height;
  let width = opts.width;
  let images = &opts.images;
  let pad: f64 = opts.pad;

  let mut rng = rng_from_hexhash(&opts.hash);

  let black = Ink("Black", "#1A1A1A", "#000000", 0.35);
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let inaho = Ink("iroshizuku ina-ho", "#ba6", "#7f6a33", 0.35);
  let imperial_purple = Ink("Imperial Purple", "#4D0066", "#260F33", 0.35);
  let amazing_amethyst = Ink("Amazing Amethyst", "#8b008b", "#550055", 0.35);
  let teal = Ink("Teal", "#274e13", "#060f02", 0.35);
  let sherwood_green = Ink("Sherwood Green", "#337239", "#194D19", 0.35);
  let evergreen = Ink("Evergreen", "#4D6633", "#263319", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let sargasso_sea = Ink("Sargasso Sea", "#162695", "#111962", 0.35);
  let aurora_borealis = Ink("Aurora Borealis", "#009999", "#004D66", 0.35);
  let pumpkin = Ink("Pumpkin", "#FF8033", "#E54D00", 0.35);
  let hope_pink = Ink("Hope Pink", "#fc839b", "#E53399", 0.35);
  let sepia = Ink("Sepia", "#ce7e00", "#986719", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let yellow = Ink("Yellow", "#FF0", "#FF0", 0.35);
  let poppy_red = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
  let red_dragon = Ink("Red Dragon", "#9e061a", "#5b0a14", 0.35);
  let writers_blood = Ink("Writers Blood", "#890f0f", "#572807", 0.35);
  let fire_and_ice = Ink("Fire And Ice", "#00BEDE", "#006478", 0.35);
  let bloody_brexit = Ink("Bloody Brexit", "#05206B", "#2E0033", 0.35);
  let skulls_and_roses = Ink("Skulls and Roses", "#05206B", "#2E0033", 0.35);
  let violet = Ink("Violet", "#c3f", "#92c", 0.35);
  let moonstone = Ink("Moonstone", "#bbb", "#ddd", 0.35);
  let spring_green = Ink("Spring Green", "#7d9900", "#6c6b00", 0.35);
  let indigo = Ink("Indigo", "#667599", "#334D66", 0.35);
  let prussian_blue = Ink("Prussian Blue", "#2f5a98", "#294061", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);

  let white_paper = Paper("White", "#fff", false);

  let white_on_paper = false;
  let paper = white_paper;

  let palettes = vec![
    vec![black, moonstone],
    vec![skulls_and_roses, indigo, moonstone],
    vec![aurora_borealis, hope_pink, amber],
    vec![seibokublue, inaho, moonstone],
    vec![black, poppy_red, amber],
    vec![black, prussian_blue, pink],
    vec![writers_blood, sepia, amber],
    vec![imperial_purple, violet, hope_pink],
    vec![bloody_brexit, fire_and_ice],
    vec![sherwood_green, aurora_borealis, spring_green],
    vec![black, amazing_amethyst, violet],
    vec![bloody_brexit, sargasso_sea],
    vec![teal, evergreen, inaho],
    vec![black, sherwood_green, soft_mint],
    vec![black, red_dragon, pumpkin],
    vec![black, pumpkin, yellow],
  ];

  let mut colors = palettes[(rng.gen_range(0.0, palettes.len() as f64)
    * rng.gen_range(0.3, 1.0))
  .floor() as usize]
    .clone();

  if rng.gen_bool(0.3) {
    let index = rng.gen_range(0, colors.len());
    colors.remove(index);
  }

  let perlin = Perlin::new();

  let mut map = ValueMap::new(width, height, 0.38);

  let max_density = 6.0;

  let count_imgs = images.len();

  let quinconce = rng.gen_bool(0.7);

  let collages = (0..count_imgs)
    .map(|i| {
      let p = if count_imgs == 1 {
        1.0
      } else {
        (i as f64 + 0.5) / (count_imgs as f64)
      };
      let p = p + rng.gen_range(-0.2, 0.5) * rng.gen_range(0.0, 1.0);
      let density = p * max_density;
      let effects = (
        if rng.gen_bool(0.6) {
          Wrapping::Repeat(quinconce)
        } else if rng.gen_bool(0.5) {
          Wrapping::RepeatMirror(quinconce)
        } else if rng.gen_bool(0.2) {
          Wrapping::ClampRepeat(rng.gen_range(0.0, 1.0))
        } else {
          if i == count_imgs - 1 {
            Wrapping::RepeatMirror(quinconce)
          } else {
            Wrapping::None
          }
        },
        if rng.gen_bool(0.2) {
          Wrapping::ClampRepeat(rng.gen_range(0.1, 0.5))
        } else if rng.gen_bool(0.5) {
          Wrapping::Repeat(quinconce)
        } else if rng.gen_bool(0.5) {
          Wrapping::RepeatMirror(quinconce)
        } else {
          if i == count_imgs - 1 {
            Wrapping::RepeatMirror(quinconce)
          } else {
            Wrapping::None
          }
        },
      );
      (
        // index
        i,
        // offset away from center
        (
          rng.gen_range(-0.5, 0.5) * rng.gen_range(-0.2f64, 1.0).max(0.0),
          rng.gen_range(-0.5, 0.5) * rng.gen_range(-0.2f64, 1.0).max(0.0),
        ),
        // scale
        1.0 + rng.gen_range(-0.3, 10.0) * rng.gen_range(0.5, 1.0),
        // rotate
        rng.gen_range(-1.0, 1.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(-0.5f64, 1.0).max(0.0),
        // repeat
        effects,
        // density of strokes
        density,
      )
    })
    .collect::<Vec<_>>();

  map.fill_fn(|(x, y)| {
    if x < pad || x > width - pad || y < pad || y > height - pad {
      return 0.0;
    }

    let mut sum: f64 = 0.0;

    for (index, v, s, r, wrapping, multiplier) in collages.clone() {
      let img = &images[index];
      let ratio = img.width as f64 / img.height as f64;

      let p = (x / opts.width, y / opts.height);
      let p = (p.0 - 0.5, p.1 - 0.5); // image transaction apply on center
      let p = rotate(p, r); // image rotation
      let p = (p.0 / ratio, p.1); // respect the ratio of the image
      let p = (p.0 * s, p.1 * s); // image scale
      let p = (p.0 + 0.5, p.1 + 0.5); // move back to canvas center
      let p = (p.0 - v.0, p.1 - v.1); // translate the image
      let p = apply_wrapping(p, wrapping);

      // TODO apply the image crop positionning. the img ratio would then have to take it into account

      // TODO global displacement (noise)

      // protect from out of bounds cases
      if p.0 < 0.0 || p.0 > 1.0 || p.1 < 0.0 || p.1 > 1.0 {
        continue;
      }

      let c = img.get_color(p);
      let mut value = c.0;
      if !white_on_paper {
        value = 1.0 - value;
        //value = value.powf(2.0);
        value = value * 1.1 - 0.1;
      } else {
        value = value.powf(2.0);
      }
      //let value = value.powf(1.5);
      let value = multiplier * value;
      sum += value;
    }

    sum.min(max_density)
  });

  let mut sum = 0.0;
  for v in map.values.iter() {
    sum += v;
  }
  let avg = sum / map.values.len() as f64;

  // normalize the map to try to be consistent between generations
  let b = mix(2.0 * avg, max_density, 0.5);
  let a = rng.gen_range(0.0, 0.2) * b;
  for v in map.values.iter_mut() {
    *v = max_density * ((*v - a) / (b - a)).min(1.0).max(0.0);
  }

  let rot = PI / rng.gen_range(1.0, 4.0);
  let step = 0.45;
  let straight = 0.1;
  let count = 20000;
  let min_l = 5;
  let max_l = 40;
  let decrease_value = 1.0;
  let search_max = 500;
  let min_weight = 1.0;
  let mut bail_out = 0;

  let mut routes = Vec::new();

  let seed = rng.gen_range(0.0, 10000.0);

  for _i in 0..count {
    let top = map.search_highest_point(&mut rng, search_max, min_weight);
    if top.is_none() {
      bail_out += 1;
      if bail_out > 10 {
        break;
      }
    }
    if let Some(o) = top {
      let angle =
        perlin.get([2. * o.0 / height as f64, 2. * o.1 / height as f64, seed]);

      if let Some(a) =
        map.search_highest_direction(o, step, angle, PI, PI / 4.0, 0.0)
      {
        let v = map.get(o);
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
          let clr =
            ((1.0 - (v / max_density)) * (colors.len() as f64)) as usize;
          routes.push((clr, rt));
        }
      }
    }
  }

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
  };

  let feature_json = serde_json::to_string(&feature).unwrap();

  let palette_json = serde_json::to_string(&Palette {
    paper,
    primary: colors[0 % colors.len()],
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
      "@greweb - 2023 - GREWEBPAJONCOLLAB".to_string(),
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

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
  let opts = val.into_serde().unwrap();
  let (doc, _) = art(&opts, true);
  let str = doc.to_string();
  return str;
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

#[inline]
fn significant_str(f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

#[derive(Deserialize)]
pub struct ImageData {
  pub data: Vec<u8>,
  pub width: usize,
  pub height: usize,
}

impl ImageData {
  // point is normalized in 0..1
  // returned value is a rgb tuple in 0..1 range
  fn get_color(&self, (x, y): (f64, f64)) -> (f64, f64, f64) {
    let width = self.width;
    let height = self.height;
    let d = &self.data;
    // quadratic implementation
    let xi: f64 = x.max(0.0).min(1.0) * ((width - 1) as f64);
    let yi: f64 = y.max(0.0).min(1.0) * ((height - 1) as f64);
    let x1 = xi.floor() as usize;
    let x2 = xi.ceil() as usize;
    let y1 = yi.floor() as usize;
    let y2 = yi.ceil() as usize;
    let w1 = width * y1;
    let p1i = (x1 + w1) * 4;
    let p2i = (x2 + w1) * 4;
    let w2 = width * y2;
    let p3i = (x2 + w2) * 4;
    let p4i = (x1 + w2) * 4;
    let xp = xi - xi.floor();
    let yp = yi - yi.floor();
    let r = (mix(
      mix(d[p1i + 0] as f64, d[p2i + 0] as f64, xp),
      mix(d[p4i + 0] as f64, d[p3i + 0] as f64, xp),
      yp,
    )) / 255.0;
    let g = (mix(
      mix(d[p1i + 1] as f64, d[p2i + 1] as f64, xp),
      mix(d[p4i + 1] as f64, d[p3i + 1] as f64, xp),
      yp,
    )) / 255.0;
    let b = (mix(
      mix(d[p1i + 2] as f64, d[p2i + 2] as f64, xp),
      mix(d[p4i + 2] as f64, d[p3i + 2] as f64, xp),
      yp,
    )) / 255.0;
    return (r, g, b);
  }
}

#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}

/**
 * A general utility 2D value map.
 */
struct ValueMap {
  values: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}

impl ValueMap {
  fn new(width: f64, height: f64, precision: f64) -> ValueMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let values = vec![0.0; w * h];
    ValueMap {
      values,
      w,
      h,
      width,
      height,
      precision,
    }
  }

  // initialize the map with a function
  fn fill_fn(&mut self, f: impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.values[y * self.w + x] = v;
      }
    }
  }

  /*
  // modify values to the map with a function
  fn apply_fn(&mut self, f: impl Fn((f64, f64), f64) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p, self.values[y * self.w + x]);
        self.values[y * self.w + x] = v;
      }
    }
  }
  */

  // do a simple bilinear interpolation
  fn get(&self, p: (f64, f64)) -> f64 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let w00 = self.values[y0 * self.w + x0];
    let w01 = self.values[y0 * self.w + x1];
    let w10 = self.values[y1 * self.w + x0];
    let w11 = self.values[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the values around the point p with a given radius
  fn add_gaussian(&mut self, p: (f64, f64), radius: f64, value: f64) {
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
          let w = self.values[y * self.w + x];
          let v = value * (1.0 - d / radius);
          self.values[y * self.w + x] = w + v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  fn search_highest_direction(
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
      let weight = self.get(np) * wmul;
      if weight > best_weight {
        best_weight = weight;
        best_ang = Some(ang);
      }
      a += angle_precision;
    }
    return best_ang;
  }

  // randomly search for a point having a high value in the map
  fn search_highest_point<R: Rng>(
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
      let w = self.get(p);
      if w > best_w {
        best_w = w;
        best_p = Some(p);
      }
    }
    return best_p;
  }

  // dig a route from a given point following the highest values & decrease the values on the travel path
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
      if let Some(ang) = self.search_highest_direction(
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
        self.add_gaussian(prev, step, -decrease_value);
      } else {
        break;
      }
    }

    route
  }
}

#[derive(Clone, Debug, Copy)]
pub enum Wrapping {
  Repeat(bool),
  RepeatMirror(bool),
  Clamp,
  ClampRepeat(f64),
  None,
}

fn apply_wrapping_value(v: f64, wrapping: Wrapping) -> (f64, f64) {
  let v2 = v + 1000.0;
  match wrapping {
    Wrapping::None => (v, 0.0),
    Wrapping::Clamp => (v.max(0.0).min(1.0), 0.0),
    Wrapping::Repeat(quinconce) => (
      v2 % 1.0,
      if !quinconce || v2 % 2.0 < 1.0 {
        0.0
      } else {
        0.5
      },
    ),
    Wrapping::ClampRepeat(r) => (
      if v < 0.0 || v > 1.0 {
        let v = if v < 0.0 { -v } else { v };
        v % r
      } else {
        v
      },
      0.0,
    ),
    Wrapping::RepeatMirror(quinconce) => {
      let m = v2 % 2.0;
      (
        if m > 1.0 { 2.0 - m } else { m },
        if !quinconce || v2 % 4.0 < 2.0 {
          0.0
        } else {
          0.5
        },
      )
    }
  }
}

fn apply_wrapping(p: (f64, f64), wrapping: (Wrapping, Wrapping)) -> (f64, f64) {
  let (_x, dx) = apply_wrapping_value(p.0, wrapping.0);
  let (y, dy) = apply_wrapping_value(p.1 + dx, wrapping.1);
  let (x, _dx) = apply_wrapping_value(p.0 + dy, wrapping.0);
  (x, y)
}

fn rotate(point: (f64, f64), angle: f64) -> (f64, f64) {
  let sin_angle = angle.sin();
  let cos_angle = angle.cos();
  (
    point.0 * cos_angle - point.1 * sin_angle,
    point.0 * sin_angle + point.1 * cos_angle,
  )
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

fn rng_from_hexhash(hash: &String) -> impl Rng {
  let mut bs = [0; 32];
  bs.copy_from_slice(&Vec::<u8>::from_hex(hash).unwrap().as_slice());
  let rng = StdRng::from_seed(bs);
  return rng;
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

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}
