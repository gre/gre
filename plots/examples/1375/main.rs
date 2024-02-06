use std::f64::consts::PI;

use clap::*;
use gre::letters::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "148.5")]
  pub width: f64,
  #[clap(short, long, default_value = "0.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.3")]
  pub precision: f64,
  #[clap(short, long, default_value = "2.25")]
  pub size: f64,
  #[clap(short, long, default_value = "2.3")]
  pub word_dist: f64,
  #[clap(short, long, default_value = "3.0")]
  pub radius: f64,
  #[clap(short, long, default_value = "1.0")]
  pub inside_multiplier: f64,
  #[clap(short, long, default_value = "0.0")]
  pub logo_pad: f64,
  #[clap(short, long, default_value = "0.1")]
  pub letter_precision: f64,
  #[clap(short, long, default_value = "1.0")]
  pub non_attached_pad: f64,
  #[clap(short, long, default_value = "2.0")]
  pub breaking_angle_factor: f64,
  #[clap(short, long, default_value = "80")]
  pub depth: usize,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "warpcast-followers.txt")]
  list_file: String,
  #[clap(short, long, default_value = "images/letters.svg")]
  letters_file: String,
  #[clap(short, long, default_value = "images/w.png")]
  image_layer: String,
  #[clap(short, long)]
  debug: bool,
}

fn art(
  opts: &Opts,
  list: Vec<&str>,
  letters_ref: &LetterSvgReferential,
) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;
  let precision = opts.precision;
  let radius = opts.radius;
  let logo_pad = opts.logo_pad;
  let word_dist = opts.word_dist;
  let breaking_angle_factor = opts.breaking_angle_factor;

  let mut data = vec![];

  let mut list = list.clone();
  let mut rng = rng_from_seed(opts.seed);
  rng.shuffle(&mut list);
  // list.sort_by(|a, b| b.len().cmp(&a.len()));

  // FILL THE VALUE MAP AND CALC THE CURVES

  let get_color = image_get_color(opts.image_layer.as_str()).unwrap();

  // value map is filled with the inside part of the logo + text
  let mut value_map = ValueMap::new(precision, width, height);
  let get_value = |(x, y): (f64, f64)| {
    let px = x / width;
    let py = y / height;
    let c = get_color((px, py));
    if c.2 > 0.5 {
      1
    } else {
      0
    }
  };
  value_map.fill(&get_value);

  // value_map_outside is filled with the reverse
  let mut value_map_outside = ValueMap::new(precision, width, height);
  let get_value_outside = |(x, y): (f64, f64)| 1 - get_value((x, y));
  value_map_outside.fill(&get_value_outside);
  // we grow it to create a halo
  value_map_outside.grow_passage(logo_pad);

  let mut curves: Vec<(Vec<(f64, f64)>, bool)> = vec![];

  let precisioncontour = precision;
  let rdp_precision = precision;

  let w = (width as f64 / precisioncontour) as u32;
  let h = (height as f64 / precisioncontour) as u32;

  for (mut value_map, inside) in
    vec![(value_map_outside, false) /*, (value_map, true)*/]
  {
    let mut all = vec![];
    for _i in 0..opts.depth {
      let f = |p: (f64, f64)| {
        if inside {
          value_map.get((p.0 * width, p.1 * height)) as f64
        } else {
          let g = (p.0 * width, p.1 * height);
          // actively crop the outside part out
          if g.0 < pad || g.0 > width - pad || g.1 < pad || g.1 > height - pad {
            return 1.;
          }
          value_map.get(g) as f64
        }
      };

      // we use marching square on the current value map to infer contours
      let features = contour(w, h, &f, &vec![0.5]);
      let mut routes = features_to_routes(features, precisioncontour);
      routes = crop_routes(&routes, (pad, pad, width - pad, height - pad));

      if routes.len() == 0 {
        break;
      }

      // for each contouring, we convert them into curves
      // - rdp to avoid too much zig zag problems
      // - slicing the paths into smaller pieces to avoid sharp edges cases
      // both of these are needed for the word readability
      for route in routes.clone() {
        all.extend(
          slice_on_sharp_edges(
            &rdp(&route, rdp_precision),
            breaking_angle_factor,
          )
          .iter()
          .map(|r| (r.clone(), inside))
          .collect::<Vec<_>>(),
        );
      }

      // on each iteration of the loop, we spread the area of the value map
      value_map.grow_passage(if inside {
        opts.inside_multiplier * radius
      } else {
        radius
      });
    }
    if inside {
      all.reverse();
    }
    curves.extend(all);
  }

  // debug will draw all the curves
  if opts.debug {
    let mut routes = vec![];
    for (c, _) in curves.clone() {
      routes.push(c);
    }
    data.push(("debug".to_string(), routes));
  }

  // offset text exactly on the curve line
  let yoffset = -size * 0.7;

  // passage is used to do collision and avoid two lines to collide too mcuh
  let passage_precision = 0.2 * size;
  let mut passage = ValueMap::new(passage_precision, width, height);

  // we will unpile the list of username from this list
  let mut words = list
    .iter()
    .map(|s| s.to_string().to_lowercase())
    .collect::<Vec<String>>();

  let mut routes = vec![];
  let mut word_count = 0;

  rng.shuffle(&mut curves);

  for (c, inside) in curves.clone() {
    if words.len() == 0 {
      break;
    }
    let word_dist = if inside {
      word_dist * opts.inside_multiplier
    } else {
      word_dist
    };
    let length = curve_length(&c);
    let extrapad = word_dist;
    let mut subset = vec![];
    let mut sum = extrapad;
    let mut sum_words = 0.0;
    // we try to pull as much word as we can to fill the curve
    while let Some(word) = words.pop() {
      let text = word.clone();
      let measure = measure_text(&letters_ref, text.clone(), size);
      if sum + measure + word_dist < length {
        sum += measure + word_dist;
        sum_words += measure;
        subset.push(text);
      } else {
        words.push(word.clone());
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
          passage.count_once(p);
        }
      }

      // we only use the word if it was not colliding
      if !collides {
        word_count += 1;
        routes.extend(res.0.clone());

        let mut sumx = 0.0;
        let mut sumy = 0.0;
        let mut count = 0;
        for r in res.0.clone() {
          for p in r.clone() {
            sumx += p.0;
            sumy += p.1;
            count += 1;
          }
        }
        let center = (sumx / count as f64, sumy / count as f64);
        println!("{} {} {}", text, center.0.round(), center.1.round());

        if word_count % 100 == 0 {
          let name = format!("words {}", word_count);
          data.push((name, routes.clone()));
          routes = vec![];
        }
      } else {
        words.push(text.clone());
      }
      xstart += res.1 + pad / 2.0;
    }
  }

  if routes.len() > 0 {
    data.push((format!("words {}", word_count), routes));
  }

  // this log is needed to make sure we have used all the words
  println!("{} remaining words", words.len());

  data
    .iter()
    .map(|(name, routes)| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(name.as_str());
      l = l.add(base_path("white", 0.35, data));
      l
    })
    .collect()
}

