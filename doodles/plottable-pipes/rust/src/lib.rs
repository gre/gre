/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Pipes
 */
mod utils;
use noise::*;
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
  // how much pipes
  pub pipes: String,
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

  // rng utilities
  let mut rng = rng_from_fxhash(&opts.hash);

  // Prepare all the colors

  let red_gel = Ink("Red Gel", "#BF738C", "#D880A6", 0.35);
  let orange_gel = Ink("Orange Gel", "#B27333", "#E68C4D", 0.35);
  let blue_gel = Ink("Blue Gel", "#338CFF", "#4D8CFF", 0.35);
  let green_gel = Ink("Green Gel", "#00B2A6", "#19CCBF", 0.35);

  let gold_gel = Ink("Gold Gel", "#D8B240", "#FFE38C", 0.6);
  let white_gel = Ink("White Gel", "#E5E5E5", "#FFFFFF", 0.35);

  let black_paper = Paper("Black", "#202020", true);

  let (colors, paper) = (
    vec![
      white_gel,
      gold_gel,
      if rng.gen_bool(0.3) {
        red_gel
      } else if rng.gen_bool(0.3) {
        orange_gel
      } else if rng.gen_bool(0.3) {
        blue_gel
      } else {
        green_gel
      },
    ],
    black_paper,
  );

  // Generate the art

  let mut routes = vec![];
  let mut mask = PaintMask::new(0.2, width, height);

  mask.paint_borders(pad - 0.1);

  let filling = WormsFilling::rand(&mut rng);

  let is_gold_one = rng.gen_bool(0.4);
  let pgold = rng.gen_range(0.0, 0.2);
  let pthirdcolor =
    rng.gen_range(0.0, 0.5) * rng.gen_range(0f64, 1.0).powf(8.0);
  let pfill = rng.gen_range(0.0, 0.5);
  let panomalyendpipe = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);

  let diamondeffect = rng.gen_range(-0.1f64, 0.5).max(0.0);
  let diamondeffect2 =
    rng.gen_range(-0.2f64, 0.8).max(0.0) * rng.gen_range(0.0, 1.0);

  let rand_displacement = (
    rng.gen_range(-0.4f64, 0.2).max(0.0) * pad,
    rng.gen_range(-0.3f64, 0.1).max(0.0) * pad,
  );
  let pdisplace = rng.gen_range(-0.5, 1.5);

  let pipe_size = 2.0 + rng.gen_range(0.0, 10.0) * rng.gen_range(0.5, 1.0);
  let pipe_pad = rng.gen_range(0.0, 4.0) * pipe_size;

  let filldist = 0.4 + rng.gen_range(0.5, 2.0) * rng.gen_range(0.1, 1.0);
  let fill = (pipe_size / filldist) as usize;

  let pipe_count = (width * 0.8 / (pipe_size + pipe_pad)).max(1.0) as usize;

  let mut connects: Vec<_> = (0..pipe_count).collect();
  rng.shuffle(&mut connects);
  let mut pipes: Vec<_> = connects
    .iter()
    .enumerate()
    .map(|(i, &to)| (i, to))
    .collect();
  rng.shuffle(&mut pipes);

  let ydivs = (5. + rng.gen_range(0., 60.) * rng.gen_range(0.0, 1.0)) as usize;
  let incrmax = rng.gen_range(1, 2 * ydivs);

  let xmul = (width - 2.0 * pad) / (pipe_count as f64);
  let ymul = (height + pad) / (ydivs as f64);

  let clr = if is_gold_one { 1 } else { rng.gen_range(0, 2) };
  let mut incr = 0.0;
  while incr < pipe_size {
    let y = pad + 0.5 * ymul - incr;
    routes.push((clr, vec![(pad, y), (width - pad, y)]));
    incr += filldist;
  }
  routes.push((
    clr,
    vec![
      (pad, pad + 0.5 * ymul),
      (pad, pad + 0.5 * ymul - pipe_size + filldist),
    ],
  ));
  routes.push((
    clr,
    vec![
      (width - pad, pad + 0.5 * ymul),
      (width - pad, pad + 0.5 * ymul - pipe_size + filldist),
    ],
  ));

  let lastcellistoi = rng.gen_bool(0.5);

  let goldone_i = rng.gen_range(0, pipe_count);
  let always_here = rng.gen_range(0, pipe_count);

  let mut pipes_nb = 0;

  let droprate = rng.gen_range(0.0, 0.8);
  for (fromindex, toindex) in pipes {
    if always_here != fromindex
      && rng.gen_bool(droprate)
      && !(is_gold_one && fromindex == goldone_i)
    {
      continue;
    }
    pipes_nb += 1;
    let dc = (fromindex as f64 / (pipe_count as f64) - 0.5).abs() * 2.0;
    let fromi = (dc * (ydivs as f64 * diamondeffect)) as usize;
    let dc = ((toindex as f64) / (pipe_count as f64) - 0.5).abs() * 2.0;
    let mut toi = ydivs - 1 - (dc * (ydivs as f64 * diamondeffect2)) as usize;
    let lastcell = if rng.gen_bool(panomalyendpipe) && fromi < toi {
      rng.gen_range(fromi, toi)
    } else if lastcellistoi {
      toi
    } else {
      ydivs - 1
    };
    if lastcell > toi {
      toi = lastcell;
    }

    let mut xi = fromindex;
    let mut yi = 0;
    let mut route = vec![];

    loop {
      let mut dx = 0.0;
      let mut dy = 0.0;
      if yi > fromi && yi < toi {
        dx = rng.gen_range(-0.5, 0.5);
        dy = rng.gen_range(-0.5, 0.5);
      }
      let p = (
        pad + (xi as f64 + 0.5 + dx) * xmul,
        pad + (yi as f64 + 0.5 + dy) * ymul,
      );

      route.push(p);

      if yi == lastcell {
        break;
      }
      if yi == 0 && fromi > 0 {
        yi = fromi;
        continue;
      }
      if yi >= toi && toi != lastcell {
        xi = toindex;
        yi = lastcell;
        continue;
      }
      xi = rng.gen_range(0, pipe_count);
      yi += if incrmax < 2 {
        1
      } else {
        rng.gen_range(1, incrmax)
      };
      if yi >= toi {
        xi = toindex;
        yi = toi;
      }
    }
    let clr = if is_gold_one {
      if fromindex == goldone_i {
        1
      } else {
        0
      }
    } else if rng.gen_bool(pthirdcolor) {
      2
    } else if rng.gen_bool(pgold) {
      1
    } else {
      0
    };
    let coloring = if clr > 0 {
      fill
    } else {
      if rng.gen_bool(pfill) && fill > 0 {
        rng.gen_range(0, fill)
      } else {
        0
      }
    };
    let disp = pdisplace > 0.99 || pdisplace > 0. && rng.gen_bool(pdisplace);
    let pipes: Vec<(usize, Vec<(f64, f64)>)> = make_pipes(
      &mut rng,
      route,
      pipe_size,
      coloring,
      clr,
      if disp { rand_displacement } else { (0., 0.) },
      &mut mask,
    );
    routes.extend(pipes);
  }

  let count = rng.gen_range(5, 20);
  for i in 0..count {
    let f = (i as f64 + 1.) / (count as f64);
    let xincr = 0.5 + rng.gen_range(0.5, 20.0) * rng.gen_range(0.0, 1.0);
    let y = height - pad;
    let yamp = (height * (0.5 + 0.5 * f)) * rng.gen_range(0.0, 1.0);
    let perlin = Perlin::new();
    let density = 1.0 + rng.gen_range(0.0, 5.0) * rng.gen_range(0.1, 1.0);
    let iterations = (rng.gen_range(0.5, 1.0) * width * height / 20.0) as usize;
    let clr = 0;

    let f1 = rng.gen_range(0.0, 0.04) * rng.gen_range(0.1, 1.0);
    let amp2 = rng.gen_range(0.2, 2.0) * rng.gen_range(0.1, 1.0);
    let f2 = rng.gen_range(0.0, 0.1) * rng.gen_range(0., 1.0);
    let amp3 = rng.gen_range(0.1, 2.0) * rng.gen_range(0., 1.0);
    let f3 = rng.gen_range(0.0, 0.1) * rng.gen_range(0., 1.0);
    let seed1 = rng.gen_range(0.0, 100.0);
    let seed2 = rng.gen_range(0.0, 100.0);
    let seed3 = rng.gen_range(0.0, 100.0);

    let valuef = |x, y| {
      let n = 0.5
        + 0.5
          * perlin.get([
            f1 * x,
            f1 * y,
            amp2
              * perlin.get([
                f2 * x,
                seed2 + amp3 * perlin.get([seed3, f3 * x, f3 * y]),
                f2 * y,
              ])
              + seed1
              + i as f64 * 55.5,
          ]);
      n
    };

    routes.extend(filled_mountains(
      &mut rng,
      &mut mask,
      pad,
      width - pad,
      xincr,
      y,
      yamp,
      &valuef,
      &filling,
      iterations,
      density,
      clr,
    ));
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

  let pipes_str = if pipes_nb < 1 {
    "None"
  } else if pipes_nb < 2 {
    "One"
  } else if pipes_nb < 3 {
    "Two"
  } else if pipes_nb < 8 {
    "Some"
  } else if pipes_nb < 20 {
    "Many"
  } else {
    "A lot"
  };

  let feature = Feature {
    inks: inks.join(", "),
    inks_count: inks_length,
    paper: paper.0.to_string(),
    pipes: pipes_str.to_string(),
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
      "@greweb - 2023 - Plottable Pipes".to_string(),
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

#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}

fn make_pipes<R: Rng>(
  rng: &mut R,
  route: Vec<(f64, f64)>,
  pipe_width: f64,
  coloring: usize,
  clr: usize,
  (rand_displacementx, rand_displacementy): (f64, f64),
  mask: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  // it will make intermediate stops to make orthogonal pipes
  let first = route[0];
  let w = pipe_width / 2.0;

  let mut polygons = vec![];

  let mut leftpath = vec![];
  let mut rightpath = vec![];
  let mut stops = vec![];
  leftpath.push((first.0 - w, first.1));
  rightpath.push((first.0 + w, first.1));
  stops.push(first);

  let mut filling = vec![];
  let lines = coloring + 2;
  for i in 0..lines {
    let x = (i as f64) / (lines as f64 - 1.) - 0.5;
    let v = w * 2. * x;
    let p = (first.0 + v, first.1);
    filling.push(vec![p]);
  }

  for i in 0..route.len() - 1 {
    let p1 = route[i];
    let p2 = route[i + 1];
    let dispx = rand_displacementx
      .min(0.2 * (p2.0 - p1.0).abs())
      .min(0.2 * (p2.1 - p1.1).abs());
    let dispy = rand_displacementy
      .min(0.2 * (p2.0 - p1.0).abs())
      .min(0.2 * (p2.1 - p1.1).abs());
    let a1 = rng.gen_range(-PI, PI);
    let a2 = rng.gen_range(-PI, PI);
    let h1 = (
      p1.0 + dispx * a1.cos(),
      (p1.1 + p2.1) / 2.0 + dispy * a1.sin(),
    );
    let h2 = (p2.0 + dispx * a2.cos(), h1.1 + dispy * a2.sin());
    stops.push(h1);
    stops.push(h2);
    stops.push(p2);

    let l0 = leftpath[leftpath.len() - 1];
    let r0 = rightpath[rightpath.len() - 1];

    let hd = if p1.0 < p2.0 { w } else { -w };
    let l1 = (h1.0 - w, h1.1 + hd);
    let l2 = (h2.0 - w, h2.1 + hd);
    let l3 = (p2.0 - w, p2.1);

    leftpath.push(l1);
    leftpath.push(l2);
    leftpath.push(l3);

    let r1 = (h1.0 + w, h1.1 - hd);
    let r2 = (h2.0 + w, h2.1 - hd);
    let r3 = (p2.0 + w, p2.1);

    rightpath.push(r1);
    rightpath.push(r2);
    rightpath.push(r3);

    for i in 0..lines {
      let x = (i as f64) / (lines as f64 - 1.) - 0.5;
      let v = w * 2. * x;
      let l1 = (h1.0 + v, h1.1 - hd * v / w);
      let l2 = (h2.0 + v, h2.1 - hd * v / w);
      let l3 = (p2.0 + v, p2.1);
      filling[i].push(l1);
      filling[i].push(l2);
      filling[i].push(l3);
    }

    let poly = vec![l0, r0, r1, l1];
    polygons.push(poly);
    let poly = vec![l1, r1, r2, l2];
    polygons.push(poly);
    let poly = vec![l2, r2, r3, l3];
    polygons.push(poly);
  }

  let mut routes = vec![];
  for f in filling {
    routes.push((clr, f));
  }

  let is_outside = |p: (f64, f64)| mask.is_painted(p);

  routes = clip_routes_with_colors(&routes, &is_outside, 0.5, 5);

  for poly in &polygons {
    mask.paint_polygon(poly);
  }

  /*
  let mut routes = vec![];
  for poly in &polygons {
    let mut route = vec![];
    route.extend(poly.clone());
    route.push(poly[0]);
    routes.push(route);
  }
  */

  routes
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

fn filled_mountains<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  xfrom: f64,
  xto: f64,
  xincr: f64,
  ybase: f64,
  yamp: f64,
  valuef: &dyn Fn(f64, f64) -> f64,
  filling: &WormsFilling,
  iterations: usize,
  density: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  // sample the curve with f
  let mut curve = vec![];
  let mut x = xfrom;
  while x < xto {
    let y = ybase - yamp * valuef(x, ybase);
    curve.push((x, y));
    x += xincr;
  }
  if x > xto {
    let y = ybase - yamp * valuef(xto, ybase);
    curve.push((xto, y));
  }

  if curve.len() < 2 {
    return routes;
  }

  // routes.push((clr, curve.clone()));

  // make the polygons
  let mut polys = vec![];
  let len = curve.len();
  for i in 0..len {
    let j = (i + 1) % len;
    let mut poly = vec![];
    let a = curve[i];
    let b = curve[j];
    poly.push(a);
    poly.push(b);
    poly.push((b.0, ybase));
    poly.push((a.0, ybase));
    polys.push(poly);
  }

  // fill them
  let f = |x, y| {
    if paint.is_painted((x, y)) {
      return 0.0;
    }
    let collides = polys
      .iter()
      .any(|poly| polygon_includes_point(poly, (x, y)));
    if collides {
      density
    } else {
      0.0
    }
  };
  let extra = 2.0;
  let bound: (f64, f64, f64, f64) = (
    xfrom - extra,
    ybase - yamp - extra,
    xto + extra,
    ybase + extra,
  );
  routes.extend(filling.fill(rng, &f, bound, clr, iterations));

  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }
  routes
}

struct WormsFilling {
  rot: f64,
  step: f64,
  straight: f64,
  min_l: usize,
  max_l: usize,
  decrease_value: f64,
  search_max: usize,
  min_weight: f64,
  freq: f64,
  seed: f64,
}
impl WormsFilling {
  // new
  fn rand<R: Rng>(rng: &mut R) -> Self {
    let seed = rng.gen_range(-999., 999.);
    let rot = PI / rng.gen_range(1.0, 2.0);
    let step = 0.6;
    let straight = rng.gen_range(0.0, 0.1);
    let min_l = 5;
    let max_l = 20;
    let decrease_value = 1.;
    let search_max = 500;
    let min_weight = 1.;
    let freq = 0.05;
    Self {
      rot,
      step,
      straight,
      min_l,
      max_l,
      decrease_value,
      search_max,
      min_weight,
      freq,
      seed,
    }
  }

  fn fill<R: Rng>(
    &self,
    rng: &mut R,
    f: &dyn Fn(f64, f64) -> f64,
    bound: (f64, f64, f64, f64),
    clr: usize,
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let perlin = Perlin::new();
    let w = bound.2 - bound.0;
    let h = bound.3 - bound.1;
    let precision = 0.4;
    if w <= 2. * precision || h <= 2. * precision {
      return routes;
    }
    let mut map = WeightMap::new(w, h, 0.4);

    map.fill_fn(&|p| f(p.0 + bound.0, p.1 + bound.1));

    let seed = self.seed;
    let rot = self.rot;
    let step = self.step;
    let straight = self.straight;
    let min_l = self.min_l;
    let max_l = self.max_l;
    let decrease_value = self.decrease_value;
    let search_max = self.search_max;
    let min_weight = self.min_weight;
    let freq = self.freq;

    let mut bail_out = 0;

    for _i in 0..iterations {
      let top = map.search_weight_top(rng, search_max, min_weight);
      if top.is_none() {
        bail_out += 1;
        if bail_out > 10 {
          break;
        }
      }
      if let Some(o) = top {
        let angle = perlin.get([seed, freq * o.0, freq * o.1]);

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
            let pts = rdp(&route, 0.05);
            // remap
            let rt = pts
              .iter()
              .map(|&i| (route[i].0 + bound.0, route[i].1 + bound.1))
              .collect();
            routes.push((clr, rt));
          }
        }
      }
    }

    routes
  }
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

  fn paint_rectangle(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        self.mask[x + y * wi] = true;
      }
    }
  }

  fn paint_borders(&mut self, pad: f64) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
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

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes_with_colors(
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

  for (clrp, input_route) in input_routes.iter() {
    let clr = *clrp;
    if input_route.len() < 2 {
      continue;
    }

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
  fn fill_fn(&mut self, f: &impl Fn((f64, f64)) -> f64) {
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
    let x0 = ((x - radius).floor().max(0.) as usize).min(self.w);
    let y0 = ((y - radius).floor().max(0.) as usize).min(self.h);
    let x1 = ((x + radius).ceil().max(0.) as usize).min(self.w);
    let y1 = ((y + radius).ceil().max(0.) as usize).min(self.h);
    if x0 >= self.w || y0 >= self.h {
      return;
    }
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

// adapted from library "ramer_douglas_peucker"
/// Given a set of points and an epsilon, returns a list of indexes to keep.
/// If the first and last points are the same, then the points are treated as a closed polygon
pub fn rdp(points: &Vec<(f64, f64)>, epsilon: f64) -> Vec<usize> {
  if points.len() < 3 {
    return (0..points.len()).collect();
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

  results
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
