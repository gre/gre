use clap::Parser;
use gre::{mix, project_in_boundaries, strictly_in_boundaries};
use livedraw::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use svg::node::element::path::Data;

#[derive(Debug, Parser, Clone, Copy)]
#[clap()]
struct Args {
  #[clap(long, default_value_t = 0.0)]
  seed: f64,
  #[clap(long, default_value_t = 210.0)]
  width: f64,
  #[clap(long, default_value_t = 297.0)]
  height: f64,
  #[clap(long, default_value_t = 5.0)]
  padding: f64,
  #[clap(long, default_value_t = 8)]
  max_layers: usize,
  #[clap(long)]
  simulation: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PollValue {
  winner: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RangeValue {
  value: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct XYValue {
  value: (f64, f64),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ArtInput {
  pos: XYValue,
  more: PollValue,
  spirals: RangeValue,
  radius: RangeValue,
  density: RangeValue,
  dashed: RangeValue,
  dashlength: RangeValue,
}

#[derive(Clone)]
struct Art {
  args: Args,
  count_in_layer: usize,
  layer: usize,
}

impl Art {
  fn new(args: Args) -> Self {
    Art {
      args,
      count_in_layer: 0,
      layer: 0,
    }
  }
}

impl LivedrawArt for Art {
  fn delay_between_increments(&self) -> Duration {
    Duration::from_secs(5)
  }

  fn get_dimension(&self) -> (f64, f64) {
    (self.args.width, self.args.height)
  }

  fn estimate_total_increments(&self) -> usize {
    100
  }

  fn get_predictive_max_next_increments(&self) -> Option<usize> {
    Some(1)
  }

  fn actions_before_increment(&self, i: usize) -> Vec<ArtAction> {
    if i == 0 {
      return vec![ArtAction::Pause(String::from("Let's plot spirals!"), 30.0)];
    }
    if self.count_in_layer == 0 && i > 1 {
      return vec![
        ArtAction::ChatMessage(String::from("!reset more")),
        ArtAction::ChatMessage(String::from("!rand")),
        ArtAction::Pause(String::from("Ink change!"), 30.0),
      ];
    }

    return vec![ArtAction::ChatMessage(String::from("!pos rand"))];
  }

  fn draw_increment(&mut self, value: &Value, index: usize) -> ArtIncrement {
    if self.layer >= self.args.max_layers {
      return ArtIncrement::End;
    }
    let input: ArtInput = serde_json::from_value(value.clone()).unwrap();
    if self.count_in_layer == 0 && input.more.winner == "no" {
      return ArtIncrement::End;
    }

    self.count_in_layer += 1;
    if self.count_in_layer >= input.spirals.value.ceil() as usize {
      self.count_in_layer = 0;
      self.layer += 1;
    }

    let args = self.args;
    let padding = args.padding;
    let width = args.width;
    let height = args.height;
    let bound = (padding, padding, width - padding, height - padding);

    let p = project_in_boundaries(input.pos.value, bound);
    let r = input.radius.value;

    let full_length = input.dashlength.value as usize;
    let stroke_length =
      (full_length as f64 * mix(1.05, 0.2, input.dashed.value)) as usize;
    let dr = mix(2.0, 0.4, input.density.value);

    let mut routes = spiral_dashed(p, r, dr, stroke_length, full_length);

    routes = repeat_in_bound(routes, bound);

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
      pos: XYValue {
        value: (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))
      },
      more: PollValue {
        winner: String::from(if rng.gen_bool(0.3) { "no" } else { "yes" })
      },
      spirals: RangeValue {
        value: rng.gen_range(1.0, 20.0)
      },
      radius: RangeValue {
        value: rng.gen_range(10.0, 80.0)
      },
      density: RangeValue {
        value: rng.gen_range(0.0, 1.0)
      },
      dashed: RangeValue {
        value: rng.gen_range(0.0, 1.0)
      },
      dashlength: RangeValue {
        value: rng.gen_range(5., 100.)
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

fn spiral_dashed(
  (x, y): (f64, f64),
  r: f64,
  dr: f64,
  stroke_length: usize,
  full_length: usize,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = vec![];
  let route = gre::spiral_optimized(x, y, r, dr, 0.1);
  if stroke_length >= full_length {
    routes.push(route);
  } else {
    for route in route
      .chunks(full_length)
      .map(|r| r.iter().take(stroke_length).map(|&p| p).collect())
    {
      routes.push(route);
    }
  }
  routes
}

fn constraint_in_bound(
  routes: Vec<Vec<(f64, f64)>>,
  bound: (f64, f64, f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  let mut copy = vec![];
  for route in routes {
    let mut r = vec![];
    for p in route {
      if strictly_in_boundaries(p, bound) {
        r.push(p);
      } else {
        if r.len() > 1 {
          copy.push(r);
          r = vec![];
        } else if r.len() == 1 {
          r = vec![];
        }
      }
    }
    if r.len() > 1 {
      copy.push(r);
    }
  }
  copy
}

fn translate_routes(
  routes: Vec<Vec<(f64, f64)>>,
  (tx, ty): (f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    .collect()
}

fn repeat_in_bound(
  routes: Vec<Vec<(f64, f64)>>,
  bound: (f64, f64, f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  let w = bound.2 - bound.0;
  let h = bound.3 - bound.1;
  constraint_in_bound(
    vec![
      routes.clone(),
      translate_routes(routes.clone(), (w, 0.0)),
      translate_routes(routes.clone(), (-w, 0.0)),
      translate_routes(routes.clone(), (w, h)),
      translate_routes(routes.clone(), (w, -h)),
      translate_routes(routes.clone(), (-w, h)),
      translate_routes(routes.clone(), (-w, -h)),
      translate_routes(routes.clone(), (0.0, h)),
      translate_routes(routes.clone(), (0.0, -h)),
    ]
    .concat(),
    bound,
  )
}
