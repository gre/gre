use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "420.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

#[derive(Clone)]
struct Value2D {
  width: f64,
  height: f64,
  ang1: f64,
  ang2: f64,
  ang3: f64,
  f1: f64,
  f2: f64,
  f3: f64,
  amp2: f64,
  linearf: f64,
  noisefactor: f64,
  noisebalance: f64,
  yfactor: f64,
  seed: f64,
  count: usize,
  symx: bool,
  symy: bool,
  rotation: f64,
}
impl Value2D {
  fn map(&self, x: f64, y: f64) -> f64 {
    let ratio = self.width / self.height;
    let ang1 = self.ang1;
    let ang2 = self.ang2;
    let ang3 = self.ang3;
    let f1 = self.f1;
    let f2 = self.f2;
    let f3 = self.f3;
    let amp2 = self.amp2;
    let noisefactor = self.noisefactor;
    let noisebalance = self.noisebalance;
    let yfactor = self.yfactor;
    let seed = self.seed;
    let linearf = self.linearf;
    let symx = self.symx;
    let symy = self.symy;
    let rot = self.rotation;

    let (x, y) = p_r((x - 0.5, y - 0.5), rot);
    let x = x + 0.5;
    let y = y + 0.5;

    let x = if symx { (1. - x).min(x) } else { x };
    let y = if symy { (1. - y).min(y) } else { y };

    let perlin = Perlin::new();

    let mut q = p_r((ratio * (x - 0.5), y - 0.5), ang1);
    q.0 += 0.5;
    q.1 += 0.5;
    let mut p = p_r((ratio * (x - 0.5), y - 0.5), ang2);
    p.0 += 0.5;
    p.1 += 0.5;
    let mut r = p_r((ratio * (x - 0.5), y - 0.5), ang3);
    r.0 += 0.5;
    r.1 += 0.5;

    let n = 0.5
      + noisefactor
        * (noisebalance
          * perlin.get([
            f1 * p.0 + seed,
            f1 * p.1
              + seed
              + amp2 * perlin.get([f2 * q.0, f2 * q.1, seed / 0.3]),
            seed,
          ])
          + (1. - noisebalance) * perlin.get([f3 * r.0, f3 * r.1, seed / 0.7]));
    (n + linearf * (mix(x, y, yfactor) - 0.5)).max(0.0).min(1.0)
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let precision = 0.2;

  let mut rng = rng_from_seed(opts.seed);
  let mut paint = PaintMask::new(precision, width, height);
  paint.paint_borders(pad);

  let f1 = rng.gen_range(0.6, 3.0);
  let f2 = rng.gen_range(0.6, 3.0);
  let f3 = rng.gen_range(0.6, 3.0);

  let amp2 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

  let linearf = rng.gen_range(0.0, 0.7);
  let noisefactor = rng.gen_range(0.0, 0.5);
  let noisebalance = rng.gen_range(0.1, 0.9);
  let ang1 = if rng.gen_bool(0.5) {
    rng.gen_range(-PI, PI)
  } else {
    0.0
  };
  let ang2 = if rng.gen_bool(0.5) {
    rng.gen_range(-PI, PI)
  } else {
    ang1
  };
  let ang3 = if rng.gen_bool(0.5) {
    rng.gen_range(-PI, PI)
  } else {
    ang2
  };
  let rotation = rng.gen_range(-PI, PI) * rng.gen_range(-1.0f64, 1.0).max(0.0);

  let mut routes = vec![];
  let mut values = vec![];

  let count = 2;

  for i in 0..2 {
    let yfactor = (2.0 * (i as f64) / (count as f64)) % 2.0;
    let count = rng.gen_range(10, 80);

    let valuef = Value2D {
      width,
      height,
      ang1,
      ang2,
      ang3,
      f1,
      f2,
      f3,
      amp2,
      linearf,
      noisefactor,
      noisebalance,
      yfactor,
      seed: opts.seed,
      count,
      symx: rng.gen_bool(0.8),
      symy: rng.gen_bool(0.3),
      rotation,
    };
    values.push(valuef.clone());
  }

  // 0: grid
  // 1: intersecting grid
  // 2: empty
  let fill_mode = (rng.gen_range(0.0, 3.0) * rng.gen_range(0.0, 1.0)) as usize;

  let density = 2.0;

  let filling = WormsFilling::rand(&mut rng);
  routes.extend(filling.fill(
    &mut rng,
    &|x, y| match fill_mode {
      0 => {
        let mut i = 0;
        for f in values.iter() {
          let v = f.map(x / width, y / height);
          let i1 = (v * (f.count as f64 + 1.)) as usize;
          i += i1;
        }
        if i % 2 == 0 {
          density
        } else {
          0.0
        }
      }
      1 => {
        let mut filled = true;
        for f in values.iter() {
          let v = f.map(x / width, y / height);
          let i = (v * (f.count as f64 + 1.)) as usize;
          filled = filled && i % 2 == 0;
        }
        if filled {
          density
        } else {
          0.0
        }
      }
      _ => 0.0,
    },
    (0., 0., width, height),
    0,
    10000,
  ));

  routes = regular_clip(&routes, &mut paint);

  // sample the first point of routes that will be spawn points for sparkles
  let mut sparkles = vec![];
  let count = rng.gen_range(2000, 5000).min(routes.len() - 1);
  for i in 0..count {
    let route = &routes[i];
    let p = route.1[0];
    sparkles.push(p);
  }

  // add sparkles
  let mut routes = vec![];
  let min_size = rng.gen_range(0.5, 1.2);
  let add_size: f64 = rng.gen_range(0.0, 12.0);
  for sparkle in sparkles {
    let ratio = rng.gen_range(1.2, 2.0);
    let mut size = min_size
      + rng.gen_range(0.0, add_size)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
    let dx = size;
    let dy = ratio * size;
    let route = vec![
      (sparkle.0, sparkle.1 - dy),
      (sparkle.0 + dx, sparkle.1),
      (sparkle.0, sparkle.1 + dy),
      (sparkle.0 - dx, sparkle.1),
      (sparkle.0, sparkle.1 - dy),
    ];
    let clr = (rng.gen_range(0., 2.) * rng.gen_range(0.3, 1.0)) as usize;
    let mut rts = vec![(clr, route.clone())];
    if rng.gen_bool(0.3) {
      while size > 0.1 {
        let dx = size;
        let dy = ratio * size;
        let route = vec![
          (sparkle.0, sparkle.1 - dy),
          (sparkle.0 + dx, sparkle.1),
          (sparkle.0, sparkle.1 + dy),
          (sparkle.0 - dx, sparkle.1),
          (sparkle.0, sparkle.1 - dy),
        ];
        rts.push((clr, route));
        size -= 0.4;
      }
    }

    let extra = 0.4;
    let poly = vec![
      (sparkle.0, sparkle.1 - dy - extra),
      (sparkle.0 + dx + extra, sparkle.1),
      (sparkle.0, sparkle.1 + dy + extra),
      (sparkle.0 - dx - extra, sparkle.1),
      (sparkle.0, sparkle.1 - dy - extra),
    ];
    let rts = regular_clip_polys(&rts, &mut paint, &vec![poly]);
    routes.extend(rts);
  }

  vec!["silver", "gold"]
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
      l = l.add(base_path(color, 0.5, data));
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

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision).floor() as usize;
    let miny = ((miny).max(0.).min(self.height) / precision).floor() as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision).ceil() as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision).ceil() as usize;
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

fn regular_clip(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let is_outside = |p| paint.is_painted(p);
  clip_routes_with_colors(&routes, &is_outside, 0.5, 3)
}

fn regular_clip_polys(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
  polys: &Vec<Vec<(f64, f64)>>,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let rts = regular_clip(routes, paint);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }
  rts
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
