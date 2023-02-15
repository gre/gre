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
  #[clap(short, long, default_value = "500.0")]
  pub width: f64,
  #[clap(short, long, default_value = "700.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "2.8")]
  pub size: f64,
  // TODO letter precision config
  #[clap(short, long, default_value = "0.5")]
  pub vertical_chance: f64,
  #[clap(short, long, default_value = "500000")]
  pub max_iterations: usize,
  #[clap(short, long, default_value = "40")]
  pub optimized_count: usize,
  #[clap(short, long, default_value = "followers.txt")]
  list_file: String,
  #[clap(short, long, default_value = "images/letters.svg")]
  letters_file: String,
  #[clap(short, long)]
  debug: bool,
}

struct Letter {
  routes: Vec<Vec<(f64, f64)>>,
  width: f64,
  height: f64,
  can_attach: bool,
}
impl Letter {
  fn new(
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

  fn width_for_size(&self, size: f64) -> f64 {
    self.width * size / self.height
  }

  fn render(
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

struct LetterSvgReferential {
  letters: HashMap<String, Letter>,
}

impl LetterSvgReferential {
  fn new(svg_file: String) -> LetterSvgReferential {
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
      let polylines = svg2polylines::parse(svg.as_str(), 0.1, true).unwrap();

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
      let width = maxx - minx;

      let routes = polylines
        .iter()
        .map(|l| l.iter().map(|p| (p.x - minx, p.y)).collect())
        .collect();

      let can_attach = !"1234567890".contains(c);
      letters.insert(c.clone(), Letter::new(routes, width, height, can_attach));
    }

    LetterSvgReferential { letters }
  }
}

struct Word {
  text: String,
  x: f64,
  y: f64,
  size: f64,
  vertical: bool,
  calculated_width: f64,
  debug: bool,
}

fn collide_aabb(
  bound1: (f64, f64, f64, f64),
  bound2: (f64, f64, f64, f64),
) -> bool {
  let (x1, y1, x2, y2) = bound1;
  let (x3, y3, x4, y4) = bound2;
  x1 < x4 && x2 > x3 && y1 < y4 && y2 > y3
}

impl Word {
  fn new(
    text: String,
    size: f64,
    vertical: bool,
    letter_ref: &LetterSvgReferential,
    debug: bool,
  ) -> Word {
    let mut calculated_width = 0.0;
    for c in text.chars() {
      if let Some(letter) = letter_ref.letters.get(&c.to_string()) {
        let w = letter.width_for_size(size);
        calculated_width += w;
      }
    }
    Word {
      text,
      x: 0.,
      y: 0.,
      size,
      vertical,
      calculated_width,
      debug,
    }
  }
  fn set_pos(&mut self, x: f64, y: f64) {
    self.x += x;
    self.y += y;
  }
  fn get_pos_end_word(&self) -> (f64, f64) {
    let (w, h) = self.size();
    if self.vertical {
      (self.x + w / 2.0, self.y + h)
    } else {
      (self.x + w, self.y + h / 2.0)
    }
  }
  fn size(&self) -> (f64, f64) {
    let w = self.calculated_width;
    let h = self.size;
    if self.vertical {
      (h, w)
    } else {
      (w, h)
    }
  }
  fn bounds(&self) -> (f64, f64, f64, f64) {
    let (w, h) = self.size();
    (self.x, self.y, self.x + w, self.y + h)
  }

  fn find_location<R: Rng>(
    &self,
    rng: &mut R,
    words: &Vec<Word>,
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

  fn draw(&self, letter_ref: &LetterSvgReferential) -> Vec<Vec<(f64, f64)>> {
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

fn art(
  opts: &Opts,
  list: Vec<&str>,
  letters_ref: &LetterSvgReferential,
) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;

  let mut routes_inner = Vec::new();
  let mut routes_outer = Vec::new();
  let mut words = Vec::new();

  let mut list = list.clone();
  list.sort_by(|a, b| b.len().cmp(&a.len()));

  let mut rng = rng_from_seed(opts.seed);

  let get_color = image_get_color("images/twitter.png").unwrap();

  let mut last_point = (width * 0.5, height * 0.5);

  let max_iterations = opts.max_iterations;
  let optimized_count = opts.optimized_count;

  for (i, item) in list.iter().enumerate() {
    if i % 100 == 0 {
      println!("{} / {}", i, list.len());
    }
    let mut word = Word::new(
      item.to_string().to_lowercase(),
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
      let sz = word.size();
      let px = (x + sz.0 * 0.5) / width;
      let py = ((y + sz.1 * 0.5) - (height - width) / 2.0) / width;
      let c = get_color((px, py));
      word.set_pos(x, y);
      last_point = word.get_pos_end_word();
      if c.2 > 0.5 {
        routes_inner.extend(word.draw(letters_ref));
      } else {
        routes_outer.extend(word.draw(letters_ref));
      }
      words.push(word);
    } else {
      println!("no location found for {} at index {}", item, i);
      break;
    }

    // HACK
    /*
    if i == 100 {
      break;
    }
    */
  }

  /*
  routes_outer = letters_ref
    .letters
    .values()
    .cloned()
    .collect::<Vec<_>>()
    .concat();
  */
  /*
  routes_outer = vec![];
  let mut x = 10.0;
  let mut y = 10.0;
  for l in letters_ref.letters.values() {
    let (routes, (dx, dy)) = l.render((x, y), 10.0, false);
    x += dx;
    y += dy;
    routes_outer.extend(routes);
  }
  */

  vec![(routes_inner, "#888"), (routes_outer, "#000")]
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
  let list_file = opts.list_file.clone();
  let file_content = match std::fs::read_to_string(list_file.clone()) {
    Ok(list) => list,
    Err(_) => {
      let bearer = std::env::var("TWITTER_BEARER").unwrap();
      let mut list = String::new();
      let mut cursor = "-1".to_string();
      while cursor != "0" {
        let url = format!(
          "https://api.twitter.com/1.1/followers/list.json?cursor={}&skip_status=1&count=200&screen_name=greweb&skip_status=true&include_user_entities=false",
          cursor
        );
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
          reqwest::header::AUTHORIZATION,
          format!("Bearer {}", bearer).parse().unwrap(),
        );
        let client = reqwest::blocking::Client::builder()
          .default_headers(headers)
          .build()
          .unwrap();
        let response = client.get(url.as_str()).send().unwrap();
        let body = response.text().unwrap();
        let json: serde_json::Value =
          serde_json::from_str(body.as_str()).unwrap();
        cursor = json["next_cursor_str"].as_str().unwrap().to_string();
        for user in json["users"].as_array().unwrap() {
          list.push_str(
            format!("{}\n", user["screen_name"].as_str().unwrap()).as_str(),
          );
        }
      }
      std::fs::write(list_file.clone(), list.clone()).unwrap();
      list
    }
  };

  let letters_ref = LetterSvgReferential::new(opts.letters_file.clone());

  let list = file_content.split_ascii_whitespace().collect::<Vec<_>>();
  let groups = art(&opts, list, &letters_ref);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
