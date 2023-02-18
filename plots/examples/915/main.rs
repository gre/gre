use clap::Parser;
use livedraw::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::f64::consts::PI;
use svg::node::element::path::Data;

// IDEA triangular raise instead of gaussian
// IDEA a second level curve using digits + a mix factor between the 2
// IDEA: sun as a dedicated step to place? only case of spiral?

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

type MountainValue = Vec<f64>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ArtInput {
  speed: RangeValue,
  scale: RangeValue,
  rotate: RangeValue,
  sky: RangeValue,
  amp: RangeValue,
  dy: RangeValue,
  polygon: PollValue,
  mountain: MountainValue,
}

#[derive(Clone)]
struct Art {
  args: Args,
  mountains: Mountains,
  sky: Option<ClippedShapeBuilder>,
  sky_started_at: usize,
  index: usize,
  remaining_mountains: usize,
  sky_limit: usize,
  sky_should_enable_next_time: bool,
}

impl Art {
  fn new(args: Args) -> Self {
    let mountains =
      Mountains::new(args.width, args.height, args.padding, args.height, 0.2);
    Art {
      args,
      mountains,
      sky_started_at: 0,
      sky: None,
      index: 0,
      remaining_mountains: 0,
      sky_limit: 200,
      sky_should_enable_next_time: false,
    }
  }
}

impl LivedrawArt for Art {
  fn get_dimension(&self) -> (f64, f64) {
    (self.args.width, self.args.height)
  }

  fn estimate_total_increments(&self) -> usize {
    self.index + self.remaining_mountains + self.sky_limit
  }

  fn actions_before_increment(&self, i: usize) -> Vec<ArtAction> {
    if i == 0 {
      return vec![ArtAction::Pause(
        String::from("Get ready to build mountains!"),
        30.0,
      )];
    }
    if self.sky_should_enable_next_time {
      return vec![ArtAction::Pause(
        String::from("Get ready to plot the sky!"),
        40.0,
      )];
    }
    return vec![];
  }

  fn get_predictive_max_next_increments(&self) -> Option<usize> {
    Some(100)
  }

