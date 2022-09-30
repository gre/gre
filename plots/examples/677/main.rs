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
  #[clap(short, long, default_value = "420.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "10.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

/*
  L’air sec se compose d’environ 78 % d’azote, 21 % d’oxygène et 1 % d’argon.
  L’air contient également de la vapeur d’eau qui représente entre 0,1 et 4 % de la troposphère.
  L’air chaud contient généralement plus de vapeur d’eau que l’air froid.
*/
#[derive(PartialEq, Copy, Clone)]
enum Color {
  O,
  H,
  N,
  Ar,
  C,
}

trait PlottableMolecule {
  fn draw(
    self: &Self,
    c: &VCircle,
    spiral_dr: f64,
  ) -> Vec<(Color, Vec<(f64, f64)>)>;
}

struct MoleculeO2 {}
struct MoleculeH2O {}
struct MoleculeN2 {}
struct MoleculeAr {}
struct MoleculeCO2 {}

fn draw_o(x: f64, y: f64, r: f64, spiral_dr: f64) -> (Color, Vec<(f64, f64)>) {
  (Color::O, spiral_optimized(x, y, r, spiral_dr))
}
fn draw_h(x: f64, y: f64, r: f64, _spiral_dr: f64) -> (Color, Vec<(f64, f64)>) {
  (Color::H, circle_route((x, y), r, (r * 3.0 + 8.0) as usize))
}
fn draw_n(x: f64, y: f64, r: f64, spiral_dr: f64) -> (Color, Vec<(f64, f64)>) {
  (Color::N, spiral_optimized(x, y, r, spiral_dr))
}
fn draw_ar(x: f64, y: f64, r: f64, spiral_dr: f64) -> (Color, Vec<(f64, f64)>) {
  (Color::Ar, spiral_optimized(x, y, r, spiral_dr))
}
fn draw_c(x: f64, y: f64, r: f64, spiral_dr: f64) -> (Color, Vec<(f64, f64)>) {
  (Color::C, spiral_optimized(x, y, r, spiral_dr))
}

