/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Sliced Spiral
 */
mod utils;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
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
}

// Feature tells caracteristics of a given art variant
// It is returned in the .SVG file
#[derive(Clone, Serialize)]
pub struct Feature {
  // how much times the spiral is sliced into parts
  pub splits: String,
  // how much times the spiral is turned
  pub spins: String,
  // how much axes are used to slice the parts
  pub axes: String,
  // how much sliding is applied on the slices
  pub sliding: String,
  // which inks are used
  pub inks: String,
  // how much inks are used
  pub inks_count: usize,
  // which paper is used
  pub paper: String,
}

#[derive(Clone, Copy, Serialize)]
pub struct Color(&'static str, &'static str, &'static str);
#[derive(Clone, Copy, Serialize)]
pub struct Paper(&'static str, &'static str, bool);

// This is also returned in the SVG to have more metadata for the JS side to render a digital version
#[derive(Clone, Serialize)]
pub struct Palette {
  pub primary: Color,
  pub secondary: Color,
  pub paper: Paper,
}

// This is the main art function that will render the generative art piece
pub fn art(opts: &Opts, mask_mode: bool) -> (svg::Document, Feature) {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;

  // Prepare all the random values
  let mut rng = rng_from_fxhash(&opts.hash);

  let seibokublue = Color("Sailor Sei-boku", "#1060a3", "#153a5d");
  let inaho = Color("iroshizuku ina-ho", "#ba6", "#7f6a33");
  let black = Color("Black", "rgb(20%,20%,20%)", "rgb(0%,0%,0%)");
  let amber = Color("Amber", "rgb(100%,78%,28%)", "rgb(100%,50%,0%)");
  let pink = Color("Hope Pink", "rgb(100%, 40%, 75%)", "rgb(90%,20%,60%)");
  let gold_gel = Color("Gold Gel", "rgb(85%,70%,25%)", "rgb(100%,90%,55%)");
  let white_gel = Color("White Gel", "rgb(90%,90%,90%)", "rgb(100%,100%,100%)");
  let blue_gel = Color("Blue Gel", "#06B2FB", "#2CF");

  let white_paper = Paper("White", "#eee", false);
  let red_paper = Paper("Red", "#aa0000", true);
  let darkblue_paper = Paper("Dark Blue", "#202040", true);
  let black_paper = Paper("Black", "#202020", true);

  let (mut colors, paper) = if rng.gen_bool(0.18) {
    (vec![seibokublue, inaho], white_paper)
  } else if rng.gen_bool(0.18) {
    (vec![white_gel, blue_gel], black_paper)
  } else if rng.gen_bool(0.13) {
    (vec![amber, pink], white_paper)
  } else if rng.gen_bool(0.15) {
    (vec![white_gel, gold_gel], black_paper)
  } else if rng.gen_bool(0.1) {
    (vec![black, amber], white_paper)
  } else if rng.gen_bool(0.7) {
    (vec![white_gel, gold_gel], darkblue_paper)
  } else {
    (vec![white_gel, gold_gel], red_paper)
  };

  if rng.gen_bool(0.2) {
    colors.reverse();
  }

  if rng.gen_bool(0.2) {
    colors = vec![colors[0]];
  }

  let colors_count = colors.len();

  // global random values that drives the variation
  let a_delta = rng.gen_range(-PI, PI);
  let disp = rng.gen_range(2.0, 4.0);
  let adisp = rng.gen_range(0.4, 1.0);
  let dr = disp + rng.gen_range(10.0, 20.0) * rng.gen_range(0.0, 1.0);
  let r = 80.0;
  let count = (8.0 * (disp + adisp)) as usize;

  let mut routes = vec![];
  for _i in 0..count {
    // randomly offset of the position
    let x = width / 2.0 + rng.gen_range(-disp, disp);
    let y = height / 2.0 + rng.gen_range(-disp, disp);
    // randomly offset of the initial angle
    let start_a = a_delta + rng.gen_range(-adisp, adisp);
    let points = spiral(x, y, r, dr, start_a);
    routes.push((0, points));
  }

  let count = rng.gen_range(2, 10);
  let mut prev_a = PI / 2.0;
  let rotations = (0..count)
    .map(|_i| {
      let a = if rng.gen_bool(0.3) {
        prev_a + PI / 2.0
      } else if rng.gen_bool(0.3) {
        prev_a + PI / 4.0
      } else {
        prev_a
          + rng.gen_range(-PI, PI)
            * rng.gen_range(0.0, 1.0)
            * rng.gen_range(0.0, 1.0)
      };
      prev_a = a;
      a
    })
    .collect::<Vec<f64>>();

  // statistic way to store the rots used
  let mut dedup_rot = vec![];

  let count = rng.gen_range(1usize, 20);
  let split = rng.gen_range(0.2, 1.2);
  let max_slide = rng.gen_range(0.0, 20.0);
  let shake = rng.gen_range(0.0, 1.0);
  let mut total_displacement = 0.0;
  for i in 0..count {
    let (x, y) = if i == 0 {
      (width / 2.0, height / 2.0)
    } else {
      (
        width * rng.gen_range(0.3, 0.7),
        height * rng.gen_range(0.3, 0.7),
      )
    };
    let a = rotations[(rng.gen_range(0., rotations.len() as f64)
      * rng.gen_range(0.0, 1.0)) as usize];
    let dx = a.cos();
    let dy = a.sin();
    let amp = 200.0;
    let left = (x - amp * dx, y - amp * dy);
    let right = (x + amp * dx, y + amp * dy);
    let slice = slice_routes(routes.clone(), left, right);
    let slide = rng.gen_range(0.0, max_slide)
      * rng.gen_range(shake, 1.0)
      * rng.gen_range(shake, 1.0);
    let l = euclidian_dist(slice.a, slice.b);

    let v = if l == 0.0 {
      (0.0, 0.0)
    } else {
      ((slice.b.0 - slice.a.0) / l, (slice.b.1 - slice.a.1) / l)
    };
    let n = (v.1, -v.0);

    if slice.routes_above.len() > 0 && slice.routes_below.len() > 0 {
      let r = (((a / PI + 100.0) % 1.0) * 360.0) as usize;
      if !dedup_rot.contains(&r) {
        dedup_rot.push(r);
      }
      total_displacement += slide;
    }

    routes = vec![
      translate_routes(
        slice.routes_above,
        (v.0 * slide + n.0 * split, v.1 * slide + n.1 * split),
      )
      .iter()
      .map(|(ci, route)| ((ci + 1) % colors_count, route.clone()))
      .collect(),
      translate_routes(
        slice.routes_below,
        (-v.0 * slide - n.0 * split, -v.1 * slide - n.1 * split),
      ),
    ]
    .concat();
  }

  let mut min_x = width;
  let mut min_y = height;
  let mut max_x = 0.0f64;
  let mut max_y = 0.0f64;
  for (_, route) in routes.iter() {
    for &(x, y) in route.iter() {
      min_x = min_x.min(x);
      min_y = min_y.min(y);
      max_x = max_x.max(x);
      max_y = max_y.max(y);
    }
  }
  let w = max_x - min_x;
  let h = max_y - min_y;

  let scale = if h < w {
    ((width - 2.0 * pad) / w).min((height - 2.0 * pad) / h)
  } else {
    ((width - 2.0 * pad) / h).min((height - 2.0 * pad) / w)
  };

  let mut color_presence = vec![false, false];

  routes = routes
    .iter()
    .flat_map(|(ci, route)| {
      if route.len() == 0 {
        return None;
      }
      color_presence[*ci] = true;
      let route = route.iter().map(|&(x, y)| {
        let mut p = (x - min_x - w / 2., y - min_y - h / 2.);
        if h > w {
          // rotate 90°
          p = (p.1, -p.0);
        }
        p = (scale * p.0 + width / 2., scale * p.1 + height / 2.);
        p
      });
      Some((*ci, route.collect()))
    })
    .collect();

  // Infer the features from the generated pieces

  let mut inks = vec![];
  for (i, &present) in color_presence.iter().enumerate() {
    if present {
      inks.push(colors[i].0);
    }
  }
  inks.sort();
  let inks_length = inks.len();

  let feature = Feature {
    splits: (match count {
      0..=5 => "Low",
      6..=10 => "Medium",
      _ => "High",
    })
    .to_string(),
    spins: (match (r / dr).ceil() as usize {
      0..=7 => "Low",
      8..=15 => "Medium",
      _ => "High",
    })
    .to_string(),
    axes: (match dedup_rot.len() {
      0 => "None",
      1 => "One",
      2 => "Two",
      3 => "Three",
      4 => "Four",
      5 => "Five",
      _ => "Many",
    })
    .to_string(),
    sliding: (match total_displacement.round() as usize {
      0..=1 => "None",
      2..=10 => "Low",
      11..=25 => "Medium",
      26..=50 => "High",
      _ => "Extreme",
    })
    .to_string(),
    inks: inks.join(", "),
    inks_count: inks_length,
    paper: paper.0.to_string(),
  };

  let feature_json = serde_json::to_string(&feature).unwrap();
  let palette_json = serde_json::to_string(&Palette {
    paper,
    primary: colors[0],
    secondary: colors[1 % colors.len()],
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
      "@greweb - 2023 - Plottable Sliced Spiral".to_string(),
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

// The slime primitive =>

// Generic helper to simplify and clean up a path

// render helper

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

fn rng_from_fxhash(hash: &String) -> impl Rng {
  let mut bs = [0; 32];
  bs58::decode(hash.chars().skip(2).take(43).collect::<String>())
    .into(&mut bs)
    .unwrap();
  let rng = StdRng::from_seed(bs);
  return rng;
}

fn make_layers(data: Vec<(&str, String, Vec<Vec<(f64, f64)>>)>) -> Vec<Group> {
  let layers: Vec<Group> = data
    .iter()
    .filter(|(_color, _label, routes)| routes.len() > 0)
    .map(|(color, label, routes)| {
      let mut l = Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", label.clone())
        .set("fill", "none")
        .set("stroke", color.clone())
        .set("stroke-linecap", "round")
        .set("stroke-width", 0.35);
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
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

fn spiral(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  start_a: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = start_a;
  loop {
    route.push((x + r * a.cos(), y + r * a.sin()));
    let da = 1.0 / (r + 8.0);
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.1 {
      break;
    }
  }
  route
}

struct Slice {
  routes_above: Vec<(usize, Vec<(f64, f64)>)>,
  routes_below: Vec<(usize, Vec<(f64, f64)>)>,
  a: (f64, f64),
  b: (f64, f64),
}

fn slice_routes(
  routes: Vec<(usize, Vec<(f64, f64)>)>,
  cuta: (f64, f64),
  cutb: (f64, f64),
) -> Slice {
  let mut routes_above = Vec::new();
  let mut routes_below = Vec::new();

  let mut amin = lerp_point(cuta, cutb, 0.5);
  let mut bmin = amin;
  let mut dista = 99999.0;
  let mut distb = 0.0;

  for (clr, r) in routes.clone() {
    if r.len() < 2 {
      continue;
    }
    let mut prev = r[0];
    let mut route = vec![prev];
    for &p in r.iter().skip(1) {
      if let Some(c) = collides_segment(prev, p, cuta, cutb) {
        let la = euclidian_dist(c, cuta);
        if la > distb {
          distb = la;
          bmin = c;
        }
        if la < dista {
          dista = la;
          amin = c;
        }

        route.push(c);
        if route.len() > 1 {
          if !is_left(cuta, cutb, prev) {
            routes_above.push((clr, route));
          } else {
            routes_below.push((clr, route));
          }
        }
        route = vec![c, p];
      } else {
        route.push(p);
      }
      prev = p;
    }
    if route.len() > 1 {
      if !is_left(cuta, cutb, prev) {
        routes_above.push((clr, route));
      } else {
        routes_below.push((clr, route));
      }
    }
  }

  Slice {
    routes_above,
    routes_below,
    a: amin,
    b: bmin,
  }
}

fn is_left(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
  ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)) > 0.0
}

fn translate_routes(
  routes: Vec<(usize, Vec<(f64, f64)>)>,
  (tx, ty): (f64, f64),
) -> Vec<(usize, Vec<(f64, f64)>)> {
  routes
    .iter()
    .map(|(i, route)| {
      (*i, route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    })
    .collect()
}

// collides segments (p0,p1) with (p2,p3)
fn collides_segment(
  p0: (f64, f64),
  p1: (f64, f64),
  p2: (f64, f64),
  p3: (f64, f64),
) -> Option<(f64, f64)> {
  let s10_x = p1.0 - p0.0;
  let s10_y = p1.1 - p0.1;
  let s32_x = p3.0 - p2.0;
  let s32_y = p3.1 - p2.1;
  let d = s10_x * s32_y - s32_x * s10_y;
  if d.abs() < 0.000001 {
    return None;
  }
  let s02_x = p0.0 - p2.0;
  let s02_y = p0.1 - p2.1;
  let s_numer = s10_x * s02_y - s10_y * s02_x;
  if (s_numer < 0.) == (d > 0.) {
    return None;
  }
  let t_numer = s32_x * s02_y - s32_y * s02_x;
  if (t_numer < 0.) == (d > 0.) {
    return None;
  }
  if (s_numer > d) == (d > 0.) || (t_numer > d) == (d > 0.) {
    return None;
  }
  let t = t_numer / d;
  return Some((p0.0 + t * s10_x, p0.1 + t * s10_y));
}
