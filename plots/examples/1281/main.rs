use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "420.0")]
  pub width: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "5")]
  pub parallels: usize,
  #[clap(short, long, default_value = "0.5")]
  pub target: f64,
}

#[derive(Clone)]
struct CellularAutomaton {
  // we are making an hexagonal grid
  // x and y are represented like orthogonal coordinates, but when y%2==1, x is shifted by 0.5
  // indexes are stored in a 1D array
  pub width: usize,
  pub height: usize,
  pub alpha: f64,
  pub beta: f64,
  pub gamma: f64,
  pub s: Vec<f64>,
  pub u: Vec<f64>,
  pub v: Vec<f64>,
  pub generation: usize,
  pub cache_neighbors: Vec<Vec<(usize, usize)>>,
}

// inspired by this paper https://www.patarnott.com/pdf/SnowCrystalGrowth.pdf
impl CellularAutomaton {
  fn new<R: Rng>(
    rng: &mut R,
    width: usize,
    height: usize,
    alpha: f64,
    beta: f64,
    gamma: f64,
    // this allow for local effects
    beta_variation: f64,
    // we also allow a chance to spawn random frozen cells
    chancetobefrozen: f64,
  ) -> Self {
    // for local effects, we will use a perlin noise and random frequencies
    let perlin = Perlin::new();
    let f1 = rng.gen_range(0.5, 2.0);
    let f2 = rng.gen_range(0.5, 2.0);
    let seed1 = rng.gen_range(0.0, 100.0);
    let seed2 = rng.gen_range(0.0, 100.0);
    let mut s = vec![0.0; width * height];
    let mut u = vec![0.0; width * height];
    let mut v = vec![0.0; width * height];
    for x in 0..width {
      for y in 0..height {
        let is_center = x == width / 2 && y == height / 2;
        let frozen = is_center || rng.gen_bool(chancetobefrozen);
        let initial = if frozen {
          1.0
        } else if beta_variation > 0.0 {
          let xp = x as f64 / width as f64;
          let yp = y as f64 / width as f64;
          beta
            + beta_variation
              * perlin.get([
                f1 * xp,
                f1 * yp,
                seed1 + perlin.get([f2 * xp, f2 * yp, seed2]),
              ])
        } else {
          beta
        };
        let i = x + y * width;
        u[i] = initial;
        v[i] = 0.0;
        s[i] = u[i] + v[i];
      }
    }
    let mut s = Self {
      width,
      height,
      alpha,
      beta,
      gamma,
      s,
      u,
      v,
      generation: 0,
      cache_neighbors: vec![],
    };

    // for performance, it is better to cache the neighbors array
    let mut cache_neighbors = vec![];
    for i in 0..width * height {
      let x = i % width;
      let y = i / width;
      cache_neighbors.push(s._neighbors_of(x, y));
    }
    s.cache_neighbors = cache_neighbors;

    s
  }

  fn _neighbors_of(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut n = vec![];
    // hexadecimal neighbors
    if y % 2 == 0 {
      n.push((x, y - 1));
      n.push((x, y + 1));
      n.push((x - 1, y - 1));
      n.push((x - 1, y + 1));
      n.push((x - 1, y));
      n.push((x + 1, y));
    } else {
      n.push((x + 1, y - 1));
      n.push((x + 1, y + 1));
      n.push((x, y - 1));
      n.push((x, y + 1));
      n.push((x - 1, y));
      n.push((x + 1, y));
    }
    let mut filtered = vec![];
    for (x, y) in n {
      if x > 0 && y > 0 && x < self.width - 1 && y < self.height - 1 {
        filtered.push((x, y));
      }
    }
    filtered
  }

  fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
    let i = self.index(x, y);
    self.cache_neighbors[i].clone()
  }

  fn index(&self, x: usize, y: usize) -> usize {
    x + y * self.width
  }

  // avg of neighbors that aren't receptive yet
  fn average(&self, x: usize, y: usize) -> f64 {
    let mut sum = 0.0;
    for (x, y) in self.neighbors(x, y) {
      if !self.is_receptive(x, y) {
        sum += self.s[self.index(x, y)];
      }
    }
    sum / 6.0
  }

  fn contains(&self, x: usize, y: usize) -> bool {
    x < self.width && y < self.height
  }

  fn state(&self, x: usize, y: usize) -> f64 {
    self.s[self.index(x, y)]
  }

  fn frozen_avg(&self) -> f64 {
    let mut sum = 0.0;
    for y in 0..self.height {
      let mut line_sum = 0.0;
      for x in 0..self.width {
        line_sum += if self.state(x, y) >= 1.0 { 1.0 } else { 0.0 };
      }
      sum += line_sum / self.width as f64;
    }
    sum / self.height as f64
  }

  fn is_receptive(&self, x: usize, y: usize) -> bool {
    // FIXME we could cache the calculation of receptive cells
    if self.state(x, y) >= 1.0 {
      return true;
    }
    for (x, y) in self.neighbors(x, y) {
      if self.state(x, y) >= 1.0 {
        return true;
      }
    }
    return false;
  }

  // this implements one world update step
  fn step(&mut self) {
    let old = self.clone();
    for x in 0..self.width {
      for y in 0..self.height {
        let i = self.index(x, y);
        let is_edge =
          x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1;

        let is_receptive = old.is_receptive(x, y);

        let mut ut = if is_receptive { 0.0 } else { old.state(x, y) };
        let mut vt = if is_receptive { old.state(x, y) } else { 0.0 };
        let st = ut + vt;

        if is_receptive {
          if is_edge {
            ut = old.beta;
            vt = st + old.gamma;
          } else {
            let avg = old.average(x, y);
            ut = (old.alpha / 2.) * avg;
            vt = st + old.gamma;
          }
        } else {
          if is_edge {
            ut = old.beta;
            vt = 0.;
          } else {
            let avg = old.average(x, y);
            ut = st;
            ut += (old.alpha / 2.) * (avg - ut);
            vt = 0.;
          }
        }

        self.s[i] = ut + vt;
        self.u[i] = ut;
        self.v[i] = vt;
      }
    }

    self.generation += 1;
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let parallels = opts.parallels;
  let target = opts.target;

  let mut rng = rng_from_seed(opts.seed);

  let min_its = rng.gen_range(400, 1000);
  let max_its = 3 * min_its;
  let cellsize = 0.5 + rng.gen_range(1.0, 5.0) * rng.gen_range(0.0, 1.0);

  println!("cellsize: {}", cellsize);
  println!("min_its: {}", min_its);

  let basechance =
    rng.gen_range(-0.005f64, 0.01).max(0.0) * rng.gen_range(0.0, 1.0);

  // as the simulation is very chaotic, we run a bunch in parallel and keep the best

  let mut simulations: Vec<_> = (0..parallels)
    .into_par_iter()
    .map(|i| {
      let mut rng = rng_from_seed(opts.seed + (i as f64) / 0.037);
      let count = rng.gen_range(min_its, max_its);
      let alpha = 1.0 + rng.gen_range(-0.1, 0.1)*rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
      let beta = 0.5 + rng.gen_range(-0.3, 0.3)*rng.gen_range(0.0, 1.0);
      let betavariation = rng.gen_range(-0.5f64, 1.0).max(0.0) * beta;
      let gamma = rng.gen_range(0.0, 0.02) * rng.gen_range(0.2, 1.0);
      let chancetobefrozen = basechance
        * rng.gen_range(0.0, 1.0);

      let simw = (width / cellsize) as usize;
      let simh = (1.25 * height / cellsize) as usize;

      let mut ca = CellularAutomaton::new(
        &mut rng,
        simw,
        simh,
        alpha,
        beta,
        gamma,
        betavariation,
        chancetobefrozen,
      );

      for _i in 0..count {
        ca.step();
      }

      let avg = ca.frozen_avg();
      println!("i: {} avg: {} count: {} alpha: {} beta: {} gamma: {} betavariation: {} chancetobefrozen: {}", i, avg, count, alpha, beta, gamma, betavariation, chancetobefrozen);

      (ca, avg)
    })
    .collect();

  simulations.sort_by_key(|s| ((s.1 - target).abs() * 1000.0) as u32);

  println!("best: {}", simulations[0].1);

  let ca = simulations[0].0.clone();

  let density = 4.0;

  // this functions looks up a density value for an absolute position
  let f = |x, y| {
    let (xi, yi) = hex_index((x - pad, y - pad), cellsize * 0.5);

    if xi < 0 || yi < 0 {
      return 0.0;
    }

    let xi = xi as usize;
    let yi = yi as usize;
    if ca.contains(xi, yi) {
      let s = ca.state(xi, yi);
      if s >= 1.0 {
        density * smoothstep(1.0, 3.0, s)
      } else {
        0.0
      }
    } else {
      0.0
    }
  };

  // we use a "worms filling" home made algo to fill the space
  // using the density function driven by the cellular automaton state
  let filling = WormsFilling::rand(&mut rng);

  let routes = filling.fill(
    &mut rng,
    &f,
    (pad, pad, width - pad, height - pad),
    0,
    20000,
  );

  // we will clip the borders in case we go out of boundaries
  let mut paint = PaintMask::new(0.1, width, height);
  paint.paint_borders(pad);
  let routes = regular_clip(&routes, &mut paint);

  // we can finally generate our SVG
  vec!["white"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
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
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

// implement clipping for colored polylines
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

// implement an efficient 2D lookup mask, used for clipping
#[derive(Clone)]
pub struct PaintMask {
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

  fn is_painted(&self, point: (f64, f64)) -> bool {
    let precision = self.precision;
    let wi = (self.width / precision) as usize;
    let hi = (self.height / precision) as usize;
    let x = ((point.0.max(0.) / precision) as usize).min(wi - 1);
    let y = ((point.1.max(0.) / precision) as usize).min(hi - 1);
    self.mask[x + y * wi]
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
}

fn regular_clip(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let is_outside = |p| paint.is_painted(p);
  clip_routes_with_colors(&routes, &is_outside, 0.3, 5)
}

// homemade implementation of a filling technique that will spawn random worms that eat the space to colorize it!
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
    let step = 0.4;
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
            let points: Vec<(f64, f64)> = rdp(&route, 0.05);
            // remap
            let rt = points
              .iter()
              .map(|&p| (p.0 + bound.0, p.1 + bound.1))
              .collect();
            routes.push((clr, rt));
          }
        }
      }
    }

    routes
  }
}

