use clap::Parser;
use instant::Instant;
use livedraw::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::f64::consts::PI;
use svg::node::element::path::Data;

// TODO rotate to be delta rotation
// TODO replace xy by just a head angle
// TODO angle component

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
struct RangeValue {
  value: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PollValue {
  winner: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ArtInput {
  speed: RangeValue,
  scale: RangeValue,
  rotate: RangeValue,
  polygon: PollValue,
}

#[derive(Clone)]
struct Art {
  args: Args,
  start_time: Instant,
  pos: (f64, f64),
  ang: f64,
}

impl Art {
  fn new(args: Args) -> Self {
    Art {
      args,
      start_time: Instant::now(),
      pos: (args.width / 2.0, args.height / 3.0),
      ang: PI / 2.0,
    }
  }
}

impl LivedrawArt for Art {
  fn get_dimension(&self) -> (f64, f64) {
    (self.args.width, self.args.height)
  }

  fn estimate_total_increments(&self) -> usize {
    200
  }

  fn actions_before_increment(&self, i: usize) -> Vec<ArtAction> {
    if i == 0 {
      return vec![ArtAction::Pause(
        String::from("Get ready to draw 200 shapes!"),
        20.0,
      )];
    }
    return vec![];
  }

  fn get_predictive_max_next_increments(&self) -> Option<usize> {
    Some(40)
  }

  fn draw_increment(&mut self, value: &Value, index: usize) -> ArtIncrement {
    if index > 200 {
      return ArtIncrement::End;
    }

    let input: ArtInput = serde_json::from_value(value.clone()).unwrap();

    let r = input.scale.value;
    let p = r + self.args.padding;

    self.ang += input.rotate.value;

    self.pos = (
      repeat_between(
        p,
        self.args.width - p,
        self.pos.0 + input.speed.value * self.ang.cos(),
      ),
      repeat_between(
        p,
        self.args.height - p,
        self.pos.1 + input.speed.value * self.ang.sin(),
      ),
    );

    let routes = vec![circle_route(
      self.pos,
      r,
      infer_polygon_size(input.polygon.winner),
      self.ang,
    )];
    if routes.len() == 0 {
      return ArtIncrement::Continue;
    }
    let data = routes.iter().fold(Data::new(), livedraw::render_route);

    let layers =
      vec![svg_layer("black").add(svg_base_path("black", 0.35, data))];

    return ArtIncrement::SVG(layers);
  }
}

impl LivedrawArtSimulation for Art {
  fn simulate_input(&mut self, _index: usize) -> Value {
    let mut rng = rand::thread_rng();
    return json!(ArtInput {
      speed: RangeValue {
        value: rng.gen_range(0.0, 20.0)
      },
      scale: RangeValue {
        value: rng.gen_range(1.0, 20.0)
      },
      rotate: RangeValue {
        value: rng.gen_range(-1.0, 1.0)
      },
      polygon: PollValue {
        winner: vec!["triangle", "circle", "hexagon"][rng.gen_range(0, 3)]
          .to_string()
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

fn circle_route(
  center: (f64, f64),
  r: f64,
  count: usize,
  rotation: f64,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = rotation + 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

fn repeat_between(min: f64, max: f64, value: f64) -> f64 {
  let range = max - min;
  if range <= 0.0 {
    return value;
  }
  let mut result = value;
  while result < min {
    result += range;
  }
  while result > max {
    result -= range;
  }
  result
}

fn infer_polygon_size(str: String) -> usize {
  match str.as_str() {
    "circle" => 64,
    "triangle" => 3,
    "square" => 4,
    "pentagon" => 5,
    "hexagon" => 6,
    "heptagon" => 7,
    "octagon" => 8,
    "enneagon" => 9,
    "decagon" => 10,
    "hendecagon" => 11,
    "dodecagon" => 12,
    "tridecagon" => 13,
    "tetradecagon" => 14,
    "pentadecagon" => 15,
    "hexadecagon" => 16,
    "heptadecagon" => 17,
    "octadecagon" => 18,
    "enneadecagon" => 19,
    "icosagon" => 20,
    "henicosagon" => 21,
    "dicosagon" => 22,
    "tricosagon" => 23,
    "tetracosagon" => 24,
    "pentacosagon" => 25,
    "hexacosagon" => 26,
    _ => 3,
  }
}
