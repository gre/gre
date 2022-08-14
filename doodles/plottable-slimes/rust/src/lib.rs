/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Plottable Slimes
 */
mod utils;
use instant::Instant;
use noise::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::f64::consts::PI;
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
  pub primary_name: String,
  pub secondary_name: String,
  pub debug: bool,
}

pub fn art(opts: &Opts) -> Document {
  let mut perf = PerfRecords::start(opts.debug);
  // constants
  let width = 297.0;
  let height = 210.0;
  let pad = 10.0;
  let precision = 2.0;
  let max_slimes = 300;
  let max_search = 100000;

  // Now, we'll determine most of the rng properties
  let mut rng = rng_from_fxhash(opts.hash.clone());
  let r_increment = rng.gen_range(0.45, 0.55);

  let lowpoly = rng.gen_bool(0.05);
  let is_smooth = rng.gen_bool(0.3);

  let snow_effect = if lowpoly {
    0.0
  } else {
    rng.gen_range(-10.0f64, 2.0).max(0.0).min(1.0)
  };
  let high_map_size = (8000.0 - 4000.0 * snow_effect) as usize;
  let amp1pow = 0.7 + rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let amp_factor = rng.gen_range(0.0, 1.0);
  let freq1 = rng.gen_range(0.03, 0.06) * (1. - amp_factor);
  let amp1 = 0.1 + 0.4 * amp_factor;
  let freq2 = rng.gen_range(0.02, 0.06);
  let amp2 = rng.gen_range(2.0, 4.0);
  let freq3 = rng.gen_range(0.4, 0.6);
  let amp3 = if is_smooth { 0.0 } else { 0.08 };
  let min_r = rng.gen_range(1.0, 2.0);
  let max_r = rng.gen_range(10.0, 100.0);
  let rotations = if lowpoly {
    50.
  } else {
    (400f64 + 2. * max_r).floor()
  };
  let disp = (rng.gen_range(0.0, 10.0) * rng.gen_range(-0.5, 1f64)).max(0.);
  let safe_h = -rng.gen_range(-0.5f64, 1.0).max(0.0) * rng.gen_range(0.0, 6.0);
  let dispfreq = rng.gen_range(0.05, 0.07);
  let padding = if lowpoly { 4.0 } else { 1.0 } + rng.gen_range(0.0, 10.0);

  // this is where we aggregate our paths
  let mut primary: Vec<Vec<Vec<(f64, f64)>>> = Vec::new();
  let mut secondary: Vec<Vec<Vec<(f64, f64)>>> = Vec::new();

  // this global passage map will be used for collision to know where we can draw or not
  let mut passage = Passage::new(precision, width, height);
  passage.prepare(|(x, y)| {
    if x < pad || y < pad || x > width - pad || y > height - pad {
      1
    } else {
      0
    }
  });

  let mut center = (width / 2.0, height / 2.0);
  let mut slimes = 0;
  for i in 0..max_slimes {
    perf.span("Slime");
    let r = slime(
      &mut rng,
      &passage,
      SlimeOpts {
        center,
        amp1pow,
        freq1,
        amp1,
        freq2,
        amp2,
        freq3,
        amp3,
        max_r,
        min_r,
        disp,
        dispfreq,
        rotations,
        safe_h,
        snow_effect,
        high_map_size,
        r_increment,
      },
    );
    perf.span_end("Slime");

    if let Some(r) = r {
      let mut local_passage = r.passage;

      let p = padding * (1.0 + 1.0 / (i as f64 + 1.0));
      perf.span("grow_passage");
      local_passage.grow_passage(p);
      passage = passage.add(&local_passage);
      perf.span_end("grow_passage");

      if i < 1 && r.radius > 40.0 {
        primary.push(r.routes);
      } else {
        secondary.push(r.routes);
      }
      slimes += 1;
    }

    // find interesting random center
    perf.span("search center");
    let r = passage.search_space(&mut rng, min_r, max_r, pad, max_search);
    perf.span_end("search center");

    if let Some(p) = r {
      center = p;
    } else {
      break;
    }
  }

  perf.span("svg");
  let (layers, inks) = make_layers(vec![
    ("#0FF", opts.primary_name.clone(), primary.concat()),
    ("#F0F", opts.secondary_name.clone(), secondary.concat()),
  ]);
  perf.span_end("svg");

  let mut traits = Map::new();
  traits.insert(String::from("Inks Count"), json!(inks.len()));
  traits.insert(String::from("Inks"), json!(inks.join(" + ")));
  traits.insert(
    String::from("Slimes"),
    json!(if slimes == 0 {
      "Solo"
    } else if slimes < 9 {
      "Few"
    } else if slimes < 50 {
      "Some"
    } else if slimes < 200 {
      "Many"
    } else {
      "A lot"
    }),
  );

  if slimes > 1 {
    traits.insert(
      String::from("Padding"),
      json!(if padding < 3.0 {
        "Tight"
      } else if padding < 7.0 {
        "Normal"
      } else {
        "Distant"
      }),
    );
  }

  traits.insert(
    String::from("Size"),
    json!(if max_r < 28.0 {
      "Small"
    } else if max_r < 80.0 {
      "Medium"
    } else {
      "Big"
    }),
  );
  traits.insert(
    String::from("Curving"),
    json!(if safe_h > -0.5 {
      "Strict"
    } else if safe_h > -2.0 {
      "Contour"
    } else {
      "Inside"
    }),
  );

  let intensity = amp1 * amp2 + disp / 10.0;
  traits.insert(
    String::from("Intensity"),
    json!(if intensity < 0.5 {
      "Light"
    } else if intensity < 1.1 {
      "Medium"
    } else if intensity < 1.9 {
      "Intense"
    } else {
      "Extreme"
    }),
  );
  traits.insert(
    String::from("Shape"),
    json!(vec![
      if is_smooth { vec!["Smooth"] } else { vec![] },
      if lowpoly { vec!["Low-Poly"] } else { vec![] },
      if snow_effect < 0.001 {
        vec!["Slime"]
      } else if snow_effect > 0.5 {
        vec!["Snow"]
      } else {
        vec!["Wind"]
      },
    ]
    .concat()
    .join(" ")),
  );

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
  if opts.debug {
    document = document.set("data-perf", json!(perf.end()).to_string());
  }
  for l in layers {
    document = document.add(l);
  }
  document
}

