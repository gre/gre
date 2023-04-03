use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> svg::Document {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  let fire_and_ice = Ink("Fire And Ice", "#00BEDE", "#006478", 0.35);
  let white_paper = Paper("White", "#fff", false);

  let paper = white_paper;
  let inks = vec![fire_and_ice];

  let perlin = Perlin::new();

  let mut routes = Vec::new();

  let modfactor = rng.gen_range(0.0, 2.0);

  let f1 = rng.gen_range(3.0, 10.0);
  let amp1 = rng.gen_range(0.5, 2.0);
  let f2 = f1 * rng.gen_range(0.0, 4.0);
  let amp2 = rng.gen_range(0.0, 10.0);
  let f3 = f2 * rng.gen_range(0.0, 4.0);
  let amp3 = rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0);
  let f4 = rng.gen_range(0.5, 4.0);
  let amp4 = rng.gen_range(-0.5f64, 0.5).max(0.0);
  let f5 = f4 * rng.gen_range(0.0, 3.0);
  let amp5 = rng.gen_range(0.0, 2.0);
  let f6 = f5 * rng.gen_range(0.0, 3.0);
  let amp6 = rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);

  let mirrorx = rng.gen_bool(0.5);

  let density = 5.0;
  let precision = 0.4;
  let r = 2.0 + rng.gen_range(4.0, 20.0) * rng.gen_range(0.0, 1.0);

  let f = |(x, y): (f64, f64)| -> (usize, f64) {
    let x = if mirrorx {
      (x - width / 2.0).abs() + width / 2.0
    } else {
      x
    };
    if x < pad || x > width - pad || y < pad || y > height - pad {
      return (0, 0.0);
    }
    let value1 = amp1
      * perlin.get([
        f1 * x / height as f64,
        f1 * y / height as f64,
        opts.seed
          + amp2
            * perlin.get([
              f2 * x / height as f64,
              f2 * y / height as f64,
              66.6
                + 5.555 * opts.seed
                + amp3
                  * perlin.get([
                    f3 * x / height as f64,
                    f3 * y / height as f64,
                    777. + opts.seed / 0.177,
                  ]),
            ]),
      ])
      + amp4
        * perlin.get([
          f4 * x / height as f64,
          f4 * y / height as f64,
          8.8 * opts.seed
            + amp5
              * perlin.get([
                f5 * x / height as f64,
                f5 * y / height as f64,
                9.6
                  + 75.55 * opts.seed
                  + amp6
                    * perlin.get([
                      f6 * x / height as f64,
                      f6 * y / height as f64,
                      77. + opts.seed / 0.77,
                    ]),
              ]),
        ]);

    let valuemod = (y / r) % (inks.len() as f64);
    let color = valuemod.floor() as usize;
    let value2 = 1.0 - (valuemod % 1.0);
    //let value2 = 2.0 * (0.5 - (valuemod % 1.0 - 0.5).abs());

    let value = modfactor * value1 + value2;

    (color, value * density)
  };

  let mut map = WeightMap::new(width, height, precision);

  // let mut rng = rng_from_seed(opts.seed + ink_i as f64);

  map.fill_fn(&|p| f(p).1);

  let rot = PI / rng.gen_range(1.0, 3.0);
  let step = 0.6;
  let straight = 0.1;
  let count = 20000;
  let min_l = 5;
  let max_l = rng.gen_range(10, 80);
  let decrease_value = 1.0;
  let search_max = 500;
  let min_weight = 1.0;
  let mut bail_out = 0;

  for _i in 0..count {
    let top = map.search_weight_top(&mut rng, search_max, min_weight);
    if top.is_none() {
      bail_out += 1;
      if bail_out > 10 {
        break;
      }
    }
    if let Some(o) = top {
      let angle = perlin.get([
        2. * o.0 / height as f64,
        2. * o.1 / height as f64,
        opts.seed,
      ]);

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
          let rt = rdp(&route, 0.05);
          let ink_i = f(o).0;
          routes.push((ink_i, rt));
        }
      }
    }
  }

  let layers = inks
    .iter()
    .enumerate()
    .map(|(ci, &ink)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(ink.0)).as_str());
      l = l.add(base_path(ink.1, ink.3, data));
      l
    })
    .collect::<Vec<_>>();

  let mut document = base_document(paper.1, opts.width, opts.height);
  for g in layers {
    document = document.add(g);
  }
  document
}

fn main() {
  let opts: Opts = Opts::parse();
  let document = art(&opts);
  svg::save(opts.file, &document).unwrap();
}

#[derive(Clone, Copy)]
pub struct Ink(&'static str, &'static str, &'static str, f64);
#[derive(Clone, Copy)]
pub struct Paper(&'static str, &'static str, bool);

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
    let x0 = (x - radius).floor() as usize;
    let y0 = (y - radius).floor() as usize;
    let x1 = (x + radius).ceil() as usize;
    let y1 = (y + radius).ceil() as usize;
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

pub fn grayscale((r, g, b): (f64, f64, f64)) -> f64 {
  return 0.299 * r + 0.587 * g + 0.114 * b;
}
