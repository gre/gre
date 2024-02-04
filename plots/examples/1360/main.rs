use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element;
use svg::node::element::path::Data;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  width: f64,
  #[clap(short, long, default_value = "420.0")]
  height: f64,
  #[clap(short, long, default_value = "10.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn square_spiral(
  c: (f64, f64),
  r: f64,
  initial_a: f64,
  d_length: f64,
) -> Vec<(f64, f64)> {
  let mut route = vec![];
  let mut a: f64 = initial_a;
  let length = r * 2. / (2. as f64).sqrt();
  let delta = p_r((-length / 2., length / 2.), a);
  let mut p = (c.0 + delta.0, c.1 + delta.1);
  let mut l = length;
  let mut i = 0;
  route.push(p);
  loop {
    if l < 0.0 {
      break;
    }
    p = (p.0 + l * a.cos(), p.1 + l * a.sin());
    route.push(p);
    a -= PI / 2.;
    if i > 0 {
      l -= d_length;
    }
    i += 1;
  }
  route
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn path_subdivide_to_curve_it(
  path: &Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path.clone();
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

struct PathLookup {
  path: Vec<(f64, f64)>,
  local_length: Vec<f64>,
  partial_length: Vec<f64>,
  length: f64,
}

impl PathLookup {
  fn init(path: Vec<(f64, f64)>) -> Self {
    let mut length = 0.0;
    let mut partial_length = vec![];
    let mut local_length = vec![];
    for i in 0..path.len() - 1 {
      partial_length.push(length);
      let l = euclidian_dist(path[i], path[i + 1]);
      local_length.push(l);
      length += l;
    }
    partial_length.push(length);

    Self {
      path,
      length,
      local_length,
      partial_length,
    }
  }

  fn length(&self) -> f64 {
    self.length
  }

  fn lookup_pos(&self, l: f64) -> (f64, f64) {
    let path = &self.path;
    if l < 0.0 {
      return path[0];
    }
    for i in 0..path.len() - 1 {
      let pl = self.partial_length[i + 1];
      if l < pl {
        let a = path[i];
        let b = path[i + 1];
        let m = (pl - l) / self.local_length[i];
        return lerp_point(b, a, m);
      }
    }
    return path[path.len() - 1];
  }

  fn lookup_angle(&self, l: f64) -> f64 {
    let path = &self.path;
    if l < 0.0 {
      return angle2(path[0], path[1]);
    }
    for i in 0..path.len() - 1 {
      let pl = self.partial_length[i + 1];
      if l < pl {
        let a = path[i];
        let b = path[i + 1];
        let angle = angle2(a, b);
        return angle;
      }
    }
    let len = path.len();
    return angle2(path[len - 2], path[len - 1]);
  }
}

fn angle2(p1: (f64, f64), p2: (f64, f64)) -> f64 {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  let dx = x2 - x1;
  let dy = y2 - y1;
  dy.atan2(dx)
}

fn art(opts: &Opts) -> Vec<element::Group> {
  let width = opts.width;
  let height = opts.height;
  let mut rng = rng_from_seed(opts.seed);
  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = vec![];
  let clrs = vec!["black"];

  let mut route;

  route = square_spiral((width / 2., height / 2.), 180.0, 0.0, 5.0);

  route = path_subdivide_to_curve_it(&route, rng.gen_range(0.6, 0.9));
  route = path_subdivide_to_curve_it(&route, 0.8);
  route = path_subdivide_to_curve_it(&route, 0.75);
  route = path_subdivide_to_curve_it(&route, 0.75);
  route = path_subdivide_to_curve_it(&route, 0.7);
  route = path_subdivide_to_curve_it(&route, 0.7);

  let lookup = PathLookup::init(route.clone());

  let count = 10000;
  let skipstart = 40.0;
  let ampstart = 80.0;
  let maxamp = rng.gen_range(6.0, 10.0);
  let minamp = rng.gen_range(0.0, 1.0) * maxamp;
  let rngcurve = rng.gen_range(0.0, 1.0);
  let mut route = vec![];
  for i in 0..count {
    let disp = rng.gen_range(0.0, 1.0);
    let amp = rng.gen_range(minamp, maxamp) * rng.gen_range(rngcurve, 1.0);
    let l = skipstart
      + (lookup.length() - skipstart) * (i as f64 + disp) / count as f64;
    let amp = amp * smoothstep(skipstart, skipstart + ampstart, l);
    let p = lookup.lookup_pos(l);
    let a = lookup.lookup_angle(l);
    let ang = if i % 2 == 0 {
      a - PI / 2.0
    } else {
      a + PI / 2.0
    };
    let p = (p.0 + amp * ang.cos(), p.1 + amp * ang.sin());
    route.push(p);
  }

  routes.push((0, route));

  clrs
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
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