// The slime primitive =>

struct SlimeOpts {
  center: (f64, f64),
  amp1pow: f64,
  freq1: f64,
  amp1: f64,
  freq2: f64,
  amp2: f64,
  freq3: f64,
  amp3: f64,
  min_r: f64,
  max_r: f64,
  disp: f64,
  dispfreq: f64,
  rotations: f64,
  safe_h: f64,
  snow_effect: f64,
  high_map_size: usize,
  r_increment: f64,
}

struct SlimeOut {
  routes: Vec<Vec<(f64, f64)>>,
  passage: Passage,
  radius: f64,
}

fn slime<R: Rng>(
  mut rng: R,
  globp: &Passage,
  opts: SlimeOpts,
) -> Option<SlimeOut> {
  let (cx, cy) = opts.center;
  let amp1pow = opts.amp1pow;
  let freq1 = opts.freq1;
  let amp1 = opts.amp1;
  let freq2 = opts.freq2;
  let amp2 = opts.amp2;
  let freq3 = opts.freq3;
  let amp3 = opts.amp3;
  let max_r = opts.max_r;
  let disp = opts.disp;
  let dispfreq = opts.dispfreq;
  let rotations = opts.rotations;
  let r_increment = opts.r_increment;
  let seed = rng.gen_range(0.0, 1000.0);

  // this passage is used for inter slime collisions
  let mut passage = Passage::new(globp.precision, globp.width, globp.height);
  // this passage is used to not accumulate too much lines
  let mut collision_passage = Passage::new(0.5, globp.width, globp.height);

  let perlin = Perlin::new();
  let mut routes = Vec::new();
  let mut highest_by_angle = vec![0f64; opts.high_map_size];

  let safe_h = opts.safe_h;
  let mut base_r = 0.2;
  let mut end = false;
  loop {
    if base_r > max_r || end {
      break;
    }
    let mut route = Vec::new();
    let angle_delta =
      rng.gen_range(0, rotations as usize) as f64 / rotations * 2.0 * PI;
    let mut a = angle_delta;
    let angle_precision =
      2. * PI / mix(rotations, 1.0 + 30.0 * base_r, opts.snow_effect).round();
    loop {
      if a - angle_delta > 2. * PI + 0.0001 {
        break;
      }
      let hba_index = (highest_by_angle.len() as f64 * ((a) / 2. * PI))
        as usize
        % highest_by_angle.len();

      let mut r = base_r;
      let x = cx + r * a.cos();
      let y = cy + r * a.sin();
      r += amp1
        * base_r
        * (base_r / max_r).powf(amp1pow)
        * perlin.get([
          -seed
            + amp2
              * perlin.get([
                freq2 * x,
                seed * 7.7 - 4.,
                freq2 * y
                  + amp3 * perlin.get([freq3 * x, seed * 2.7 + 11., freq3 * y]),
              ]),
          freq1 * x,
          freq1 * y,
        ]);

      let should_draw = r > highest_by_angle[hba_index] + safe_h;

      if should_draw {
        let mut x = cx + r * a.cos();
        let mut y = cy + r * a.sin();

        x += disp * perlin.get([77. + seed, dispfreq * x, dispfreq * y]);
        y += disp * perlin.get([99. + seed, dispfreq * x, dispfreq * y]);

        let p = (x, y);
        if globp.get(p) > 0 {
          end = true;
          break;
        }
        passage.count(p);

        highest_by_angle[hba_index] = highest_by_angle[hba_index].max(r);
        route.push(p);
      } else {
        add_route_simplified(&mut routes, &route, &mut collision_passage);
        route = Vec::new();
      }
      a += angle_precision;
    }

    if end {
      break;
    }

    add_route_simplified(&mut routes, &route, &mut collision_passage);

    base_r += r_increment;
  }

  if base_r < opts.min_r {
    return None;
  }

  Some(SlimeOut {
    passage,
    routes,
    radius: base_r,
  })
}

