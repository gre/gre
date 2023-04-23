use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::collections::VecDeque;
use std::f64::consts::PI;
use svg::node::element::path::Data;

fn art(opts: &Opts) -> svg::Document {
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let sherwood_green = Ink("Sherwood Green", "#337239", "#194D19", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let turquoise = Ink("Turquoise", "#00B4E6", "#005A8C", 0.35);
  let aurora_borealis = Ink("Aurora Borealis", "#009999", "#004D66", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let moonstone = Ink("Moonstone", "#bbb", "#ddd", 0.35);
  let spring_green = Ink("Spring Green", "#7d9900", "#6c6b00", 0.35);

  let paper = Paper("White", "#fff", false);

  let mut rng = rng_from_seed(opts.seed);

  let inks_water = vec![
    (seibokublue, 1.0),
    (turquoise, 1.0),
    (soft_mint, 1.0),
    (aurora_borealis, 1.0),
  ];

  let inks_ground = vec![
    (moonstone, 1.0),
    (amber, 1.0),
    (spring_green, 1.0),
    (sherwood_green, 1.0),
  ];
  let pad1 = 2.0;
  let pad2 = 2.0;
  let pad3 = 2.0;

  let inks = vec![inks_water.clone(), inks_ground.clone()].concat();

  let get_color = image_get_color("images/frioul.jpg").unwrap();

  let perlin = Perlin::new();

  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut routes = Vec::new();

  let rot = PI / rng.gen_range(0.8, 3.0);
  let precision = 0.35;
  let step = 0.4;
  let straight = rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
  let count = 20000;
  let min_l = rng.gen_range(4, 8);
  let max_l = rng.gen_range(min_l, 100);
  let island_max_l = 12;
  let decrease_value = 1.0;
  let search_max = 500;
  let min_weight = 1.0;
  let mut bail_out = 0;

  let mut map = WeightMap::new(width, height, precision);

  let max_density = 2.5;
  let distfactor = 1.0;
  let valueadd = 0.7;
  // let sea_randomness = 0.8;
  let sea_randomness = rng.gen_range(0.8, 0.9);
  let island_randomness = 0.2;
  let amp = rng.gen_range(1.0, 10.0);
  let f1 = rng.gen_range(1., 10.);
  let amp1 = rng.gen_range(0.3, 1.2);
  let f2 = rng.gen_range(2.0, 20.0);
  let amp2 = rng.gen_range(0.5, 1.0);
  let f3 = rng.gen_range(8.0, 36.0);
  let amp3 = rng.gen_range(0.5, 2.0);
  let f4 = rng.gen_range(4.0, 5.0);

  let z = 1.1;
  let ratio = 1.2;

  let mut passage = Passage::new(precision, width, height);
  let mut revpassagelayer2 = Passage::new(precision, width, height);
  let mut x = 0.0;
  while x < width {
    let mut y = 0.0;
    while y < height {
      let p = (x / width, y / height);
      let p = (p.0 - 0.5, p.1 - 0.5);
      let p = (p.0 * z * ratio, p.1 * z);
      let p = (p.0 + 0.5, p.1 + 0.5);
      if grayscale(get_color(p)) > 0.1 {
        passage.count((x, y));
      } else {
        revpassagelayer2.count((x, y));
      }
      y += precision;
    }
    x += precision;
  }
  passage.grow_passage(pad1);

  revpassagelayer2.grow_passage(pad2);
  let mut revpassagelayer3 = revpassagelayer2.clone();
  revpassagelayer3.grow_passage(pad3);

  map.fill_fn(&mut rng, &mut |(x, y): (f64, f64), rng| {
    if x < pad || x > width - pad || y < pad || y > height - pad {
      return (0, 0.0);
    }

    let p = (x / width, y / height);
    let p = (p.0 - 0.5, p.1 - 0.5);
    let p = (p.0 * z * ratio, p.1 * z);
    let p = (p.0 + 0.5, p.1 + 0.5);

    let clr = get_color(p);

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
                  ]),
            ]),
      ]) + amp2
        * perlin.get([
          f2 * x / height as f64,
          f2 * y / height as f64,
          77.75 + 8.8 * opts.seed,
        ])
        + amp3
          * perlin
            .get([
              f3 * x / height as f64,
              f3 * y / height as f64,
              opts.seed / 0.17 + 3.0,
            ])
            .abs());

    let dx = x - width as f64 / 2.0;
    let dy = y - height as f64 / 2.0;
    let d = (dx * dx + dy * dy).sqrt();
    let distf = (d / height).min(1.0);

    let gs = grayscale(clr);

    let value;

    if gs < 0.05 {
      if passage.get((x, y)) > 0 {
        return (0, 0.0);
      } else {
        value = rng.gen_range(0.0, 1.0)
          + 0.5
            * (inks_water.len() - 1) as f64
            * (valueadd - distfactor * distf
              + mix(
                mix(n1, rng.gen_range(0.0, 1.0), sea_randomness),
                0.0,
                distf.powf(2.0),
              ))
            .max(0.0)
            .min(1.0);
      }
    } else {
      let v = if revpassagelayer3.get((x, y)) == 0 {
        2
      } else if revpassagelayer2.get((x, y)) == 0 {
        1
      } else {
        0
      };
      value = inks_water.len() as f64
        + mix(
          v as f64,
          rng.gen_range(0.0, inks_ground.len() as f64),
          island_randomness,
        );
    }

    let clr = value.floor().max(0.0).min(inks.len() as f64 - 1.) as usize;

    let density = max_density * inks[clr].1;

    (clr, density)
  });

  {
    // add text
    let letters_ref: letters::LetterSvgReferential =
      gre::letters::LetterSvgReferential::new(
        "images/letters.svg".to_string(),
        0.1,
        1.0,
      );

    let grow = 6.0;
    let precision = 0.4;
    let breaking_angle_factor = 1.5;
    let size = 3.3;
    let word_dist = 1.2;

    let w = (width as f64 / precision) as u32;
    let h = (height as f64 / precision) as u32;

    passage.grow_passage(grow);

    let f = |p: (f64, f64)| {
      let g = (p.0 * width, p.1 * height);
      // actively crop the outside part out
      if g.0 < pad || g.0 > width - pad || g.1 < pad || g.1 > height - pad {
        return 1.;
      }
      passage.get(g) as f64
    };

    // we use marching square on the current value map to infer contours
    let features = contour(w, h, &f, &vec![0.5]);
    let mut rts = features_to_routes(features, precision);
    rts = crop_routes(&rts, (pad, pad, width - pad, height - pad));

    // for each contouring, we convert them into curves
    // - rdp to avoid too much zig zag problems
    // - slicing the paths into smaller pieces to avoid sharp edges cases
    // both of these are needed for the word readability
    let mut curves = Vec::new();
    for route in rts.clone() {
      curves.extend(slice_on_sharp_edges(
        &rdp(&route, precision),
        breaking_angle_factor,
      ));
    }
    // offset text exactly on the curve line
    let yoffset = -size * 0.7;
    // passage is used to do collision and avoid two lines to collide too mcuh
    let passage_precision = 0.2 * size;
    let mut passage = Passage::new(passage_precision, width, height);

    let mut text_routes = vec![];

    let mut words = VecDeque::new();
    for word in opts.sentence.split(" ") {
      words.push_back(word.to_string());
    }

    for c in curves.clone() {
      let length = curve_length(&c);
      let extrapad = word_dist;
      let mut subset = vec![];
      let mut sum = extrapad;
      let mut sum_words = 0.0;
      // we try to pull as much word as we can to fill the curve
      while let Some(word) = words.pop_front() {
        let text = word.clone();
        let measure = measure_text(&letters_ref, text.clone(), size);
        if sum + measure + word_dist < length {
          sum += measure + word_dist;
          sum_words += measure;
          subset.push(text);
          words.push_back(word.clone());
        } else {
          words.push_front(word.clone());
          break;
        }
      }
      if subset.len() == 0 {
        continue;
      }
      // we will equilibrate the padding for all the words to smoothly occupy the curve
      let pad = (length - sum_words) / (subset.len() as f64);
      let mut xstart = 0.0;
      for text in subset {
        xstart += pad / 2.0;
        let res =
          draw_text(&letters_ref, text.clone(), size, xstart, yoffset, &c);

        // check collision and record where the strokes are

        let mut collides = false;
        for r in res.0.clone() {
          for p in r.clone() {
            if passage.get(p) > 0 {
              collides = true;
              break;
            }
          }
        }
        for r in res.0.clone() {
          for p in r.clone() {
            map.decrease_weight_gaussian(p, 1.0, 1.0);
            passage.count_once(p);
          }
        }

        // we only use the word if it was not colliding
        if !collides {
          text_routes.extend(res.0.clone());
        } else {
          words.push_front(text.clone());
        }
        xstart += res.1 + pad / 2.0;
      }
    }

    routes.extend(
      text_routes
        .iter()
        .map(|r| (1, r.clone()))
        .collect::<Vec<_>>(),
    );
  }

  let noisemul = rng.gen_range(0.0, PI);

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

      // let angle = rng.gen_range(-PI, PI);

      let ml = if ink_i >= inks_water.len() {
        island_max_l
      } else {
        max_l
      };

      if let Some(a) = map.best_direction(o, step, angle, PI, PI / 4.0, 0.0) {
        let route =
          map.dig_random_route(o, a, step, rot, straight, ml, decrease_value);
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
      let ink = c.0;
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
  #[clap(short, long, default_value = "l")]
  pub sentence: String,
}

