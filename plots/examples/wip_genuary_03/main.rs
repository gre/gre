/**
 * GLITCH ART
 * idea: scan lines on mona lisa
 * it is drawn by block (interleaved lines based on amount of color)
 * it is x offset by an input (-1, +1) kind of input
 * at the end, we have a 30s pause, switch of pen
 * N successive glitch (controlled by people)
 * each split by a 20s pause where i have time to change pen
 * there will be an intended message from the art that will "rand" that will reset pos,scale,ratio during that 20s
 * ink color as a poll
 * x, y positions are controlled by inputs (2D)
 * scale is an input, ratio is an input (instead of w,h)
 */
use clap::Parser;
use instant::Instant;
use livedraw::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{f64::consts::PI, time::Duration};
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
  #[clap(long)]
  simulation: bool,
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
  mv: XYValue,
  speed: RangeValue,
  scale: RangeValue,
  rotate: RangeValue,
}

#[derive(Clone)]
struct Art {
  args: Args,
  start_time: Instant,
  pos: (f64, f64),
}

impl Art {
  fn new(args: Args) -> Self {
    Art {
      args,
      start_time: Instant::now(),
      pos: (args.width / 2.0, args.height / 2.0),
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
    120
  }

  fn actions_before_increment(&self, i: usize) -> Vec<ArtAction> {
    if i == 0 {
      return vec![ArtAction::Pause(
        String::from("Get ready for 10 minutes!"),
        10.0,
      )];
    }
    return vec![];
  }

  fn draw_increment(&mut self, value: &Value, index: usize) -> ArtIncrement {
    if self.start_time.elapsed().as_secs() > 10 * 60 || index > 120 {
      return ArtIncrement::End;
    }

    let input: ArtInput = serde_json::from_value(value.clone()).unwrap();

    let r = input.scale.value;
    let p = r + self.args.padding;

    let center = (
      repeat_between(
        p,
        self.args.width - p,
        self.pos.0 + input.scale.value * input.mv.value.0,
      ),
      repeat_between(
        p,
        self.args.height - p,
        self.pos.1 + input.scale.value * input.mv.value.1,
      ),
    );

    self.pos = center;

    let routes = vec![circle_route(center, r, 3, input.rotate.value)];
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
      mv: XYValue {
        value: (rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0))
      },
      speed: RangeValue {
        value: rng.gen_range(0.0, 20.0)
      },
      scale: RangeValue {
        value: rng.gen_range(1.0, 20.0)
      },
      rotate: RangeValue {
        value: rng.gen_range(0.0, 10.0)
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
  let mut result = value;
  while result < min {
    result += range;
  }
  while result > max {
    result -= range;
  }
  result
}
