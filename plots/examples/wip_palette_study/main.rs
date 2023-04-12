use clap::*;
use gre::*;
use image::io::Reader as ImageReader;
use image::RgbaImage;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> svg::Document {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  // possible colors

  let black = Ink("Black", "#1A1A1A", "#000000", 0.35);
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let inaho = Ink("iroshizuku ina-ho", "#ba6", "#7f6a33", 0.35);
  let imperial_purple = Ink("Imperial Purple", "#4D0066", "#260F33", 0.35);
  let amazing_amethyst = Ink("Amazing Amethyst", "#8b008b", "#550055", 0.35);
  let claret = Ink("Claret", "#808", "#606", 0.35);
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
  let hope_pink = Ink("Hope Pink", "#fc839b", "#E53399", 0.35);
  let sepia = Ink("Sepia", "#ce7e00", "#986719", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let yellow = Ink("Yellow", "#FF0", "#FF0", 0.35);
  let brillant_red = Ink("Brillant Red", "#F22", "#912", 0.35);
  let poppy_red = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
  let red_dragon = Ink("Red Dragon", "#9e061a", "#5b0a14", 0.35);
  let writers_blood = Ink("Writer's Blood", "#890f0f", "#572807", 0.35);
  let fire_and_ice = Ink("Fire And Ice", "#00BEDE", "#006478", 0.35);
  let bloody_brexit = Ink("Bloody Brexit", "#05206B", "#2E0033", 0.35);
  let skulls_and_roses = Ink("Skulls and Roses", "#05206B", "#2E0033", 0.35);
  let violet = Ink("Violet", "#c3f", "#92c", 0.35);
  let moonstone = Ink("Moonstone", "#bbb", "#ddd", 0.35);
  let spring_green = Ink("Spring Green", "#7d9900", "#6c6b00", 0.35);
  let indigo = Ink("Indigo", "#667599", "#334D66", 0.35);
  let midnight_sapphire = Ink("Midnight Sapphire", "#6c80b8", "#39394d", 0.35);
  let prussian_blue = Ink("Prussian Blue", "#2f5a98", "#294061", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);
  let flamingo_pink = Ink("Flamingo Pink", "#f78", "#E64", 0.35);

  let white_paper = Paper("White", "#fff", false);
  let paper = white_paper;

  let inks = vec![
    teal,
    sherwood_green,
    evergreen,
    spring_green,
    inaho,
    yellow,
    amber,
    sepia,
    pumpkin,
    brillant_red,
    poppy_red,
    red_dragon,
    writers_blood,
    pink,
    hope_pink,
    flamingo_pink,
    violet,
    amazing_amethyst,
    claret,
    imperial_purple,
    misty_blue,
    sargasso_sea,
    seibokublue,
    aurora_borealis,
    soft_mint,
    beau_blue,
    turquoise,
    fire_and_ice,
    moonstone,
    indigo,
    midnight_sapphire,
    prussian_blue,
    skulls_and_roses,
    bloody_brexit,
    black,
  ];

  let palettes = vec![
    vec![black, poppy_red, amber],
    vec![black, pumpkin, yellow],
    vec![writers_blood, sepia, amber],
    vec![aurora_borealis, hope_pink, amber],
    vec![black, amazing_amethyst, violet],
    vec![imperial_purple, violet, hope_pink],
    vec![black, red_dragon, pumpkin],
    vec![black, prussian_blue, pink],
    vec![teal, soft_mint, flamingo_pink],
    vec![black, aurora_borealis, brillant_red],
    vec![bloody_brexit, claret, midnight_sapphire],
    vec![black, poppy_red, beau_blue],
    vec![bloody_brexit, fire_and_ice],
    vec![black, misty_blue, turquoise],
    vec![sherwood_green, aurora_borealis, spring_green],
    vec![black, sherwood_green, soft_mint],
    vec![teal, evergreen, inaho],
    vec![seibokublue, inaho, moonstone],
    vec![skulls_and_roses, indigo, moonstone],
    vec![bloody_brexit, sargasso_sea],
    vec![black],
  ];

  let mut map = WeightMap::new(width, height, 0.4);

  let max_density = 10.0;

  let ww = width * 0.7;
  let pp = 1.0;

  map.fill_fn(|(x, y)| {
    if x <= pad || x >= width - pad || y <= pad + 0.1 || y >= height - pad - 0.1
    {
      return 0.0;
    }
    if x >= ww - pp {
      return 0.0;
    }
    let yf = palettes.len() as f64 * (y - pad) / (height - 2.0 * pad);
    let yp = yf % 1.0;
    if yp > 0.9 {
      return 0.0;
    }
    let v = mix(0.66, 1.0, ((yp - 0.45).abs() / 0.45).powf(0.5));
    return max_density * v * (x - pad) / (ww - pad - pp);
  });

  let rot = PI / rng.gen_range(1.0, 4.0);
  let step = 0.8;
  let straight = 0.1;
  let count = 50000;
  let min_l = 5;
  let max_l = 30;
  let decrease_value = 1.0;
  let search_max = 500;
  let min_weight = 1.0;
  let mut bail_out = 0;

  let mut routes = Vec::new();

  for _i in 0..count {
    let top = map.search_weight_top(&mut rng, search_max, min_weight);
    if top.is_none() {
      bail_out += 1;
      if bail_out > 10 {
        break;
      }
    }
    if let Some(o) = top {
      let angle = rng.gen_range(0.0, 2.0 * PI);

      if let Some(a) = map.best_direction(o, step, angle, PI, PI / 4.0, 0.0) {
        let v = map.get_weight(o);
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
          let palindex = (palettes.len() as f64 * (o.1 - pad)
            / (height - 2.0 * pad))
            .floor() as usize;
          let palinks = &palettes[palindex % palettes.len()];
          let clrindex =
            ((1.0 - (v / max_density)) * (palinks.len() as f64)) as usize;
          let clr = inks
            .iter()
            .position(|&ink| ink == palinks[clrindex % palinks.len()])
            .unwrap();
          routes.push((clr, rt));
        }
      }
    }
  }

  let w = (height - 2.0 * pad) / (inks.len() as f64);
  for i in 0..inks.len() {
    let x1 = ww;
    let x2 = x1 + w;
    let y1 = pad + (i as f64) * w;
    let y2 = y1 + w;
    // square spiral
    let center = ((x1 + x2) / 2.0, (y1 + y2) / 2.0);
    let r = w * (2.0f64).sqrt() / 2.0 - 0.5 * pp;
    let initial_a = 0.0;
    let d_length = 0.5;
    let d_length_to = 0.1;
    let route = square_spiral(center, r, initial_a, d_length, d_length_to);
    routes.push((i, route));
    for xi in 0..8 {
      routes.push((
        i,
        vec![
          (x2 + xi as f64 * 0.5, y1 + 0.3),
          (x2 + xi as f64 * 0.5, y2 - 0.3),
        ],
      ));
    }
  }

  /*
  routes.push((
    0,
    vec![
      (pad, pad),
      (width - pad, pad),
      (width - pad, height - pad),
      (pad, height - pad),
      (pad, pad),
    ],
  ));
  */

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

#[derive(Clone, Copy, PartialEq)]
pub struct Ink(&'static str, &'static str, &'static str, f64);
#[derive(Clone, Copy)]
pub struct Paper(&'static str, &'static str, bool);

pub struct ImageTexture {
  img: RgbaImage,
  width: u32,
  height: u32,
}

impl ImageTexture {
  pub fn from_file(path: &str) -> Result<Self, image::ImageError> {
    let img = ImageReader::open(path)?.decode()?.to_rgba8();
    let (width, height) = img.dimensions();
    Ok(Self { img, width, height })
  }

  pub fn get_color(&self, point: (f64, f64)) -> (f64, f64, f64) {
    let (x, y) = point;
    let xi: f64 = x.max(0.0).min(1.0) * ((self.width - 1) as f64);
    let yi: f64 = y.max(0.0).min(1.0) * ((self.height - 1) as f64);
    let x1 = xi.floor() as u32;
    let x2 = xi.ceil() as u32;
    let y1 = yi.floor() as u32;
    let y2 = yi.ceil() as u32;
    let p1 = self.img.get_pixel(x1, y1);
    let p2 = self.img.get_pixel(x2, y1);
    let p3 = self.img.get_pixel(x2, y2);
    let p4 = self.img.get_pixel(x1, y2);
    let xp = xi - xi.floor();
    let yp = yi - yi.floor();
    let r = (Self::mix(
      p1[0] as f64,
      p2[0] as f64,
      xp,
      p4[0] as f64,
      p3[0] as f64,
      yp,
    )) / 255.0;
    let g = (Self::mix(
      p1[1] as f64,
      p2[1] as f64,
      xp,
      p4[1] as f64,
      p3[1] as f64,
      yp,
    )) / 255.0;
    let b = (Self::mix(
      p1[2] as f64,
      p2[2] as f64,
      xp,
      p4[2] as f64,
      p3[2] as f64,
      yp,
    )) / 255.0;
    (r, g, b)
  }

  fn mix(a: f64, b: f64, x: f64, c: f64, d: f64, y: f64) -> f64 {
    (1.0 - y) * ((1.0 - x) * a + x * b) + y * ((1.0 - x) * c + x * d)
  }
}

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
  fn fill_fn(&mut self, f: impl Fn((f64, f64)) -> f64) {
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

fn square_spiral(
  c: (f64, f64),
  r: f64,
  initial_a: f64,
  d_length: f64,
  d_length_to: f64,
) -> Vec<(f64, f64)> {
  let mut a: f64 = initial_a;
  let length = r * 2. / (2. as f64).sqrt();
  let delta = p_r((-length / 2., length / 2.), a);
  let mut p = (c.0 + delta.0, c.1 + delta.1);
  let mut l = length;
  let mut i = 0;
  let mut route = Vec::new();
  loop {
    if l < 0.0 {
      break;
    }
    p = (p.0 + l * a.cos(), p.1 + l * a.sin());
    route.push(p);
    a -= PI / 2.;
    if i > 0 {
      let dl = mix(d_length_to, d_length, l / length);
      l -= dl;
    }
    i += 1;
  }
  route
}
