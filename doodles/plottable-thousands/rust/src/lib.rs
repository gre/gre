/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Thousands
 */
mod utils;
use byteorder::*;
use rand::prelude::*;
use rand::Rng;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub struct Opts {
  pub seed: f64,
  pub hash: String,
  pub primary_name: String,
  pub secondary_name: String,
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
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
  fn contains(self: &Self, c: &VCircle) -> bool {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - self.r + c.r < 0.0
  }
  fn inside_bounds(self: &Self, (x1, y1, x2, y2): (f64, f64, f64, f64)) -> bool {
    x1 <= self.x - self.r && self.x + self.r <= x2 && y1 <= self.y - self.r && self.y + self.r <= y2
  }
}

fn scaling_search<F: FnMut(f64) -> bool>(mut f: F, min_scale: f64, max_scale: f64) -> Option<f64> {
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
  container_boundaries: (f64, f64, f64, f64),
  container_circle: &VCircle,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    c.inside_bounds(container_boundaries)
      && container_circle.contains(&c)
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  container_boundaries: (f64, f64, f64, f64),
  container: &VCircle,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = container.x - container.r;
  let y1 = container.y - container.r;
  let x2 = container.x + container.r;
  let y2 = container.y + container.r;
  let max_scale = max_scale.min(container.r);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    if let Some(size) = search_circle_radius(
      container_boundaries,
      &container,
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

fn rec_inception(
  mut acc: &mut Vec<VCircle>,
  inception: usize,
  index: usize,
  circle: &VCircle,
  cache: &RecPackingCirclesCache,
  pad_min: f64,
  pad_max: f64,
  pad_mod: usize,
  angle_incr: f64,
  threshold_radius: f64,
) {
  let hard_threshold_radius = 0.15;
  let max_inception = 48;

  if circle.r < hard_threshold_radius {
    return;
  }

  let pad = if pad_mod < 2 {
    pad_min
  } else {
    mix(
      pad_min,
      pad_max,
      (inception % pad_mod) as f64 / ((pad_mod - 1) as f64),
    )
  };

  acc.push(circle.clone());

  if circle.r > threshold_radius && inception < max_inception {
    let circles = cache.get_circles(index + inception);
    for (i, sub_no_pad) in circles.iter().enumerate() {
      let sub = VCircle::new(sub_no_pad.x, sub_no_pad.y, sub_no_pad.r - pad);
      if sub.r < hard_threshold_radius {
        continue;
      }
      let ratio = sub.r / circle.r;
      if ratio >= 0.999 {
        continue;
      }
      let next_cache = cache.project_in_circle(&sub, angle_incr);
      rec_inception(
        &mut acc,
        inception + 1,
        i,
        &sub,
        &next_cache,
        pad_min,
        pad_max,
        pad_mod,
        angle_incr,
        threshold_radius,
      );
    }
  }
}

struct RecPackingCirclesCache {
  container: VCircle,
  circles_collections: Vec<Vec<VCircle>>,
}
impl RecPackingCirclesCache {
  fn new(seed: f64, variety: usize, optimize_size: usize, min_scale: f64, max_scale: f64) -> Self {
    let container = VCircle::new(100.0, 100.0, 100.0);
    let circles_collections = (0..variety)
      .map(|i| {
        packing(
          seed * 7.7 + (i as f64 / 0.3),
          1000000,
          10000,
          optimize_size,
          0.0,
          (0.0, 0.0, 200.0, 200.0),
          &container,
          min_scale,
          max_scale,
        )
      })
      .collect();
    RecPackingCirclesCache {
      container,
      circles_collections,
    }
  }
  fn get_circles(self: &Self, index: usize) -> &Vec<VCircle> {
    &(self.circles_collections[index % self.circles_collections.len()])
  }
  fn project_in_circle(self: &Self, sub: &VCircle, angle_incr: f64) -> Self {
    let hard_threshold_radius = 0.15;
    let ratio = sub.r / self.container.r;
    let circles_collections = self
      .circles_collections
      .iter()
      .map(|circles| {
        let next = circles
          .iter()
          .filter_map(|s| {
            let r = s.r * ratio;
            if r < hard_threshold_radius {
              return None;
            }
            let (x, y) = p_r((s.x - self.container.x, s.y - self.container.y), angle_incr);
            Some(VCircle::new(sub.x + x * ratio, sub.y + y * ratio, r))
          })
          .collect();
        next
      })
      .collect();

    RecPackingCirclesCache {
      container: sub.clone(),
      circles_collections,
    }
  }
}

fn recursive_packing(
  pad_min: f64,
  pad_max: f64,
  pad_mod: usize,
  angle_incr: f64,
  container: &VCircle,
  min_scale: f64,
  cache: &RecPackingCirclesCache,
) -> Vec<VCircle> {
  let mut res = Vec::new();
  rec_inception(
    &mut res, 0, 0, &container, &cache, pad_min, pad_max, pad_mod, angle_incr, min_scale,
  );
  res
}

#[derive(Clone, Copy, Debug)]
enum Particle {
  Circle(usize),
  Plus,
  Stroke,
  Stroke2,
}

#[derive(Clone, Copy, Debug)]
enum Shape {
  GoldSpiral(Particle, f64),
  EmptyCircle,
  RecursiveCircles(f64, f64, usize, f64),
}

fn inner(
  center: (f64, f64),
  radius: f64,
  shape: Shape,
  count: usize,
  shape_size: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  match shape {
    Shape::EmptyCircle | Shape::RecursiveCircles(_, _, _, _) => {
      let count = 6 + (radius * 4.0) as usize;
      routes.push(circle_route(center, radius, count, 0.0));
    }
    Shape::GoldSpiral(particle, count_multiply) => {
      let golden_ratio = (1. + (5f64).sqrt()) / 2.;
      let center_offset = 0.0; // 0.5 * shape_size;
      let size_base = 0.7;
      let ct = (count as f64 * count_multiply) as usize;
      for i in 0..ct {
        let k = i as f64 / (ct as f64);
        let a = 2. * PI * (i as f64) / (golden_ratio * golden_ratio);
        let r = center_offset + (radius - center_offset) * k.powf(0.56);
        let ad = shape_size * mix(size_base, 1.0, k);
        let x = center.0 + r * a.cos();
        let y = center.1 + r * a.sin();
        match particle {
          Particle::Stroke => {
            let x2 = x + (a + ad).cos();
            let y2 = y + (a + ad).sin();
            let x3 = x + (a - ad).cos();
            let y3 = y + (a - ad).sin();
            routes.push(vec![(x3, y3), (x2, y2)]);
          }
          Particle::Stroke2 => {
            let x2 = x + (a + ad).cos();
            let y2 = y + (a + ad).sin();
            let x3 = x + (a - ad).cos();
            let y3 = y + (a - ad).sin();
            routes.push(vec![(x3, y2), (x2, y3)]);
          }
          Particle::Circle(s) => {
            routes.push(circle_route((x, y), ad, s, a));
          }
          Particle::Plus => {
            routes.push(vec![(x - ad, y), (x + ad, y)]);
            routes.push(vec![(x, y - ad), (x, y + ad)]);
          }
        }
      }
    }
  }

  routes
}

fn gen_shape<R: Rng>(rng: &mut R) -> Shape {
  if rng.gen_bool(0.5) {
    let min = 0.16 + rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
    let max = min + rng.gen_range(-0.2f64, 2.5).max(0.0);
    let angle_incr = if rng.gen_bool(0.8) {
      rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 3.0) * rng.gen_range(0.0, 1.0)
    } else {
      0.0
    };
    Shape::RecursiveCircles(min, max, rng.gen_range(1, 5), angle_incr)
  } else if rng.gen_bool(0.5) {
    Shape::GoldSpiral(
      Particle::Circle(2 + (2f64).powf(rng.gen_range(0usize, 4) as f64) as usize),
      2.0,
    )
  } else if rng.gen_bool(0.4) {
    Shape::GoldSpiral(Particle::Stroke, 4.0)
  } else if rng.gen_bool(0.6) {
    Shape::GoldSpiral(Particle::Stroke2, 4.0)
  } else if rng.gen_bool(0.6) {
    Shape::GoldSpiral(Particle::Plus, 3.0)
  } else {
    Shape::EmptyCircle
  }
}

pub fn art(opts: &Opts) -> Document {
  let width = 297.0;
  let height = 210.0;
  let pad = 10.0;
  let bounds = (pad, pad, width - pad, height - pad);
  let mut seed = opts.seed / 7.;
  let mut rng = rng_from_seed(seed);
  let packing_pad_1 = 5.0;
  let min_scale_1 = 5.0;
  let max_scale_1 = min_scale_1 + rng.gen_range(40.0, 120.0);
  let packing_pad_2 = 0.5;
  let min_scale_2 = 2.0;
  let max_scale_2 = min_scale_2 + rng.gen_range(20.0, 120.0);

  let shape = gen_shape(&mut rng);

  let color_split =
    1 + (rng.gen_range(0f64, 6.0) * rng.gen_range(0f64, 1.0) * rng.gen_range(0.0, 1.0)) as usize;

  let primary = packing(
    seed,
    4000000,
    1000,
    rng.gen_range(1, 30),
    packing_pad_1,
    bounds,
    &VCircle::new(width / 2., height / 2., width + height),
    min_scale_1,
    max_scale_1,
  );

  let mut cache: Vec<RecPackingCirclesCache> = vec![];
  let retries = 1 + (rng.gen_range(0., 80.)) as usize;

  let mut circles = Vec::new();
  for (i, &c) in primary.iter().enumerate() {
    let subshape = if rng.gen_bool(0.999 - 0.4 * (1.0 / (i as f64 / 2.0 + 1.0))) {
      shape
    } else {
      gen_shape(&mut rng)
    };

    match subshape {
      Shape::RecursiveCircles(pad_min, pad_max, pad_mod, angle_incr) => {
        if c.r > min_scale_2 * 2.0 {
          if cache.len() == 0 {
            let variety = 1;
            let v =
              RecPackingCirclesCache::new(opts.seed, variety, retries, min_scale_2, max_scale_2);
            cache = vec![v];
          }
          for c in recursive_packing(
            pad_min,
            pad_max,
            pad_mod,
            angle_incr,
            &c,
            min_scale_2,
            &(cache[0].project_in_circle(&c, 0.0)),
          ) {
            circles.push((c, i, subshape));
          }
        } else {
          circles.push((c, i, subshape));
        }
      }
      Shape::EmptyCircle | Shape::GoldSpiral(_, _) => {
        if c.r > 2. * min_scale_2 && i < 5 && rng.gen_bool(0.9) {
          let retries = rng.gen_range(1, 5);
          for (j, &c2) in packing(
            seed,
            1000000,
            10000,
            retries,
            packing_pad_2,
            bounds,
            &c,
            min_scale_2,
            max_scale_2,
          )
          .iter()
          .enumerate()
          {
            if i == 0 && j == 0 && retries == 4 {
              // skip to leave empty space
              continue;
            }
            circles.push((c2, i, subshape));
          }
        } else {
          circles.push((c, i, subshape));
        }
      }
    }
    seed = seed * 1.1 + 0.3;
  }

  let density_factor: f64 = 0.8 + rng.gen_range(0.0, 2.5) * rng.gen_range(0.0, 1.0);
  let count_base_mul = rng.gen_range(1.0, 2.0) / density_factor;
  let shape_size = rng.gen_range(0.8, 1.2) * density_factor.powf(2.0);

  let mut particles_count = 0;
  let stats_shape_particle_labels = vec![
    "Tri",
    "Square",
    "Hex",
    "Circ",
    "Plus",
    "Line",
    "Line2",
    "EmptyCircle",
    "Recursive",
  ];
  let mut stats_shape_particle_count = vec![0; stats_shape_particle_labels.len()];
  let mut uses_density = false;

  let mut layer_primary: Vec<Vec<Vec<(f64, f64)>>> = Vec::new();
  let mut layer_secondary: Vec<Vec<Vec<(f64, f64)>>> = Vec::new();
  for (_i, &(c, g, s)) in circles.iter().enumerate() {
    let count = (count_base_mul * c.r.powf(1.6)).max(0.0) as usize;
    if c.r > 4.0 {
      match s {
        Shape::GoldSpiral(particle, _) => {
          uses_density = true;
          particles_count += count;
          match particle {
            Particle::Circle(l) => {
              if l <= 3 {
                stats_shape_particle_count[0] += 1;
              } else if l <= 4 {
                stats_shape_particle_count[1] += 1;
              } else if l <= 6 {
                stats_shape_particle_count[2] += 1;
              } else {
                stats_shape_particle_count[3] += 1;
              }
            }
            Particle::Plus => {
              stats_shape_particle_count[4] += 1;
            }
            Particle::Stroke => {
              stats_shape_particle_count[5] += 1;
            }
            Particle::Stroke2 => {
              stats_shape_particle_count[6] += 1;
            }
          }
        }
        Shape::EmptyCircle => {
          stats_shape_particle_count[7] += 1;
        }
        Shape::RecursiveCircles(_, _, _, _) => {
          stats_shape_particle_count[8] += 1;
        }
      }
    }
    let routes = inner((c.x, c.y), c.r, s, count, shape_size);
    if g < color_split {
      layer_primary.push(routes);
    } else {
      layer_secondary.push(routes);
    }
  }

  let mut stats_shape_particle: Vec<(&str, isize)> = (0..stats_shape_particle_labels.len())
    .filter_map(|i| {
      let count = stats_shape_particle_count[i];
      if count == 0 {
        None
      } else {
        Some((stats_shape_particle_labels[i], count))
      }
    })
    .collect();
  stats_shape_particle.sort_by_key(|o| -o.1);

  let mut inks = Vec::new();
  let layers: Vec<Group> = vec![
    ("#0FF", opts.primary_name.clone(), layer_primary.concat()),
    (
      "#F0F",
      opts.secondary_name.clone(),
      layer_secondary.concat(),
    ),
  ]
  .iter()
  .filter(|(_color, _label, routes)| routes.len() > 0)
  .map(|(color, label, routes)| {
    inks.push(label.clone());
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

  inks.sort();
  if inks.len() == 2 && inks[0].eq(&inks[1]) {
    inks.remove(1);
  }

  let mut map = Map::new();
  map.insert(String::from("Inks Count"), json!(inks.len()));
  map.insert(String::from("Inks"), json!(inks.join(" + ")));
  let quantity = particles_count + circles.len();
  map.insert(
    String::from("Elements"),
    json!(String::from(if quantity < 600 {
      "Very Low"
    } else if quantity < 1200 {
      "Low"
    } else if quantity < 5000 {
      "Normal"
    } else if quantity < 10000 {
      "High"
    } else {
      "Very High"
    })),
  );
  let shapes: Vec<String> = stats_shape_particle
    .iter()
    .map(|c| String::from(c.0))
    .collect();
  if shapes.len() > 0 {
    map.insert(String::from("Primary Shape"), json!(shapes[0]));
  }
  if shapes.len() > 1 {
    map.insert(String::from("Second Shape"), json!(shapes[1]));
  }
  if shapes.len() > 2 {
    let copy: Vec<String> = shapes.iter().skip(2).map(|s| s.clone()).collect();
    map.insert(String::from("Other Shapes"), json!(copy.join(", ")));
  }

  if uses_density {
    map.insert(
      String::from("Spiral Density"),
      json!(String::from(if density_factor < 1.0 {
        "Low"
      } else if density_factor < 1.4 {
        "Normal"
      } else if density_factor < 2.0 {
        "High"
      } else {
        "Very High"
      })),
    );
  }

  let traits = Value::Object(map);

  let mut document = svg::Document::new()
    .set("data-hash", opts.hash.to_string())
    .set("data-traits", traits.to_string())
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

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
  let opts = val.into_serde().unwrap();
  let doc = art(&opts);
  let str = doc.to_string();
  return str;
}

fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
  if route.len() == 0 {
    return data;
  }
  let first_p = route[0];
  if f64::is_nan(first_p.0) {
    let mut copy = route.clone();
    copy.remove(0);
    return render_route_curve(data, copy);
  }
  let mut d = data.move_to((significant_str(first_p.0), significant_str(first_p.1)));
  for p in route {
    d = d.line_to((significant_str(p.0), significant_str(p.1)));
  }
  return d;
}

pub fn render_route_curve(data: Data, route: Vec<(f64, f64)>) -> Data {
  if route.len() == 0 {
    return data;
  }
  let mut first = true;
  let mut d = data;
  let mut last = route[0];
  for p in route {
    if first {
      first = false;
      d = d.move_to((significant_str(p.0), significant_str(p.1)));
    } else {
      d = d.quadratic_curve_to((
        significant_str(last.0),
        significant_str(last.1),
        significant_str((p.0 + last.0) / 2.),
        significant_str((p.1 + last.1) / 2.),
      ));
    }
    last = p;
  }
  return d;
}

#[inline]
fn significant_str(f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

fn rng_from_seed(s: f64) -> impl Rng {
  let mut bs = [0; 16];
  bs.as_mut().write_f64::<BigEndian>(s).unwrap();
  let mut rng = SmallRng::from_seed(bs);
  // run it a while to have better randomness
  for _i in 0..50 {
    rng.gen::<f64>();
  }
  return rng;
}

#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

fn circle_route(center: (f64, f64), r: f64, count: usize, dangle: f64) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = dangle + 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}
