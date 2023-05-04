use clap::*;
use gre::*;
use image::io::Reader as ImageReader;
use image::RgbaImage;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use std::fs;
use svg::node::element::path::Data;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "/Users/gre/Downloads/PHOTOS")]
  pub photos_folder: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "15.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "260.0")]
  pub seed: f64,
}

#[derive(Clone, Copy)]
pub enum Wrapping {
  Repeat,
  RepeatMirror,
  Clamp,
  ClampRepeat(f64),
  None,
}

fn art(opts: &Opts) -> svg::Document {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  // possible colors

  let black = Ink("Black", "#1A1A1A", "#000000", 0.35);

  let gold_gel = Ink("Gold Gel", "#D8B240", "#FFE38C", 0.6);
  let silver_gel = Ink("Silver Gel", "#CCCCCC", "#FFFFFF", 0.6);
  let white_gel = Ink("White Gel", "#E5E5E5", "#FFFFFF", 0.35);
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let inaho = Ink("iroshizuku ina-ho", "#ba6", "#7f6a33", 0.35);
  let imperial_purple = Ink("Imperial Purple", "#4D0066", "#260F33", 0.35);
  let sherwood_green = Ink("Sherwood Green", "#337239", "#194D19", 0.35);
  let evergreen = Ink("Evergreen", "#4D6633", "#263319", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let turquoise = Ink("Turquoise Ink", "#00B4E6", "#005A8C", 0.35);
  let sargasso_sea = Ink("Sargasso Sea", "#162695", "#111962", 0.35);
  let aurora_borealis = Ink("Aurora Borealis", "#009999", "#004D66", 0.35);
  let pumpkin = Ink("Pumpkin", "#FF8033", "#E54D00", 0.35);
  let hope_pink = Ink("Hope Pink", "#fc839b", "#E53399", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let poppy_red = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
  let moonstone = Ink("Moonstone", "#bbb", "#ddd", 0.35);
  let red_dragon = Ink("Red Dragon", "#9e061a", "#5b0a14", 0.35);
  let fire_and_ice = Ink("Fire And Ice", "#00BEDE", "#006478", 0.35);
  let bloody_brexit = Ink("Bloody Brexit", "#05206B", "#2E0033", 0.35);

  let indigo = Ink("Indigo", "#667599", "#334D66", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);

  let white_paper = Paper("White", "#fff", false);

  let red_paper = Paper("Red", "#aa0000", true);
  let darkblue_paper = Paper("Dark Blue", "#202040", true);
  let black_paper = Paper("Black", "#202020", true);
  let grey_paper = Paper("Grey", "#959fa8", true);

  let white_on_paper = false;
  let paper = white_paper;
  let inks = vec![black, moonstone];

  let perlin = Perlin::new();

  let images = find_images(opts.photos_folder.as_str());

  let mut map = ValueMap::new(width, height, 0.4);

  let count_imgs =
    (1.0 + rng.gen_range(0f64, 2.0) * rng.gen_range(0.0, 1.0)).round() as usize;
  let paths = pick_random_images(&images, count_imgs, &mut rng);

  println!("paths: {:?}", paths);

  let images: Vec<ImageTexture> = paths
    .iter()
    .map(|src| ImageTexture::from_file(src).unwrap())
    .collect();

  let max_density = 8.0;

  let collages = vec![(
    // index
    0,
    // offset away from center
    (0.0, 0.0),
    // scale
    4.0,
    // rotate
    -0.2,
    // repeat
    (Wrapping::RepeatMirror, Wrapping::ClampRepeat(0.2)),
    // density of strokes
    max_density,
  )];

  // TODO crop in
  // TODO radial repeat
  // TODO gradient value map, to compose two images together

  map.fill_fn(|(x, y)| {
    if x < pad || x > width - pad || y < pad || y > height - pad {
      return 0.0;
    }

    let mut sum: f64 = 0.0;

    for (index, v, s, r, wrapping, multiplier) in collages.clone() {
      let img = &images[index];
      let ratio = img.width as f64 / img.height as f64;

      let p = (x / opts.width, y / opts.height);
      let p = (p.0 - 0.5, p.1 - 0.5); // image transaction apply on center
      let p = rotate(p, r); // image rotation
      let p = (p.0 / ratio, p.1); // respect the ratio of the image
      let p = (p.0 * s, p.1 * s); // image scale
      let p = (p.0 + 0.5, p.1 + 0.5); // move back to canvas center
      let p = (p.0 - v.0, p.1 - v.1); // translate the image
      let p = apply_wrapping(p, wrapping);

      // protect from out of bounds cases
      if p.0 < 0.0 || p.0 > 1.0 || p.1 < 0.0 || p.1 > 1.0 {
        continue;
      }

      let c = img.get_color(p);
      // we use grayscale to get a value between 0.0 and 1.0 and normalize the intensity
      // TODO: smarter way to normalize?
      let mut value = grayscale(c);
      if !white_on_paper {
        value = 1.0 - value;
        //value = value.powf(2.0);
        value = value * 1.2 - 0.1;
      } else {
        value = value.powf(2.0);
      }
      //let value = value.powf(1.5);
      let value = multiplier * value;
      sum += value;
    }

    sum.min(max_density)
  });

  let rot = PI / rng.gen_range(1.0, 4.0);
  let step = 0.5;
  let straight = 0.1;
  let count = 20000;
  let min_l = 5;
  let max_l = 40;
  let decrease_value = 1.0;
  let search_max = 500;
  let min_weight = 1.0;
  let mut bail_out = 0;

  let mut routes = Vec::new();

  for _i in 0..count {
    let top = map.search_highest_point(&mut rng, search_max, min_weight);
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

      if let Some(a) =
        map.search_highest_direction(o, step, angle, PI, PI / 4.0, 0.0)
      {
        let v = map.get(o);
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
          let clr = ((1.0 - (v / max_density)) * (inks.len() as f64)) as usize;
          routes.push((clr, rt));
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

fn find_images(dir_path: &str) -> Vec<String> {
  let mut images = Vec::new();
  match fs::read_dir(dir_path) {
    Ok(dir_entries) => {
      for entry in dir_entries {
        if let Ok(entry) = entry {
          let path = entry.path();
          if path.is_dir() {
            images.append(&mut find_images(path.to_str().unwrap()));
          } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_str().unwrap().to_lowercase();
            if ext_str == "jpg" || ext_str == "jpeg" || ext_str == "png" {
              images.push(path.to_str().unwrap().to_owned());
            }
          }
        }
      }
    }
    Err(_) => {}
  }
  images
}

#[derive(Clone, Copy)]
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

/**
 * A general utility 2D value map.
 */
struct ValueMap {
  values: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl ValueMap {
  fn new(width: f64, height: f64, precision: f64) -> ValueMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let values = vec![0.0; w * h];
    ValueMap {
      values,
      w,
      h,
      width,
      height,
      precision,
    }
  }

  // initialize the map with a function
  fn fill_fn(&mut self, f: impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.values[y * self.w + x] = v;
      }
    }
  }

  // add values to the map with a function
  fn add_fn(&mut self, f: impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.values[y * self.w + x] += v;
      }
    }
  }

  // do a simple bilinear interpolation
  fn get(&self, p: (f64, f64)) -> f64 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let w00 = self.values[y0 * self.w + x0];
    let w01 = self.values[y0 * self.w + x1];
    let w10 = self.values[y1 * self.w + x0];
    let w11 = self.values[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the values around the point p with a given radius
  fn add_gaussian(&mut self, p: (f64, f64), radius: f64, value: f64) {
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
          let w = self.values[y * self.w + x];
          let v = value * (1.0 - d / radius);
          self.values[y * self.w + x] = w + v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  fn search_highest_direction(
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
      let weight = self.get(np) * wmul;
      if weight > best_weight {
        best_weight = weight;
        best_ang = Some(ang);
      }
      a += angle_precision;
    }
    return best_ang;
  }

  // randomly search for a point having a high value in the map
  fn search_highest_point<R: Rng>(
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
      let w = self.get(p);
      if w > best_w {
        best_w = w;
        best_p = Some(p);
      }
    }
    return best_p;
  }

  // dig a route from a given point following the highest values & decrease the values on the travel path
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
      if let Some(ang) = self.search_highest_direction(
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
        self.add_gaussian(prev, step, -decrease_value);
      } else {
        break;
      }
    }

    route
  }
}

