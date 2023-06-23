/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Clouds
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
  // which inks are used
  pub inks: String,
  // how much inks are used
  pub inks_count: usize,
  // which paper is used
  pub paper: String,
  // eagles count
  pub eagles_density: String,
  // cloud density
  pub clouds_density: String,
  // has_sun_particle
  pub has_sun_particle: String,
  // has montgolfiere
  pub has_montgolfiere: String,
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
  let bounds = (pad, pad, width - pad, height - pad);

  // rng utilities
  let mut rng = rng_from_fxhash(&opts.hash);

  // Prepare all the colors

  let gold_gel = Ink("Gold Gel", "#D8B240", "#FFE38C", 0.6);
  let white_gel = Ink("White Gel", "#E5E5E5", "#FFFFFF", 0.35);
  let black = Ink("Black", "#1A1A1A", "#000000", 0.35);
  let inaho = Ink("iroshizuku ina-ho", "#ba6", "#7f6a33", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let poppy_red = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
  let moonstone = Ink("Moonstone", "#bbb", "#ddd", 0.35);

  let white_paper = Paper("White", "#fff", false);
  let black_paper = Paper("Black", "#202020", true);

  let (mut colors, paper) = if rng.gen_bool(0.2) {
    (
      vec![
        white_gel,
        gold_gel,
        if rng.gen_bool(0.8) {
          white_gel
        } else {
          gold_gel
        },
      ],
      black_paper,
    )
  } else {
    (
      vec![
        black,
        if rng.gen_bool(0.8) {
          amber
        } else if rng.gen_bool(0.12) {
          pink
        } else if rng.gen_bool(0.3) {
          inaho
        } else {
          poppy_red
        },
        if rng.gen_bool(0.04) { moonstone } else { black },
      ],
      white_paper,
    )
  };

  if rng.gen_bool(0.04) {
    colors[1] = colors[0];
  }
  if rng.gen_bool(0.01) {
    colors[2] = colors[1];
  }

  // Prepare the generative code

  let mut mask = PaintMask::new(0.1, width, height);
  let mut routes = Vec::new(); // all the paths to draw are stored here

  let in_shape = |p: (f64, f64)| -> bool {
    !mask.is_painted(p) && strictly_in_boundaries(p, bounds)
  };

  let does_overlap = |c: &VCircle| {
    in_shape((c.x, c.y))
      && circle_route((c.x, c.y), c.r, 8)
        .iter()
        .all(|&p| in_shape(p))
  };

  let mut all = vec![];

  for _i in 0..rng.gen_range(50, 100) {
    let count = (rng.gen_range(0., 100.) * rng.gen_range(0.0, 1.0)) as usize;
    let min = rng.gen_range(8.0, 12.0);
    let max = min + rng.gen_range(0.0, 40.0) * rng.gen_range(0.0, 1.0);
    let optim = (1. + rng.gen_range(0., 10.) * rng.gen_range(0., 1.)) as usize;
    let ppad = rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0);

    let circles = packing(
      &mut rng,
      vec![],
      50000,
      count,
      optim,
      ppad,
      bounds,
      &does_overlap,
      min,
      max,
    );

    all.extend(circles);
  }

  rng.shuffle(&mut all);

  let pow = rng.gen_range(0.8, 1.2);

  let sunx = rng.gen_range(0.4, 0.6) * width;
  let suny = rng.gen_range(0.4, 0.6) * height;
  let sunr = rng.gen_range(0.1, 0.2) * width;

  let mut best_circle_out = VCircle::new(width / 2.0, height / 2.0, 0.0);

  all = all
    .iter()
    .filter(|&c| {
      let dx = c.x - width / 2.0;
      let dy = c.y - height / 2.0;
      let d = 0.5 * (dx.abs() + dy.abs() - c.r).max(0.) / (width + height);
      let sund =
        (euclidian_dist((c.x, c.y), (sunx, suny)) - sunr) / (width + height);
      let keep = rng.gen_bool(d.powf(pow));
      if !keep && c.r > 0.05 * width && sund < 0. && rng.gen_bool(0.5) {
        best_circle_out = c.clone();
      }
      keep
    })
    .cloned()
    .collect();

  // golden spiral distribution of eagles inside best_circle_out
  let golden_angle = PI * (3. - (5f64).sqrt());
  let eagles_count = if rng.gen_bool(0.7) {
    0
  } else {
    (best_circle_out.r * rng.gen_range(0.2, 0.5)) as usize
  };
  let basea = rng.gen_range(-PI, PI);
  let basesize = rng.gen_range(2.0, 5.0);
  if eagles_count > 0 {
    for i in 0..eagles_count {
      let percent = 0.2 + 0.8 * (i as f64 / eagles_count as f64);
      // golden
      let a = basea + i as f64 * golden_angle;
      let r = percent.powf(0.6) * best_circle_out.r;
      let x = best_circle_out.x + r * a.cos();
      let y = best_circle_out.y + r * a.sin();
      let sz = basesize * rng.gen_range(0.8, 1.2);
      let rot = rng.gen_range(-PI, PI)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
      let xreverse = rng.gen_bool(0.5);
      for path in eagle((x, y), sz, rot, xreverse, &mut rng) {
        routes.push((0, path));
      }
    }
  }

  let with_mongolfiere = rng.gen_bool(0.5);

  let mut copy = all.clone();
  if eagles_count > 0 {
    copy.push(best_circle_out);
  }

  let threshold = rng.gen_range(0.0, 0.1) * width.min(height);

  copy = copy
    .iter()
    .filter(|c| c.r > threshold)
    .cloned()
    .collect::<Vec<_>>();

  let mut has_montgolfiere = false;

  if with_mongolfiere {
    let count = copy.len() + 1;
    let min = rng.gen_range(0.05, 0.1) * width.min(height);
    let max = min + rng.gen_range(0.1, 0.3) * width.min(height);
    let res = packing(
      &mut rng,
      copy.clone(),
      100000,
      count,
      50,
      0.0,
      bounds,
      &does_overlap,
      min,
      max,
    );
    if res.len() >= count {
      let last = res[res.len() - 1];
      let px = last.x / width;
      let py = last.y / height;

      if px > 0.1 && px < 0.9 && py > 0.2 && py < 0.8 {
        routes.extend(montgolfiere(&mut rng, &mut mask, last));
        has_montgolfiere = true;
      }
    }
  }

  /*
  routes.extend(montgolfiere(
    &mut rng,
    &mut mask,
    VCircle::new(width / 2.0, height / 2.0, 0.1 * height),
  ));
  */

  /*
  routes.push((
    2,
    circle_route(
      (best_circle_out.x, best_circle_out.y),
      best_circle_out.r,
      20,
    ),
  ));*/

  all.iter().enumerate().for_each(|(i, &c)| {
    let (rts, circles) = cloud_in_circle(&mut rng, &c);
    let rts = clip_routes(
      &rts.iter().map(|r| (2 * (i % 2), r.clone())).collect(),
      &|p| mask.is_painted(p),
      0.3,
      7,
    );
    routes.extend(rts);
    for c in circles.clone() {
      mask.paint_circle(&c);
    }
  });

  let dr = rng.gen_range(1.1, 1.2) * colors[1].3;

  let mut sun = vec![
    (1, spiral_optimized(sunx, suny, sunr, dr, 0.05)),
    (
      1,
      circle_route((sunx, suny), sunr, (20. + 2. * sunr) as usize),
    ),
  ];

  let has_sun_particle = rng.gen_bool(0.3);

  // particle of sun
  if has_sun_particle {
    let approx = 0.3;
    let mut route = Vec::new();
    let mut r: f64 = sunr + dr;
    let mut a = 0f64;
    let rmul = rng.gen_range(0.0, 10.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
    let rmul2 = rng.gen_range(-2.0f64, 5.0).max(0.0);
    let ymul = (rng.gen_range(-2.0f64, 5.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0))
    .max(0.0);
    let xmul = if rng.gen_bool(0.5) {
      ymul
    } else {
      (rng.gen_range(-5.0f64, 5.0)
        * rng.gen_range(0., 1.0)
        * rng.gen_range(0., 1.0))
      .max(0.0)
    };
    let space_mod = 2.0 + rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0);
    loop {
      let ar = r;
      let p = round_point((sunx + ar * a.cos(), suny + ar * a.sin()), 0.01);
      let l = route.len();
      let disabled = (rmul * r + p.0 * xmul + p.1 * ymul) % space_mod > 1.0
        || r * rmul2 % 4.0 > 1.0;
      if l == 0 || euclidian_dist(route[l - 1], p) > approx {
        if disabled || !strictly_in_boundaries(p, bounds) {
          if l > 1 {
            sun.push((1, route));
            route = vec![];
          } else if l > 0 {
            route = vec![];
          }
        } else {
          route.push(p);
        }
      }
      let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
      a = (a + da) % (PI * 2.);
      r += dr * da / (PI * 2.);
      if r > width {
        break;
      }
    }
  }

  routes.extend(clip_routes(&sun, &|p| mask.is_painted(p), 0.3, 7));

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

  let eagles_density = if eagles_count == 0 {
    "None".to_string()
  } else if eagles_count < 5 {
    "Some".to_string()
  } else {
    "Many".to_string()
  };

  let density = mask.density();

  let clouds_density = if density < 0.4 {
    "Low".to_string()
  } else if density < 0.6 {
    "Normal".to_string()
  } else {
    "High".to_string()
  };

  let feature = Feature {
    inks: inks.join(", "),
    inks_count: inks_length,
    paper: paper.0.to_string(),
    eagles_density,
    clouds_density,
    has_sun_particle: if has_sun_particle { "Yes" } else { "No" }.to_string(),
    has_montgolfiere: if has_montgolfiere { "Yes" } else { "No" }.to_string(),
  };

  let feature_json = serde_json::to_string(&feature).unwrap();
  let palette_json = serde_json::to_string(&Palette {
    paper,
    primary: colors[0],
    secondary: colors[1],
    third: colors[2],
  })
  .unwrap();

  let mask_colors = vec!["#0FF", "#F0F", "#FF0"];

  // TODO optimise lines with rdp(&route, 0.1)

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
      "@greweb - 2023 - Plottable Clouds".to_string(),
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

