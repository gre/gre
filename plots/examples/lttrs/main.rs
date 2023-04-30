use clap::*;
use gre::*;
use rand::prelude::*;
use std::collections::VecDeque;
use std::f64::consts::PI;
use svg::node::element::path::Data;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(
    short,
    long,
    default_value = "independence liberty autonomy sovereignty ruler governance determination reliance free will choice flexibility openness diversity tolerance acceptance equality justice fairness democracy rights creativity progress growth expansion exploration adventure courage empowerment release peace joy bliss mindfulness awareness awakening enlightenment truthfulness authenticity honesty integrity accountability responsibility honor respect dignity worthiness appreciation gratitude compassion empathy kindness generosity service altruism forgiveness harmony unity cooperation collaboration solidarity fraternity"
  )]
  pub sentence: String,
}

fn art(opts: &Opts) -> svg::Document {
  let black = Ink("Black", "#1A1A1A", "#000000", 0.35);
  let brillant_red = Ink("Brillant Red", "#F22", "#912", 0.35);
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let turquoise = Ink("Turquoise", "#00B4E6", "#005A8C", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);
  let spring_green = Ink("Spring Green", "#7d9900", "#6c6b00", 0.35);

  let paper = Paper("White", "#fff", false);

  let mut rng = rng_from_seed(opts.seed);

  let inks = vec![
    black,
    seibokublue,
    turquoise,
    soft_mint,
    spring_green,
    amber,
    pink,
    brillant_red,
  ];

  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut routes = Vec::new();

  // add text
  let letters_ref: LetterSvgReferential = LetterSvgReferential::new(
    "/Users/gre/Documents/arthur-letters-1.svg".to_string(),
    0.1,
    0.2,
  );

  let precision = 0.4;
  let size = 7.0;
  let word_dist = 2.0;
  let line_dist = 5.0;

  let mut unsafe_curves = Vec::new();

  if rng.gen_bool(0.9) {
    unsafe_curves.push(spiral_optimized(
      width / 2.,
      height / 2.,
      90.,
      line_dist,
      0.1,
    ));
  } else {
    unsafe_curves.push(spiral_angular(
      PI / 2.0,
      (pad, pad),
      1.0,
      width - 2.0 * pad,
      line_dist,
    ));
  }

  /*
  // for each contouring, we convert them into curves
  // - rdp to avoid too much zig zag problems
  // - slicing the paths into smaller pieces to avoid sharp edges cases
  // both of these are needed for the word readability
  let mut curves = Vec::new();
  for route in unsafe_curves.clone() {
    curves.extend(slice_on_sharp_edges(
      &rdp(&route, precision),
      breaking_angle_factor,
    ));
  }
  */
  let curves = unsafe_curves.clone();

  // offset text exactly on the curve line
  let yoffset = -size * 0.7;
  // passage is used to do collision and avoid two lines to collide too mcuh
  let passage_precision = 0.2 * size;
  let mut passage = Passage::new(passage_precision, width, height);

  let mut words = VecDeque::new();
  for word in opts.sentence.split(" ") {
    words.push_back(word.to_string());
  }

  let mut clr_index = 0;

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

      // for now, disable collision
      /*
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
          passage.count_once(p);
        }
      }
      */

      // we only use the word if it was not colliding
      if !collides {
        routes.extend(
          res
            .0
            .iter()
            .map(|r| (clr_index, r.clone()))
            .collect::<Vec<_>>(),
        );

        if rng.gen_bool(0.3) {
          clr_index = (clr_index + 1) % inks.len();
        }
      } else {
        words.push_front(text.clone());
      }
      xstart += res.1 + pad / 2.0;
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

#[derive(Clone, Copy)]
pub struct Ink(&'static str, &'static str, &'static str, f64);
#[derive(Clone, Copy)]
pub struct Paper(&'static str, &'static str, bool);

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
  letter_ref: &LetterSvgReferential,
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
  letter_ref: &LetterSvgReferential,
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

use std::collections::HashMap;
use svg::node::element::*;
use svg::parser::Event;
use svg::Document;

#[derive(Clone)]
pub struct Letter {
  pub routes: Vec<Vec<(f64, f64)>>,
  pub width: f64,
  pub height: f64,
  pub can_attach: bool,
}
impl Letter {
  pub fn new(
    routes: Vec<Vec<(f64, f64)>>,
    width: f64,
    height: f64,
    can_attach: bool,
  ) -> Letter {
    Letter {
      routes,
      width,
      height,
      can_attach,
    }
  }

  pub fn width_for_size(&self, size: f64) -> f64 {
    self.width * size / self.height
  }

  pub fn render(
    &self,
    (x, y): (f64, f64),
    size: f64,
    // TODO deprecate. userland responsability to rotate something
    vertical: bool,
  ) -> (Vec<Vec<(f64, f64)>>, (f64, f64)) {
    let mut routes = self.routes.clone();
    let w = self.width;
    let h = self.height;
    let ratio = w / h;
    let scale = size / h;

    for route in routes.iter_mut() {
      for p in route.iter_mut() {
        p.0 *= scale;
        p.1 *= scale;
        if vertical {
          *p = (h * scale - p.1, p.0);
        }
        p.0 += x;
        p.1 += y;
      }
    }
    let delta = if vertical {
      (0.0, ratio * size)
    } else {
      (ratio * size, 0.0)
    };
    (routes, delta)
  }
}

#[derive(Clone)]
pub struct LetterSvgReferential {
  letters: HashMap<String, Letter>,
}

impl LetterSvgReferential {
  pub fn new(
    svg_file: String,
    letter_precision: f64,
    non_attached_pad: f64,
  ) -> LetterSvgReferential {
    let mut content = String::new();

    let mut height = 0.0;
    let mut documents_per_char: HashMap<String, String> = HashMap::new();

    for event in svg::open(svg_file, &mut content).unwrap() {
      match event {
        Event::Tag(_, _, attributes) => {
          if let Some(c) = attributes.get("inkscape:label") {
            if let Some(d) = attributes.get("d") {
              let data: String = d.to_string();
              let document =
                Document::new().add(Path::new().set("d", data)).to_string();
              documents_per_char.insert(c.to_string(), document);
            }
          }

          if let Some(h) = attributes.get("height") {
            let mut hv = h.to_string();
            hv = hv.replace("mm", "");
            if let Some(h) = hv.parse::<f64>().ok() {
              height = h;
            }
          }
        }
        _ => {}
      }
    }

    let mut letters = HashMap::new();
    for (c, svg) in documents_per_char.iter() {
      let polylines =
        svg2polylines::parse(svg.as_str(), letter_precision, true).unwrap();
      let can_attach = false; // FIXME

      let mut minx = std::f64::INFINITY;
      let mut maxx = -std::f64::INFINITY;
      for poly in polylines.iter() {
        for p in poly.iter() {
          if p.x < minx {
            minx = p.x;
          }
          if p.x > maxx {
            maxx = p.x;
          }
        }
      }

      let mut width = maxx - minx;

      let mut dx = minx;
      if !can_attach {
        dx -= non_attached_pad;
        width += 2.0 * non_attached_pad;
      }

      let routes = polylines
        .iter()
        .map(|l| l.iter().map(|p| (p.x - dx, p.y)).collect())
        .collect();

      letters.insert(c.clone(), Letter::new(routes, width, height, can_attach));
    }

    letters.insert(
      " ".to_string(),
      Letter::new(vec![], 0.5 * height, height, false),
    );

    LetterSvgReferential { letters }
  }

  pub fn get_letter(&self, c: &String) -> Option<&Letter> {
    self.letters.get(c)
  }
}

fn spiral_angular(
  rot: f64,
  origin: (f64, f64),
  initial_offset: f64,
  length: f64,
  d_length: f64,
) -> Vec<(f64, f64)> {
  let mut d = Vec::new();
  let mut a: f64 = 0.0;
  let mut p = origin;
  let mut l = length;
  d.push((p.0 + initial_offset, p.1));
  loop {
    if l < 0.0 {
      break;
    }
    p = (p.0 + l * a.cos(), p.1 + l * a.sin());
    d.push(p);
    a += rot;
    l -= d_length;
  }
  d
}
