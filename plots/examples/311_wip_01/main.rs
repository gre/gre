use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

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
      tries.push(circle.clone());
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c);
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }

  circles
}

fn art(opts: Opts) -> Vec<Group> {
  let seed = opts.seed;
  let width = 420.0;
  let height = 297.0;
  let pad = 20.0;
  let precision = 1.0;
  let w = ((width - 2. * pad) as f64 / precision) as u32;
  let h = ((height - 2. * pad) as f64 / precision) as u32;
  let stroke_width = 0.3;

  let bounds_container = VCircle::new(width / 2.0, height / 2.0, width);

  let primaries = packing(
    opts.seed,
    1000000,
    2000,
    1,
    2.0,
    &bounds_container,
    4.0,
    40.0,
  );

  let themes = vec![
    ("darkturquoise", 4.0, -50.0, 60),
    ("darkblue", -4.0, 20.0, 30),
  ];
  let mut layers = Vec::new();

  for (color, from, to, samples) in themes {
    let perlin = Perlin::new();

    let f = |(x, y): (f64, f64)| {
      let mut d = 1000f64;
      let mut g =
        project_in_boundaries((x, y), (pad, pad, width - pad, height - pad));
      let a = 2.
        * PI
        * perlin.get([
          x,
          y,
          0.4 * seed
            + 0.5
              * perlin.get([
                2. * x,
                2. * y,
                10. + seed + 0.5 * perlin.get([10. * x, 10. * y, seed]),
              ]),
        ]);
      let amp = 1.0;
      g.0 += amp * a.cos();
      g.1 += amp * a.sin();
      for p in primaries.iter() {
        d = d.min(euclidian_dist((p.x, p.y), g) - p.r);
      }
      d += 2.0
        * perlin.get([
          2. * x,
          2. * y,
          7.4 * seed
            + 0.5
              * perlin.get([
                4. * x,
                4. * y,
                7. + 8.8 * seed
                  + 0.5 * perlin.get([7. * x, 7. * y, 7.7 * seed]),
              ]),
        ]);
      smoothstep(from, to, d)
    };

    let thresholds: Vec<f64> = (0..samples)
      .map(|i| (i as f64) / (samples as f64))
      .collect();
    let res = contour(w, h, f, &thresholds);
    let routes = features_to_routes(res, precision);
    let inside = |from, to| {
      strictly_in_boundaries(from, (pad, pad, width - pad, height - pad))
        && strictly_in_boundaries(to, (pad, pad, width - pad, height - pad))
    };

    let mut l = layer(color);
    let mut data = Data::new();
    for route in routes.clone() {
      let r = route.iter().map(|&p| (p.0 + pad, p.1 + pad)).collect();
      data = render_route_when(data, r, inside);
    }
    l = l.add(base_path(color, stroke_width, data));
    l = l.add(signature(0.8, (width - 60.0, height - 30.0), color));
    layers.push(l);
  }

  layers
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a3_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
