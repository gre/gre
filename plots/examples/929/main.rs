use clap::*;
use gre::letters::*;
use gre::*;
use rand::prelude::*;
use std::collections::VecDeque;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

pub fn draw_text(
  letter_ref: &LetterSvgReferential,
  text: String,
  origin: (f64, f64),
  size: f64,
  angle: f64,
  aligned_right: bool,
) -> Vec<Vec<(f64, f64)>> {
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

  let endx = x;

  // rotate with angle and translate to origin all routes
  let mut rotated_routes = Vec::new();
  for route in routes {
    let mut rotated_route = Vec::new();
    for (x, y) in route {
      let x = if aligned_right { x - endx } else { x };
      let (x, y) = p_r((x, y), angle);
      let x = x + origin.0;
      let y = y + origin.1;
      rotated_route.push((x, y));
    }
    rotated_routes.push(rotated_route);
  }

  rotated_routes
}

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "7.8")]
  pub gridsize: f64,
  #[clap(short, long, default_value = "3.0")]
  pub fontsize: f64,
  #[clap(short, long, default_value = "gmgm")]
  pub text: String,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
  Wall,
  Path,
}

type Point = (usize, usize);
type Line = (Point, Point);

fn generate_maze<R: Rng>(
  rng: &mut R,
  width: usize,
  height: usize,
) -> Vec<Line> {
  let mut maze = vec![vec![Cell::Wall; width]; height];
  let mut stack = VecDeque::new();
  let start = (rng.gen_range(0, height), rng.gen_range(0, width));
  maze[start.0][start.1] = Cell::Path;
  stack.push_back(start);

  let mut lines = vec![];

  while let Some(current) = stack.pop_back() {
    let mut neighbors = vec![];

    if current.0 > 0 {
      neighbors.push((current.0 - 1, current.1));
    }
    if current.0 < height - 1 {
      neighbors.push((current.0 + 1, current.1));
    }
    if current.1 > 0 {
      neighbors.push((current.0, current.1 - 1));
    }
    if current.1 < width - 1 {
      neighbors.push((current.0, current.1 + 1));
    }

    rng.shuffle(&mut neighbors);

    for neighbor in neighbors {
      if maze[neighbor.0][neighbor.1] == Cell::Wall {
        maze[neighbor.0][neighbor.1] = Cell::Path;
        stack.push_back(current);
        stack.push_back(neighbor);

        if current.0 == neighbor.0 {
          if current.1 < neighbor.1 {
            lines.push((current, (current.0, current.1 + 1)));
          } else {
            lines.push(((current.0, current.1 - 1), current));
          }
        } else {
          if current.0 < neighbor.0 {
            lines.push((current, (current.0 + 1, current.1)));
          } else {
            lines.push(((current.0 - 1, current.1), current));
          }
        }

        break;
      }
    }
  }

  lines
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mul = 1.0 / opts.gridsize;
  let size = opts.fontsize;
  let text = opts.text.clone();

  let mut routes = Vec::new();

  let mut rng = rng_from_seed(opts.seed);

  let wspace = width - 2. * pad;
  let hspace = height - 2. * pad;

  let w = (mul * wspace) as usize;
  let h = (mul * hspace) as usize;

  let lines = generate_maze(&mut rng, w, h);

  let letters_ref =
    LetterSvgReferential::new("images/letters.svg".to_string(), 0.1, 1.0);

  for line in lines {
    let vertical = line.0 .0 != line.1 .0;

    let minx = line.0 .1.min(line.1 .1);
    let miny = line.0 .0.min(line.1 .0);
    let mut x = pad + (minx as f64) * (width - 2. * pad) / (w as f64);
    let mut y = pad + (miny as f64) * (height - 2. * pad) / (h as f64);

    if !vertical {
      y -= 0.7 * size;
    } else {
      x -= 0.7 * size;
    }

    routes.extend(draw_text(
      &letters_ref,
      text.clone(),
      (x, y),
      size,
      if vertical { PI / 2.0 } else { 0.0 },
      vertical,
    ));
  }

  vec![(routes, "#ddd")]
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
  let groups = art(&opts);
  let mut document = base_document("#222", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
