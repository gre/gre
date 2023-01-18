/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – ArtDeco
 */
mod utils;
use contour::ContourBuilder;
use geojson::Feature;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::ops::RangeInclusive;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
  let opts = val.into_serde().unwrap();
  let doc = art(&opts);
  let str = doc.to_string();
  return str;
}

#[derive(Deserialize)]
pub struct Opts {
  pub hash: String,
  pub width: f64,
  pub height: f64,
  pub pad: f64,
  pub layer1_name: String,
  pub debug: bool,
}

fn sd_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
  manhattan: bool,
) -> f64 {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;

  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let h = (dot_pa_ba / dot_ba_ba).max(0.0).min(1.0);

  let h_x = ba_x * h;
  let h_y = ba_y * h;

  if manhattan {
    return (pa_x - h_x).abs().max((pa_y - h_y).abs());
  } else {
    ((pa_x - h_x) * (pa_x - h_x) + (pa_y - h_y) * (pa_y - h_y)).sqrt()
  }
}

pub fn art(opts: &Opts) -> Document {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let precision = 0.4;
  let bound = (pad, pad, width - pad, height - pad);
  let mut rng = rng_from_fxhash(opts.hash.clone());

  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;

  let length = rng.gen_range(2.0, 3.0);
  let samples = (rng.gen_range(300.0, 400.0) / length) as usize;
  let pattern = (4.0, rng.gen_range(0.2f64, 4.0).max(0.0).round());

  let divisor = (samples as f64 * (pattern.0 + pattern.1) / pattern.0).floor();
  let thresholds: Vec<f64> = (0..samples)
    .map(|i| (i as f64 + pattern.1 * (i as f64 / pattern.0).floor()) / divisor)
    .collect();

  let balance = rng.gen_range(-0.5, 2.0);

  let offsetmax = rng.gen_range(0.0, 0.05);

  let segments: Vec<((f64, f64), (f64, f64), bool, f64)> =
    (0..(2.0 + rng.gen_range(0., 50.) * rng.gen_range(0.0, 1.0)) as usize)
      .map(|_| {
        (
          (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
          (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
          if balance >= 1.0 {
            true
          } else if balance <= 0.0 {
            false
          } else {
            rng.gen_bool(balance)
          },
          offsetmax * rng.gen_range(-2.0f64, 1.0).max(0.0),
        )
      })
      .collect();

  let distortion = rng.gen_range(-0.05f64, 0.02).max(0.0);
  let ratio = width / height;

  let xflip = rng.gen_bool(0.9);
  let yflip = rng.gen_bool(0.3);
  let yoffset = rng.gen_range(-10.0f64, 1.0).max(0.0);

  let f = |p: (f64, f64)| {
    let mut p = p;

    p = (
      p.0,
      p.1 + distortion * (p.0 - 0.5).abs() * (p.0 * 40.0 + p.1.sin()).cos(),
    );

    if xflip {
      p.0 = p.0.min(1.0 - p.0);
    }
    if yflip {
      p.1 = p.1.min(1.0 - p.1);
    }

    let mut s = 9999.0f64;

    for &(from, to, manhattan, offset) in segments.iter() {
      s = s.min(
        sd_segment(
          (p.0 * ratio, p.1),
          (from.0 * ratio, from.1),
          (to.0 * ratio, to.1),
          manhattan,
        ) - offset,
      );
    }
    s + yoffset * (p.1 - 0.5)
  };

  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);

  let should_crop = |p| !strictly_in_boundaries(p, bound);
  let mut cutted_points = vec![];
  routes =
    crop_routes_with_predicate(&routes, &should_crop, &mut cutted_points);

  let offset = rng.gen_range(-5.0f64, 5.0).max(0.0);
  let mul = width / divisor;
  let mut frame = vec![];
  for i in 0..(pattern.0).round() as usize {
    let l = offset + i as f64 * mul;
    frame.push(vec![
      (pad - l, pad - l),
      (width - pad + l, pad - l),
      (width - pad + l, height - pad + l),
      (pad - l, height - pad + l),
      (pad - l, pad - l),
    ]);
  }

  let mut data = Data::new();
  for route in routes.clone() {
    let simplified = rdp(&route, 0.1);
    if route_length(&simplified) > 2.0 {
      data = render_route(data, simplified);
    }
  }
  for route in frame.clone() {
    data = render_route(data, route);
  }

  let mut l = Group::new()
    .set("inkscape:groupmode", "layer")
    .set("inkscape:label", "Gold")
    .set("fill", "none")
    .set("stroke", "#0FF")
    .set("stroke-linecap", "round")
    .set("stroke-width", 0.2);

  l = l.add(Path::new().set("opacity", 0.6).set("d", data));
  let layers = vec![l];

  // add the traits
  let mut traits = Map::new();
  traits.insert(String::from("Color"), json!(opts.layer1_name.clone()));

  if pattern.1 < 0.1 {
    traits.insert(String::from("Pattern"), json!(String::from("Full")));
  }

  traits.insert(
    String::from("Mirror"),
    json!(String::from(if xflip && yflip {
      "Both"
    } else if xflip {
      "X"
    } else if yflip {
      "Y"
    } else {
      "None"
    })),
  );

  let mut specials = vec![];

  if yoffset > 0.1 {
    specials.push(String::from("Y-Offset"));
  }

  if yoffset > 0.01 {
    specials.push(String::from("Distorted"));
  }

  if specials.len() > 0 {
    traits.insert(String::from("Special"), json!(specials.join(", ")));
  }

  let mut document = svg::Document::new()
    .set("data-hash", opts.hash.to_string())
    .set("data-traits", Value::Object(traits).to_string())
    .set("viewBox", (0, 0, width, height))
    .set("width", format!("{}mm", width))
    .set("height", format!("{}mm", height))
    .set("style", "background:white")
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("xmlns", "http://www.w3.org/2000/svg");
  for l in layers {
    document = document.add(l);
  }
  document
}

pub fn contour<F: FnMut((f64, f64)) -> f64>(
  width: u32,
  height: u32,
  mut f: F,
  thresholds: &Vec<f64>,
) -> Vec<Feature> {
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
  features: Vec<Feature>,
  precision: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  for f in features {
    for g in f.geometry {
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn route_length(route: &Vec<(f64, f64)>) -> f64 {
  let mut length = 0.0;
  for i in 0..route.len() - 1 {
    length += (route[i].0 - route[i + 1].0).powi(2)
      + (route[i].1 - route[i + 1].1).powi(2);
  }
  length.sqrt()
}

fn crop_routes_with_predicate(
  input_routes: &Vec<Vec<(f64, f64)>>,
  should_crop: &dyn Fn((f64, f64)) -> bool,
  cutted_points: &mut Vec<(f64, f64)>,
) -> Vec<Vec<(f64, f64)>> {
  let search = |a_, b_, n| {
    let mut a = a_;
    let mut b = b_;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if should_crop(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes {
    if input_route.len() < 2 {
      continue;
    }
    let mut prev = input_route[0];
    let mut route = vec![];
    if !should_crop(prev) {
      // prev is not to crop. we can start with it
      route.push(prev);
    } else {
      if !should_crop(input_route[1]) {
        // prev is to crop, but [1] is outside. we start with the exact intersection
        let intersection = search(input_route[1], prev, 7);
        prev = intersection;
        cutted_points.push(intersection);
        route.push(intersection);
      } else {
        cutted_points.push(prev);
      }
    }
    // cut segments with crop logic
    for &p in input_route.iter().skip(1) {
      // TODO here, we must do step by step to detect crop inside the segment (prev, p)

      if should_crop(p) {
        if route.len() > 0 {
          // prev is outside, p is to crop
          let intersection = search(prev, p, 7);
          cutted_points.push(intersection);
          route.push(intersection);
          routes.push(route);
          route = vec![];
        } else {
          cutted_points.push(p);
        }
      } else {
        // nothing to crop
        route.push(p);
      }
      prev = p;
    }
    if route.len() >= 2 {
      routes.push(route);
    }
  }

  routes
}

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

fn rng_from_fxhash(hash: String) -> impl Rng {
  let mut bs = [0; 32];
  bs58::decode(hash.chars().skip(2).take(43).collect::<String>())
    .into(&mut bs)
    .unwrap();
  let rng = StdRng::from_seed(bs);
  return rng;
}

#[inline]
fn strictly_in_boundaries(
  p: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> bool {
  p.0 > boundaries.0
    && p.0 < boundaries.2
    && p.1 > boundaries.1
    && p.1 < boundaries.3
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
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