fn cloud_in_circle<R: Rng>(
  rng: &mut R,
  circle: &VCircle,
) -> (Vec<Vec<(f64, f64)>>, Vec<VCircle>) {
  // FIXME the clouds have a weird issue on the fact we don't always see the edges

  let mut routes = vec![];

  let mut circles: Vec<VCircle> = vec![];

  let stretchy = rng.gen_range(0.2, 1.0);

  let count = rng.gen_range(16, 80);
  for _i in 0..count {
    let radius = circle.r * rng.gen_range(0.3, 0.5) * rng.gen_range(0.2, 1.0);
    let angle = rng.gen_range(0.0, 2.0 * PI);
    let x = circle.x + angle.cos() * (circle.r - radius);
    let y = circle.y
      + angle.sin() * (circle.r - radius) * rng.gen_range(0.5, 1.0) * stretchy;
    let circle = VCircle::new(x, y, radius);

    let should_crop = |p| circles.iter().any(|c| c.includes(p));

    let mut input_routes = vec![];
    let mut r = radius;
    let dr = rng.gen_range(0.5, 2.0);
    loop {
      if r < 1.0 {
        break;
      }
      let count = (r * 2.0 + 10.0) as usize;
      let amp = rng.gen_range(0.5 * PI, 1.2 * PI);
      let ang = angle
        + PI
          * rng.gen_range(-1.0, 1.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0);
      let start = ang - amp / 2.0;
      let end = ang + amp / 2.0;
      input_routes.push(arc((x, y), r, start, end, count));
      r -= dr;
    }

    routes.extend(crop_routes_with_predicate_rng(
      rng,
      0.0,
      input_routes,
      &should_crop,
      &mut vec![],
    ));

    circles.push(circle);
  }

  (routes, circles)
}

