use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;

fn art(opts: &Opts) -> svg::Document {
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let inaho = Ink("iroshizuku ina-ho", "#ba6", "#7f6a33", 0.35);
  let teal = Ink("Teal", "#274e13", "#060f02", 0.35);
  let sherwood_green = Ink("Sherwood Green", "#337239", "#194D19", 0.35);
  let evergreen = Ink("Evergreen", "#4D6633", "#263319", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let turquoise = Ink("Turquoise", "#00B4E6", "#005A8C", 0.35);
  let beau_blue = Ink("Beau Blue", "#0AD", "#058", 0.35);
  let sargasso_sea = Ink("Sargasso Sea", "#162695", "#111962", 0.35);
  let misty_blue = Ink("Misty Blue", "#1e59ad", "#0d3772", 0.35); // TODO unclear hex
  let aurora_borealis = Ink("Aurora Borealis", "#009999", "#004D66", 0.35);
  let pumpkin = Ink("Pumpkin", "#FF8033", "#E54D00", 0.35);
  let sepia = Ink("Sepia", "#ce7e00", "#986719", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let poppy_red = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
  let red_dragon = Ink("Red Dragon", "#9e061a", "#5b0a14", 0.35);
  let writers_blood = Ink("Writer's Blood", "#890f0f", "#572807", 0.35);
  let fire_and_ice = Ink("Fire And Ice", "#00BEDE", "#006478", 0.35);
  let bloody_brexit = Ink("Bloody Brexit", "#05206B", "#2E0033", 0.35);
  let skulls_and_roses = Ink("Skulls and Roses", "#05206B", "#2E0033", 0.35);
  let moonstone = Ink("Moonstone", "#bbb", "#ddd", 0.35);
  let spring_green = Ink("Spring Green", "#7d9900", "#6c6b00", 0.35);
  let indigo = Ink("Indigo", "#667599", "#334D66", 0.35);
  let midnight_sapphire = Ink("Midnight Sapphire", "#6c80b8", "#39394d", 0.35);
  let prussian_blue = Ink("Prussian Blue", "#5e7fa0", "#324d71", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);

  let paper = Paper("White", "#fff", false);

  let mut rng = rng_from_seed(opts.seed);

  let inks = vec![
    inaho,
    evergreen,
    amber,
    spring_green,
    pumpkin,
    soft_mint,
    poppy_red,
  ];

  let perlin = Perlin::new();

  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut routes = Vec::new();

  let rot = PI / rng.gen_range(0.5, 2.0);
  let precision = 0.35;
  let step = 0.5;
  let straight = rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
  let count = 20000;
  let min_l = 10;
  let max_l = 60;
  let decrease_value = 1.0;
  let search_max = 500;
  let min_weight = 1.0;
  let mut bail_out = 0;

  let mut map = WeightMap::new(width, height, precision);

  let max_density = 2.0;
  let distfactor = rng.gen_range(0.0, 4.0);
  let valueadd = rng.gen_range(0.2, 0.9);
  let amp = rng.gen_range(2.2, 4.0);
  let f1 = rng.gen_range(2.0, 12.0);
  let amp1 = rng.gen_range(0.5, 2.0);
  let f2 = rng.gen_range(4.0, 8.0);
  let amp2 = rng.gen_range(0.2, 2.0);
  let f3 = rng.gen_range(8.0, 36.0);
  let amp3 = rng.gen_range(0.1, 2.0);
  let f4 = rng.gen_range(4.0, 20.0);

  map.fill_fn(&mut rng, &mut |(x, y): (f64, f64), rng| {
    if x < pad || x > width - pad || y < pad || y > height - pad {
      return (0, 0.0);
    }

    let n1 = amp
      * (perlin.get([
        f1 * x / height as f64,
        f1 * y / height as f64,
        opts.seed
          + amp1
            * perlin.get([
              f1 * 2.0 * x / height as f64,
              f1 * 2.0 * y / height as f64,
              66.6
                + 5.555 * opts.seed
                + 2.0
                  * perlin.get([
                    f1 * 4.0 * x / height as f64,
                    f1 * 4.0 * y / height as f64,
                    777. + opts.seed / 0.177,
                  ])
                + amp2
                  * perlin.get([
                    f2 * x / height as f64,
                    f2 * y / height as f64,
                    77.75
                      + 8.8 * opts.seed
                      + amp3
                        * perlin.get([
                          f3 * x / height as f64,
                          f3 * y / height as f64,
                          opts.seed / 0.17 + 3.0,
                        ]),
                  ]),
            ]),
      ]));

    let dx = x - width as f64 / 2.0;
    let dy = y - height as f64 / 2.0;
    let d = (dx * dx + dy * dy).sqrt();
    let distf = (d / height).min(1.0);

    let inksf = inks.len() as f64;

    let value: f64 = rng.gen_range(0.0, 1.0)
      + 0.5
        * inksf
        * (valueadd - distfactor * distf + mix(n1, 0.0, distf.powf(2.0)));

    let clr = value.floor().max(0.0).min(inksf - 1.) as usize;

    let density = max_density;

    (clr, density)
  });

  let noisemul = rng.gen_range(1.0, PI);

  for _i in 0..count {
    let top = map.search_weight_top(&mut rng, search_max, min_weight);
    if top.is_none() {
      bail_out += 1;
      if bail_out > 10 {
        break;
      }
    }
    if let Some(o) = top {
      let ink_i = map.get_color(o);

      let angle = noisemul
        * perlin.get([
          opts.seed,
          f4 * o.0 / height as f64,
          f4 * o.1 / height as f64,
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
          routes.push((ink_i, rt));
        }
      }
    }
  }

  let layers = inks
    .iter()
    .enumerate()
    .map(|(ci, &c)| {
      let ink = c;
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

#[derive(Clone, Copy)]
pub struct Ink(&'static str, &'static str, &'static str, f64);
#[derive(Clone, Copy)]
pub struct Paper(&'static str, &'static str, bool);

struct WeightMap {
  weights: Vec<f64>,
  colors: Vec<usize>,
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
      colors: vec![0; w * h],
      w,
      h,
      width,
      height,
      precision,
    }
  }
  fn fill_fn<R: Rng>(
    &mut self,
    rng: &mut R,
    f: &mut impl Fn((f64, f64), &mut R) -> (usize, f64),
  ) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p, rng);
        self.weights[y * self.w + x] = v.1;
        self.colors[y * self.w + x] = v.0;
      }
    }
  }

  fn get_color(&self, p: (f64, f64)) -> usize {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    self.colors[y0 * self.w + x0]
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
  #[clap(short, long, default_value = "65.0")]
  pub seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let document = art(&opts);
  svg::save(opts.file, &document).unwrap();
}