impl PlottableMolecule for MoleculeO2 {
  fn draw(
    self: &Self,
    c: &VCircle,
    spiral_dr: f64,
  ) -> Vec<(Color, Vec<(f64, f64)>)> {
    let mut routes = Vec::new();
    let r = 0.6 * c.r;
    let dx = 0.4 * c.r;
    routes.push(draw_o(c.x + dx, c.y, r, spiral_dr));
    routes.push(draw_o(c.x - dx, c.y, r, spiral_dr));
    routes
  }
}
impl PlottableMolecule for MoleculeH2O {
  fn draw(
    self: &Self,
    c: &VCircle,
    spiral_dr: f64,
  ) -> Vec<(Color, Vec<(f64, f64)>)> {
    let mut routes = Vec::new();
    let r = 0.4 * c.r;
    let dx = 0.5 * c.r;
    let dy = 0.35 * c.r;
    let r2 = 0.6 * c.r;
    routes.push(draw_h(c.x + dx, c.y - dy, r, spiral_dr));
    routes.push(draw_h(c.x - dx, c.y - dy, r, spiral_dr));
    routes.push(draw_o(c.x, c.y + dy, r2, spiral_dr));
    routes
  }
}
impl PlottableMolecule for MoleculeN2 {
  fn draw(
    self: &Self,
    c: &VCircle,
    spiral_dr: f64,
  ) -> Vec<(Color, Vec<(f64, f64)>)> {
    let mut routes = Vec::new();
    let r = 0.55 * c.r;
    let dx = 0.35 * c.r;
    routes.push(draw_n(c.x + dx, c.y, r, spiral_dr));
    routes.push(draw_n(c.x - dx, c.y, r, spiral_dr));
    routes
  }
}
impl PlottableMolecule for MoleculeAr {
  fn draw(
    self: &Self,
    c: &VCircle,
    spiral_dr: f64,
  ) -> Vec<(Color, Vec<(f64, f64)>)> {
    let mut routes = Vec::new();
    let r = 0.8 * c.r;
    routes.push(draw_ar(c.x, c.y, r, spiral_dr));
    routes
  }
}
impl PlottableMolecule for MoleculeCO2 {
  fn draw(
    self: &Self,
    c: &VCircle,
    spiral_dr: f64,
  ) -> Vec<(Color, Vec<(f64, f64)>)> {
    let mut routes = Vec::new();
    let r = 0.45 * c.r;
    let dx = 0.55 * c.r;
    let dy = 0.35 * c.r;
    let r2 = 0.6 * c.r;
    routes.push(draw_o(c.x + dx, c.y - dy, r, spiral_dr));
    routes.push(draw_o(c.x - dx, c.y - dy, r, spiral_dr));
    routes.push(draw_c(c.x, c.y + dy, r2, spiral_dr));
    routes
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  let mut all: Vec<Vec<(Color, Vec<(f64, f64)>)>> = Vec::new();
  let mut count_co2 = 0;

  let bound = (pad, pad, width - pad, height - pad);
  let does_overlap = |_p| true;
  let circles = packing(
    opts.seed,
    1000000,
    10000,
    1,
    0.1,
    bound,
    &does_overlap,
    3.0,
    5.0,
  );
  println!("{} circles", circles.len());
  let data = vec![(0.3, 0.5, circles)];

  for (from_spiral_dr, to_spiral_dr, circles) in data {
    for c in circles {
      let spiral_dr = rng.gen_range(from_spiral_dr, to_spiral_dr);
      let n = rng.gen_range(0, 100usize);
      let routes = if rng.gen_bool(0.01) && count_co2 == 0 {
        count_co2 += 1;
        MoleculeCO2 {}.draw(&c, spiral_dr)
      } else
      // L’air contient également de la vapeur d’eau qui représente entre 0,1 et 4 % de la troposphère.
      if rng.gen_bool(0.02) {
        MoleculeH2O {}.draw(&c, spiral_dr)
      } else {
        // L’air sec se compose d’environ 78 % d’azote, 21 % d’oxygène et 1 % d’argon.
        if n < 78 {
          MoleculeN2 {}.draw(&c, spiral_dr)
        } else if n < 99 {
          MoleculeO2 {}.draw(&c, spiral_dr)
        } else {
          MoleculeAr {}.draw(&c, spiral_dr)
        }
      };
      let angle = rng.gen_range(0.0, 2.0 * PI);
      let routes = routes
        .iter()
        .map(|(clr, route)| {
          let r: Vec<(f64, f64)> = route
            .iter()
            .map(|&p| {
              let mut q = (p.0 - c.x, p.1 - c.y);
              q = p_r(q, angle);
              (q.0 + c.x, q.1 + c.y)
            })
            .collect();
          (clr.clone(), r)
        })
        .collect();
      all.push(routes);
    }
  }

  let routes = all.concat();

  let groups = 2;

  vec![
    (Color::O, "red"),
    (Color::H, "black"),
    (Color::N, "darkblue"),
    (Color::Ar, "lightblue"),
    (Color::C, "black"),
  ]
  .iter()
  .enumerate()
  .flat_map(|(ci, &(clr, color))| {
    (0..groups)
      .map(|i| {
        let mut data = Data::new();
        for (ni, (c, route)) in routes.iter().enumerate() {
          if ni % groups == i && *c == clr {
            data = render_route(data, route.clone());
          }
        }
        let mut l = layer(
          format!("{} {}", ci * groups + i, String::from(color)).as_str(),
        );
        l = l.add(base_path(color, 0.5, data));
        l
      })
      .collect::<Vec<_>>()
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
  for i in 0..(2 * count + 2) {
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y)) && !circles.iter().any(|other| c.collides(other))
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
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

pub fn spiral_optimized(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
) -> Vec<(f64, f64)> {
  let approx = 0.1;
  let extra = 0.5;
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius + extra;
  let mut a = 0f64;
  loop {
    let mr = r.min(radius);
    let p = round_point((x + mr * a.cos(), y + mr * a.sin()), 0.01);
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
  route.push((x, y));
  route
}
