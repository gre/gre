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
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
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
  let color = "#000";
  let mut seed = opts.seed;
  let width = opts.width;
  let height = opts.height;
  let pad = 8.0;
  let bounds = (pad, pad, width - pad, height - pad);
  let stroke_width = 0.35;
  let precision = 0.002;
  let target_size = 36;
  let particles = 3000;
  let mut rng = rng_from_seed(opts.seed);
  let mut layers = Vec::new();
  let mut passage = Passage2DCounter::new(0.4, width, height);
  let max_passage = 8;
  let min_c = 4.0 + rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0);
  let max_c = min_c + rng.gen_range(1.0, 60.0);
  let secondary_threshold = rng.gen_range(20.0, 100.0);
  let packing_padding =
    2.0 + rng.gen_range(0.0, 30.0) * rng.gen_range(0.0, 1.0);
  let primaries = packing(
    opts.seed,
    1000000,
    2000,
    rng.gen_range(1, 4),
    packing_padding,
    bounds,
    &VCircle::new(width / 2., height / 2., width + height),
    min_c,
    min_c + rng.gen_range(20.0, 300.0),
  );
  let mut circles = Vec::new();
  for c in primaries {
    if c.r > secondary_threshold {
      for c2 in packing(
        seed,
        500000,
        2000,
        1,
        packing_padding,
        bounds,
        &c,
        min_c,
        max_c,
      ) {
        circles.push(c2);
      }
      seed = seed * 1.1 + 0.3;
    } else {
      circles.push(c);
    }
  }

  rng.shuffle(&mut circles);

  let ampang = rng.gen_range(0.4, 1.2);
  let f1 = rng.gen_range(0.0, 2.0);
  let f2 = rng.gen_range(0.0, 2.0);
  let f3 = rng.gen_range(0.0, 2.0);

  let perlin = Perlin::new();

  let samples = sample_2d_candidates_f64(
    &|p| {
      let g = project_in_boundaries(p, bounds);
      let mut d = 99f64;
      for circle in circles.iter() {
        d = d.min(euclidian_dist((circle.x, circle.y), g) - circle.r);
      }
      smoothstep(50.0, -20.0, d)
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
          let r = smoothstep(40.0, -40.0, dist);
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

        ang += ampang
          * perlin.get([
            f1 * 0.02 * g.0,
            f1 * 0.02 * g.1,
            2.4 * seed
              + rng.gen_range(1.0, 2.0)
                * perlin.get([
                  f2 * 0.04 * g.0,
                  f2 * 0.04 * g.1,
                  seed
                    + rng.gen_range(1.0, 2.0)
                      * perlin.get([f3 * 0.1 * g.0, seed, f3 * 0.1 * g.1]),
                ]),
          ]);

        let front = (p.0 + precision * ang.cos(), p.1 + precision * ang.sin());
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
  layers
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