// Generic helper to simplify and clean up a path

fn add_route_simplified(
  routes: &mut Vec<Vec<(f64, f64)>>,
  route: &Vec<(f64, f64)>,
  passage: &mut Passage,
) {
  if route.len() < 2 {
    return;
  }

  // simplify the path
  let mut simplified = Vec::new();
  let mut last = route[0];
  simplified.push(last);
  /*
  let mut dist = 0.0;
  let l = route.len();
  for i in 1..l {
    dist += euclidian_dist(route[i - 1], route[i]);
    if dist > 0.5 {
      simplified.push(route[i]);
      dist = 0.0;
    }
  }
  if dist > 0.0 {
    simplified.push(route[l - 1]);
  }
  */

  let l = route.len();
  let threshold = 0.12;
  for i in 1..l {
    let p = route[i];
    let dx = last.0 - p.0;
    let dy = last.1 - p.1;
    let d = dx * dx + dy * dy;
    let t = if i == l - 1 { 0.0 } else { threshold };
    if d > t {
      simplified.push(route[i]);
      last = p;
    }
  }

  if simplified.len() < 2 {
    return;
  }
  // split the path using passage if there are too much density
  let mut route = Vec::new();
  for p in simplified {
    if passage.count(p) < 10 {
      route.push(p);
    } else {
      let l = route.len();
      if l > 1 {
        routes.push(route);
        route = Vec::new();
      } else if l > 0 {
        route = Vec::new();
      }
    }
  }
  let l = route.len();
  if l > 1 {
    routes.push(route);
  }
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
fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}

