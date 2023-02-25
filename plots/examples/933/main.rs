use clap::Parser;
use gre::euclidian_dist;
use gre::letters::LetterSvgReferential;
use livedraw::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use svg::node::element::path::Data;

// TODO cut letter by letter!

#[derive(Debug, Parser, Clone, Copy)]
#[clap()]
struct Args {
  #[clap(long, default_value_t = 0.0)]
  seed: f64,
  #[clap(long, default_value_t = 105.0)]
  width: f64,
  #[clap(long, default_value_t = 148.5)]
  height: f64,
  #[clap(long, default_value_t = 5.0)]
  padding: f64,
  #[clap(long)]
  simulation: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TextValue {
  value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ArtInput {
  text: TextValue,
}

#[derive(Clone)]
struct Art {
  args: Args,
  cursor: (f64, f64),
  letter_ref: LetterSvgReferential,
  size: f64,
}

impl Art {
  fn new(args: Args) -> Self {
    let pad = args.padding;
    let letter_ref = gre::letters::LetterSvgReferential::new(
      "images/letters.svg".to_string(),
      0.1,
      1.0,
    );
    Art {
      args,
      cursor: (pad, pad),
      size: 3.0,
      letter_ref,
    }
  }
}

impl LivedrawArt for Art {
  fn get_dimension(&self) -> (f64, f64) {
    (self.args.width, self.args.height)
  }

  fn estimate_total_increments(&self) -> usize {
    100
  }

  fn actions_before_increment(&self, i: usize) -> Vec<ArtAction> {
    if i == 0 {
      return vec![ArtAction::Pause(String::from("Get ready to chat!"), 30.0)];
    }
    return vec![];
  }

  fn get_predictive_max_next_increments(&self) -> Option<usize> {
    Some(10)
  }

  fn draw_increment(&mut self, value: &Value, index: usize) -> ArtIncrement {
    let input: ArtInput = serde_json::from_value(value.clone()).unwrap();

    let mut cursor = self.cursor;

    let text = input.text.value;

    if text.len() == 0 {
      return ArtIncrement::Continue;
    }

    let path = vec![(cursor.0, cursor.1), (cursor.0 + 200.0, cursor.1)];

    let (routes, len) =
      draw_text(&self.letter_ref, text.clone(), self.size, 0.0, 0.0, &path);

    let mut all = vec![];

    if cursor.0 + len > self.args.width - self.args.padding {
      cursor = (self.args.padding, cursor.1 + 1.2 * self.size);
      if cursor.1 > self.args.height - self.args.padding {
        return ArtIncrement::End;
      }
      let path = vec![(cursor.0, cursor.1), (cursor.0 + 200.0, cursor.1)];
      let (routes, len) =
        draw_text(&self.letter_ref, text.clone(), self.size, 0.0, 0.0, &path);
      cursor = (cursor.0 + len, cursor.1);
      all.extend(routes);
    } else {
      cursor = (cursor.0 + len, cursor.1);
      all.extend(routes);
    }

    self.cursor = cursor;

    let data = all.iter().fold(Data::new(), livedraw::render_route);

    let layers =
      vec![svg_layer("black").add(svg_base_path("black", 0.35, data))];

    return ArtIncrement::SVG(layers);
  }
}

impl LivedrawArtSimulation for Art {
  fn simulate_input(&mut self, _index: usize) -> Value {
    return json!(ArtInput {
      text: TextValue {
        value: "test".to_string()
      },
    });
  }
}

fn main() {
  let args = Args::parse();
  println!("{:#?}", args);
  let mut art = Art::new(args.clone());
  if args.simulation {
    livedraw_start_simulation(&mut art);
  } else {
    livedraw_start(&mut art);
  }
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
