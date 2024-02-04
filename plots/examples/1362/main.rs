use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  width: f64,
  #[clap(short, long, default_value = "297.0")]
  height: f64,
  #[clap(short, long, default_value = "20.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  let mut routes = vec![];

  let mut rng = rng_from_seed(opts.seed);

  let mul = rng.gen_range(0.0, 10.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);

  let mul3 = rng.gen_range(0.0, 10.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);

  let compmul = rng.gen_range(2.0f64, 16.0).min(8.0);
  let compdif = rng.gen_range(0.0f64, 8.0);

  let perlin = Perlin::new();
  let component_value = |cx: usize, cy: usize, i: usize| -> u8 {
    let n = perlin.get([
      mul * cx as f64 / 0.1971,
      mul * cy as f64 / 0.2371 + 8.761,
      mul3 * i as f64 / 0.3317 + 6.42142 + opts.seed / 3.71012,
    ]);
    ((n + 0.5) * compmul + compdif).floor() as u8 % 8
  };

  let pxsize = 50.0;
  let wdiv = ((width - 2. * pad) / pxsize).ceil() as usize;
  let hdiv = ((height - 2. * pad) / pxsize).ceil() as usize;
  let pos_value = |x: f64, y: f64| {
    let xf = (x - pad) / (width - 2. * pad) * wdiv as f64;
    let yf = (y - pad) / (height - 2. * pad) * hdiv as f64;
    let x = xf % 1.0;
    let y = yf % 1.0;
    let i = (x * 2.0) as usize + (y * 2.0) as usize * 2;
    let dx = x - 0.5;
    let dy = y - 0.5;
    let dist = 2.0 * (dx * dx + dy * dy).sqrt();
    let ring = if dist < 0.5 {
      0
    } else if dist < 1.0 {
      1
    } else {
      2
    };
    let i = i * 3 + ring;
    component_value(xf as usize, yf as usize, i)
  };

  for clr in 0..3 {
    let filling = WormsFilling::rand(&mut rng);
    let prec = 0.4;
    let its = 100000;
    routes.extend(filling.fill(
      &|x, y| {
        let v = pos_value(x, y);
        if v & (2u8).pow(clr) != 0 {
          0.0
        } else {
          3.0
        }
      },
      bound,
      &|_rt| clr as usize,
      prec,
      its,
    ));

    routes = clip_routes_with_colors(
      &routes,
      &|p| out_of_boundaries(p, bound),
      0.3,
      4,
    );
  }

  vec!["#fc0", "darkturquoise", "red"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

struct WormsFilling {
  rot: f64,
  step: f64,
  straight: f64,
  min_l: usize,
  max_l: usize,
  decrease_value: f64,
  min_weight: f64,
  freq: f64,
  seed: f64,
  angle_precision: f64,
}
impl WormsFilling {
  // new
  fn rand<R: Rng>(rng: &mut R) -> Self {
    // local rng to not have impredictibility.
    let mut rng = StdRng::from_rng(rng).unwrap();
    let seed = rng.gen_range(-999.0, 999.);
    let rot = PI / rng.gen_range(1.0, 2.0);
    let step = 0.6;
    let straight = rng.gen_range(0.0, 0.1);
    let min_l = 5;
    let max_l = 25;
    let decrease_value = 1.;
    let min_weight = 1.;
    let freq = 0.1;
    let angle_precision = PI / 4.0;
    Self {
      rot,
      step,
      straight,
      min_l,
      max_l,
      decrease_value,
      min_weight,
      freq,
      seed,
      angle_precision,
    }
  }

  fn fill<F: Fn(f64, f64) -> f64>(
    &self,
    f: &F,
    bound: (f64, f64, f64, f64),
    clr: &dyn Fn(&Vec<(f64, f64)>) -> usize,
    precision: f64,
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let perlin = Perlin::new();
    let w = bound.2 - bound.0;
    let h = bound.3 - bound.1;
    if w <= 2. * precision || h <= 2. * precision {
      return routes;
    }
    let min_weight = self.min_weight;
    let mut map = WeightMap::new(w, h, precision, min_weight);

    map.fill_fn(&|p| f(p.0 + bound.0, p.1 + bound.1));

    let seed = self.seed;
    let rot = self.rot;
    let step = self.step;
    let straight = self.straight;
    let min_l = self.min_l;
    let max_l = self.max_l;
    let decrease_value = self.decrease_value;
    let freq = self.freq as f64;

    let mut bail_out = 0;

    for _i in 0..iterations {
      let top = map.search_weight_top();
      if top.is_none() {
        bail_out += 1;
        if bail_out > 100 {
          break;
        }
      }
      if let Some(o) = top {
        let angle =
          perlin.get([seed, freq * o.0 as f64, freq * o.1 as f64]) as f64;

        if let Some(a) =
          map.best_direction(o, step, angle, PI, self.angle_precision, 0.0)
        {
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
            let points: Vec<(f64, f64)> = rdp(&route, 0.05);
            // remap
            let rt = points
              .iter()
              .map(|&p| (p.0 + bound.0, p.1 + bound.1))
              .collect::<Vec<_>>();
            let c = clr(&rt);
            routes.push((c, rt));
          }
        }
      }
    }

    routes
  }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap_or(Ordering::Equal)
  }
}

