use crate::algo::paintmask::*;
use crate::algo::rdp::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
// homemade implementation of a filling technique that will spawn random worms that eat the space to colorize it!
pub struct WormsFilling {
  rot: f64,
  step: f64,
  straight: f64,
  min_l: usize,
  max_l: usize,
  decrease_value: f64,
  min_weight: f64,
  freq: f64,
  seed: f64,
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
    }
  }

  pub fn fill_in_paint<R: Rng>(
    &self,
    rng: &mut R,
    drawings: &PaintMask,
    clr: usize,
    density: f64,
    bound: (f64, f64, f64, f64),
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let f = |x, y| {
      if drawings.is_painted((x, y)) {
        density
      } else {
        0.0
      }
    };
    let coloring = |_: &Vec<(f64, f64)>| clr;
    let precision = 0.4;
    self.fill(rng, &f, bound, &coloring, precision, iterations, 1)
  }

  pub fn fill<R: Rng>(
    &self,
    rng: &mut R,
    f: &dyn Fn(f64, f64) -> f64,
    bound: (f64, f64, f64, f64),
    clr: &dyn Fn(&Vec<(f64, f64)>) -> usize,
    precision: f64,
    iterations: usize,
    search_max: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let perlin = Perlin::new(rng.gen());
    let w = bound.2 - bound.0;
    let h = bound.3 - bound.1;
    if w <= 2. * precision || h <= 2. * precision {
      return routes;
    }
    let mut map = WeightMap::new(w, h, precision);

    map.fill_fn(&|p| f(p.0 + bound.0, p.1 + bound.1));

    let seed = self.seed;
    let rot = self.rot;
    let step = self.step;
    let straight = self.straight;
    let min_l = self.min_l;
    let max_l = self.max_l;
    let decrease_value = self.decrease_value;
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

// data model that stores values information in 2D
pub struct WeightMap {
  weights: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl WeightMap {
  pub fn new(width: f64, height: f64, precision: f64) -> WeightMap {
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
  pub fn fill_fn(&mut self, f: &impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.weights[y * self.w + x] = v;
      }
    }
  }

  // do a simple bilinear interpolation
  pub fn get_weight(&self, p: (f64, f64)) -> f64 {
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
  pub fn decrease_weight_gaussian(
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
    let weights = &mut self.weights;
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f64 * precision, y as f64 * precision);
        let dx = p.0 - p.0;
        let dy = p.1 - p.1;
        let d2 = dx * dx + dy * dy;
        if d2 < radius2 {
          let i = y * w + x;
          let d = d2.sqrt();
          let w = weights[i];
          let v = value * (1.0 - d / radius);
          weights[i] = w - v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  pub fn best_direction(
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

  pub fn search_weight_top<R: Rng>(
    &mut self,
    rng: &mut R,
    search_max: usize,
    min_weight: f64,
  ) -> Option<(f64, f64)> {
    let mut tries = 0;
    let mut best_w = min_weight;
    let mut best_p: Option<usize> = None;
    let weights = &self.weights;
    let l = weights.len();
    let rand_off = rng.gen_range(0..l);
    for j in 0..l {
      let i = (j + rand_off) % l;
      let w = weights[i];
      if w > best_w {
        tries += 1;
        best_w = w;
        best_p = Some(i);
        if tries > search_max {
          break;
        }
      }
    }
    return best_p.map(|i| {
      let x = (i % self.w) as f64 * self.precision;
      let y = (i / self.w) as f64 * self.precision;
      (x, y)
    });
  }

  pub fn dig_random_route(
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
