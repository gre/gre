use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::{path::Data, Group};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
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

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;
  let stroke_width = 0.35;

  let mut rng = rng_from_seed(opts.seed);
  let cpad = 1.0;
  let min_scale = cpad + 2.0;
  let max_scale =
    min_scale + rng.gen_range(10.0, 100f64) * rng.gen_range(0.2, 1.0);

  let iterations = 1000000;

  let drstart = rng.gen_range(0.5, 0.8);
  let drmul = rng.gen_range(0.5, 1.0);
  let count = rng.gen_range(5, 10);
  let xpadlayer = (width - height) / (count as f64 * 2.0);

  let squarebound = (
    pad + (width - height) / 2.0,
    pad,
    width - pad - (width - height) / 2.0,
    height - pad,
  );

  let get_color = image_get_color(
    "images/greweb_abstract_artist_using_computers_and_pens_hat_glasses.png",
  )
  .unwrap();

  let f = |p| get_color(p).0;

  let routes = (0..count)
    .map(|i| {
      let dr = drstart + drmul * (count - 1 - i) as f64 / (count as f64);
      let xpad = i as f64 * xpadlayer;
      let bound = (pad + xpad, pad, width - pad - xpad, height - pad);
      let circles = packing(
        opts.seed * 7.7 + i as f64 / 3.0,
        iterations,
        5000,
        1,
        cpad,
        bound,
        min_scale,
        max_scale,
      );

      let mut routes = Vec::new();
      for c in circles {
        routes.push(circle_route((c.x, c.y), c.r, 200));
        routes.push(spiral_optimized(c.x, c.y, c.r, dr, 0.1));
      }

      let min_r = 2;
      let routes_copy = routes.clone();
      let mut routes = Vec::new();
      for route in routes_copy {
        let mut r = vec![];
        for p in route {
          let n = (
            (p.0 - squarebound.0) / (squarebound.2 - squarebound.0),
            (p.1 - squarebound.1) / (squarebound.3 - squarebound.1),
          );
          let should_draw = f(n) < (i as f64 + 0.5) / (count as f64);
          if should_draw {
            r.push(p);
          } else {
            if r.len() > 0 {
              if r.len() >= min_r {
                routes.push(r);
              }
              r = vec![];
            }
          }
        }
        if r.len() >= min_r {
          routes.push(r);
        }
      }

      routes
    })
    .collect::<Vec<_>>()
    .concat();

  let color = "black";
  let mut data = Data::new();
  for route in routes {
    data = render_route(data, route);
  }
  vec![layer(color).add(base_path(color, stroke_width, data))]
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
  bound: (f64, f64, f64, f64),
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    bound.0 < c.x - c.r
      && c.x + c.r < bound.2
      && bound.1 < c.y - c.r
      && c.y + c.r < bound.3
      && !circles.iter().any(|other| c.collides(other))
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
      search_circle_radius(bound, &circles, x, y, min_scale, max_scale)
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
