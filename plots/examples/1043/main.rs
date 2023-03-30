use clap::*;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "500000")]
  pub iterations: usize,
  #[clap(short, long, default_value = "1.5")]
  pub inner_pad: f64,
}

fn crab<R: Rng>(
  origin: (f64, f64),
  scale: f64,
  rot: f64,
  rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = vec![];

  // MAKE THE BODY
  let bodycenter = (0.5, -0.6);
  let body = spiral_ovale(
    bodycenter.0,
    bodycenter.1,
    rng.gen_range(0.2, 0.3),
    0.7,
    rng.gen_range(0.3, 1.0) / scale,
    rng.gen_range(0.08, 0.18) / scale,
  );
  routes.push(body);

  // MAKE THE EYES
  let eyes = vec![rng.gen_range(-0.15, -0.05), rng.gen_range(0.05, 0.15)];
  for dx in eyes {
    routes.push(vec![(0.5 + dx, -0.8), (0.5 + dx * 2.0, -0.95)]);
  }

  // MAKE THE PAWS
  let paws = vec![-1.3, -1.0, -0.6, 0.6, 1.0, 1.3];
  for da in paws {
    let ang = PI / 2.0 + da + rng.gen_range(-0.3, 0.3);
    let ax = ang.cos();
    let ay = ang.sin();
    routes.push(vec![
      (bodycenter.0 + 0.2 * ax, bodycenter.1 + 0.2 * ay),
      (bodycenter.0 + 0.4 * ax, bodycenter.1 + 0.4 * ay),
      (
        bodycenter.0 + rng.gen_range(0.4, 0.6) * ax,
        (bodycenter.1 + rng.gen_range(0.4, 0.6) * ay + rng.gen_range(0.1, 0.3))
          .min(0.0),
      ),
    ]);
  }

  // MAKE THE PLIERS
  let ampincr = rng.gen_range(0.2, 0.3);
  let arms = vec![-1.0, 1.0];
  for arm in arms {
    let mut ang = -PI / 2.0 + arm + rng.gen_range(-0.2, 0.2);
    let mut route = Vec::new();
    let mut amp = rng.gen_range(0.0, 0.2);
    for _i in 0..3 {
      ang += rng.gen_range(-0.3, 0.3) * amp;
      let ax = ang.cos();
      let ay = ang.sin();
      route.push((bodycenter.0 + amp * ax, bodycenter.1 + amp * ay));
      amp = 1.1 * amp + ampincr;
    }
    routes.push(route.clone());
    routes.push(route.iter().map(|&p| (p.0, p.1 - 0.05)).collect());
    let mut last = route[route.len() - 1];
    last.1 -= 0.025;
    for i in 0..2 {
      let p = i as f64 - 0.5;
      let a = ang + p;
      let a2 = ang + rng.gen_range(0.05, 0.9) * p;
      let ax = a.cos();
      let ay = a.sin();
      let ax2 = a2.cos();
      let ay2 = a2.sin();
      let route = vec![
        last,
        (
          last.0 + rng.gen_range(0.2, 0.4) * ax,
          last.1 + rng.gen_range(0.2, 0.4) * ay,
        ),
        (
          last.0 + rng.gen_range(0.4, 0.6) * ax2,
          last.1 + rng.gen_range(0.4, 0.6) * ay2,
        ),
      ];
      // duplicate multiple time to make it bolder and random
      for _i in 0..rng.gen_range(2, 4) {
        routes.push(path_subdivide_to_curve(
          shake(route.clone(), 0.05, rng),
          1,
          0.8,
        ));
      }
    }
  }

  // scale, rotate & translate
  let routes: Vec<Vec<(f64, f64)>> = routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&p| {
          let p = p_r(p, rot);
          round_point((scale * p.0 + origin.0, scale * p.1 + origin.1), 0.01)
        })
        .collect()
    })
    .collect();

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;
  let iterations = opts.iterations;
  let bound = (pad, pad, width - pad, height - pad);

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  // Prepare all the random values
  let mut rng = rng_from_seed(opts.seed);

  let precision = 1.0;
  let grow = opts.inner_pad;
  let mut passage = Passage::new(precision, width, height);

  for _i in 0..iterations {
    let x = rng.gen_range(bound.0, bound.2);
    let y = rng.gen_range(bound.1, bound.3);
    let scale = rng.gen_range(4.0, 5.0);
    let rot = rng.gen_range(-PI, PI)
      * 0.5
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
    let rts = crab((x, y), scale, rot, &mut rng);
    let mut local_passage = Passage::new(precision, width, height);
    let mut out_of_bounds = false;
    for route in rts.clone() {
      if out_of_bounds {
        break;
      }
      for r in route.clone() {
        if !strictly_in_boundaries(r, bound) {
          out_of_bounds = true;
          break;
        }
        local_passage.count(r);
      }
    }
    if out_of_bounds {
      continue;
    }
    local_passage.grow_passage(grow);
    if !local_passage.collides(&passage) {
      routes.extend(rts);
      passage.add(&local_passage);
    }
  }

  // Make the SVG
  vec![("black", routes, 0.35)]
    .iter()
    .map(|(color, routes, stroke_width)| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, *stroke_width, data));
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
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

fn spiral_ovale(
  x: f64,
  y: f64,
  radius: f64,
  wmul: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = (x + wmul * r * a.cos(), y + r * a.sin());
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < approx {
      break;
    }
  }
  route
}

#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn add(self: &mut Self, other: &Passage) {
    for i in 0..self.counters.len() {
      self.counters[i] += other.counters[i];
    }
  }

  pub fn collides(self: &Self, other: &Passage) -> bool {
    for i in 0..self.counters.len() {
      if self.counters[i] > 0 && other.counters[i] > 0 {
        return true;
      }
    }
    false
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }

  pub fn count_once(self: &mut Self, p: (f64, f64)) {
    let i = self.index(p);
    let v = self.counters[i];
    if v == 0 {
      self.counters[i] = 1;
    }
  }

  pub fn grow_passage(self: &mut Self, radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters: Vec<usize> = self.counters.iter().cloned().collect();
    let mut mask = Vec::new();
    // TODO, in future for even better perf, I will rewrite this
    // working directly with index integers instead of having to use index() / count_once()
    let mut x = -radius;
    loop {
      if x >= radius {
        break;
      }
      let mut y = -radius;
      loop {
        if y >= radius {
          break;
        }
        if x * x + y * y < radius * radius {
          mask.push((x, y));
        }
        y += precision;
      }
      x += precision;
    }

    let mut x = 0.0;
    loop {
      if x >= width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= height {
          break;
        }
        let index = self.index((x, y));
        if counters[index] > 0 {
          for &(dx, dy) in mask.iter() {
            self.count_once((x + dx, y + dy));
          }
        }
        y += precision;
      }
      x += precision;
    }
  }
}