fn main() {
  let opts: Opts = Opts::parse();
  let document = art(&opts);
  svg::save(opts.file, &document).unwrap();
}

pub fn slice_on_sharp_edges(
  path: &Vec<(f64, f64)>,
  max_angle: f64,
) -> Vec<Vec<(f64, f64)>> {
  if path.len() < 3 {
    return vec![path.clone()];
  }
  let mut slices = Vec::new();
  let mut slice = Vec::new();
  let (x1, y1) = path[0];
  let (x2, y2) = path[1];
  let mut last_angle = (y2 - y1).atan2(x2 - x1);
  slice.push(path[0]);
  let mut prev = path[0];
  for &p in path.iter().skip(1) {
    let (x1, y1) = prev;
    let (x2, y2) = p;
    let angle = (y2 - y1).atan2(x2 - x1);
    let diff = angle_delta(angle, last_angle);
    if diff.abs() > max_angle {
      if slice.len() > 1 {
        slices.push(slice);
      }
      slice = vec![prev, p];
    } else {
      slice.push(p);
    }
    last_angle = angle;
    prev = p;
  }
  if slice.len() > 1 {
    slices.push(slice);
  }
  slices
}

pub fn draw_text(
  letter_ref: &gre::letters::LetterSvgReferential,
  text: String,           // text to draw
  size: f64,              // font size
  xstart: f64,            // x move on the path
  yoffset: f64,           // make diff baseline
  path: &Vec<(f64, f64)>, // curve to follow
) -> (Vec<Vec<(f64, f64)>>, f64) {
  let mut routes = Vec::new();
  let mut x = 0.;
  let mut y = 0.;
  let mut can_attach = true;
  let mut last: Vec<(f64, f64)> = vec![];
  for c in text.chars() {
    if let Some(letter) = letter_ref.get_letter(&c.to_string()) {
      let (rts, (dx, dy)) = letter.render((x, y), size, false);
      if letter.can_attach && can_attach {
        let mut rts = rts.clone();

        let mut add = rts.pop().unwrap();
        // interpolate curve to attach more smoothly
        if last.len() > 0 {
          let lastp = last[last.len() - 1];
          let firstp = add[0];
          // ygap between last and first
          let ygap = firstp.1 - lastp.1;
          let mut i = 1;
          let mut maxlen = 0.5 * size;
          while i < add.len() {
            if maxlen < 0. {
              break;
            }
            let l = euclidian_dist(add[i - 1], add[i]);
            if ygap > 0.0 {
              if add[i].1 < lastp.1 {
                break;
              }
            } else {
              if add[i].1 > lastp.1 {
                break;
              }
            }
            i += 1;
            maxlen -= l;
          }
          if i == add.len() {
            i -= 1;
          }
          let stopi = i;
          add = add
            .iter()
            .enumerate()
            .map(|(i, &p)| {
              if i <= stopi {
                let y = p.1 - ygap * (1.0 - i as f64 / stopi as f64);
                (p.0, y)
              } else {
                p
              }
            })
            .collect();
        }

        last.extend(add);

        routes.extend(rts); // ° on i and j
      } else {
        if last.len() > 0 {
          routes.push(last);
          last = vec![];
        }
        routes.extend(rts);
      }
      can_attach = letter.can_attach;
      x += dx;
      y += dy;
    } else {
      println!("letter not found: {}", c);
    }
  }
  if last.len() > 0 {
    routes.push(last);
  }

  // rotate with angle and translate to origin all routes
  let mut proj_routes = Vec::new();
  for route in routes {
    let mut proj_route = Vec::new();
    for (x, y) in route {
      // use x to find position in path and project x,y
      let (origin, a) = lookup_curve_point_and_angle(&path, x + xstart);

      let y = y + yoffset;
      let disp = (-y * a.sin(), y * a.cos());

      let p = (origin.0 + disp.0, origin.1 + disp.1);

      proj_route.push(p);
    }
    proj_routes.push(proj_route);
  }

  (proj_routes, x)
}

