use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn square(c: (f64, f64), r: f64, angle: f64) -> Vec<(f64, f64)> {
  let mut v: Vec<(f64, f64)> = Vec::new();
  for i in 0..4 {
    let a = (angle + 0.5 + i as f64) * PI * 0.5;
    v.push((c.0 + r * a.cos(), c.1 + r * a.sin()));
  }
  v.push(v[0]);
  v
}

fn art(opts: Opts) -> Vec<Group> {
  let pad = 10.0;
  let width = 210.0;
  let height = 210.0;
  let frame = opts.index as f64 / (opts.frames as f64);
  let perlin = Perlin::new();

  let colors = vec!["black"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut routes = Vec::new();

      let mut rng = rng_from_seed(opts.seed);
      let freq = rng.gen_range(2000.0, 10000.0);
      let f1 = rng.gen_range(2.0, 100.0);
      let f2 = rng.gen_range(2.0, 100.0);
      let amp = 0.3;
      let l = 2.0;

      let mut passage = Passage2DCounter::new(1.0, width, height);

      for i in 0..10000 {
        let center = (
          width
            * (1.
              + perlin.get([
                6.6 + i as f64 / freq,
                5.0
                  + 4.4 * opts.seed
                  + amp
                    * perlin.get([6.6 + i as f64 / f1, 7.0 + opts.seed * 0.6]),
              ]))
            / 2.,
          height
            * (1.
              + perlin.get([
                9.0
                  + 7.7 * opts.seed
                  + amp * perlin.get([opts.seed / 3.7, i as f64 / f2]),
                8.888 + i as f64 / (freq * 0.9),
              ]))
            / 2.,
        );
        if center.0 < pad
          || center.1 < pad
          || center.0 > width - pad
          || center.1 > height - pad
        {
          continue;
        }
        if passage.count(center) < 3 {
          let ang = rng.gen_range(0.0, PI);
          routes.push(square(center, l, ang));
        }
      }

      println!("{}", routes.len());

      let mut data = Data::new();
      for route in routes {
        data = render_route(data, route);
      }

      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "0")]
  index: usize,
  #[clap(short, long, default_value = "8")]
  frames: usize,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_square("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
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