#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }

  pub fn count_once(self: &mut Self, p: (f64, f64)) {
    let i = self.index(p);
    let v = self.counters[i];
    if v == 0 {
      self.counters[i] = 1;
    }
  }

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }

  pub fn add(self: &Self, other: &Self) -> Self {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters = self
      .counters
      .iter()
      .enumerate()
      .map(|(i, v)| v + other.counters[i])
      .collect();
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  pub fn prepare<F: Fn((f64, f64)) -> usize>(self: &mut Self, f: F) {
    let mut x = 0.0;
    loop {
      if x >= self.width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= self.height {
          break;
        }
        let index = self.index((x, y));
        self.counters[index] = f((x, y));
        y += self.precision;
      }
      x += self.precision;
    }
  }

  pub fn grow_passage(self: &mut Self, radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters: Vec<usize> = self.counters.iter().cloned().collect();
    let mut mask = Vec::new();
    // TODO, in future for even better perf, I will rewrite this
    // working directly with index integers instead of having to use index() / count_once()
    let mut x = -radius;
    loop {
      if x >= radius {
        break;
      }
      let mut y = -radius;
      loop {
        if y >= radius {
          break;
        }
        if x * x + y * y < radius * radius {
          mask.push((x, y));
        }
        y += precision;
      }
      x += precision;
    }

    let mut x = 0.0;
    loop {
      if x >= width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= height {
          break;
        }
        let index = self.index((x, y));
        if counters[index] > 0 {
          for &(dx, dy) in mask.iter() {
            self.count_once((x + dx, y + dy));
          }
        }
        y += precision;
      }
      x += precision;
    }
  }

  pub fn search_space<R: Rng>(
    self: &Self,
    rng: &mut R,
    min_r: f64,
    max_r: f64,
    pad: f64,
    max_search: usize,
  ) -> Option<(f64, f64)> {
    for j in 0..max_search {
      let optim_r = mix(min_r, max_r, 1.0 / (1.0 + j as f64 * 0.01));
      let minx = pad + optim_r;
      let miny = pad + optim_r;
      let maxx = self.width - pad - optim_r;
      let maxy = self.height - pad - optim_r;

      if minx >= maxx || miny >= maxy {
        break;
      }

      let p = (rng.gen_range(minx, maxx), rng.gen_range(miny, maxy));
      if self.get(p) == 0
        && self.get((p.0 - optim_r, p.1)) == 0
        && self.get((p.0 + optim_r, p.1)) == 0
        && self.get((p.0, p.1 - optim_r)) == 0
        && self.get((p.0, p.1 + optim_r)) == 0
      {
        return Some(p);
      }
    }

    None
  }
}

fn make_layers(
  data: Vec<(&str, String, Vec<Vec<(f64, f64)>>)>,
) -> (Vec<Group>, Vec<String>) {
  let mut inks = Vec::new();
  let layers: Vec<Group> = data
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
  // remove inks that have no paths at all
  inks.sort();
  if inks.len() == 2 && inks[0].eq(&inks[1]) {
    inks.remove(1);
  }
  (layers, inks)
}

// PERFORMANCE HELPERS
struct Span {
  label: String,
  start: Instant,
  stop: Instant,
}
struct PerfRecords {
  debug: bool,
  started: HashMap<String, Instant>,
  spans: Vec<Span>,
}
struct PerfResult {
  per_label: HashMap<String, f64>,
}
impl PerfRecords {
  /**
   * let mut perf = PerfRecords::start();
   */
  pub fn start(debug: bool) -> Self {
    let mut r = PerfRecords {
      debug,
      started: HashMap::new(),
      spans: Vec::new(),
    };
    r.span("total");
    r
  }
  /**
   * perf.span("calc_circles");
   */
  pub fn span(self: &mut Self, s: &str) {
    if self.debug {
      self.started.insert(String::from(s), Instant::now());
    }
  }
  /**
   * perf.span_end("calc_circles");
   */
  pub fn span_end(self: &mut Self, s: &str) {
    if self.debug {
      let label = String::from(s);
      if let Some(&start) = self.started.get(&label) {
        self.spans.push(Span {
          label,
          start,
          stop: Instant::now(),
        });
      }
    }
  }
  /**
   * let perf_res = perf.end();
   */
  pub fn end(self: &mut Self) -> PerfResult {
    let mut per_label = HashMap::new();
    if self.debug {
      self.span_end("total");
      self.spans.iter().for_each(|span| {
        let maybe_time = per_label.get(&span.label).unwrap_or(&0.);
        per_label.insert(
          span.label.clone(),
          maybe_time + span.stop.duration_since(span.start).as_secs_f64(),
        );
      });
    }
    PerfResult { per_label }
  }
}

impl Serialize for PerfResult {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("Perf", 1)?;
    state.serialize_field("per_label", &self.per_label)?;
    state.end()
  }
}
