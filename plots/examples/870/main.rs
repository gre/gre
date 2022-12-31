use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::Group;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "71.0")]
  seed: f64,
  #[clap(short, long, default_value = "420.0")]
  width: f64,
  #[clap(short, long, default_value = "297.0")]
  height: f64,
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
  fn inside_bounds(
    self: &Self,
    (x1, y1, x2, y2): (f64, f64, f64, f64),
  ) -> bool {
    x1 <= self.x - self.r
      && self.x + self.r <= x2
      && y1 <= self.y - self.r
      && self.y + self.r <= y2
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
  container_boundaries: (f64, f64, f64, f64),
  container_circle: &VCircle,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    c.inside_bounds(container_boundaries)
      && container_circle.contains(&c)
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
  container_boundaries: (f64, f64, f64, f64),
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
    if let Some(size) = search_circle_radius(
      container_boundaries,
      &container,
      &circles,
      x,
      y,
      min_scale,
      max_scale,
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

fn art(opts: &Opts) -> Vec<Group> {
  let seed = opts.seed;
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;
  let bounds = (pad, pad, width - pad, height - pad);
  let stroke_width = 0.35;
  let precision = 0.002;
  let mut rng = rng_from_seed(opts.seed);
  let target_size = rng.gen_range(15, 22);
  let colors = vec![
    ("royalblue", 20000),
    ("darkcyan", 20000),
    ("darkkhaki", 10000),
    ("#bbb", 6000),
  ];
  let mut layers = Vec::new();
  let mut passage = Passage2DCounter::new(0.5, width, height);
  let max_passage = 5;
  let size = 10.0 + rng.gen_range(0.0, 100.0) * rng.gen_range(0.0, 1.0);
  let min_threshold = rng.gen_range(4.0, 20.0);
  let circle_bounds = (-0.2 * width, -0.2 * height, 1.2 * width, 1.2 * height);
  let dycenter = rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);

  let offset = rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0);

  let mut circles = packing(
    opts.seed,
    1000000,
    1000,
    1,
    min_threshold,
    circle_bounds,
    &VCircle::new(width / 2., height / 2., width + height),
    min_threshold,
    rng.gen_range(2.0 * min_threshold, 7. * min_threshold),
  );

  rng.shuffle(&mut circles);
  let ampang = rng.gen_range(0.0, 2.0);
  let f1 = rng.gen_range(0.0, 2.5) * rng.gen_range(0.0, 1.0);
  let f2 = rng.gen_range(0.0, 2.5) * rng.gen_range(0.0, 1.0);

  for (ci, &(color, particles)) in colors.iter().enumerate() {
    let perlin = Perlin::new();

    let samples = sample_2d_candidates_f64(
      &|p| {
        let g = project_in_boundaries(p, bounds);
        let mut d = 99f64;
        for (i, circle) in circles.iter().enumerate() {
          if i % colors.len() == ci {
            d = d.min(euclidian_dist((circle.x, circle.y), g) - circle.r);
          }
        }
        smoothstep(60.0, -30.0, d)
      },
      1000,
      particles,
      &mut rng,
    );

    let routes: Vec<Vec<(f64, f64)>> = samples
      .par_iter()
      .enumerate()
      .map(|(si, &sample)| {
        let mut rng = rng_from_seed(seed + si as f64 * PI);
        let mut route = Vec::new();
        let mut p = sample;
        let mut ang = rng.gen_range(0.0, 2. * PI);
        loop {
          if out_of_boundaries(p, (0.0, 0.0, 1.0, 1.0)) {
            break;
          }
          if route.len() >= target_size {
            break;
          }
          let g = project_in_boundaries(
            p,
            (0.0, 0.0, width - 2.0 * pad, height - 2.0 * pad),
          );
          route.push(g);

          let mut v = (0f64, 0f64);
          for p in circles.iter() {
            let dist = euclidian_dist((p.x, p.y), g) - p.r;
            let a = (p.y - g.1).atan2(p.x - g.0) + (si as f64 - 0.5) * PI;
            let r = smoothstep(80.0, -10.0, dist);
            v.0 += r * a.cos();
            v.1 += r * a.sin();
          }

          if v.0 != 0.0 || v.1 != 0.0 {
            let mut a = (v.1.atan2(v.0) + 2.0 * PI) % (2. * PI);
            if (a - ang).abs() > PI / 2.0 {
              a += PI;
            }
            ang = a;
          }
          let (xi, yi) = hex_index(g, size);
          // println!("Hexagon at ({}, {}) has index ({}, {})", g.0, g.1, xi, yi);
          ang += dycenter
            + 2.0 * perlin.get([xi as f64 / 0.7, yi as f64 / 0.7, seed]).abs();

          let f = 0.05;
          let mixed = smoothstep(
            -0.2,
            0.2,
            offset + perlin.get([seed, f * g.0, f * g.1]),
          );

          ang += ampang
            * dycenter
            * mix(
              perlin.get([
                f1 * 0.03 * g.0 + xi as f64 * 0.07,
                f1 * 0.03 * g.1 + yi as f64 * 0.07,
                1.4 * seed
                  + 1.8
                    * perlin.get([
                      f2 * 0.04 * g.0,
                      f2 * 0.04 * g.1,
                      seed
                        + 0.4
                          * perlin.get([
                            f2 * 0.02 * g.0,
                            f2 * 0.02 * g.1,
                            seed,
                          ]),
                    ]),
              ]),
              perlin.get([
                3.0 * f1 * 0.03 * g.0 + xi as f64 / 13.0,
                3.0 * f1 * 0.03 * g.1 + yi as f64 / 13.0,
                10.4 * seed
                  + 1.5
                    * perlin.get([
                      f2 * 0.1 * g.0,
                      f2 * 0.1 * g.1,
                      seed
                        + 0.3
                          * perlin.get([
                            f2 * 0.05 * g.0,
                            f2 * 0.05 * g.1,
                            seed,
                          ]),
                    ]),
              ]),
              mixed,
            );

          let front =
            (p.0 + precision * ang.cos(), p.1 + precision * ang.sin());
          p = front;
        }
        route
      })
      .collect();

    let mut l = layer(color);
    let mut data = Data::new();
    for route in routes.clone() {
      let inside = |from, to| {
        strictly_in_boundaries(from, bounds)
          && strictly_in_boundaries(to, bounds)
          && passage.count(from) < max_passage
      };
      let r = route.iter().map(|&p| (p.0 + pad, p.1 + pad)).collect();
      data = render_route_when(data, r, inside);
    }
    l = l.add(base_path(color, stroke_width, data));
    layers.push(l);
  }
  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("#fff", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn hex_index(point: (f64, f64), size: f64) -> (i64, i64) {
  let x = point.0;
  let y = point.1;

  // Convert the Cartesian coordinates to axial coordinates
  let q = x * 2.0 / 3.0 / size;
  let r = (-x / 3.0 + (f64::sqrt(3.0) / 3.0) * y) / size;

  // Round the axial coordinates to the nearest integer values
  let mut q_rounded = q.round() as i64;
  let mut r_rounded = r.round() as i64;

  // Find the fractional part of each coordinate
  let q_frac = q - q_rounded as f64;
  let r_frac = r - r_rounded as f64;

  // Determine which hexagon the point is in based on the fractional part of the coordinates
  if q_frac > 0.5 {
    q_rounded += 1;
  }
  if q_frac < -0.5 {
    q_rounded -= 1;
  }
  if r_frac > 0.5 {
    r_rounded += 1;
  }
  if r_frac < -0.5 {
    r_rounded -= 1;
  }

  // Convert the axial coordinates back to Cartesian coordinates
  let xindex = q_rounded as i64;
  let yindex = r_rounded as i64;

  (xindex, yindex)
}
