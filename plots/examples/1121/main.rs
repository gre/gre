use clap::*;
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "600.0")]
  pub width: f64,
  #[clap(short, long, default_value = "500.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "1.0")]
  pub dr: f64,
  #[clap(short, long, default_value = "0.8")]
  pub circle_pad: f64,
  #[clap(short, long, default_value = "1.0")]
  pub circle_min: f64,
  #[clap(short, long, default_value = "10")]
  pub recursions: usize,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let dr = opts.dr;

  let mut routes = Vec::new();

  let bounds_container = VCircle::new(
    width / 2.0,
    height / 2.0,
    height / 2.0 - 2.0 * pad,
    CircleShape::Spiral,
  );
  let mut circles = rec_packing(
    opts.recursions,
    opts.seed,
    &bounds_container,
    opts.circle_pad,
    opts.circle_min,
  );
  println!("{} circles", circles.len());
  circles.push(bounds_container);

  let ppad = 2.0;
  let mins = 3.0;
  let maxs = 4.0;

  circles.extend(packing_rect_exclude_circle(
    opts.seed,
    1000000,
    5000,
    1,
    ppad,
    &VCircle::new(
      bounds_container.x,
      bounds_container.y,
      bounds_container.r + ppad,
      bounds_container.shape,
    ),
    (pad, pad, width - pad, height - pad),
    mins,
    maxs,
    CircleShape::Circle,
  ));

  let mut rng = rng_from_seed(opts.seed);

  for c in circles {
    let count = (6. + 2.0 * c.r) as usize;
    routes.push(circle_route((c.x, c.y), c.r, count));
    match c.shape {
      CircleShape::Spiral => {
        if c.r > 0.5 {
          let ddr = dr * rng.gen_range(0.8, 1.2);
          routes.push(spiral_optimized(c.x, c.y, c.r, ddr, 0.01));
        }
      }
      _ => {}
    };
  }

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
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

#[derive(Clone, Copy, Debug)]
enum CircleShape {
  Circle,
  Spiral,
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f64,
  y: f64,
  r: f64,
  shape: CircleShape,
}

impl VCircle {
  fn new(x: f64, y: f64, r: f64, shape: CircleShape) -> Self {
    VCircle { x, y, r, shape }
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
    let c = VCircle::new(x, y, size, CircleShape::Circle);
    container.contains(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn search_circle_radius_rect_exclude_circle(
  container: &(f64, f64, f64, f64),
  exclude: &VCircle,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size, CircleShape::Circle);
    !exclude.collides(&c)
      && c.x > container.0 + c.r
      && c.y > container.1 + c.r
      && c.x < container.2 - c.r
      && c.y < container.3 - c.r
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing_rect_exclude_circle(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  exclude: &VCircle,
  bound: (f64, f64, f64, f64),
  min_scale: f64,
  max_scale: f64,
  shape: CircleShape,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = bound.0;
  let y1 = bound.1;
  let x2 = bound.2;
  let y2 = bound.3;
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    if let Some(size) = search_circle_radius_rect_exclude_circle(
      &bound, &exclude, &circles, x, y, min_scale, max_scale,
    ) {
      let circle = VCircle::new(x, y, size - pad, shape);
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

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  container: &VCircle,
  min_scale: f64,
  max_scale: f64,
  shape: CircleShape,
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
      let circle = VCircle::new(x, y, size - pad, shape);
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
  pad: f64,
  min_scale: f64,
) -> Vec<VCircle> {
  let mut rng = rng_from_seed(seed);
  if i == 0 || rng.gen_bool(0.2) {
    return Vec::new();
  }
  let ppad =
    pad + rng.gen_range(0.0, 6.0) * rng.gen_range(-1.0, 1.0f64).max(0.0);
  let optimize = 1 + ((rng.gen_range(0.0, 100.0)) as usize);

  let max_scale =
    (min_scale + 0.1f64).max(rng.gen_range(0.2, 1.2) * container.r);

  let primaries = packing(
    seed,
    1000000,
    1000,
    optimize,
    ppad,
    &container,
    min_scale,
    max_scale,
    CircleShape::Circle,
  );

  primaries
    .par_iter()
    .filter(|p| p.r > pad)
    .map(|&p| {
      let seed = 7.7777 * p.x + 9.95731 * p.y + seed / 3.;
      let mut children = rec_packing(i - 1, seed, &p.clone(), pad, min_scale);
      let mut rng = rng_from_seed(seed);
      let shape = if rng.gen_bool(0.1) {
        CircleShape::Circle
      } else {
        CircleShape::Spiral
      };
      let circle = VCircle::new(p.x, p.y, p.r - pad / 2.0, shape);

      children.push(circle);
      children
    })
    .collect::<Vec<_>>()
    .concat()
  // vec![primaries, secondaries].concat()
}
