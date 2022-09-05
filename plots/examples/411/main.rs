use clap::*;
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "42.")]
  seed: f64,
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
  fn contains(self: &Self, c: &VCircle) -> bool {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - self.r + c.r < 0.0
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
  container: &VCircle,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    container.contains(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  container: &VCircle,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = container.x - container.r;
  let y1 = container.y - container.r;
  let x2 = container.x + container.r;
  let y2 = container.y + container.r;
  let max_scale = max_scale.min(container.r);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    if let Some(size) =
      search_circle_radius(&container, &circles, x, y, min_scale, max_scale)
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

fn rec_packing(
  i: usize,
  seed: f64,
  container: &VCircle,
) -> Vec<(usize, VCircle)> {
  if container.r < 4. {
    return Vec::new();
  }
  let pad = 0.1 + 0.6 * ((i % 3) as f64);
  let primaries =
    packing(seed, 1000000, 1500, 40, pad, &container, 0.2 + pad, 80.0);

  let secondaries = primaries
    .par_iter()
    .filter(|p| p.r > pad)
    .map(|p| rec_packing(i + 1, 7.7777 * p.x + 9.95731 * p.y + seed / 3., &p))
    .collect::<Vec<_>>()
    .concat();

  let circles: Vec<(usize, VCircle)> =
    primaries.iter().map(|&c| ((i / 3) % 2, c)).collect();

  vec![circles, secondaries].concat()
}

fn art(opts: Opts) -> Vec<Group> {
  let width = 297.0;
  let height = 210.0;
  let pad = 10.0;
  let stroke_width = 0.3;

  let bounds_container =
    VCircle::new(width / 2.0, height / 2.0, height / 2.0 - pad);
  let mut circles = rec_packing(0, opts.seed, &bounds_container);
  circles.push((0, bounds_container));

  println!("{} circles", circles.len());

  let colors = vec!["#09F", "#F90"];

  colors
    .iter()
    .enumerate()
    .map(|(ci, &color)| {
      let mut l = layer(color);
      for &(color_index, c) in circles.iter() {
        if ci == color_index {
          l = l.add(
            Circle::new()
              .set("r", c.r)
              .set("cx", c.x)
              .set("cy", c.y)
              .set("stroke", color)
              .set("stroke-width", stroke_width)
              .set("fill", "none")
              .set("style", "mix-blend-mode: multiply;"),
          );
        }
      }
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
