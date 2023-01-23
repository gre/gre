use clap::Parser;
use gre::rng_from_seed;
use livedraw::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{f64::consts::PI, time::Duration};
use svg::node::element::path::Data;

// TODO rotate to be delta rotation
// TODO replace xy by just a head angle
// TODO angle component

#[derive(Debug, Parser, Clone, Copy)]
#[clap()]
struct Args {
  #[clap(long, default_value_t = 0.0)]
  seed: f64,
  #[clap(long, default_value_t = 210.0)]
  width: f64,
  #[clap(long, default_value_t = 297.0)]
  height: f64,
  #[clap(long, default_value_t = 10.0)]
  padding: f64,
  #[clap(long)]
  simulation: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RangeValue {
  value: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CounterBtnValue {
  value: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ArtInput {
  speed: RangeValue,
  scale: RangeValue,
  rotate: RangeValue,
  newtrail: CounterBtnValue,
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
  fn signed_distance(self: &Self, p: (f64, f64)) -> f64 {
    euclidian_dist((self.x, self.y), p)
  }
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    self.signed_distance(p) < self.r
  }
}

#[derive(Clone)]
struct Art {
  args: Args,
  pos: (f64, f64),
  ang: f64,
  circles: Vec<VCircle>,
  trail_index: i32,
  count_since_trail: usize,
}

impl Art {
  fn new(args: Args) -> Self {
    Art {
      args,
      pos: (args.width / 2.0, args.height / 3.0),
      ang: PI / 2.0,
      circles: vec![],
      trail_index: -1,
      count_since_trail: 0,
    }
  }
}

impl LivedrawArt for Art {
  fn delay_between_increments(&self) -> Duration {
    Duration::from_secs(1)
  }

  fn get_predictive_max_next_increments(&self) -> Option<usize> {
    Some(50)
  }

  fn get_dimension(&self) -> (f64, f64) {
    (self.args.width, self.args.height)
  }

  fn estimate_total_increments(&self) -> usize {
    10000
  }

  fn actions_before_increment(&self, i: usize) -> Vec<ArtAction> {
    if i == 0 {
      return vec![
        ArtAction::Pause(String::from("Get ready to draw circles"), 60.0),
        ArtAction::ChatMessage(String::from("Here we go. Play with controls or let it smoothly evolve. We need a balance of chaos and order! Don't hesitate to use !rand")),
      ];
    }
    return vec![];
  }

  fn draw_increment(&mut self, value: &Value, index: usize) -> ArtIncrement {
    let input: ArtInput = serde_json::from_value(value.clone()).unwrap();
    let scale = input.scale.value;
    let rotate = input.rotate.value;
    let speed = input.speed.value;
    let newtrail = input.newtrail.value as i32;

    let pad = scale + self.args.padding;
    let is_new_trail =
      self.trail_index != newtrail || self.count_since_trail > 100;
    self.trail_index = newtrail;

    if is_new_trail {
      self.count_since_trail = 0;
      let mut min_dist = 2.0 * scale;
      // find new location
      let mut is_valid = true;
      let mut rng = rng_from_seed(self.args.seed + (index as f64) * 7.7);
      for _i in 0..10000 {
        let p = (
          rng.gen_range(pad, self.args.width - pad),
          rng.gen_range(pad, self.args.height - pad),
        );
        is_valid = true;
        // check if not too close to existing circles
        for c in self.circles.iter() {
          if c.signed_distance(p) < c.r + scale + min_dist {
            is_valid = false;
            break;
          }
        }
        if is_valid {
          self.pos = p;
          break;
        }
        min_dist *= 0.95;
      }
      if !is_valid {
        println!("No more valid position found, ending at {}", index);
        return ArtIncrement::End;
      }
    }

    self.ang += rotate;

    self.pos = (
      repeat_between(
        pad,
        self.args.width - pad,
        self.pos.0 + speed * self.ang.cos(),
      ),
      repeat_between(
        pad,
        self.args.height - pad,
        self.pos.1 + speed * self.ang.sin(),
      ),
    );

    let count = 10 + (scale * 4.0) as usize;
    let mut routes = vec![circle_route(self.pos, scale, count, self.ang)];
    let dr = 0.4 + (self.count_since_trail as f64 / 8.0).powf(2.0);
    if dr < 3.0 {
      routes.push(spiral_optimized(self.pos.0, self.pos.1, scale, dr, 0.1));
    }

    let mut cutted = vec![];
    let should_crop = |p| {
      for c in self.circles.iter() {
        if c.includes(p) {
          return true;
        }
      }
      return false;
    };
    routes = crop_routes_with_predicate(&routes, &should_crop, &mut cutted);

    if routes.len() == 0 {
      return ArtIncrement::Continue;
    }

    self.count_since_trail += 1;

    self
      .circles
      .push(VCircle::new(self.pos.0, self.pos.1, scale));

    let data = routes.iter().fold(Data::new(), livedraw::render_route);

    let layers =
      vec![svg_layer("black").add(svg_base_path("black", 0.35, data))];

    return ArtIncrement::SVG(layers);
  }
}

impl LivedrawArtSimulation for Art {
  fn simulate_input(&mut self, index: usize) -> Value {
    let mut rng = rand::thread_rng();
    return json!(ArtInput {
      speed: RangeValue {
        value: rng.gen_range(0.0, 10.0)
      },
      scale: RangeValue {
        value: rng.gen_range(4.0, 20.0)
      },
      rotate: RangeValue {
        value: rng.gen_range(-0.1, 0.1)
      },
      newtrail: CounterBtnValue {
        value: (index / 50) as f64
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

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

pub fn spiral_optimized(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = (x + r * a.cos(), y + r * a.sin());
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 0.2 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
}

fn crop_routes_with_predicate(
  input_routes: &Vec<Vec<(f64, f64)>>,
  should_crop: &dyn Fn((f64, f64)) -> bool,
  cutted_points: &mut Vec<(f64, f64)>,
) -> Vec<Vec<(f64, f64)>> {
  let search = |a_, b_, n| {
    let mut a = a_;
    let mut b = b_;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if should_crop(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes {
    if input_route.len() < 2 {
      continue;
    }
    let mut prev = input_route[0];
    let mut route = vec![];
    if !should_crop(prev) {
      // prev is not to crop. we can start with it
      route.push(prev);
    } else {
      if !should_crop(input_route[1]) {
        // prev is to crop, but [1] is outside. we start with the exact intersection
        let intersection = search(input_route[1], prev, 7);
        prev = intersection;
        cutted_points.push(intersection);
        route.push(intersection);
      } else {
        cutted_points.push(prev);
      }
    }
    // cut segments with crop logic
    for &p in input_route.iter().skip(1) {
      // TODO here, we must do step by step to detect crop inside the segment (prev, p)

      if should_crop(p) {
        if route.len() > 0 {
          // prev is outside, p is to crop
          let intersection = search(prev, p, 7);
          cutted_points.push(intersection);
          route.push(intersection);
          routes.push(route);
          route = vec![];
        } else {
          cutted_points.push(p);
        }
      } else {
        // nothing to crop
        route.push(p);
      }
      prev = p;
    }
    if route.len() >= 2 {
      routes.push(route);
    }
  }

  routes
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}
