use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

// HIGH test combination of 2 inks. alternate 1-1. or N-N.
// HIGH reintroduce the destruction of the shapes of these mountains.
// ??? LOW make a bridge / connected lines, when shapes are closed?

#[derive(Clap)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "420.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "1176.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let mut rng = rng_from_seed(opts.seed);
  let cross_shape_pad: f64 = rng.gen_range(0.0, 30.0);
  let pad = (20.0 - cross_shape_pad).min(20.0);
  let circles = packing(
    opts.seed,
    10000000,
    1000,
    rng.gen_range(1, 8),
    cross_shape_pad,
    (pad, pad, opts.width - pad, opts.height - pad),
    cross_shape_pad + 3.0,
    rng.gen_range(cross_shape_pad + 10.0, 200.0),
    rng.gen_range(0.0, 1.0),
  );
  let mut routes = Vec::new();
  let mut passage = Passage2DCounter::new(0.3, opts.width, opts.height);

  let amp1pow = rng.gen_range(0.6, 1.8);
  let amp_factor = rng.gen_range(0.0, 1.0);
  let freq1 = rng.gen_range(0.03, 0.06) * (1. - amp_factor);
  let amp1 = rng.gen_range(0.0, 0.2) + 0.3 * amp_factor;
  let freq2 = rng.gen_range(0.02, 0.06);
  let amp2 = rng.gen_range(2.0, 4.0);
  let freq3 = rng.gen_range(0.4, 0.6);
  let amp3 = 0.08;
  let displacement_amp = rng.gen_range(0.0, 1.0);
  let safe_h = rng.gen_range(-8.0, 0.0);

  for c in circles {
    let cx = c.x;
    let cy = c.y;
    let max_r = c.r;

    // logic
    let perlin = Perlin::new();
    let mut highest_by_angle = vec![0f64; 8000];
    let mut shape_bound = (opts.width, opts.height, 0.0, 0.0);

    let r_increment = 0.5;
    let mut base_r = 0.2;
    loop {
      if base_r > max_r {
        break;
      }
      let mut route = Vec::new();
      let rotations = 800.0;
      let angle_delta = rng.gen_range(0, rotations as usize) as f64 / rotations * 2.0 * PI;
      let mut a = angle_delta;
      // TODO activate to create "snow" + prevent small lines < 0.3mm
      let angle_precision = 2. * PI / rotations;
      loop {
        if a - angle_delta > 2. * PI + 0.0001 {
          break;
        }

        let mut r = base_r;
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        r += amp1
          * base_r
          * (base_r / max_r).powf(amp1pow)
          * perlin.get([
            -opts.seed
              + amp2
                * perlin.get([
                  freq2 * x,
                  opts.seed * 7.7 - 4.,
                  freq2 * y + amp3 * perlin.get([freq3 * x, opts.seed * 2.7 + 11., freq3 * y]),
                ]),
            freq1 * x,
            freq1 * y,
          ]);

        let hba_index =
          (highest_by_angle.len() as f64 * ((a) / 2. * PI)) as usize % highest_by_angle.len();

        let should_draw = r > highest_by_angle[hba_index] + safe_h;

        let mut x = cx + r * a.cos();
        let mut y = cy + r * a.sin();

        let displacement_angle = 2.
          * PI
          * perlin.get([
            7.3 * opts.seed + 2.0 * perlin.get([0.005 * x, 0.005 * y, opts.seed * 3.7]),
            0.003 * x,
            0.003 * y,
          ]);
        let amp = displacement_amp
          * base_r
          * (base_r / max_r).powf(amp1pow)
          * perlin.get([
            opts.seed / 3.0 + perlin.get([0.005 * x, 0.005 * y, -opts.seed]),
            0.02 * x,
            0.02 * y,
          ]);
        x += amp * displacement_angle.cos();
        y += amp * displacement_angle.sin();

        let p = (x, y);

        if x < shape_bound.0 {
          shape_bound.0 = x;
        }
        if y < shape_bound.1 {
          shape_bound.1 = y;
        }
        if x > shape_bound.2 {
          shape_bound.2 = x;
        }
        if y > shape_bound.3 {
          shape_bound.3 = y;
        }

        if should_draw {
          highest_by_angle[hba_index] = r;
          route.push(p);
        } else {
          if route.len() > 1 {
            let mut simplified = Vec::new();
            simplified.push(route[0]);
            let mut dist = 0.0;
            let l = route.len();
            for i in 1..l {
              dist += euclidian_dist(route[i - 1], route[i]);
              if dist > 0.5 {
                simplified.push(route[i]);
                dist = 0.0;
              }
            }
            if dist > 0.0 {
              simplified.push(route[l - 1]);
            }
            if route.len() > 2 {
              routes.push(simplified);
            }
          }
          route = Vec::new();
        }
        a += angle_precision;
      }

      if route.len() > 0 {
        let mut simplified = Vec::new();
        simplified.push(route[0]);
        let mut dist = 0.0;
        let l = route.len();
        for i in 1..l {
          dist += euclidian_dist(route[i - 1], route[i]);
          if dist > 0.5 {
            simplified.push(route[i]);
            dist = 0.0;
          }
        }
        if dist > 0.0 {
          simplified.push(route[l - 1]);
        }
        if route.len() > 2 {
          routes.push(simplified);
        }
      }

      base_r += r_increment;
    }
  }

  /*
  // center our shape
  let shape_cx = (shape_bound.0 + shape_bound.2) / 2.;
  let shape_cy = (shape_bound.1 + shape_bound.3) / 2.;
  let dx = cx - shape_cx;
  let dy = cy - shape_cy;
  routes = routes
    .iter()
    .map(|route| {
      route.iter().map(|p| (p.0 + dx, p.1 + dy)).collect()
    })
    .collect();
    */

  println!("{}", routes.len());

  // render
  let colors = vec!["black"];
  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      let mut should_draw_line = |a, _b| passage.count(a) < 6;
      for route in routes.clone() {
        data = render_route_when(data, route, &mut should_draw_line);
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

fn scaling_search<F: FnMut(f64) -> bool>(mut f: F, min_scale: f64, max_scale: f64) -> Option<f64> {
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
  multiply_max: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let half = (max_scale - min_scale) / 2.0;
  let mut m = max_scale;
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) = search_circle_radius(bound, &circles, x, y, min_scale, m) {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        m = (m * multiply_max).max(half);
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
}