fn arc(
  center: (f64, f64),
  r: f64,
  start: f64,
  end: f64,
  count: usize,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = start + (end - start) * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f64,
  y: f64,
  r: f64,
}
impl VCircle {
  fn new(x: f64, y: f64, r: f64) -> Self {
    VCircle { x, y, r }
  }
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist(p, (self.x, self.y)) < self.r
  }
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

fn scaling_search<F: FnMut(f64) -> bool>(
  mut f: F,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let mut from = min_scale;
  let mut to = max_scale;
  loop {
    if !f(from) {
      return None;
    }
    if to - from < 0.1 {
      return Some(from);
    }
    let middle = (to + from) / 2.0;
    if !f(middle) {
      to = middle;
    } else {
      from = middle;
    }
  }
}

fn search_circle_radius(
  does_overlap: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing<R: Rng>(
  rng: &mut R,
  initial_circles: Vec<VCircle>,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = initial_circles.clone();
  let mut tries = Vec::new();
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) =
      search_circle_radius(&does_overlap, &circles, x, y, min_scale, max_scale)
    {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
}

// TODO rework with clip_routes
fn crop_routes_with_predicate_rng<R: Rng>(
  rng: &mut R,
  proba_skip: f64,
  input_routes: Vec<Vec<(f64, f64)>>,
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
    if proba_skip > 0.0 && rng.gen_bool(proba_skip) {
      routes.push(input_route);
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes(
  input_routes: &Vec<(usize, Vec<(f64, f64)>)>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  // locate the intersection where inside and outside cross
  let search = |inside: (f64, f64), outside: (f64, f64), n| {
    let mut a = inside;
    let mut b = outside;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if is_outside(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for (c, input_route) in input_routes.iter() {
    if input_route.len() < 2 {
      continue;
    }
    let clr = *c;

    let mut prev = input_route[0];
    let mut prev_is_outside = is_outside(prev);
    let mut route = vec![];
    if !prev_is_outside {
      // prev is not to crop. we can start with it
      route.push(prev);
    }

    for &p in input_route.iter().skip(1) {
      // we iterate in small steps to detect any interruption
      let static_prev = prev;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let d = (dx * dx + dy * dy).sqrt();
      let vx = dx / d;
      let vy = dy / d;
      let iterations = (d / stepping).ceil() as usize;
      let mut v = 0.0;
      for _i in 0..iterations {
        v = (v + stepping).min(d);
        let q = (static_prev.0 + vx * v, static_prev.1 + vy * v);
        let q_is_outside = is_outside(q);
        if prev_is_outside != q_is_outside {
          // we have a crossing. we search it precisely
          let intersection = if prev_is_outside {
            search(q, prev, dichotomic_iterations)
          } else {
            search(prev, q, dichotomic_iterations)
          };

          if q_is_outside {
            // we close the path
            route.push(intersection);
            if route.len() > 1 {
              // we have a valid route to accumulate
              routes.push((clr, route));
            }
            route = vec![];
          } else {
            // we open the path
            route.push(intersection);
          }
          prev_is_outside = q_is_outside;
        }

        prev = q;
      }

      // prev should be == p
      if !prev_is_outside {
        // prev is not to crop. we can start with it
        route.push(p);
      }
    }

    if route.len() > 1 {
      // we have a valid route to accumulate
      routes.push((clr, route));
    }
  }

  routes
}

struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
}

impl PaintMask {
  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    }
  }

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  fn density(&self) -> f64 {
    let mut count = 0;
    for &p in self.mask.iter() {
      if p {
        count += 1;
      }
    }
    count as f64 / (self.mask.len() as f64)
  }

  fn is_painted(&self, point: (f64, f64)) -> bool {
    // check out of bounds
    if point.0 <= 0.0
      || point.0 >= self.width
      || point.1 <= 0.0
      || point.1 >= self.height
    {
      return false;
    }
    let precision = self.precision;
    let width = self.width;
    let x = (point.0 / precision) as usize;
    let y = (point.1 / precision) as usize;
    let wi = (width / precision) as usize;
    self.mask[x + y * wi]
  }

  fn paint_circle(&mut self, circle: &VCircle) {
    let (minx, miny, maxx, maxy) = (
      circle.x - circle.r,
      circle.y - circle.r,
      circle.x + circle.r,
      circle.y + circle.r,
    );
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if euclidian_dist(point, (circle.x, circle.y)) < circle.r {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
}

#[inline]
fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  point: (f64, f64),
) -> bool {
  let mut c = false;
  for i in 0..polygon.len() {
    let j = (i + 1) % polygon.len();
    if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
      && (point.0
        < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
          / (polygon[j].1 - polygon[i].1)
          + polygon[i].0)
    {
      c = !c;
    }
  }
  c
}
fn polygon_bounds(polygon: &Vec<(f64, f64)>) -> (f64, f64, f64, f64) {
  let mut minx = f64::MAX;
  let mut miny = f64::MAX;
  let mut maxx = f64::MIN;
  let mut maxy = f64::MIN;
  for &(x, y) in polygon {
    minx = minx.min(x);
    miny = miny.min(y);
    maxx = maxx.max(x);
    maxy = maxy.max(y);
  }
  (minx, miny, maxx, maxy)
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

fn circle_route(center: (f64, f64), r: f64, count: usize) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

#[inline]
fn round_point((x, y): (f64, f64), precision: f64) -> (f64, f64) {
  (
    (x / precision).round() * precision,
    (y / precision).round() * precision,
  )
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

fn spiral_optimized(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = round_point((x + r * a.cos(), y + r * a.sin()), 0.01);
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
}

fn eagle<R: Rng>(
  origin: (f64, f64),
  sz: f64,
  rotation: f64,
  xreverse: bool,
  rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let scale = sz / 5.0;
  let xmul = if xreverse { -1.0 } else { 1.0 };
  let count = 2 + (scale * 3.0) as usize;
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let shaking = scale * 0.1;

  // body
  let bodyw = 5.0;
  let bodyh = 1.5;
  let headcompression = rng.gen_range(0.1, 0.5);
  let headoff = rng.gen_range(0.1, 0.5);
  for i in 0..count {
    let yp = (i as f64 - (count - 1) as f64 * 0.5) / (count as f64);
    let ybase = bodyh * yp;
    let route = shake(
      path_subdivide_to_curve(
        vec![
          (-rng.gen_range(0.4, 0.6) * bodyw, 1.5 * ybase),
          (-0.3 * bodyw, ybase),
          (0.2 * bodyw, ybase),
          (0.45 * bodyw, headcompression * ybase + headoff * bodyh),
        ],
        1,
        0.8,
      ),
      shaking,
      rng,
    );
    routes.push(route);
  }

  let count = 2 + (scale * rng.gen_range(4.0, 6.0)) as usize;

  // wings
  let wingw = 1.4;
  let wingh = 8.0;
  let dx1 = rng.gen_range(-4.0, 4.0) * rng.gen_range(0.0, 1.0);
  let dx2 = if rng.gen_bool(0.8) {
    -dx1
  } else {
    rng.gen_range(-3.0, 3.0)
  };
  let spread1 = 1.0 + rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let spread2 = 1.0 + rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let offset1 = rng.gen_range(-1.0, 0.6) * rng.gen_range(0.0, 1.0);
  let offset2 = rng.gen_range(-1.0, 0.6) * rng.gen_range(0.0, 1.0);
  let interp = 0.5;
  let wing1m = 1.0 - rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
  let wing2m = 1.0 - rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
  let wing2up = rng.gen_bool(0.5);

  for i in 0..count {
    let xp = (i as f64 - (count - 1) as f64 * 0.5) / (count as f64);
    let xbase = wingw * xp;
    let wing1 = rng.gen_range(0.8, 1.1) * wing1m;
    let wing2 =
      rng.gen_range(0.8, 1.1) * wing2m * (if wing2up { -1.0 } else { 1.0 });
    let route = shake(
      path_subdivide_to_curve(
        vec![
          (
            xbase * spread1 + dx1 + wingw * offset1,
            -wingh * 0.5 * wing1,
          ),
          (xbase + dx1 * interp, -wingh * 0.5 * interp * wing1),
          (xbase, 0.0),
          (xbase + dx2 * interp, wingh * 0.5 * interp * wing2),
          (xbase * spread2 + dx2 + wingw * offset2, wingh * 0.5 * wing2),
        ],
        2,
        0.8,
      ),
      shaking,
      rng,
    );
    routes.push(route);
  }

  // scale, rotate & translate
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&p| {
          let p = p_r(p, rotation);
          let p = (xmul * scale * p.0 + origin.0, scale * p.1 + origin.1);
          p
        })
        .collect()
    })
    .collect()
}

fn path_subdivide_to_curve_it(
  path: Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path;
  }
  let mut route = Vec::new();
  let mut first = path[0];
  let mut last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if looped {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for i in 1..(l - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if looped {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if looped {
    route.push(first);
  }
  route
}

fn path_subdivide_to_curve(
  path: Vec<(f64, f64)>,
  n: usize,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}

fn shake<R: Rng>(
  path: Vec<(f64, f64)>,
  scale: f64,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  path
    .iter()
    .map(|&(x, y)| {
      let dx = rng.gen_range(-scale, scale);
      let dy = rng.gen_range(-scale, scale);
      (x + dx, y + dy)
    })
    .collect()
}

fn montgolfiere<R: Rng>(
  rng: &mut R,
  mask: &mut PaintMask,
  c: VCircle,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  let incr = 0.4;

  let r = rng.gen_range(0.8, 1.4) * c.r;

  let count = rng.gen_range(6, 10);
  let h = rng.gen_range(1.2, 1.6) * r;
  let pow = rng.gen_range(1.8, 3.0);
  let sz = rng.gen_range(0.3, 0.4);

  let connects = vec![
    0,
    ((rng.gen_range(0.02, 0.1) * h) / incr) as usize,
    ((rng.gen_range(0.3, 0.4) * h) / incr) as usize,
    ((rng.gen_range(0.5, 0.6) * h) / incr) as usize,
    ((rng.gen_range(0.7, 0.9) * h) / incr) as usize,
    ((0.92 * h) / incr) as usize,
    ((0.95 * h) / incr) as usize,
    ((0.97 * h) / incr) as usize,
    ((0.99 * h) / incr) as usize,
  ];
  let mut transverse: Vec<Vec<(f64, f64)>> = vec![];
  for _c in connects.iter() {
    transverse.push(vec![]);
  }
  let mut zigzag = vec![];
  let mut lastroute = vec![];

  let v1 = rng.gen_range(1.0, 1.3);
  let v2 = rng.gen_range(0.4, 0.6);

  let mut r1 = vec![];
  let mut r2 = vec![];

  for i in 0..count {
    let p = i as f64 / (count as f64 - 1.) - 0.5;
    let mut dy = 0.0;
    let mut route = vec![];
    let mut j = 0;
    while dy < h {
      let py = dy / h;
      let v = 0.9 - 0.6 * ((py.min(sz * 2.) - sz).abs() / sz).powf(pow)
        + 0.4 * (1.0 - py) * py;

      let x = c.x + p * v * r;
      let y = c.y + dy - h / 2.0
        + 0.05
          * h
          * (v1 - (1.0 - 2. * (0.5 - p.abs())).powf(2.0))
          * (v2 - smoothstep(0.05, 0.0, py)
            + (2.0 * (0.5 - (py - 0.5).abs())).powf(2.0));
      route.push((x, y));

      let index = connects.iter().position(|&c| c == j);
      if let Some(index) = index {
        transverse[index].push((x, y));
        if index == 2 + (i % 2) {
          zigzag.push((x, y));
        }
      }

      mask.paint_circle(&VCircle::new(x, y, 1.8 * incr));
      dy += incr;
      j += 1;
      if dy >= h {
        lastroute.push((x, y));
      }
    }
    if i == 0 {
      r1 = route.clone();
    } else if i == count - 1 {
      r2 = route.clone();
    }
    routes.push((0, route));
  }

  r2.reverse();

  mask.paint_polygon(&vec![r1, r2].concat());

  for route in transverse {
    routes.push((0, route));
  }
  routes.push((0, zigzag));
  routes.push((0, lastroute));

  routes
}

#[inline]
fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
  let k = ((x - a) / (b - a)).max(0.0).min(1.0);
  return k * k * (3.0 - 2.0 * k);
}
