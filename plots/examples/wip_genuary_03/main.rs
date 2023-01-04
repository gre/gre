use clap::Parser;
use gre::{grayscale, image_get_color};
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
  #[clap(long, default_value_t = 105.0)]
  width: f64,
  #[clap(long, default_value_t = 148.5)]
  height: f64,
  #[clap(long, default_value_t = 5.0)]
  padding: f64,
  #[clap(long, default_value_t = 3.0)]
  ratiomax: f64,
  #[clap(long, default_value_t = 1.0)]
  precision: f64,
  #[clap(long, default_value_t = 0.2)]
  offsetimpact: f64,
  #[clap(long, default_value_t = 54)]
  divisions: usize,
  #[clap(long, default_value_t = 7)]
  scales: usize,
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
struct PollValue {
  winner: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ArtInput {
  pos: XYValue,
  offset: RangeValue,
  glitches: RangeValue,
  scale: RangeValue,
  ratio: RangeValue,
  density: RangeValue,
  color: PollValue,
}

#[derive(Clone)]
struct Art {
  args: Args,
  is_glitch_phase: bool,
  line: usize,
  glitch_count: usize,
  glitches: usize,
  ink: String,
}

impl Art {
  fn new(args: Args) -> Self {
    Art {
      args,
      is_glitch_phase: false,
      line: 0,
      glitch_count: 0,
      glitches: 0,
      ink: String::from(""),
    }
  }
}

impl LivedrawArt for Art {
  fn delay_between_increments(&self) -> Duration {
    Duration::from_secs(1)
  }

  fn get_dimension(&self) -> (f64, f64) {
    (self.args.width, self.args.height)
  }

  fn estimate_total_increments(&self) -> usize {
    self.args.divisions + self.glitches
  }

  fn actions_before_increment(&self, i: usize) -> Vec<ArtAction> {
    if i == 0 {
      return vec![ArtAction::Pause(
        String::from("You wouldn't glitch Mona Lisa"),
        30.0,
      )];
    }
    if self.is_glitch_phase {
      return vec![
        // TODO don't say if ink don't change
        ArtAction::ChatMessage(String::from(format!(
          "!reset color – @greweb will use ink {}",
          self.ink
        ))),
        ArtAction::ChatMessage(String::from("!rand glitch")),
        ArtAction::Pause(String::from("Plan your next glitch!"), 30.0),
      ];
    }
    return vec![];
  }

  fn draw_increment(&mut self, value: &Value, _index: usize) -> ArtIncrement {
    let input: ArtInput = serde_json::from_value(value.clone()).unwrap();
    self.ink = input.color.winner;

    self.glitches = input.glitches.value.ceil() as usize;
    let args = self.args;

    let mut routes = vec![];

    if self.is_glitch_phase {
      if self.glitch_count >= self.glitches {
        return ArtIncrement::End;
      }

      let (xp, yp) = input.pos.value;
      let scale = input.scale.value;

      let r = input.ratio.value;
      let ratio = if r < 0.0 {
        mix(1.0, args.ratiomax, -r)
      } else if r > 0.0 {
        mix(1.0, 1.0 / args.ratiomax, r)
      } else {
        1.0
      };

      let w = scale * ratio;
      let h = scale / ratio;

      let p = args.padding + w;
      let xc = mix(p, args.width - p, 0.5 + 0.5 * xp);
      let p = args.padding + h;
      let yc = mix(p, args.height - p, 0.5 + 0.5 * yp);

      let delta = mix(0.8, 0.3, input.density.value);
      let mut x = (xc - w / 2.0).max(args.padding);
      let xmax = (xc + w / 2.0).min(args.width - args.padding);
      let ystart = (yc - h / 2.0)
        .max(args.padding)
        .min(args.height - args.padding);
      let ystop = (yc + h / 2.0)
        .max(args.padding)
        .min(args.height - args.padding);
      let mut route = vec![];
      let mut reverse = false;
      loop {
        if x > xmax {
          break;
        }
        let a = (x, ystart);
        let b = (x, ystop);
        if reverse {
          route.push(b);
          route.push(a);
        } else {
          route.push(a);
          route.push(b);
        }
        x += delta;
        reverse = !reverse;
      }
      routes.push(route);

      self.glitch_count += 1;
    }

    if self.line < args.divisions {
      let precision = args.precision;
      let get_color = image_get_color("../../images/monalisa.jpg").unwrap();
      let h = (args.height - args.padding * 2.0) / (args.divisions as f64);
      let ystart = args.padding + self.line as f64 * h;
      for i in 0..args.scales {
        let y = ystart + h * (i as f64 + 0.5) / (args.scales as f64);
        let mut x = args.padding;
        let mut route = vec![];
        let threshold = (interleaved_index(i, args.scales) as f64 + 1.0)
          / (args.scales as f64 + 1.0);
        loop {
          if x >= args.width - args.padding {
            break;
          }
          let lookup = (
            (lerp(args.padding, args.width - args.padding, x)
              - args.offsetimpact * input.offset.value
              + 2.0)
              % 1.0,
            lerp(args.padding, args.height - args.padding, y),
          );
          let should_draw = grayscale(get_color(lookup)) < threshold;
          if should_draw {
            if route.len() == 0 {
              route.push((x, y));
            }
          } else {
            if route.len() > 0 {
              route.push((x, y));
              routes.push(route);
              route = vec![];
            }
          }
          x += precision;
        }
        if route.len() > 0 {
          route.push((args.width - args.padding, y));
          routes.push(route);
        }
      }
      self.line += 1;
    } else {
      self.is_glitch_phase = true;
    }

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
        value: (rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0))
      },
      offset: RangeValue {
        value: rng.gen_range(-1.0, 1.0)
      },
      glitches: RangeValue {
        value: rng.gen_range(5.0, 20.0)
      },
      scale: RangeValue {
        value: rng.gen_range(1.0, 20.0)
      },
      ratio: RangeValue {
        value: rng.gen_range(-1.0, 1.0)
      },
      density: RangeValue {
        value: rng.gen_range(0.0, 1.0)
      },
      color: PollValue {
        winner: String::from(vec!["pink", "red", "mint"][rng.gen_range(0, 3)])
      }
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

fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1.0 - x) * a + x * b
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
  (x - a) / (b - a)
}

fn interleaved_index(i: usize, size: usize) -> usize {
  if i % 2 == 0 {
    return i;
  } else {
    return size - i - (size % 2);
  }
}