  fn draw_increment(&mut self, value: &Value, index: usize) -> ArtIncrement {
    self.index = index;
    if self.sky.is_some() && index - self.sky_started_at > self.sky_limit {
      return ArtIncrement::End;
    }
    let input: ArtInput = serde_json::from_value(value.clone()).unwrap();

    let polygonsize = infer_polygon_size(input.polygon.winner);

    let mut rng = gre::rng_from_seed(self.args.seed + index as f64 / 3.3);

    let y = self.mountains.get_mountain_high_y();
    let limit = self.args.height * input.sky.value;

    self.remaining_mountains =
      ((limit - y) / input.dy.value).ceil().max(0.0) as usize;

    if self.sky_should_enable_next_time {
      let bound = (
        self.args.padding,
        self.args.padding,
        self.args.width - self.args.padding,
        self.mountains.get_mountain_low_y(),
      );
      self.sky_started_at = index;
      self.sky = Some(ClippedShapeBuilder::new(
        bound,
        (
          rng.gen_range(bound.0, bound.2),
          rng.gen_range(bound.1, bound.3),
        ),
        rng.gen_range(0.0, 2. * PI),
        input.scale.value,
        self.mountains.height_map.clone(),
        self.mountains.precision,
      ));
    }

    let mut routes = vec![];

    if index == 0 {
      let padding = self.args.padding;
      let width = self.args.width;
      let height = self.args.height;
      routes.push(vec![
        (padding, padding),
        (width - padding, padding),
        (width - padding, height - padding),
        (padding, height - padding),
        (padding, padding),
      ]);
    }

    if let Some(sky) = &mut self.sky {
      let r = sky.iterate(
        input.speed.value,
        input.rotate.value,
        input.scale.value,
        polygonsize,
      );
      if r.len() == 0 {
        if let Some(p) =
          sky.find_new_location(&mut rng, input.scale.value, 10000, polygonsize)
        {
          sky.set_location(p);
        } else {
          return ArtIncrement::End;
        }
      }
      routes.extend(r);
    } else {
      routes.extend(self.mountains.iterate(
        input.mountain,
        input.amp.value,
        input.dy.value,
      ));
    }

    self.sky_should_enable_next_time = self.sky.is_none() && y < limit;

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
        value: rng.gen_range(2.0, 3.0)
      },
      scale: RangeValue {
        value: rng.gen_range(1.0, 10.0)
      },
      rotate: RangeValue {
        value: rng.gen_range(-0.1, 0.1)
      },
      sky: RangeValue {
        value: rng.gen_range(0.35, 0.45)
      },
      amp: RangeValue {
        value: rng.gen_range(0.0, 10.0)
      },
      dy: RangeValue {
        value: rng.gen_range(0.3, 1.0)
      },
      polygon: PollValue {
        winner: vec!["triangle", "circle", "hexagon"][rng.gen_range(0, 3)]
          .to_string()
      },
      mountain: (0..26).map(|_| rng.gen_range(0.0, 1.0)).collect()
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn infer_polygon_size(str: String) -> usize {
  match str.as_str() {
    "circle" => 0,
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

fn clip_routes(
  input_routes: &Vec<Vec<(f64, f64)>>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<Vec<(f64, f64)>> {
  // locate the intersection where inside and outside cross
  let search = |inside: (f64, f64), outside: (f64, f64), n| {
    let mut a = inside;
    let mut b = outside;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if is_outside(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes.iter() {
    if input_route.len() < 2 {
      continue;
    }

    let mut prev = input_route[0];
    let mut prev_is_outside = is_outside(prev);
    let mut route = vec![];
    if !prev_is_outside {
      // prev is not to crop. we can start with it
      route.push(prev);
    }

    for &p in input_route.iter().skip(1) {
      // we iterate in small steps to detect any interruption
      let static_prev = prev;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let d = (dx * dx + dy * dy).sqrt();
      let vx = dx / d;
      let vy = dy / d;
      let iterations = (d / stepping).ceil() as usize;
      let mut v = 0.0;
      for _i in 0..iterations {
        v = (v + stepping).min(d);
        let q = (static_prev.0 + vx * v, static_prev.1 + vy * v);
        let q_is_outside = is_outside(q);
        if prev_is_outside != q_is_outside {
          // we have a crossing. we search it precisely
          let intersection = if prev_is_outside {
            search(q, prev, dichotomic_iterations)
          } else {
            search(prev, q, dichotomic_iterations)
          };

          if q_is_outside {
            // we close the path
            route.push(intersection);
            if route.len() > 1 {
              // we have a valid route to accumulate
              routes.push(route);
            }
            route = vec![];
          } else {
            // we open the path
            route.push(intersection);
          }
          prev_is_outside = q_is_outside;
        }

        prev = q;
      }

      // prev should be == p
      if !prev_is_outside {
        // prev is not to crop. we can start with it
        route.push(p);
      }
    }

    if route.len() > 1 {
      // we have a valid route to accumulate
      routes.push(route);
    }
  }

  routes
}

/**
 * A primitive to build mountains with a curve.
 */
#[derive(Clone)]
struct Mountains {
  canvas_width: f64,
  canvas_padding: f64,
  ybase: f64,
  precision: f64,
  height_map: Vec<f64>,
  lowest_reached: f64,
  bottom_reached: f64,
  reverse: bool,
}

impl Mountains {
  fn new(
    canvas_width: f64,
    canvas_height: f64,
    canvas_padding: f64,
    ybase: f64,
    precision: f64,
  ) -> Mountains {
    let count = (canvas_width / precision).ceil() as usize;
    let height_map = vec![canvas_height - canvas_padding; count];
    Mountains {
      canvas_width,
      canvas_padding,
      ybase,
      precision,
      height_map,
      lowest_reached: canvas_height,
      bottom_reached: canvas_height,
      reverse: false,
    }
  }

  fn get_mountain_high_y(&self) -> f64 {
    self.lowest_reached
  }
  fn get_mountain_low_y(&self) -> f64 {
    self.bottom_reached
  }

  fn iterate(
    &mut self,
    mountain_curve: Vec<f64>,
    amp: f64,
    dy: f64,
  ) -> Vec<Vec<(f64, f64)>> {
    let mut curve = vec![];
    let w = self.canvas_width - self.canvas_padding * 2.0;
    let mut x = self.canvas_padding;
    let xincr = w / (mountain_curve.len() as f64 - 1.0);
    for v in mountain_curve {
      let y = self.ybase - (v - 0.5) * amp;
      let p = (x, y);
      curve.push(p);
      x += xincr;
    }

    let mut lowest_reached = self.lowest_reached;
    let mut ymax = 0.0;
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
        let y = gre::mix(last.1, c.1, p);
        let i = (x / self.precision).round() as usize;
        let h = self.height_map[i];
        if y > ymax {
          ymax = y;
        }
        if y > h {
          let l = route.len();
          if l > 1 {
            routes.push(route);
            route = vec![];
          } else if l > 0 {
            route = vec![];
          }
        } else {
          if y < lowest_reached {
            lowest_reached = y;
          }
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

    if self.reverse {
      routes = routes
        .iter()
        .rev()
        .map(|r| r.iter().rev().cloned().collect())
        .collect();
    }

    self.ybase -= dy;

    self.lowest_reached = lowest_reached;
    self.bottom_reached = ymax.min(self.bottom_reached);
    self.reverse = !self.reverse;

    routes
  }
}

/**
 * A primitive to build sky area using shapes
 */
#[derive(Clone)]
struct ClippedShapeBuilder {
  bound: (f64, f64, f64, f64),
  shapes: Vec<Vec<(f64, f64)>>,
  circles: Vec<VCircle>,
  pos: (f64, f64),
  angle: f64,
  scale: f64,
  height_map: Vec<f64>,
  precision: f64,
  // todo allow circle shapes too
}

impl ClippedShapeBuilder {
  fn new(
    bound: (f64, f64, f64, f64),
    pos: (f64, f64),
    angle: f64,
    scale: f64,
    height_map: Vec<f64>,
    precision: f64,
  ) -> ClippedShapeBuilder {
    ClippedShapeBuilder {
      bound,
      shapes: vec![],
      circles: vec![],
      pos,
      angle,
      scale,
      height_map,
      precision,
    }
  }

  fn is_outside(&self, p: (f64, f64)) -> bool {
    let (x1, y1, x2, y2) = self.bound;
    p.0 < x1
      || p.0 > x2
      || p.1 < y1
      || p.1 > y2
      || p.1 > self.height_map[(p.0 / self.precision).round() as usize]
      || self
        .shapes
        .iter()
        .any(|shape| polygon_includes_point(shape, p))
      || self.circles.iter().any(|c| c.includes(p))
  }

  fn set_location(&mut self, p: (f64, f64)) {
    self.pos = p;
  }

  fn find_new_location<R: Rng>(
    &self,
    rng: &mut R,
    min_size: f64,
    max_retries: usize,
    polygonsize: usize,
  ) -> Option<(f64, f64)> {
    let (x1, y1, x2, y2) = self.bound;
    let mut p = self.pos;
    let mut retries = 0;
    let count = if polygonsize == 0 || polygonsize > 32 {
      32
    } else {
      polygonsize
    };
    loop {
      let mut ptns = vec![p];
      for ptn in circle_route(p, min_size, count, self.angle) {
        ptns.push(ptn);
      }
      if !ptns.iter().any(|p| self.is_outside(*p)) {
        return Some(p);
      }
      p = (rng.gen_range(x1, x2), rng.gen_range(y1, y2));
      retries += 1;
      if retries > max_retries {
        break;
      }
    }
    None
  }

  fn iterate(
    &mut self,
    speed: f64,
    turning: f64,
    scale: f64,
    polygonsize: usize,
  ) -> Vec<Vec<(f64, f64)>> {
    let (x1, y1, x2, y2) = self.bound;
    self.angle += turning;
    self.scale = scale;
    self.pos = (
      repeat_between(x1, x2, self.pos.0 + speed * self.angle.cos()),
      repeat_between(y1, y2, self.pos.1 + speed * self.angle.sin()),
    );

    let uses_circle = polygonsize == 0 || polygonsize > 32;

    let count = if uses_circle {
      (self.scale * 2.0 + 8.0) as usize
    } else {
      polygonsize
    };

    let route = circle_route(self.pos, self.scale, count, self.angle);

    let mut routes = vec![route.clone()];

    let uses_spiral = false; // uses_circle; // TODO this should be a param?

    if uses_spiral {
      let dr = 1.0;
      routes.push(gre::spiral_optimized(
        self.pos.0, self.pos.1, self.scale, dr, 0.1,
      ));
    }

    let is_outside = |p: (f64, f64)| self.is_outside(p);

    routes = clip_routes(&routes, &is_outside, 1.0, 3);

    if uses_circle {
      self
        .circles
        .push(VCircle::new(self.pos.0, self.pos.1, self.scale));
    } else {
      self.shapes.push(route);
    }

    routes
  }
}

fn polygon_includes_point(polygon: &Vec<(f64, f64)>, p: (f64, f64)) -> bool {
  let mut inside = false;
  let mut j = polygon.len() - 1;
  for i in 0..polygon.len() {
    let pi = polygon[i];
    let pj = polygon[j];
    if (pi.1 > p.1) != (pj.1 > p.1)
      && p.0 < (pj.0 - pi.0) * (p.1 - pi.1) / (pj.1 - pi.1) + pi.0
    {
      inside = !inside;
    }
    j = i;
  }
  inside
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
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    gre::euclidian_dist((self.x, self.y), p) < self.r
  }
}
