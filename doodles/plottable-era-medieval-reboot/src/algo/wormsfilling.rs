use crate::algo::paintmask::*;
use crate::algo::rdp::*;
use noise::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::consts::PI;

use super::polylines::Polylines;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
// homemade implementation of a filling technique that will spawn random worms that eat the space to colorize it!
pub struct WormsFilling {
  pub rot: f32,
  pub step: f32,
  pub straight: f32,
  pub min_l: usize,
  pub max_l: usize,
  pub decrease_value: f32,
  pub min_weight: f32,
  pub freq: f32,
  pub seed: f64,
  pub angle_precision: f32,
}
impl WormsFilling {
  // new
  pub fn rand<R: Rng>(rng: &mut R) -> Self {
    let seed = rng.gen_range(-999.0..999.);
    let rot = PI / rng.gen_range(1.0..2.0);
    let step = 0.4;
    let straight = rng.gen_range(0.0..0.1);
    let min_l = 5;
    let max_l = 20;
    let decrease_value = 1.;
    let min_weight = 1.;
    let freq = 0.05;
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

  pub fn fill_in_paint<R: Rng>(
    &self,
    rng: &mut R,
    drawings: &PaintMask,
    clr: usize,
    density: f32,
    bound: (f32, f32, f32, f32),
    iterations: usize,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let f = |x, y| {
      if drawings.is_painted((x, y)) {
        density
      } else {
        0.0
      }
    };
    let coloring = |_: &Vec<(f32, f32)>| clr;
    let precision = 0.4;
    let search_max = rng.gen_range(5..10);
    self.fill(rng, &f, bound, &coloring, precision, iterations, search_max)
  }

  pub fn fill<R: Rng>(
    &self,
    rng: &mut R,
    f: &dyn Fn(f32, f32) -> f32,
    bound: (f32, f32, f32, f32),
    clr: &dyn Fn(&Vec<(f32, f32)>) -> usize,
    precision: f32,
    iterations: usize,
    search_max: usize,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    let perlin = Perlin::new(rng.gen());
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
      let top = map.search_weight_top(search_max, min_weight);
      if top.is_none() {
        bail_out += 1;
        if bail_out > 10 {
          break;
        }
      }
      if let Some(o) = top {
        let angle =
          perlin.get([seed, freq * o.0 as f64, freq * o.1 as f64]) as f32;

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
            let points: Vec<(f32, f32)> = rdp(&route, 0.05);
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

// data model that stores values information in 2D
pub struct WeightMap {
  // TODO performance still aren't great. we need a map{index->weight} where we can easily update by index but also we can easily sort by weight (resorted each time we insert)
  weights: Vec<f32>,
  living_indexes: HashSet<usize>,
  living_threshold: f32,
  w: usize,
  h: usize,
  pub width: f32,
  pub height: f32,
  pub precision: f32,
}
impl WeightMap {
  pub fn new(
    width: f32,
    height: f32,
    precision: f32,
    living_threshold: f32,
  ) -> WeightMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let weights = vec![0.0; w * h];
    WeightMap {
      weights,
      living_indexes: HashSet::new(),
      living_threshold,
      w,
      h,
      width,
      height,
      precision,
    }
  }
  pub fn fill_fn(&mut self, f: &impl Fn((f32, f32)) -> f32) {
    let mut living_indexes = HashSet::new();
    for x in 0..self.w {
      for y in 0..self.h {
        let p = (x as f32 * self.precision, y as f32 * self.precision);
        let v = f(p);
        let i = y * self.w + x;
        self.weights[i] = v;
        if v > self.living_threshold {
          living_indexes.insert(i);
        }
      }
    }
    self.living_indexes = living_indexes;
  }

  // do a simple bilinear interpolation
  pub fn get_weight(&self, p: (f32, f32)) -> f32 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f32;
    let dy = y - y0 as f32;
    let w00 = self.weights[y0 * self.w + x0];
    let w01 = self.weights[y0 * self.w + x1];
    let w10 = self.weights[y1 * self.w + x0];
    let w11 = self.weights[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the weights around the point p with a given radius
  pub fn decrease_weight_gaussian(
    &mut self,
    p: (f32, f32),
    radius: f32,
    value: f32,
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
    let weights = &mut self.weights;
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f32 * precision, y as f32 * precision);
        let dx = p.0 - p.0;
        let dy = p.1 - p.1;
        let d2 = dx * dx + dy * dy;
        if d2 < radius2 {
          let i = y * w + x;
          let d = d2.sqrt();
          let w = weights[i];
          let v = value * (1.0 - d / radius);
          let newv = w - v;
          weights[i] = newv;
          if w >= self.living_threshold && newv < self.living_threshold {
            self.living_indexes.remove(&i);
          }
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  pub fn best_direction(
    &self,
    p: (f32, f32),
    step: f32,
    angle: f32,
    max_ang_rotation: f32,
    angle_precision: f32,
    straight_factor: f32,
  ) -> Option<f32> {
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

  pub fn search_weight_top(
    &mut self,
    search_max: usize,
    min_weight: f32,
  ) -> Option<(f32, f32)> {
    let living_indexes = &self.living_indexes;
    let l = living_indexes.len();
    if l == 0 {
      return None;
    }
    let mut tries = 0;
    let mut best_w = min_weight;
    let mut best_p: Option<usize> = None;
    let weights = &self.weights;
    for &i in living_indexes.iter() {
      if tries > search_max {
        break;
      }
      let w = weights[i];
      if w > best_w {
        tries += 1;
        best_w = w;
        best_p = Some(i);
      }
    }
    return best_p.map(|i| {
      let w = self.w;
      let precision = self.precision;
      let x = (i % w) as f32 * precision;
      let y = (i / w) as f32 * precision;
      (x, y)
    });
  }

  pub fn dig_random_route(
    &mut self,
    origin: (f32, f32),
    initial_angle: f32,
    step: f32,
    max_ang_rotation: f32,
    straight_factor: f32,
    max_length: usize,
    decrease_value: f32,
  ) -> Vec<(f32, f32)> {
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

pub fn worms_fill_strokes<R: Rng>(
  rng: &mut R,
  paint_ref: &PaintMask,
  its: usize,
  w: f32,
  density: f32,
  routes: &Polylines,
) -> Polylines {
  let filling = WormsFilling::rand(rng);
  let mut hash: HashMap<usize, PaintMask> = HashMap::new();
  for (clr, rt) in routes.iter() {
    if let Some(drawing) = hash.get_mut(clr) {
      drawing.paint_polyline(rt, w);
    } else {
      let mut drawing = paint_ref.clone_empty();
      drawing.paint_polyline(rt, w);
      hash.insert(*clr, drawing);
    }
  }
  let mut rts = vec![];
  for (clr, drawing) in hash.iter() {
    let bound = drawing.painted_boundaries();
    let routes = filling.fill_in_paint(rng, drawing, *clr, density, bound, its);
    rts.extend(routes);
  }
  rts
}