// data model that stores values information in 2D
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

// Hexagonal helpers below. inspired by https://www.redblobgames.com/grids/hexagons/

fn hex_index((x, y): (f64, f64), size: f64) -> (i64, i64) {
  let h = pixel_to_pointy_hex((x, y), size);
  ((h.q + 0.5 * h.r) as i64, h.r as i64)
}

#[derive(Debug)]
struct Cube {
  q: f64,
  r: f64,
  s: f64,
}

#[derive(Debug)]
struct Hex {
  q: f64,
  r: f64,
}

fn cube_round(frac: Cube) -> Cube {
  let q = frac.q.round();
  let r = frac.r.round();
  let s = frac.s.round();

  let q_diff = (q - frac.q).abs();
  let r_diff = (r - frac.r).abs();
  let s_diff = (s - frac.s).abs();

  let mut q_new = q;
  let mut r_new = r;
  let mut s_new = s;

  if q_diff > r_diff && q_diff > s_diff {
    q_new = -r - s;
  } else if r_diff > s_diff {
    r_new = -q - s;
  } else {
    s_new = -q - r;
  }

  Cube {
    q: q_new,
    r: r_new,
    s: s_new,
  }
}

fn axial_round(hex: Hex) -> Hex {
  let cube = axial_to_cube(hex);
  let rounded_cube = cube_round(cube);
  cube_to_axial(rounded_cube)
}

fn cube_to_axial(cube: Cube) -> Hex {
  Hex {
    q: cube.q,
    r: cube.r,
  }
}

fn axial_to_cube(hex: Hex) -> Cube {
  let q = hex.q;
  let r = hex.r;
  let s = -q - r;
  Cube { q, r, s }
}

fn pixel_to_pointy_hex(point: (f64, f64), size: f64) -> Hex {
  let q = (f64::sqrt(3.0) / 3.0 * point.0 - 1.0 / 3.0 * point.1) / size;
  let r = (2.0 / 3.0 * point.1) / size;
  axial_round(Hex { q, r })
}