fn angle2(p1: (f64, f64), p2: (f64, f64)) -> f64 {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  let dx = x2 - x1;
  let dy = y2 - y1;
  dy.atan2(dx)
}

fn curve_length(path: &Vec<(f64, f64)>) -> f64 {
  let mut len = 0.0;
  for i in 0..path.len() - 1 {
    len += euclidian_dist(path[i], path[i + 1]);
  }
  len
}

fn measure_text(
  letter_ref: &gre::letters::LetterSvgReferential,
  text: String,
  size: f64,
) -> f64 {
  let mut x = 0.;
  for c in text.chars() {
    if let Some(letter) = letter_ref.get_letter(&c.to_string()) {
      let (dx, _dy) = letter.render((x, 0.0), size, false).1;
      x += dx;
    }
  }
  x
}

fn lookup_curve_point_and_angle(
  path: &Vec<(f64, f64)>,
  l: f64,
) -> ((f64, f64), f64) {
  let mut i = 0;
  if l < 0.0 {
    return (path[0], angle2(path[0], path[1]));
  }
  let mut len = 0.0;
  while i < path.len() - 1 {
    let l1 = euclidian_dist(path[i], path[i + 1]);
    if len + l1 > l {
      let r = (l - len) / l1;
      let x = path[i].0 + r * (path[i + 1].0 - path[i].0);
      let y = path[i].1 + r * (path[i + 1].1 - path[i].1);
      let angle = angle2(path[i], path[i + 1]);
      return ((x, y), angle);
    }
    len += l1;
    i += 1;
  }
  return (
    path[path.len() - 1],
    angle2(path[path.len() - 2], path[path.len() - 1]),
  );
}

pub fn abs_angle(a: f64) -> f64 {
  ((2.0 * PI) + (a % (2.0 * PI))) % (2.0 * PI)
}
pub fn angle_delta(a: f64, b: f64) -> f64 {
  let delta = (abs_angle(a) - abs_angle(b)).abs();
  let sign = if abs_angle(a) > abs_angle(b) || delta >= PI {
    -1.0
  } else {
    1.0
  };
  (PI - (delta - PI).abs()) * sign
}
