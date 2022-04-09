use clap::Clap;
use gre::*;
use rand::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Group;

#[derive(Clap)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
  height: f64,
}

#[derive(Clone, Copy)]
struct TConfig {
  left_a_div: f64,
  right_a_div: f64,
  left_length_mul: f64,
  right_length_mul: f64,
  threshold_min: f64,
}
impl TConfig {
  fn new(
    left_a_div: f64,
    right_a_div: f64,
    left_length_mul: f64,
    right_length_mul: f64,
    threshold_min: f64,
  ) -> Self {
    TConfig {
      left_a_div,
      right_a_div,
      left_length_mul,
      right_length_mul,
      threshold_min,
    }
  }
}

#[derive(Clone, Copy)]
struct TLine {
  origin: (f64, f64),
  angle: f64,
  length: f64,
}

impl TLine {
  fn new(origin: (f64, f64), angle: f64, length: f64) -> Self {
    TLine {
      origin,
      angle,
      length,
    }
  }
  fn draw(self: Self, d: Data) -> Data {
    let mut data = d.move_to(self.origin);
    let x = self.length * self.angle.cos();
    let y = self.length * self.angle.sin();
    data = data.line_by((x, y));
    data
  }
  fn fork(self: Self, config: TConfig) -> Vec<TLine> {
    let mut v = Vec::new();
    let end = (
      self.origin.0 + self.length * self.angle.cos(),
      self.origin.1 + self.length * self.angle.sin(),
    );
    v.push(TLine::new(
      end,
      self.angle - config.left_a_div,
      self.length * config.left_length_mul,
    ));
    v.push(TLine::new(
      end,
      self.angle + config.right_a_div,
      self.length * config.right_length_mul,
    ));
    v
  }
  fn build(self: Self, level: usize, get_config: &dyn Fn(usize) -> TConfig) -> Vec<TLine> {
    let mut v = Vec::new();
    v.push(self);
    if level <= 0 {
      return v;
    }
    let c = get_config(level);
    if self.length < c.threshold_min {
      return v;
    }
    let children = self.fork(c);
    for child in children {
      let mut lines = child.build(level - 1, get_config);
      v.append(&mut lines);
    }
    v
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let color = "#000";
  let width = opts.width;
  let height = opts.height;
  let stroke_width = 0.35;
  let mut data = Data::new();
  let origin = (width / 2.0, height / 2.0);
  let max_level = 24;
  let initial_length = 12.;
  let angle_off = PI / 6.;
  let n = 3;
  let mut rng = rng_from_seed(opts.seed);
  let left_a_div = rng.gen_range(0.0, 1.6);
  let right_a_div = rng.gen_range(0.0, 1.6);
  let right_a_div_m = rng.gen_range(0.3, 0.9);
  let left_length_mul = rng.gen_range(0.7, 0.8);
  let right_length_mul = rng.gen_range(0.7, 0.8);
  let get_config = |level| {
    let l = level as f64 / (max_level as f64);
    TConfig::new(
      left_a_div,
      right_a_div + right_a_div_m * l,
      left_length_mul,
      right_length_mul,
      1.2,
    )
  };

  let mut tlines = Vec::new();
  for i in 0..n {
    let mut lines = TLine::new(
      origin,
      angle_off + i as f64 * 2. * PI / (n as f64),
      initial_length,
    )
    .build(max_level, &get_config);
    tlines.append(&mut lines);
  }

  data = tlines.iter().fold(data, |data, &tline| tline.draw(data));

  let mut l = layer(color);
  l = l.add(base_path(color, stroke_width, data));
  vec![l]
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