#[derive(Clone)]
struct ValueMap {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl ValueMap {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    ValueMap {
      precision,
      width,
      height,
      counters,
    }
  }

  pub fn fill(self: &mut Self, get_value: &dyn Fn((f64, f64)) -> usize) {
    for i in 0..self.counters.len() {
      let p = self.reverse_index(i);
      self.counters[i] = get_value(p);
    }
  }

  pub fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn reverse_index(self: &Self, i: usize) -> (f64, f64) {
    let wi = (self.width / self.precision).ceil() as usize;
    let xi = i % wi;
    let yi = i / wi;
    let x = xi as f64 * self.precision;
    let y = yi as f64 * self.precision;
    (x, y)
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

fn main() {
  let opts: Opts = Opts::parse();
  let list_file = opts.list_file.clone();
  let file_content = match std::fs::read_to_string(list_file.clone()) {
    Ok(list) => list,
    Err(_) => {
      let mut list = String::new();
      let url = "https://client.warpcast.com/v2/followers?fid=250260";
      let client = reqwest::blocking::Client::builder().build().unwrap();
      let response = client.get(url).send().unwrap();
      let body = response.text().unwrap();
      let json: serde_json::Value =
        serde_json::from_str(body.as_str()).unwrap();
      for user in json["result"].as_object().unwrap()["users"]
        .as_array()
        .unwrap()
      {
        list.push_str(
          format!("{}\n", user["username"].as_str().unwrap()).as_str(),
        );
      }
      std::fs::write(list_file.clone(), list.clone()).unwrap();
      list
    }
  };

  let letters_ref = LetterSvgReferential::new(
    opts.letters_file.clone(),
    opts.letter_precision,
    opts.non_attached_pad,
  );

  let list = file_content.split_ascii_whitespace().collect::<Vec<_>>();
  let groups = art(&opts, list, &letters_ref);
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
