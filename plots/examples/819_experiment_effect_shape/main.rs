use std::f64::consts::PI;

use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "420.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

trait Shape {
  fn draw(self: &Self, center: (f64, f64), radius: f64)
    -> Vec<Vec<(f64, f64)>>;
}

trait Effect {
  fn apply(self: &Self, routes: Vec<Vec<(f64, f64)>>) -> Vec<Vec<(f64, f64)>>;
}

#[derive(Clone, Copy)]
struct VerticalLines {
  ratio: f64,
  lines_distance: f64,
}
impl Shape for VerticalLines {
  fn draw(
    self: &Self,
    center: (f64, f64),
    radius: f64,
  ) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();
    let dx = radius * (self.ratio).min(1.0);
    let dy = radius * (1.0 / self.ratio).min(1.0);
    let mut x = -dx;
    loop {
      if x > dx {
        break;
      }
      routes.push(vec![
        (center.0 + x, center.1 - dy),
        (center.0 + x, center.1 + dy),
      ]);
      x += self.lines_distance;
    }
    routes
  }
}

#[derive(Clone, Copy)]
struct HorizontalLines {
  ratio: f64,
  lines_distance: f64,
}
impl Shape for HorizontalLines {
  fn draw(
    self: &Self,
    center: (f64, f64),
    radius: f64,
  ) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();
    let dx = radius * (self.ratio).min(1.0);
    let dy = radius * (1.0 / self.ratio).min(1.0);
    let mut y = -dy;
    loop {
      if y > dy {
        break;
      }
      routes.push(vec![
        (center.0 - dx, center.1 + y),
        (center.0 + dx, center.1 + y),
      ]);
      y += self.lines_distance;
    }
    routes
  }
}

#[derive(Clone, Copy)]
struct SpiralCircle {
  dr: f64,
}
impl Shape for SpiralCircle {
  fn draw(
    self: &Self,
    center: (f64, f64),
    radius: f64,
  ) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();
    let (x, y) = center;
    routes.push(spiral_optimized(x, y, radius, self.dr, 0.01));
    routes.push(circle_route(center, radius, (radius * 10.0) as usize));
    routes
  }
}

fn frame(
  bound: (f64, f64, f64, f64),
  shapes: Vec<Box<&dyn Shape>>,
  effects: Vec<Box<&dyn Effect>>,
) -> Vec<Vec<(f64, f64)>> {
  let center = (mix(bound.0, bound.2, 0.5), mix(bound.1, bound.3, 0.5));
  let size = (bound.2 - bound.0).min(bound.3 - bound.1) * 0.4;
  let mut all = Vec::new();
  for shape in shapes {
    all.push(shape.draw(center, size));
  }
  let mut routes = all.concat();
  routes.push(vec![
    (bound.0, bound.1),
    (bound.2, bound.1),
    (bound.2, bound.3),
    (bound.0, bound.3),
    (bound.0, bound.1),
  ]);
  for effect in effects {
    routes = effect.apply(routes);
  }
  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let pad2 = 0.2 * pad;

  let shapes: Vec<Box<&dyn Shape>> = vec![
    Box::new(&HorizontalLines {
      ratio: 0.5,
      lines_distance: 1.8,
    }),
    Box::new(&VerticalLines {
      ratio: 2.0,
      lines_distance: 1.8,
    }),
    Box::new(&SpiralCircle { dr: 1.0 }),
  ];

  // let effects = vec![];

  let splitx = 3;
  let splity = 5;
  let mut all = Vec::new();
  for xi in 0..splitx {
    for yi in 0..splity {
      let w = (width - pad - pad2) / (splitx as f64) - pad2;
      let h = (height - pad - pad2) / (splity as f64) - pad2;
      let x1 = xi as f64 * (w + pad2) + pad;
      let y1 = yi as f64 * (h + pad2) + pad;
      let bound = (x1, y1, x1 + w, y1 + h);
      //let i = yi * splitx + xi;
      //if i < shapes.len() {
      let s = shapes.clone();
      all.push(frame(bound, s, vec![]));
      //}
    }
  }
  let routes = all.concat();

  vec![(routes, "black")]
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
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn circle_route(center: (f64, f64), r: f64, count: usize) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}
