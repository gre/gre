use clap::Parser;
use gre::mix;
use livedraw::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use svg::node::element::path::Data;

#[derive(Parser, Clone, Copy)]
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

#[derive(Deserialize, Serialize, Clone)]
struct PollValue {
  winner: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct RangeValue {
  value: f64,
}

type MountainValue = Vec<f64>;

#[derive(Deserialize, Serialize, Clone)]
struct MountainArtInput {
  mountain: MountainValue,
  amp: RangeValue,
  dy: RangeValue,
}

#[derive(Clone)]
struct MountainArt {
  args: Args,
  ybase: f64,
  precision: f64,
  height_map: Vec<f64>,
  total_increment_estimate: usize,
}

impl MountainArt {
  fn new(args: Args) -> Self {
    let ybase = args.height - 5.0;
    let precision = 0.2;
    let count = (args.width / precision).ceil() as usize;
    let height_map = vec![args.height - args.padding; count];
    MountainArt {
      args,
      ybase,
      precision,
      height_map,
      total_increment_estimate: 1,
    }
  }
}

impl LivedrawArt for MountainArt {
  fn delay_between_increments(&self) -> Duration {
    Duration::from_secs(8)
  }

  fn get_dimension(&self) -> (f64, f64) {
    (self.args.width, self.args.height)
  }

  fn estimate_total_increments(&self) -> usize {
    self.total_increment_estimate
  }

  fn actions_before_increment(&self, i: usize) -> Vec<ArtAction> {
    if i == 0 {
      return vec![ArtAction::Pause(
        String::from("Get ready to shape the mountains!"),
        30.0,
      )];
    }
    return vec![];
  }

  fn draw_increment(&mut self, value: &Value, index: usize) -> ArtIncrement {
    let input: MountainArtInput =
      serde_json::from_value(value.clone()).unwrap();

    let h = self.args.padding;

    self.total_increment_estimate =
      index + 1 + ((self.ybase - h) / input.dy.value).ceil() as usize;

    let mountain_curve = input.mountain;

    let mut curve = vec![];
    let w = self.args.width - self.args.padding * 2.0;
    let mut x = self.args.padding;
    let xincr = w / (mountain_curve.len() as f64 - 1.0);
    for v in mountain_curve {
      let y = self.ybase - (v - 0.5) * input.amp.value;
      let p = (x, y);
      curve.push(p);
      x += xincr;
    }

    let mut last = curve[0];
    let mut routes = vec![];
    let mut route = vec![];
    for &c in curve.iter().skip(1) {
      let mut x = last.0;
      loop {
        if x > c.0 {
          break;
        }
        let p = (x - last.0) / (c.0 - last.0);
        let y = mix(last.1, c.1, p);
        let i = (x / self.precision).round() as usize;
        let h = self.height_map[i];
        if y > h || y < self.args.padding {
          let l = route.len();
          if l > 1 {
            routes.push(route);
            route = vec![];
          } else if l > 0 {
            route = vec![];
          }
        } else {
          self.height_map[i] = y;
          route.push((x, y));
        }
        x += self.precision;
      }
      last = c;
    }
    if route.len() > 1 {
      routes.push(route);
    }

    self.ybase -= input.dy.value;

    if self.ybase < h {
      return ArtIncrement::End;
    }

    if routes.len() == 0 {
      return ArtIncrement::Continue;
    }

    let data = routes.iter().fold(Data::new(), render_route);

    let layers =
      vec![svg_layer("black").add(svg_base_path("black", 0.35, data))];

    ArtIncrement::SVG(layers)
  }
}

impl LivedrawArtSimulation for MountainArt {
  fn simulate_input(&mut self, _index: usize) -> Value {
    let mut rng = rand::thread_rng();
    return json!(MountainArtInput {
      amp: RangeValue {
        value: rng.gen_range(0.0, 40.0)
      },
      dy: RangeValue {
        value: rng.gen_range(0.4, 3.0)
      },
      mountain: (0..26)
        .map(|_i| rng.gen_range(0.0, 1.0))
        .collect::<Vec<_>>()
    });
  }
}

fn main() {
  let args = Args::parse();
  let mut art = MountainArt::new(args.clone());
  if args.simulation {
    livedraw_start_simulation(&mut art);
  } else {
    livedraw_start(&mut art);
  }
}
