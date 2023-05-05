use byteorder::*;
use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::f64::INFINITY;
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
  #[clap(short, long, default_value = "200.0")]
  pub seed: f64,
  #[clap(
    short,
    long,
    default_value = "independence liberty autonomy sovereignty governance determination reliance free will choice flexibility openness diversity tolerance acceptance equality justice fairness democracy rights creativity progress growth expansion exploration adventure courage empowerment release peace joy bliss mindfulness awareness awakening enlightenment truthfulness authenticity honesty integrity accountability responsibility honor respect dignity worthiness appreciation gratitude compassion empathy kindness generosity service altruism forgiveness harmony unity cooperation collaboration solidarity fraternity"
    //default_value = "honor courage loyalty respect duty discipline integrity humility generosity courtesy valor benevolence wisdom empathy selflessness"
  )]
  pub dictionary: String,
}

pub fn rng_from_seed(s: f64) -> SmallRng {
  let mut bs = [0; 16];
  bs.as_mut().write_f64::<BigEndian>(s).unwrap();
  let mut rng = SmallRng::from_seed(bs);
  // run it a while to have better randomness
  for _i in 0..50 {
    rng.gen::<f64>();
  }
  return rng;
}

fn art(opts: &Opts) -> svg::Document {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let gold_gel = Ink("Gold Gel", "#D8B240", "#FFE38C", 0.6);
  let silver_gel = Ink("Silver Gel", "#CCCCCC", "#FFFFFF", 0.6);

  let black = Ink("Black", "#1A1A1A", "#000000", 0.35);
  let brillant_red = Ink("Brillant Red", "#F22", "#912", 0.35);
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let turquoise = Ink("Turquoise", "#00B4E6", "#005A8C", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);
  let spring_green = Ink("Spring Green", "#7d9900", "#6c6b00", 0.35);

  let white_paper = Paper("White", "#fff", false);
  let black_paper = Paper("Black", "#111", true);
  let red_paper = Paper("Red", "#aa0000", true);

  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();

  // PAPER AND INKS

  let black_paper_chance = 0.1;
  let red_paper_chance = 0.05;
  let monochrome_chance = 0.33;

  let paper = if rng.gen_bool(red_paper_chance) {
    red_paper
  } else if rng.gen_bool(black_paper_chance) {
    black_paper
  } else {
    white_paper
  };

  let count = if paper.2 {
    rng.gen_range(1, 3)
  } else {
    rng.gen_range(1, 4)
  };

  let mut words = opts.dictionary.split(" ").collect::<Vec<_>>();
  rng.shuffle(&mut words);
  words = words[0..count].to_vec();

  let mut inks = if paper.2 {
    vec![silver_gel, gold_gel]
  } else {
    // TODO probability of the colors?
    vec![
      black,
      seibokublue,
      turquoise,
      soft_mint,
      spring_green,
      amber,
      pink,
      brillant_red,
    ]
  };

  rng.shuffle(&mut inks);
  let monochrome = rng.gen_bool(monochrome_chance);
  let ink_count = if monochrome { 2 } else { count };
  inks = inks[0..ink_count].to_vec();

  let third_to_first = ink_count == 3 && rng.gen_bool(0.5);
  if third_to_first {
    inks[2] = inks[0];
  }

  // TEXT STYLES

  let font_size = rng.gen_range(22.0, 32.0) * inks[0].3;
  let word_dist =
    (0.04 + rng.gen_range(-0.1, 0.6) * rng.gen_range(0.0, 1.0)) * font_size;
  let line_dist =
    mix(0.25, 0.6, rng.gen_range(0.0, 1.0) * rng.gen_range(0.6, 1.0))
      * font_size;

  // VALUE FUNCTIONS

  // TODO maybe anomaly is increased if the word is in correct orientation (angular)
  let color_anomaly = 0.001;
  let repeat_divisor = (1.0
    + rng.gen_range(0.0, 4.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;
  let color_word_attached = rng.gen_bool(0.2);
  let color_seed = rng.gen_range(0.0, 100000.0);
  let color_freq = 0.5 + rng.gen_range(0.0, 16.0) * rng.gen_range(0.0, 1.0);
  let color_field = rng.gen_bool(0.7);

  let vsplit = rng.gen_bool(0.3);
  let hsplit = rng.gen_bool(0.3);

  let mut concentric_color_add = vec![];
  if rng.gen_bool(0.4) {
    concentric_color_add.push((0.0, rng.gen_range(0.0, 0.2)));
  }

  if rng.gen_bool(0.3) {
    concentric_color_add
      .push((rng.gen_range(0.2, 0.3), rng.gen_range(0.3, 0.4)));
  }
  if rng.gen_bool(0.4) {
    concentric_color_add.push((rng.gen_range(0.35, 0.45), 0.5));
  }

  let color_fn = |rng: &mut SmallRng, pos: (f64, f64), i: usize| -> usize {
    if rng.gen_bool(color_anomaly) {
      return rng.gen_range(0, ink_count);
    }
    if monochrome {
      return 0;
    }

    let mut color = 0;
    if color_word_attached {
      color = (i / repeat_divisor) % ink_count;
    } else if color_field {
      let v = perlin.get([
        color_seed + pos.0 * color_freq / width,
        color_seed + pos.1 * color_freq / width,
      ]);
      let v = (v + 0.5) * (ink_count as f64);
      color = v.floor() as usize % ink_count;
    }

    if concentric_color_add.len() > 0 {
      let dist_center =
        ((pos.0 - width / 2.0).powi(2) + (pos.1 - height / 2.0).powi(2)).sqrt()
          / width;
      for &(from, to) in concentric_color_add.iter() {
        if dist_center > from && dist_center < to {
          color += 1;
        }
      }
    }

    if vsplit {
      if pos.0 < width / 2.0 {
        color += 1;
      }
    }
    if hsplit {
      if pos.1 < height / 2.0 {
        color += 1;
      }
    }

    color % ink_count
  };

  let bold_anomaly = 0.001;
  let allow_bold_noise_field = rng.gen_bool(0.2);
  let mut concentric_rays_bold = vec![];
  if rng.gen_bool(0.1) {
    concentric_rays_bold.push((0.43, 0.5));
  }
  let bold_field_chance = 0.2;
  let bold_seed = rng.gen_range(0.0, 100000.0);
  let bold_freq1 = rng.gen_range(0.0, 10.0);
  let bold_freq2 = rng.gen_range(0.0, 40.0);
  let threshold = 0.48;

  let bold_fn = |rng: &mut SmallRng, pos: (f64, f64), _i: usize| -> bool {
    if rng.gen_bool(bold_anomaly) {
      return true;
    }
    if allow_bold_noise_field {
      let v1 = perlin.get([
        bold_seed + pos.0 * bold_freq1 / width,
        bold_seed + pos.1 * bold_freq1 / width,
      ]);
      let v2 = perlin.get([
        bold_seed + pos.0 * bold_freq2 / width,
        bold_seed + pos.1 * bold_freq2 / width,
      ]);
      if v1 > threshold && v2 > threshold {
        return rng.gen_bool(bold_field_chance);
      }
    }
    if concentric_rays_bold.len() > 0 {
      let dist_center =
        ((pos.0 - width / 2.0).powi(2) + (pos.1 - height / 2.0).powi(2)).sqrt()
          / width;
      for &(from, to) in concentric_rays_bold.iter() {
        if dist_center > from && dist_center < to {
          return true;
        }
      }
    }
    false
  };

  let mut routes = Vec::new();

  // add text
  let non_attached_pad = 0.0;
  let extra_pad = 1.0;
  let letters_ref: LetterSvgReferential = LetterSvgReferential::new(
    "/Users/gre/Documents/arthur-letters-1.svg".to_string(),
    0.1,
    non_attached_pad,
    extra_pad,
  );

  let mut unsafe_curves = Vec::new();

  unsafe_curves.push(spiral_optimized(
    width / 2.0,
    height / 2.0,
    width / 2.0 - pad,
    line_dist,
    0.1,
  ));

  let curves = unsafe_curves.clone();

  // offset text exactly on the curve line
  let yoffset = -font_size * 0.5;

  let mut queue = VecDeque::new();
  for word in words {
    queue.push_back(word.to_string());
  }

  let mut total_words = 0;

  let mut offsets = vec![];

  let b = 0.01 * font_size;
  for i in 0..3 {
    let angle = i as f64 * 2.0 * PI / 3.0;
    let offset = (angle.cos() * b, angle.sin() * b);
    offsets.push(offset);
  }

  for c in curves.clone() {
    let length = curve_length(&c);
    let extrapad = word_dist;
    let mut subset = vec![];
    let mut sum = extrapad;
    let mut sum_words = 0.0;
    // we try to pull as much word as we can to fill the curve
    while let Some(word) = queue.pop_front() {
      let text = word.clone();
      let measure = measure_text(&letters_ref, text.clone(), font_size);
      if sum + measure + word_dist < length {
        sum += measure + word_dist;
        sum_words += measure;
        subset.push(text);
        queue.push_back(word.clone());
      } else {
        queue.push_front(word.clone());
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
        draw_text(&letters_ref, text.clone(), font_size, xstart, yoffset, &c);

      if res.0.len() == 0 {
        continue;
      }

      let pos = calc_text_center(&res.0);

      let clr_index = color_fn(&mut rng, pos, total_words);
      let bold = bold_fn(&mut rng, pos, total_words);

      let rts = res.0;

      if bold {
        for offset in offsets.iter() {
          routes.extend(
            rts
              .iter()
              .map(|r| {
                (
                  clr_index,
                  r.iter().map(|p| (p.0 + offset.0, p.1 + offset.1)).collect(),
                )
              })
              .collect::<Vec<_>>(),
          );
        }
      } else {
        routes.extend(
          rts
            .iter()
            .map(|r| (clr_index, r.clone()))
            .collect::<Vec<_>>(),
        );
      }

      total_words += 1;

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
  let mut prev_can_attach = false;
  let mut last: Vec<(f64, f64)> = vec![];
  for c in text.chars() {
    if let Some(letter) = letter_ref.get_letter(&c.to_string()) {
      let (rts, (dx, dy)) = letter.render((x, y), size, false);
      if prev_can_attach && letter.can_attach_left {
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
      prev_can_attach = letter.can_attach_right;
      x += dx;
      y += dy;
    } else {
      prev_can_attach = false;
      // println!("letter not found: {}", c);
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
  pub can_attach_left: bool,
  pub can_attach_right: bool,
}
impl Letter {
  pub fn new(
    routes: Vec<Vec<(f64, f64)>>,
    width: f64,
    height: f64,
    can_attach_left: bool,
    can_attach_right: bool,
  ) -> Letter {
    Letter {
      routes,
      width,
      height,
      can_attach_left,
      can_attach_right,
    }
  }

  pub fn width_for_size(&self, size: f64) -> f64 {
    self.width * size / self.height
  }

  pub fn render(
    &self,
    (x, y): (f64, f64),
    size: f64,
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
    extra_pad: f64,
  ) -> LetterSvgReferential {
    let mut content = String::new();

    let mut height = 0.0;
    let mut documents_per_layer: HashMap<String, String> = HashMap::new();

    for event in svg::open(svg_file, &mut content).unwrap() {
      match event {
        Event::Tag(_, _, attributes) => {
          if let Some(c) = attributes.get("inkscape:label") {
            if let Some(d) = attributes.get("d") {
              let data: String = d.to_string();
              let document =
                Document::new().add(Path::new().set("d", data)).to_string();
              documents_per_layer.insert(c.to_string(), document);
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
    for (c, svg) in documents_per_layer.iter() {
      let polylines =
        svg2polylines::parse(svg.as_str(), letter_precision, true).unwrap();

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

      let letter_name = c[0..1].to_string();
      // < : can attach left
      let can_attach_left = c.contains("&lt;");
      // > : can attach right
      let can_attach_right = c.contains("&gt;");
      // R : add extra pad on the right
      let add_extra_pad_right = c.contains("R");

      if !can_attach_left {
        dx -= non_attached_pad;
        width += non_attached_pad;
      }

      if !can_attach_right {
        width += non_attached_pad;
      }
      if add_extra_pad_right {
        width += extra_pad;
      }

      /*
      if !can_attach {
        dx -= non_attached_pad;
        width += 2.0 * non_attached_pad;
      }
      */

      let routes: Vec<Vec<(f64, f64)>> = polylines
        .iter()
        .map(|l| l.iter().map(|p| (p.x - dx, p.y)).collect())
        .collect();

      letters.insert(
        letter_name.clone(),
        Letter::new(routes, width, height, can_attach_left, can_attach_right),
      );
    }

    letters.insert(
      " ".to_string(),
      Letter::new(vec![], 0.5 * height, height, false, false),
    );

    LetterSvgReferential { letters }
  }

  pub fn get_letter(&self, c: &String) -> Option<&Letter> {
    self.letters.get(c)
  }
}

pub fn spiral_optimized(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = round_point((x + r * a.cos(), y + r * a.sin()), 0.01);
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
}

fn calc_text_center(routes: &Vec<Vec<(f64, f64)>>) -> (f64, f64) {
  let mut min_x = INFINITY;
  let mut max_x = -INFINITY;
  let mut min_y = INFINITY;
  let mut max_y = -INFINITY;
  for route in routes.iter() {
    for p in route.iter() {
      if p.0 < min_x {
        min_x = p.0;
      }
      if p.0 > max_x {
        max_x = p.0;
      }
      if p.1 < min_y {
        min_y = p.1;
      }
      if p.1 > max_y {
        max_y = p.1;
      }
    }
  }
  ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0)
}