// data model that stores values information in 2D
struct WeightMap {
  // TODO performance still aren't great. we need a map{index->weight} where we can easily update by index but also we can easily sort by weight (resorted each time we insert)
  // FIXME the usage of HashSet here also cause the Worms Filling to not be deterministic :(
  weights: Vec<f64>,
  weight_index_map: BTreeMap<OrderedFloat, HashSet<usize>>, // Maps weight to a set of indexes

  living_threshold: f64,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl WeightMap {
  fn new(
    width: f64,
    height: f64,
    precision: f64,
    living_threshold: f64,
  ) -> WeightMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let weights = vec![0.0; w * h];
    WeightMap {
      weights,
      weight_index_map: BTreeMap::new(),
      living_threshold,
      w,
      h,
      width,
      height,
      precision,
    }
  }
  fn fill_fn<F: Fn((f64, f64)) -> f64>(&mut self, f: &F) {
    for x in 0..self.w {
      for y in 0..self.h {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        let i = y * self.w + x;
        self.update_weight(i, v);
      }
    }
  }

  // do a simple bilinear interpolation
  fn get_weight(&self, p: (f64, f64)) -> f64 {
    let x = p.0.max(0.) / self.precision;
    let y = p.1.max(0.) / self.precision;
    let x0 = (x.floor() as usize).min(self.w - 1);
    let y0 = (y.floor() as usize).min(self.h - 1);
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
    let precision = self.precision;
    let w = self.w;
    let radius2 = radius * radius;
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f64 * precision, y as f64 * precision);
        let dx = p.0 - p.0;
        let dy = p.1 - p.1;
        let d2 = dx * dx + dy * dy;
        if d2 < radius2 {
          let i = y * w + x;
          let d = d2.sqrt();
          let w = self.weights[i];
          let v = value * (1.0 - d / radius);
          let newv = w - v;
          self.update_weight(i, newv);
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

  fn update_weight(&mut self, index: usize, new_weight: f64) {
    let old_weight = OrderedFloat(self.weights[index]);
    self.weights[index] = new_weight;

    // Remove old weight entry
    if let Some(indexes) = self.weight_index_map.get_mut(&old_weight) {
      indexes.remove(&index);
      if indexes.is_empty() {
        self.weight_index_map.remove(&old_weight);
      }
    }

    if new_weight < self.living_threshold {
      return;
    }
    // Insert new weight entry
    self
      .weight_index_map
      .entry(OrderedFloat(new_weight))
      .or_insert_with(HashSet::new)
      .insert(index);
  }

  fn search_weight_top(&mut self) -> Option<(f64, f64)> {
    self
      .weight_index_map
      .iter()
      .last()
      .and_then(|(_, indexes)| {
        indexes.iter().next().map(|&index| {
          let x = (index % self.w) as f64 * self.precision;
          let y = (index / self.w) as f64 * self.precision;
          (x, y)
        })
      })
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
