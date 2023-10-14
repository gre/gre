use clap::*;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn make_step(
  o: (f64, f64),
  ang: f64,
  stepsize: f64,
  index: usize,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let dx = 0.5 * ((index % 2) as f64 - 0.5) * stepsize;
  let ang = ang + PI / 2.0;

  let mut routes = vec![];
  let s = 0.4;
  routes.push(vec![
    (dx - s, -0.5 * stepsize),
    (dx - s, -0.3 * stepsize),
    //
    (dx, -0.3 * stepsize),
    (dx, -0.5 * stepsize),
    //
    (dx + s, -0.5 * stepsize),
    (dx + s, -0.3 * stepsize),
  ]);
  routes.push(vec![
    (dx - s, -0.1 * stepsize),
    (dx - s, 0.5 * stepsize),
    //
    (dx, 0.5 * stepsize),
    (dx, -0.1 * stepsize),
    //
    (dx + s, -0.1 * stepsize),
    (dx + s, 0.5 * stepsize),
  ]);

  let mut out = vec![];
  for route in routes {
    let mut path = vec![];
    for p in route {
      let p = p_r(p, -ang);
      let p = (p.0 + o.0, p.1 + o.1);
      path.push(p);
    }
    out.push((clr, path));
  }
  out
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut routes = vec![];
  let mut rng = rng_from_seed(opts.seed);

  let sz = 5.0 + rng.gen_range(-3.0, 6.0) * rng.gen_range(0.0, 1.0);

  let circles = packing(
    &mut rng,
    1000000,
    5000,
    1,
    0.0,
    (pad, pad, width - pad, height - pad),
    sz,
    1.5 * sz,
  );

  let candidates = circles.iter().map(|c| (c.x, c.y)).collect();
  let path = tsp(candidates, time::Duration::seconds(opts.seconds));

  let path = path_subdivide_to_curve(path, 3, 0.75);

  let mut length = 0.0;
  for i in 0..(path.len() - 1) {
    length += euclidian_dist(path[i], path[i + 1]);
  }

  let footstep = 5.0;

  let mut v = 0.0;
  let mut i = 0;
  while v < length {
    let (p, a) = lookup_curve_point_and_angle(&path, v);
    routes.extend(make_step(p, a, footstep, i, 0));
    v += footstep * rng.gen_range(1.4, 1.5);
    i += 1;
  }
  // routes.push((0, path));

  vec!["#000"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
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
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "322.0")]
  seed: f64,
  #[clap(short, long, default_value = "297")]
  width: f64,
  #[clap(short, long, default_value = "420")]
  height: f64,
  #[clap(short, long, default_value = "30")]
  pad: f64,
  #[clap(short, long, default_value = "1")]
  seconds: i64,
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
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
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    !circles.iter().any(|other| c.collides(other))
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
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) =
      search_circle_radius(&circles, x, y, min_scale, max_scale)
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

fn lookup_curve_point_and_angle(
  path: &Vec<(f64, f64)>,
  l: f64,
) -> ((f64, f64), f64) {
  let mut i = 0;
  if l < 0.0 {
    return (path[0], angle2(path[0], path[1]));
  }
  let mut len = 0.0;
  while i < path.len() - 1 {
    let l1 = euclidian_dist(path[i], path[i + 1]);
    if len + l1 > l {
      let r = (l - len) / l1;
      let x = path[i].0 + r * (path[i + 1].0 - path[i].0);
      let y = path[i].1 + r * (path[i + 1].1 - path[i].1);
      let angle = angle2(path[i], path[i + 1]);
      return ((x, y), angle);
    }
    len += l1;
    i += 1;
  }
  return (
    path[path.len() - 1],
    angle2(path[path.len() - 2], path[path.len() - 1]),
  );
}

fn angle2(p1: (f64, f64), p2: (f64, f64)) -> f64 {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  let dx = x2 - x1;
  let dy = y2 - y1;
  dy.atan2(dx)
}
