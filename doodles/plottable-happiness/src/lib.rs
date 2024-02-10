use algo::{clipping::clip_routes_with_colors, paintmask::PaintMask};
use contour::{Contour, ContourBuilder};
use geo::CoordsIter;
use noise::*;
use palette::Palette;
use rand::prelude::*;
use serde_json::json;
use wasm_bindgen::prelude::*;
mod algo;
mod fxhash;
use fxhash::*;
mod svgplot;
use svgplot::*;
mod performance;
use performance::*;
mod global;
use global::*;
mod palette;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Happiness
 */

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

  let mut paint = PaintMask::new(precision, width, height);

  let perlin = Perlin::new(rng.gen());
  let perlinseed: f64 = rng.gen();

  let mut routes = vec![];

  let amp = rng.gen_range(0.0..4.0) * (rng.gen_range(-0.2f32..1.0).max(0.0));
  let f = rng.gen_range(0.0..6.0) * (rng.gen_range(-0.1f64..1.0).max(0.0));
  for i in 0..rng.gen_range(1..5) {
    let sizemul = rng.gen_range(0.5..1.0);
    let min_scale = rng.gen_range(8.0..10.0);
    let max_scale =
      min_scale + rng.gen_range(0.0..60.0) * rng.gen_range(0.0..1.0);
    let optim = 1;
    let count =
      1 + (rng.gen_range(0.0..200.0) * rng.gen_range(0.5..1.0)) as usize;

    let circles = packing(
      &mut rng,
      1000000,
      count,
      optim,
      0.0,
      (pad, pad, width - pad, height - pad),
      &VCircle::new(width / 2., height / 2., height.max(width)),
      &|_| true,
      min_scale,
      max_scale,
    );

    for c in circles {
      let clr = i % palette.inks.len();
      let ang = amp
        * perlin.get([
          f * (c.x as f64) / (width as f64),
          f * (c.y as f64) / (height as f64),
          perlinseed,
        ]) as f32;
      let size = sizemul * c.r;
      let growth = rng.gen_range(0.03..0.05);
      let samples = 1 + (size * 0.1) as usize;
      let rts = shape(clr, (c.x, c.y), size, ang, samples, growth, precision);
      let poly = vec![
        (-size, -size),
        (-size, size),
        (size, size),
        (size, -size),
        (-size, -size),
      ];
      let poly = poly
        .iter()
        .map(|&p| {
          let (x, y) = p_r(p, ang);
          (x + c.x, y + c.y)
        })
        .collect();
      let rts = clip_routes_with_colors(&rts, &|p| paint.is_painted(p), 1.0, 4);
      paint.paint_polygon(&poly);
      paint.paint_polyline(&poly, 1.0);
      routes.extend(rts);
    }
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

fn p_r(p: (f32, f32), a: f32) -> (f32, f32) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

fn sd_segment(
  (px, py): (f32, f32),
  (ax, ay): (f32, f32),
  (bx, by): (f32, f32),
) -> f32 {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;

  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let h = (dot_pa_ba / dot_ba_ba).max(0.0).min(1.0);

  let h_x = ba_x * h;
  let h_y = ba_y * h;

  ((pa_x - h_x) * (pa_x - h_x) + (pa_y - h_y) * (pa_y - h_y)).sqrt()

  // manhattan distance version
  // (pa_x - h_x).abs().max((pa_y - h_y).abs())
}

pub fn contour<F: FnMut((f32, f32)) -> f32>(
  width: u32,
  height: u32,
  mut f: F,
  thresholds: &Vec<f64>,
) -> Vec<Contour> {
  let c = ContourBuilder::new(width, height, true);
  let values = rasterize_1d(width, height, &mut f)
    .iter()
    .map(|f| *f as f64)
    .collect::<Vec<_>>();
  c.contours(&values, &thresholds).unwrap_or(Vec::new())
}

fn rasterize_1d<F: FnMut((f32, f32)) -> f32>(
  width: u32,
  height: u32,
  mut f: F,
) -> Vec<f32> {
  (0..height)
    .flat_map(|y| {
      (0..width)
        .map(|x| f((x as f32 / width as f32, y as f32 / height as f32)))
        .collect::<Vec<f32>>()
    })
    .collect::<Vec<f32>>()
}

fn euclidian_dist((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

fn features_to_routes(
  features: Vec<Contour>,
  precision: f32,
) -> Vec<Vec<(f32, f32)>> {
  let mut routes = Vec::new();
  for f in features {
    let g = f.geometry();
    for poly in g {
      let points = poly
        .exterior_coords_iter()
        .map(|p| (precision * (p.x as f32), precision * (p.y as f32)))
        .collect::<Vec<(f32, f32)>>();
      let len = points.len();
      if len < 2 {
        continue;
      }
      routes.push(points);

      for interior in poly.interiors() {
        let points = interior
          .exterior_coords_iter()
          .map(|p| (precision * (p.x as f32), precision * (p.y as f32)))
          .collect::<Vec<(f32, f32)>>();
        let len = points.len();
        if len < 2 {
          continue;
        }
        routes.push(points);
      }
    }
  }
  routes
}

fn strictly_in_boundaries(
  p: (f32, f32),
  boundaries: (f32, f32, f32, f32),
) -> bool {
  p.0 > boundaries.0
    && p.0 < boundaries.2
    && p.1 > boundaries.1
    && p.1 < boundaries.3
}

fn crop_route(
  route: &Vec<(f32, f32)>,
  boundaries: (f32, f32, f32, f32),
) -> Option<Vec<(f32, f32)>> {
  if route.len() < 2
    || route
      .iter()
      .all(|&p| !strictly_in_boundaries(p, boundaries))
  {
    return None;
  }
  return Some(route.clone());
}

fn crop_routes(
  routes: &Vec<Vec<(f32, f32)>>,
  boundaries: (f32, f32, f32, f32),
) -> Vec<Vec<(f32, f32)>> {
  return routes
    .iter()
    .filter_map(|route| crop_route(&route, boundaries))
    .collect();
}

fn shape(
  clr: usize,
  center: (f32, f32),
  size: f32,
  ang: f32,
  samples: usize,
  growth: f32,
  precision: f32,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let pad = 4.0 * precision;

  let width = 2. * (size + pad);
  let height = 2. * (size + pad);

  let w = (width as f32 / precision) as u32;
  let h = (height as f32 / precision) as u32;

  let thresholds: Vec<f64> = (0..samples)
    .map(|i| (i as f64 + 0.5) / (samples as f64).floor())
    .collect();

  let border = 0.01 + 1.4 * growth;
  let a = 0.1;
  let b = 0.36;
  let c = 0.35;
  let d = 0.23;
  let xc1 = (1.0 - c - border) / 2.0 + c;
  let yc1 = 0.5 + (0.5 - border) / 2.0;
  let f = |p: (f32, f32)| {
    let q = (p.0 - xc1, p.1 - yc1);
    let q = (q.0.abs(), q.1.abs());

    // make a H shape with sd_segment union
    let d = sd_segment(p, (border, border), (border, (1.0 - border)))
      .min(sd_segment(p, (border, border), (1.0 - border, border)))
      .min(sd_segment(
        p,
        (border, 1.0 - border),
        (1.0 - border, 1.0 - border),
      ))
      .min(sd_segment(
        p,
        (1.0 - border, border),
        (1.0 - border, (1.0 - border)),
      ))
      .min(sd_segment(p, (border, d), (1.0 - border, d)))
      .min(sd_segment(p, (c, 0.5), (1.0 - border, 0.5)))
      .min(sd_segment(p, (c, border), (c, 1.0 - border)))
      .min(sd_segment(p, (border, b), (border + a, b)))
      .min(sd_segment(p, (c - a, b), (c, b)))
      .min(sd_segment(p, (border + a, b), (border + a, 1.0 - border)))
      .min(sd_segment(p, (c - a, b), (c - a, 1.0 - border)))
      .min(sd_segment(p, (xc1 - 0.12, b), (xc1 + 0.12, b)))
      .min(sd_segment(q, (0.06, 0.08), (0.16, 0.08)));

    d / growth
  };

  let res = contour(w, h, f, &thresholds);
  let routes = features_to_routes(res, precision);
  let routes = crop_routes(
    &routes,
    (pad / 2.0, pad / 2.0, width - pad / 2.0, height - pad / 2.0),
  );

  let routes = routes.iter().map(|r| (clr, r.clone())).collect();

  let routes = translate_routes(&routes, (-pad - size, -pad - size));

  /*
  routes.push((
    1,
    vec![
      (-size, -size),
      (-size, size),
      (size, size),
      (size, -size),
      (-size, -size),
    ],
  ));
  */

  let acos = ang.cos();
  let asin = ang.sin();
  let routes = routes
    .iter()
    .map(|(i, route)| {
      (
        *i,
        route
          .iter()
          .map(|&(x, y)| {
            let (x, y) = (x * acos + y * asin, y * acos - x * asin);
            (x + center.0, y + center.1)
          })
          .collect(),
      )
    })
    .collect();

  routes
}

fn translate_routes(
  routes: &Vec<(usize, Vec<(f32, f32)>)>,
  (tx, ty): (f32, f32),
) -> Vec<(usize, Vec<(f32, f32)>)> {
  routes
    .iter()
    .map(|(i, route)| {
      (*i, route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    })
    .collect()
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f32,
  y: f32,
  r: f32,
}
impl VCircle {
  fn new(x: f32, y: f32, r: f32) -> Self {
    VCircle { x, y, r }
  }
  fn dist(self: &Self, c: &VCircle) -> f32 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
  fn contains(self: &Self, c: &VCircle) -> bool {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - self.r + c.r < 0.0
  }
  fn inside_bounds(
    self: &Self,
    (x1, y1, x2, y2): (f32, f32, f32, f32),
  ) -> bool {
    x1 <= self.x - self.r
      && self.x + self.r <= x2
      && y1 <= self.y - self.r
      && self.y + self.r <= y2
  }
}

fn scaling_search<F: FnMut(f32) -> bool>(
  mut f: F,
  min_scale: f32,
  max_scale: f32,
) -> Option<f32> {
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
  container_boundaries: (f32, f32, f32, f32),
  container_circle: &VCircle,
  is_valid: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f32,
  y: f32,
  min_scale: f32,
  max_scale: f32,
) -> Option<f32> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    c.inside_bounds(container_boundaries)
      && container_circle.contains(&c)
      && is_valid(&c)
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  rng: &mut StdRng,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f32,
  container_boundaries: (f32, f32, f32, f32),
  container: &VCircle,
  is_valid: &dyn Fn(&VCircle) -> bool,
  min_scale: f32,
  max_scale: f32,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let x1 = container.x - container.r;
  let y1 = container.y - container.r;
  let x2 = container.x + container.r;
  let y2 = container.y + container.r;
  let max_scale = max_scale.min(container.r);
  for _i in 0..iterations {
    let x: f32 = rng.gen_range(x1..x2);
    let y: f32 = rng.gen_range(y1..y2);
    if let Some(size) = search_circle_radius(
      container_boundaries,
      &container,
      is_valid,
      &circles,
      x,
      y,
      min_scale,
      max_scale,
    ) {
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
