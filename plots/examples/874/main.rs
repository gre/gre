use std::f64::consts::PI;

use clap::*;
use gre::*;
use rand::Rng;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "420.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "3")]
  pub divx: usize,
  #[clap(short, long, default_value = "4")]
  pub divy: usize,
  #[clap(short, long, default_value = "0")]
  pub page: usize,
  #[clap(short, long, default_value = "12")]
  pub total: usize,
  #[clap(short, long, default_value = "28.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "")]
  pub testing_seeds: String,
}

fn cell(
  seed: f64,
  width: f64,
  height: f64,
  offset: usize,
  total: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let progress = offset as f64 / (total as f64);
  let border = 1.0;
  // Prepare all the random values
  let mut rng = rng_from_seed(seed);

  let gridw = 4;
  let gridh = 1;
  let pager_size = 1.5;
  let pager_pad = border + 0.5;
  let pager_ratio_scale = 1.0;
  let pgr = |xf, yf| {
    (
      pager_size * xf * pager_ratio_scale + pager_pad,
      height + pager_size * (yf - (gridh as f64)) - pager_pad,
    )
  };
  let pgr_topleft = pgr(0.0, 0.0);
  let pgr_bottomright = pgr(gridw as f64, gridh as f64);
  let safep = 0.3;
  let pgr_boundaries = (
    pgr_topleft.0 - safep,
    pgr_topleft.1 - safep,
    pgr_bottomright.0 + safep,
    pgr_bottomright.1 + safep,
  );

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let total_pad = border;
  let bound = (total_pad, total_pad, width - total_pad, height - total_pad);

  let does_overlap = |c: &VCircle| {
    circle_route((c.x, c.y), c.r, 10, 0.0).iter().all(|&p| {
      !strictly_in_boundaries(p, pgr_boundaries)
        && strictly_in_boundaries(p, bound)
    })
  };

  let rad = width * rng.gen_range(0.1, 0.3);
  let glow = 0.85;
  let r = rad + 3.0;
  let w = width - 2.0 * r;
  let h = height - 2.0 * r;
  let p = calculate_position(progress, w, h);
  let x = p.0 + r;
  let y = p.1 + r;
  let main_c = VCircle::new(x, y, rad);
  let p = calculate_position((progress + 0.5) % 1.0, w, h);
  let x = p.0 + r;
  let y = p.1 + r;
  let rad = rng.gen_range(0.4, 0.8) * rad;
  let main_c2 = VCircle::new(x, y, rad);

  let div = rng.gen_range(4.0, 24.0);
  let pow = rng.gen_range(0.8, 2.0);

  let ppad = rng.gen_range(0.6, 1.2);
  let min = ppad + rng.gen_range(0.3, 1.2);
  let max = min + rng.gen_range(0.0, 10.0);
  let circles = packing(
    vec![main_c.clone(), main_c2.clone()],
    seed + offset as f64 * 7.7,
    1000000,
    2000,
    rng.gen_range(0, 4),
    ppad,
    bound,
    &does_overlap,
    min,
    max,
  );

  for (i, c) in circles.iter().enumerate() {
    let clr = if i == 0 { 1 } else { 0 };
    let r = if i < 2 { c.r * glow } else { c.r };
    if c.r > 2.0 {
      let dr = -0.5 + (i as f64 / div + 1.).powf(pow);
      if dr < 2.0 {
        routes.push((clr, spiral_optimized(c.x, c.y, r, dr, 0.1)));
      }
    }
    routes.push((
      clr,
      circle_route((c.x, c.y), r, (20.0 * r) as usize + 10, 0.0),
    ));
  }

  // pager
  let mut pager = Vec::new();
  for xj in vec![0, gridw] {
    pager.push(vec![pgr(xj as f64, 0.0), pgr(xj as f64, gridh as f64)]);
  }
  for yj in vec![0, gridh] {
    pager.push(vec![pgr(0.0, yj as f64), pgr(gridw as f64, yj as f64)]);
  }
  for yi in 0..gridh {
    for xi in 0..gridw {
      let i = gridw * gridh - 1 - (xi + yi * gridw);
      let mask = 2usize.pow(i);
      let fill = offset & mask != 0;
      if fill {
        let lines = 5;
        for l in 0..lines {
          let f = (l as f64 + 0.5) / (lines as f64);
          pager.push(vec![
            pgr(xi as f64, yi as f64 + f),
            pgr(xi as f64 + 1.0, yi as f64 + f),
          ]);
        }
      }
    }
  }

  for r in pager {
    routes.push((1, r));
  }

  // External frame to around the whole piece
  let mut d = border;
  loop {
    if d < 0.1 {
      break;
    }
    routes.push((
      0,
      vec![
        (d, d),
        (d, height - d),
        (width - d, height - d),
        (width - d, d),
        (d, d),
      ],
    ));
    d -= 0.2;
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = opts.pad;
  let divx = opts.divx;
  let divy = opts.divy;
  let pageoff = opts.page * divx * divy;
  let w = (opts.width) / (divx as f64);
  let h = (opts.height) / (divy as f64);
  let total = opts.total;

  let testing_seeds = Some(
    opts
      .testing_seeds
      .split(",")
      .filter(|s| !s.is_empty())
      .map(|s| s.parse().unwrap())
      .collect::<Vec<f64>>(),
  )
  .and_then(|v| if v.is_empty() { None } else { Some(v) });

  let indexes: Vec<(usize, usize)> = (0..divx)
    .flat_map(|xi| (0..divy).map(|yi| (xi, yi)).collect::<Vec<_>>())
    .collect();

  let all = indexes
    .par_iter()
    .map(|&(xi, yi)| {
      let offset = yi + xi * divy;
      let dx = pad + xi as f64 * w;
      let dy = pad + yi as f64 * h;
      if let Some(seed) = match testing_seeds.clone() {
        None => Some(opts.seed),
        Some(array) => array.get(offset).map(|&o| o),
      } {
        let mut routes =
          cell(seed, w - 2.0 * pad, h - 2.0 * pad, pageoff + offset, total);
        routes = routes
          .iter()
          .map(|(ci, route)| {
            let r: (usize, Vec<(f64, f64)>) =
              (*ci, route.iter().map(|&p| (p.0 + dx, p.1 + dy)).collect());
            r
          })
          .collect();
        return routes;
      }
      vec![]
    })
    .collect::<Vec<_>>();

  let routes = all.concat();
  // Make the SVG
  let colors = vec!["#9b44a2", "#fa0"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (c, route) in routes.clone() {
        if c == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
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
  initial_circles: Vec<VCircle>,
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
  let mut circles = initial_circles.clone();
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

fn circle_route(
  center: (f64, f64),
  r: f64,
  count: usize,
  ang: f64,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * (i as f64 + ang) / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

fn calculate_position(progress: f64, width: f64, height: f64) -> (f64, f64) {
  let total_distance = 2.0 * width + 2.0 * height;
  let progress_distance = progress * total_distance;

  if progress_distance < width {
    // Move along top side
    let x = progress_distance;
    let y = 0.0;
    return (x, y);
  } else if progress_distance < width + height {
    // Move along right side
    let x = width;
    let y = progress_distance - width;
    return (x, y);
  } else if progress_distance < 2.0 * width + height {
    // Move along bottom side
    let x = width - (progress_distance - (width + height));
    let y = height;
    return (x, y);
  } else {
    // Move along left side
    let x = 0.0;
    let y = height - (progress_distance - (2.0 * width + height));
    return (x, y);
  }
}
