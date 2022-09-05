use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::{path::Data, *};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
  height: f64,
}

fn render_angle_velocity_field(
  width: f64,
  height: f64,
  pad: f64,
  lines: usize,
  line_length: f64,
  dash_length: f64,
  angle_velocity_field: impl Fn(f64, f64, f64, f64) -> f64,
  origin: impl Fn(usize) -> (f64, f64),
  initial_angle: impl Fn(usize) -> f64,
) -> Group {
  let mut data = Data::new();
  let step = 1.0;
  let mut routes = Vec::new();

  for l in 0..lines {
    let mut angle = initial_angle(l);
    let mut length = 0.0;
    let mut p = origin(l);
    let mut route = Vec::new();
    route.push(p);
    loop {
      let a = angle_velocity_field(
        p.0 / width,
        p.1 / height,
        (l as f64) / (lines as f64),
        length / width.min(height),
      );
      angle += step * a;
      p.0 += step * angle.cos();
      p.1 += step * angle.sin();
      route.push(p);
      length += step;
      if length > line_length + dash_length {
        break;
      }
    }
    routes.push(route);
  }

  let mut passage = Passage2DCounter::new(0.4, width, height);
  let mut paths = Vec::new();
  for route in routes {
    let mut path = Vec::new();
    let mut length = 0.0;
    let mut i = 0;
    let mut last_p = (0.0, 0.0);
    let dashstyle = 8;
    for &p in route.iter().rev() {
      if p.0 < pad || p.1 < pad || p.0 > width - pad || p.1 > height - pad {
        // out of bounds. reset
        path = Vec::new();
        last_p = (0.0, 0.0);
      } else {
        if passage.count(p) > 5 {
          break;
        }
        if length < dash_length {
          if i % dashstyle == 0 {
            last_p = p;
          } else if i % dashstyle == 1 && last_p.0 > 0.0 {
            paths.push(vec![last_p, p]);
          }
        } else {
          path.push(p);
        }
      }
      length += step;
      i += 1;
    }
    paths.push(path);
  }

  for path in paths {
    data = render_route(data, path);
  }

  return Group::new().add(
    Path::new()
      .set("fill", "none")
      .set("stroke", "black")
      .set("stroke-width", 0.32)
      .set("d", data),
  );
}

pub struct Passage2DCounter {
  granularity: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage2DCounter {
  pub fn new(granularity: f64, width: f64, height: f64) -> Self {
    let wi = (width / granularity).ceil() as usize;
    let hi = (height / granularity).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage2DCounter {
      granularity,
      width,
      height,
      counters,
    }
  }
  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.granularity).ceil() as usize;
    let hi = (self.height / self.granularity).ceil() as usize;
    let xi = ((x / self.granularity).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.granularity).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }
  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }
  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    self.counters[self.index(p)]
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let seed = opts.seed;
  let pad = 8.0;
  let width = opts.width;
  let height = opts.height;
  let lines = 500;
  let perlin = Perlin::new();
  let mut rng = rng_from_seed(seed);
  let line_length = rng.gen_range(10.0, 50.0);
  let dash_length = 40.0;
  let amp: f64 = rng.gen_range(0.08, 0.4);
  let angle_velocity_field = |x, y, _l, _length| {
    amp
      * (0.5 * perlin.get([7.0 * x, 7.0 * y, seed])
        + 0.3 * perlin.get([11.0 * x, 11.0 * y, seed])
        + 0.2 * perlin.get([21.0 * x, 21.0 * y, seed]))
  };
  let w = rng.gen_range(6., 12.);
  let wf = 2.;
  let origin = |l| {
    (
      width / 2.0 + w * (l as f64 * wf).sin(),
      height * (1.0 - 0.5 * (l as f64 + 0.5) / (lines as f64)),
    )
  };
  let initial_angle = |_l| -PI / 2.0;
  let art = render_angle_velocity_field(
    width,
    height,
    pad,
    lines,
    line_length,
    dash_length,
    angle_velocity_field,
    origin,
    initial_angle,
  );
  vec![art]
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
