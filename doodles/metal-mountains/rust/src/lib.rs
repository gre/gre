/**
 * LICENSE ...
 * Author: ...
 */
mod utils;
use instant::Instant;
use noise::{NoiseFn, Perlin};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::f64::consts::PI;
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

pub fn art(opts: &Opts) -> Document {
  let mut rng = rng_from_fxhash(opts.hash.clone());
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let frameborder = 0.0;
  let skyborder = 1.0;

  let seed = rng.gen_range(0.0, 100000.0);
  let bound = (pad, pad, width - pad, height - pad);

  let mut perf = PerfRecords::start(opts.debug);

  perf.span("mountains");
  let mut routes = Vec::new();
  let perlin = Perlin::new();
  let min_route = 2;
  let mountainpadding = 0.0;
  let mut height_map: Vec<f64> = Vec::new();
  let mut height_map_stop: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);
  let peakfactormajor = rng.gen_range(-0.0001, 0.001);
  let precision = 0.1;
  let count = rng.gen_range(4, 12);
  let mut feature_mountain_density_plain = 0;
  let mut feature_mountain_density_light = 0;
  let mut feature_mountain_density_normal = 0;
  for j in 0..count {
    let h = rng.gen_range(3.0, 5.0);
    let stopy = rng.gen_range(0.2, 0.7) * height;
    let peakfactor =
      peakfactormajor + rng.gen_range(-0.001, 0.002) * rng.gen_range(0.0, 1.0);
    let ampfactor = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
    let ynoisefactor = rng.gen_range(0.02, 0.2);
    let yincr = 0.2
      + (rng.gen_range(-2f64, 8.0) * rng.gen_range(0.0, 1.0))
        .max(0.0)
        .min(5.0);
    let amp1 = rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(0.0, 12.0) * rng.gen_range(0.0, 1.0);
    let amp3 = rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
    let offsetstrategy = rng.gen_range(0, 5);
    let center = rng.gen_range(0.2, 0.8) * width;
    let rounding = if ampfactor < 0.05 || yincr > 1.0 {
      0.0
    } else {
      rng.gen_range(-5.0f64, 2.5).max(0.0)
    };
    let min_rounding = 0.8;

    let stopy =
      mix(height, stopy, (j as f64 / ((count - 1) as f64)) * 0.7 + 0.3);

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height
      * if rng.gen_bool(0.1) {
        rng.gen_range(0.5, 1.5)
      } else {
        rng.gen_range(2.0, 3.2)
      };
    let mut miny = base_y;
    let mut maxy = 0.0;
    let mut first = true;
    let mut visible_count = 0;
    let mut layers = 0;

    loop {
      if miny < stopy {
        break;
      }
      layers += 1;

      let mut route = Vec::new();
      let mut x = mountainpadding;
      let mut was_outside = true;
      loop {
        if x > width - mountainpadding {
          break;
        }
        let mut xv = (h - base_y / height) * (x - center);
        if rounding > min_rounding {
          xv = (xv / rounding).round() * rounding;
        }

        let amp = height * ampfactor;
        let mut y = base_y;

        if offsetstrategy == 0 {
          y += amp * peakfactor * xv * xv;
        }

        y += amp2
          * amp
          * perlin.get([
            //
            8.311 + xv * 0.00511,
            88.1 + y * ynoisefactor,
            seed * 97.311,
          ]);

        if offsetstrategy == 2 {
          y += amp * peakfactor * xv * xv;
        }

        y += amp1
          * amp
          * perlin
            .get([
              //
              xv * 0.007111 + 9.9,
              y * 0.00311 + 3.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  55. + seed * 7.3,
                  80.3 + xv * 0.017,
                  y * 0.06 + 11.3,
                ]),
            ])
            .max(0.0);

        if offsetstrategy == 1 {
          y += amp * peakfactor * xv * xv;
        }

        y += 0.05
          * amp
          * perlin.get([
            //
            6.6 + seed * 1.3,
            8.3 + xv * 0.207,
            8.1 + y * 0.31,
          ]);

        if offsetstrategy == 4 {
          y += amp * peakfactor * xv * xv;
        }

        y += amp
          * amp3
          * perlin
            .get([
              //
              xv * 0.009 + 8.33,
              88.1 + y * 0.07,
              seed / 7.7 + 6.66,
            ])
            .powf(2.0);

        if offsetstrategy == 3 {
          y += amp * peakfactor * xv * xv;
        }

        if rounding > min_rounding {
          y = (y / rounding).round() * rounding;
        }

        if y < miny {
          miny = y;
        }
        if y > maxy {
          maxy = y;
        }
        let mut collides = false;
        let xi = ((x - mountainpadding) / precision).round() as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] - 0.01 {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let inside = !collides && strictly_in_boundaries((x, y), bound);
        passage.get((x, y));
        if inside {
          if was_outside {
            let l = route.len();
            if l >= min_route {
              routes.push(route);
              visible_count += l;
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push((x, y));
          passage.count((x, y));
        } else {
          was_outside = true;
        }

        x += precision;
      }

      let l = route.len();
      if l >= min_route {
        routes.push(route);
        visible_count += l;
      }

      if first {
        first = false;
        // optim: jump directly to the visible area (estimated at the gap between highest mount and the base_y). 5mm security
        let diff = base_y - miny - 5.0;
        base_y -= yincr.max(diff);
      } else {
        base_y -= yincr;
      }
    }

    // if there are significant amount of strokes & layers, we consider the mountain to be counted in statistics
    if visible_count > 80 && layers > 6 {
      if yincr < 0.35 {
        // a lot of lines
        feature_mountain_density_plain += 1;
      } else if yincr > 3.0 {
        // a lot of empty mountains
        feature_mountain_density_light += 1;
      } else {
        feature_mountain_density_normal += 1;
      }
    }

    if height_map_stop.len() == 0 || rng.gen_bool(0.5) {
      // pushes the limit away to not have "inner mountains" packing
      height_map_stop = height_map.clone();
    }
  }
  perf.span_end("mountains");

  perf.span("optim mountains");
  let before_optim = routes;
  let mut routes = vec![];
  for r in before_optim {
    let indexes = rdp(&r, 0.02);
    routes.push(indexes.iter().map(|&i| r[i]).collect());
  }
  perf.span_end("optim mountains");

  let shapes_vertex =
    (2f64).powf(rng.gen_range(0f64, 6.0) * rng.gen_range(0.5, 1.0)) as usize;

  let mut feature_sun = "";
  let mut should_use_inception = false;
  if rng.gen_bool(0.5) {
    perf.span("spiral_passage");
    let x = width * (rng.gen_range(0.0, 0.4) * rng.gen_range(-1.0, 1.0) + 0.5);
    let ymax = height_map[((x - mountainpadding) / precision) as usize];
    let y = (1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0))
      * ymax.min(height);
    let radius = rng.gen_range(5.0, 20.0);

    if rng.gen_bool(0.6) {
      let spiral = if rng.gen_bool(0.4) {
        feature_sun = "Simple";
        vec![]
      } else {
        let dr = if rng.gen_bool(0.4) {
          feature_sun = "Swirl";
          1.0
        } else {
          feature_sun = "Full";
          0.3
        };
        spiral_optimized(x, y, radius, dr, 0.01)
      };
      let circle =
        circle_route((x, y), radius, (20.0 * radius) as usize + 10, 0.0);
      for route in vec![circle, spiral] {
        let mut r = vec![];
        for p in route {
          r.push(p);
          let drawable = strictly_in_boundaries(p, bound)
            && p.1
              < height_map[((p.0 - mountainpadding) / precision) as usize
                % height_map.len()];
          if !drawable {
            let l = r.len();
            if l > 1 {
              routes.push(r);
              r = vec![];
            } else if l > 0 {
              r = vec![];
            }
          } else {
            r.push(p);
          }
        }
        if r.len() > 1 {
          routes.push(r);
        }
      }
    } else {
      feature_sun = "Eclipse";
    }

    let route = spiral_optimized(x, y, radius, 1.0, 1.0);
    for p in route {
      passage.count_once(p);
    }
    perf.span_end("spiral_passage");
  } else {
    if shapes_vertex > 2 && shapes_vertex < 14 {
      should_use_inception = true;
      feature_sun = "Inception";
    }
  }

  perf.span("grow_passage");
  let radius = 1.0;
  passage.grow_passage(radius);
  perf.span_end("grow_passage");

  perf.span("packing");
  let p = pad + skyborder;
  let extrabound = (p, p, width - p, height - p);

  let circleang = rng.gen_range(-PI, PI);
  let shape_threshold_circle = 8;

  let overlap = |p| {
    passage.get(p) == 0
      && strictly_in_boundaries(p, extrabound)
      && p.1
        < height_map_stop[((p.0 - mountainpadding) / precision).round()
          as usize
          % height_map_stop.len()]
  };

  let does_overlap = |c: (f64, f64, f64)| {
    overlap((c.0, c.1))
      && circle_route((c.0, c.1), c.2, 10, circleang)
        .iter()
        .all(|&p| overlap(p))
  };

  let freq = if shapes_vertex <= 2 {
    0.01 + rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0)
  } else {
    rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0)
  };
  let skylayers = if shapes_vertex <= 2 {
    rng.gen_range(5, 10)
  } else {
    1 + (rng.gen_range(0.0, 5.0) * rng.gen_range(0.0, 1.0)) as usize
  };
  let maxx = if shapes_vertex <= 2 {
    rng.gen_range(0.0, 2.0)
  } else {
    rng.gen_range(0.0, 40.0)
  };
  let amp = rng.gen_range(1.0, 4.0);
  let basepad = rng.gen_range(0.0, 0.8);
  let mut first_circles: Vec<VCircle> = vec![];
  let mut inception_circles: Vec<VCircle> = vec![];
  let reserve = rng.gen_range(0, 4);
  for _i in 0..skylayers {
    let ppad = rng.gen_range(0.4, 1.0) + basepad;
    let total_pad = radius + pad + ppad;
    let min = ppad + rng.gen_range(0.35, 0.8);
    let max = min + maxx * rng.gen_range(0.0, 1.0);
    let optimize_size = rng.gen_range(0, 4);
    let first = vec![inception_circles.clone(), first_circles.clone()].concat();
    let circles = packing(
      &first,
      &mut rng,
      50000,
      10000,
      optimize_size,
      ppad,
      (total_pad, total_pad, width - total_pad, height - total_pad),
      &does_overlap,
      min,
      max,
    );

    for (i, c) in circles
      .iter()
      .skip(inception_circles.len() + first_circles.len())
      .enumerate()
    {
      let ang = circleang
        + amp
          * perlin.get([
            //
            c.x * freq,
            c.y * freq,
            3333. + 7.7 * seed,
          ]);
      if shapes_vertex <= 2 {
        let a = (c.x + c.r * ang.cos(), c.y + c.r * ang.sin());
        let b = (c.x - c.r * ang.cos(), c.y - c.r * ang.sin());
        routes.push(vec![a, b]);
      } else {
        let count = if shapes_vertex > shape_threshold_circle {
          (5.0 * (c.r + 2.0)) as usize
        } else {
          shapes_vertex
        };
        routes.push(circle_route((c.x, c.y), c.r, count, ang));

        if i == 0 && should_use_inception && inception_circles.len() == 0 {
          let d = rng.gen_range(0.5, 1.0);
          let ad = 0.1 * rng.gen_range(-1.0, 1.0);
          let mut a = ang;
          let mut r = c.r;
          loop {
            a += ad;
            r -= d;
            if r < 0.1 {
              break;
            }
            routes.push(circle_route((c.x, c.y), r, count, a));
          }
        }
      }
    }

    if should_use_inception && inception_circles.len() == 0 {
      inception_circles = circles.iter().take(1).map(|&c| c).collect();
    }

    if first_circles.len() == 0
      && shapes_vertex > shape_threshold_circle
      && reserve > 0
    {
      first_circles = circles
        .iter()
        .skip(if should_use_inception { 1 } else { 0 })
        .take(reserve)
        .map(|&c| c)
        .collect();
    }
  }
  perf.span_end("packing");

  let mut d = 0.0;
  loop {
    if d > frameborder {
      break;
    }
    routes.push(vec![
      (pad + d, pad + d),
      (pad + d, height - pad - d),
      (width - pad - d, height - pad - d),
      (width - pad - d, pad + d),
      (pad + d, pad + d),
    ]);
    d += 0.1;
  }

  perf.span_end("art");

  // Generate the svg
  perf.span("svg");
  let (layers, _inks) =
    make_layers(vec![("#000", opts.layer1_name.clone(), routes)]);
  perf.span_end("svg");

  // add the traits
  let mut traits = Map::new();
  traits.insert(String::from("Color"), json!(opts.layer1_name.clone()));

  traits.insert(String::from("Sun"), json!(feature_sun));
  traits.insert(
    String::from("Sky"),
    json!(if shapes_vertex <= 2 {
      "Flow"
    } else if shapes_vertex == 3 {
      "Sharp"
    } else if shapes_vertex == 4 {
      "Salty"
    } else if shapes_vertex == 5 {
      "Okra"
    } else if shapes_vertex == 6 {
      "Quartz"
    } else if shapes_vertex == 7 {
      "Cactus"
    } else if shapes_vertex == 8 {
      "Octo"
    } else {
      "Bubble"
    }),
  );
  traits.insert(String::from("Sky Density"), json!(skylayers));

  traits.insert(
    String::from("Mountain Density"),
    json!(if feature_mountain_density_plain > 0
      && feature_mountain_density_light == 0
      && (feature_mountain_density_normal == 0
        || feature_mountain_density_plain > 2 + feature_mountain_density_normal)
    {
      "Full"
    } else if feature_mountain_density_plain == 0 {
      if feature_mountain_density_normal > feature_mountain_density_light {
        "Medium"
      } else {
        "Light"
      }
    } else {
      "Mixed"
    }),
  );

  /*
  // DEBUG purpose
  traits.insert(
    String::from("Mountain Density Plain"),
    json!(feature_mountain_density_plain),
  );
  traits.insert(
    String::from("Mountain Density Light"),
    json!(feature_mountain_density_light),
  );
  traits.insert(
    String::from("Mountain Density Normal"),
    json!(feature_mountain_density_normal),
  );
  */

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
        .set("stroke-width", 0.2);
      let opacity: f64 = 0.7;
      for route in routes.clone() {
        let data = render_route(Data::new(), route);
        l = l.add(Path::new().set("opacity", opacity).set("d", data));
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
}

fn circle_route(
  center: (f64, f64),
  r: f64,
  count: usize,
  ang: f64,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * (i as f64 + ang) / (count as f64);
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
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y, size)) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing<R: Rng>(
  first_circles: &Vec<VCircle>,
  rng: &mut R,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = first_circles.clone();
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

pub fn spiral_optimized(
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
    let p = (x + r * a.cos(), y + r * a.sin());
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
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
