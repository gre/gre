use byteorder::*;
use clap::Parser;
use gre::rng_from_seed;
use livedraw::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{f64::consts::PI, fmt::Debug, time::Duration};
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
  sky: PollValue,
  amp: RangeValue,
  dy: RangeValue,
  top: RangeValue,
  layers: RangeValue,
  noise: RangeValue,
  rad: RangeValue,
}

#[derive(Clone)]
struct MountainArt {
  args: Args,
  ybase: f64,
  precision: f64,
  height_map: Vec<f64>,
  sky_drawn: bool,
  sky_reached: bool,
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
      sky_drawn: false,
      sky_reached: false,
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
    if self.sky_reached && !self.sky_drawn {
      return vec![
        ArtAction::Pause(String::from("Get ready to draw the sky!"), 60.0),
        ArtAction::ChatMessage(String::from("!freeze")),
      ];
    }
    if self.sky_reached && self.sky_drawn {
      return vec![ArtAction::ChatMessage(String::from("!unfreeze"))];
    }
    return vec![];
  }

  fn draw_increment(&mut self, value: &Value, index: usize) -> ArtIncrement {
    let input: MountainArtInput =
      serde_json::from_value(value.clone()).unwrap();

    let h = self.args.height * (1.0 - input.top.value);
    if self.sky_reached {
      if !self.sky_drawn {
        self.sky_drawn = true;
        self.total_increment_estimate = index + 1;

        let layers = input.layers.value.round() as usize;

        let f = 1.0 / input.noise.value;

        let all: Vec<Vec<Vec<(f64, f64)>>> = (0..layers)
          .into_par_iter()
          .map(|i| {
            let mut rng = rng_from_seed(self.args.seed + (i as f64));

            let pad = self.args.padding;
            let width = self.args.width;
            let height = self.args.height;
            let p_overlap = |p: &(f64, f64)| {
              p.0 > pad
                && p.1 > pad
                && p.0 < width - pad
                && p.1
                  < self.height_map[(p.0 / self.precision).floor() as usize]
            };
            let does_overlap = |c: &VCircle| {
              circle_route((c.x, c.y), c.r, 16).iter().all(p_overlap)
            };
            let min_scale = 2.0;
            let max_scale = min_scale + input.rad.value;

            let circles = packing(
              &mut rng,
              40000,
              1000,
              1,
              0.8,
              (pad, pad, width - pad, height - pad),
              &does_overlap,
              min_scale,
              max_scale,
            );

            let mut routes = vec![];
            let perlin = Perlin::new();

            for c in circles {
              if input.sky.winner == "circles" {
                let count = (8.0 + c.r * 3.0) as usize;
                routes.push(circle_route((c.x, c.y), c.r, count));
              } else if input.sky.winner == "squares" {
                routes.push(circle_route((c.x, c.y), c.r, 4));
              } else if input.sky.winner == "noise" {
                let a =
                  2.0 * perlin.get([f * c.x, f * c.y, self.args.seed / 7.7]);
                let dx = c.r * a.cos();
                let dy = c.r * a.sin();
                routes.push(vec![(c.x + dx, c.y + dy), (c.x - dx, c.y - dy)]);
              }
            }

            routes
          })
          .collect();

        let routes = all.concat();
        if routes.len() == 0 {
          return ArtIncrement::Continue;
        }
        let data = routes.iter().fold(Data::new(), livedraw::render_route);

        let layers =
          vec![svg_layer("black").add(svg_base_path("black", 0.35, data))];

        return ArtIncrement::SVG(layers);
      }
      return ArtIncrement::End;
    }

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
        if y > h {
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
      self.sky_reached = true;
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
      top: RangeValue {
        value: rng.gen_range(0.3, 1.0)
      },
      layers: RangeValue {
        value: rng.gen_range(1.0, 8.0)
      },
      noise: RangeValue {
        value: rng.gen_range(1.0, 1000.0)
      },
      rad: RangeValue {
        value: rng.gen_range(0.01, 10.0)
      },
      sky: PollValue {
        winner: String::from(
          vec!["circles", "noise", "squares"][rng.gen_range(0, 3)]
        )
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

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f64,
  y: f64,
  r: f64,
}
impl VCircle {
  fn new(x: f64, y: f64, r: f64) -> Self {
    VCircle { x, y, r }
  }
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

fn scaling_search<F: FnMut(f64) -> bool>(
  mut f: F,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let mut from = min_scale;
  let mut to = max_scale;
  loop {
    if !f(from) {
      return None;
    }
    if to - from < 0.1 {
      return Some(from);
    }
    let middle = (to + from) / 2.0;
    if !f(middle) {
      to = middle;
    } else {
      from = middle;
    }
  }
}

fn search_circle_radius(
  does_overlap: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing<R: Rng>(
  rng: &mut R,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) =
      search_circle_radius(&does_overlap, &circles, x, y, min_scale, max_scale)
    {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
}

fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}
