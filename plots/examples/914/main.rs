use clap::*;
use gre::*;
use rand::prelude::*;
use std::collections::HashMap;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::parser::Event;
use svg::Document;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "105.0")]
  pub width: f64,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "2.6")]
  pub size: f64,
  #[clap(short, long, default_value = "0.05")]
  pub letter_precision: f64,
  #[clap(short, long, default_value = "0.3")]
  pub vertical_chance: f64,
  #[clap(short, long, default_value = "1.0")]
  pub non_attached_pad: f64,
  #[clap(short, long, default_value = "1000000")]
  pub max_iterations: usize,
  #[clap(short, long, default_value = "50")]
  pub optimized_count: usize,
  #[clap(short, long, default_value = "images/letters.svg")]
  letters_file: String,
  #[clap(short, long)]
  debug: bool,
}

fn art(opts: &Opts, letters_ref: &LetterSvgReferential) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;
  let max_iterations = opts.max_iterations;
  let optimized_count = opts.optimized_count;

  let mut rng = rng_from_seed(opts.seed);
  let mut routes = Vec::new();
  let mut words = vec![];
  let mut last_point = (width * 0.5, height * 0.5);

  for sentence in vec![" good morning ", "gm "] {
    loop {
      let mut word = TextRendering::new(
        sentence.to_string(),
        size,
        rng.gen_bool(opts.vertical_chance),
        letters_ref,
        opts.debug,
      );
      if let Some((x, y)) = word.find_location(
        &mut rng,
        &words,
        max_iterations,
        width,
        height,
        pad,
        last_point,
        optimized_count,
      ) {
        word.set_pos(x, y);
        last_point = word.get_pos_end_word();
        routes.extend(word.draw(letters_ref));
        words.push(word);
      } else {
        break;
      }
    }
  }

  vec![(routes, "#000")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();

  let letters_ref = LetterSvgReferential::new(
    opts.letters_file.clone(),
    opts.letter_precision,
    opts.non_attached_pad,
  );

  let groups = art(&opts, &letters_ref);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

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
      let can_attach = !"1234567890".contains(c);

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
}

pub struct TextRendering {
  text: String,
  x: f64,
  y: f64,
  size: f64,
  vertical: bool,
  calculated_width: f64,
  debug: bool,
}

pub fn collide_aabb(
  bound1: (f64, f64, f64, f64),
  bound2: (f64, f64, f64, f64),
) -> bool {
  let (x1, y1, x2, y2) = bound1;
  let (x3, y3, x4, y4) = bound2;
  x1 < x4 && x2 > x3 && y1 < y4 && y2 > y3
}

impl TextRendering {
  pub fn new(
    text: String,
    size: f64,
    vertical: bool,
    letter_ref: &LetterSvgReferential,
    debug: bool,
  ) -> TextRendering {
    let mut calculated_width = 0.0;
    for c in text.chars() {
      if let Some(letter) = letter_ref.letters.get(&c.to_string()) {
        let w = letter.width_for_size(size);
        calculated_width += w;
      }
    }
    TextRendering {
      text,
      x: 0.,
      y: 0.,
      size,
      vertical,
      calculated_width,
      debug,
    }
  }
  pub fn set_pos(&mut self, x: f64, y: f64) {
    self.x += x;
    self.y += y;
  }
  pub fn get_pos_end_word(&self) -> (f64, f64) {
    let (w, h) = self.size();
    if self.vertical {
      (self.x + w / 2.0, self.y + h)
    } else {
      (self.x + w, self.y + h / 2.0)
    }
  }
  pub fn size(&self) -> (f64, f64) {
    let w = self.calculated_width;
    let h = self.size;
    if self.vertical {
      (h, w)
    } else {
      (w, h)
    }
  }
  pub fn bounds(&self) -> (f64, f64, f64, f64) {
    let (w, h) = self.size();
    (self.x, self.y, self.x + w, self.y + h)
  }

  pub fn find_location<R: Rng>(
    &self,
    rng: &mut R,
    words: &Vec<TextRendering>,
    max: usize,
    width: f64,
    height: f64,
    pad: f64,
    optimized_closed_to: (f64, f64),
    optimized_count: usize,
  ) -> Option<(f64, f64)> {
    let mut x = rng.gen_range(0.0, width);
    let mut y = rng.gen_range(0.0, height);
    let mut i = 0;
    let mut candidates = Vec::new();
    let (w, h) = self.size();
    while i < max {
      if x < pad {
        x = pad;
      }
      if x > width - pad - w {
        x = width - pad - w;
      }
      if y < pad {
        y = pad;
      }
      if y > height - pad - h {
        y = height - pad - h;
      }

      let mut found = true;
      for word in words {
        let bound = (x, y, x + w, y + h);
        if collide_aabb(bound, word.bounds()) {
          found = false;
          break;
        }
      }
      if found {
        candidates.push((x, y));
        // return Some((x, y));
      }
      if candidates.len() > optimized_count {
        break;
      }
      x = rng.gen_range(0.0, width);
      y = rng.gen_range(0.0, height);
      i += 1;
    }
    if candidates.len() > 0 {
      // sort candidates by distance to optimized_closed_to
      candidates.sort_by(|a, b| {
        let d1 = (a.0 - optimized_closed_to.0).powi(2)
          + (a.1 - optimized_closed_to.1).powi(2);
        let d2 = (b.0 - optimized_closed_to.0).powi(2)
          + (b.1 - optimized_closed_to.1).powi(2);
        d1.partial_cmp(&d2).unwrap()
      });
      return Some(candidates[0]);
    }
    None
  }

  pub fn draw(
    &self,
    letter_ref: &LetterSvgReferential,
  ) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();

    if self.debug {
      let (w, h) = self.size();
      let x = self.x;
      let y = self.y;
      let mut route = Vec::new();
      route.push((x, y));
      route.push((x + w, y));
      route.push((x + w, y + h));
      route.push((x, y + h));
      route.push((x, y));
      routes.push(route);
    }

    let mut x = self.x;
    let mut y = self.y;
    let mut can_attach = true;
    let mut last: Vec<(f64, f64)> = vec![];
    for c in self.text.chars() {
      if let Some(letter) = letter_ref.letters.get(&c.to_string()) {
        let (rts, (dx, dy)) = letter.render((x, y), self.size, self.vertical);
        if letter.can_attach && can_attach {
          // convention: last path attached
          let mut rts = rts.clone();
          let add = rts.pop().unwrap();
          last.extend(add);
          routes.extend(rts);
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

    routes
  }
}