fn pick_random_images<'a>(
  paths: &'a [String],
  count_imgs: usize,
  rng: &mut impl rand::Rng,
) -> Vec<&'a String> {
  let mut indices: Vec<usize> = (0..paths.len()).collect();
  rng.shuffle(&mut indices);
  indices
    .into_iter()
    .take(count_imgs)
    .map(|i| &paths[i])
    .collect()
}

pub fn grayscale((r, g, b): (f64, f64, f64)) -> f64 {
  return 0.299 * r + 0.587 * g + 0.114 * b;
}

fn apply_wrapping_value(v: f64, wrapping: Wrapping) -> f64 {
  match wrapping {
    Wrapping::None => v,
    Wrapping::Clamp => v.max(0.0).min(1.0),
    Wrapping::Repeat => (v + 1000.0) % 1.0,
    Wrapping::ClampRepeat(r) => {
      if v < 0.0 || v > 1.0 {
        let v = if v < 0.0 { -v } else { v };
        v % r
      } else {
        v
      }
    }
    Wrapping::RepeatMirror => {
      let m = (v + 1000.0) % 2.0;
      if m > 1.0 {
        2.0 - m
      } else {
        m
      }
    }
  }
}

fn apply_wrapping(p: (f64, f64), wrapping: (Wrapping, Wrapping)) -> (f64, f64) {
  (
    apply_wrapping_value(p.0, wrapping.0),
    apply_wrapping_value(p.1, wrapping.1),
  )
}

fn rotate(point: (f64, f64), angle: f64) -> (f64, f64) {
  let sin_angle = angle.sin();
  let cos_angle = angle.cos();
  (
    point.0 * cos_angle - point.1 * sin_angle,
    point.0 * sin_angle + point.1 * cos_angle,
  )
}
