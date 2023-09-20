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
  #[clap(short, long, default_value = "200.0")]
  pub width: f64,
  #[clap(short, long, default_value = "200.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
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
  let bound = (pad, pad, width - pad, height - pad);

  let mut routes = Vec::new();
  let mut rng = rng_from_seed(opts.seed);

  let concensus_circle = VCircle::new(width / 2.0, height / 2.0, 0.15 * width);

  let dy = height * 1.2;
  let area_circle = VCircle::new(width / 2.0, dy, dy - height / 2.0);

  let mut circles = packing_rect_exclude_circle(
    opts.seed,
    100000,
    2000,
    1,
    0.5,
    &area_circle,
    bound,
    8.0,
    9.0,
  );

  circles = circles
    .iter()
    .filter(|p| {
      let c = VCircle::new(p.x, p.y, p.r + 4.0);
      !c.collides(&concensus_circle)
    })
    .cloned()
    .collect();

  circles.push(concensus_circle);

  let positions = circles
    .iter()
    .map(|c| (c.x, c.y))
    .collect::<Vec<(f64, f64)>>();

  let mut path = tsp(positions, time::Duration::seconds(5));

  // TODO move the path in order to isolate out the concensus position and find the "edge"

  path = path_subdivide_to_curve(path, 2, 0.8);

  routes.push(path);

  circles.pop();

  for c in circles {
    let count = 20;
    routes.push(circle_route((c.x, c.y), c.r, count));
  }

  let circles = packing_rect_include_circle(
    opts.seed,
    100000,
    2000,
    1,
    0.5,
    &concensus_circle,
    bound,
    2.0,
    3.0,
  );

  for c in circles {
    let count = 20;
    routes.push(circle_route((c.x, c.y), c.r, count));
  }

  let mut circles = packing_rect_include_circle(
    opts.seed,
    100000,
    2000,
    1,
    1.0,
    &VCircle::new(area_circle.x, area_circle.y, area_circle.r - 8.0),
    bound,
    4.0,
    5.0,
  );

  let exclude = VCircle::new(
    concensus_circle.x,
    concensus_circle.y,
    concensus_circle.r + 6.0,
  );

  circles = circles
    .iter()
    .filter(|p| !exclude.collides(&p))
    .cloned()
    .collect();

  for c in circles {
    let count = 20;
    routes.push(circle_route((c.x, c.y), c.r, count));
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
    let c = VCircle::new(x, y, size);
    !exclude.collides(&c)
      && c.x > container.0 + c.r
      && c.y > container.1 + c.r
      && c.x < container.2 - c.r
      && c.y < container.3 - c.r
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn search_circle_radius_rect_include_circle(
  container: &(f64, f64, f64, f64),
  inside: &VCircle,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    inside.collides(&c)
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

fn packing_rect_include_circle(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  inside: &VCircle,
  bound: (f64, f64, f64, f64),
  min_scale: f64,
  max_scale: f64,
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
    if let Some(size) = search_circle_radius_rect_include_circle(
      &bound, &inside, &circles, x, y, min_scale, max_scale,
    ) {
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

fn path_subdivide_to_curve_it(
  path: Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path;
  }
  let mut route = Vec::new();
  let mut first = path[0];
  let mut last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if looped {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for i in 1..(l - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if looped {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if looped {
    route.push(first);
  }
  route
}

fn path_subdivide_to_curve(
  path: Vec<(f64, f64)>,
  n: usize,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}

fn shake<R: Rng>(
  path: Vec<(f64, f64)>,
  scale: f64,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  path
    .iter()
    .map(|&(x, y)| {
      let dx = rng.gen_range(-scale, scale);
      let dy = rng.gen_range(-scale, scale);
      (x + dx, y + dy)
    })
    .collect()
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}
