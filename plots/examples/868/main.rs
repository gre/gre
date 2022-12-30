use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let clrs = vec![
    ("Aurora Borealis", "darkcyan"),
    ("Iroshizuku", "DarkKhaki"),
    ("Moonstone", "DarkGray"),
    ("Amazing Amethyst", "DarkMagenta"),
    //("Turquoise", "#07f"),
    //("Skull and Roses", "#106"),
    // ("Indigo", "#447"),
    // ("Violet", "#70a"),
  ];

  let mut routes = Vec::new();

  let bound = (pad, pad, width - pad, height - pad);

  let in_shape = |p: (f64, f64)| -> bool { strictly_in_boundaries(p, bound) };

  let does_overlap = |c: &VCircle| {
    circle_route((c.x, c.y), c.r, 32)
      .iter()
      .all(|&p| in_shape(p))
  };

  let mut rng = rng_from_seed(opts.seed);

  let optimize_incr = rng.gen_range(0, 4);
  let passes = rng.gen_range(4, 6);
  let max_scale = rng.gen_range(0.1, 0.3) * height;
  let circles: Vec<VCircle> = (0..passes)
    .into_par_iter()
    .flat_map(|i| {
      packing(
        opts.seed + i as f64 / 0.07,
        200000,
        1000,
        1 + i * optimize_incr,
        0.5,
        bound,
        &does_overlap,
        3.0,
        max_scale,
      )
    })
    .collect();

  let perlin = Perlin::new();

  let mut stats = vec![0.0; clrs.len()];

  let f = rng.gen_range(0.01, 2.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);
  let mult = rng.gen_range(0.2, 2.0) * rng.gen_range(0.0, 1.0);
  let offset = rng.gen_range(1.0, 5.0);
  let freq = rng.gen_range(0.0, 0.05) * rng.gen_range(0.0, 1.0);
  let disp = rng.gen_range(0.0, 10.0);
  let dispersion = 0.3;
  for (i, &c) in circles.iter().enumerate() {
    let v = offset
      + mult * perlin.get([f * c.x, f * c.y, opts.seed])
      + dispersion * rng.gen_range(-0.5, 0.5);
    let clr = ((clrs.len() - 1) as f64 * v) as usize % clrs.len();
    stats[clr] += c.r * c.r;
    let dr = rng.gen_range(0.6, 1.4);
    let mut route = spiral_optimized(c.x, c.y, c.r, dr, 0.1);
    route = route
      .iter()
      .map(|p| {
        let a = 2.0
          * perlin.get([
            freq * p.0,
            freq * p.1,
            i as f64 / 0.037
              + opts.seed / 0.077
              + perlin.get([2.0 * freq * p.0, 2.0 * freq * p.1, opts.seed]),
          ]);
        (p.0 + disp * a.cos(), p.1 + disp * a.sin())
      })
      .collect();
    routes.push((clr, route));
  }

  let mut s: Vec<(usize, f64)> =
    stats.iter().enumerate().map(|(i, &f)| (i, f)).collect();
  s.sort_by(|a, &b| b.1.partial_cmp(&a.1).unwrap());

  clrs
    .iter()
    .enumerate()
    .map(|(ci, (label, css))| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*label)).as_str());
      l = l.add(base_path(css, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
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

fn packing(
  seed: f64,
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
  let mut rng = rng_from_seed(seed);
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
