use clap::*;
use gre::*;
use ndarray::Array2;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::{path::Data, Group};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "10")]
  kmeans_clusters: usize,
  #[clap(short, long, default_value = "30")]
  seconds: i64,
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

fn art(opts: Opts) -> Vec<Group> {
  let width = 420.0;
  let height = 297.0;
  let pad = 20.0;
  let stroke_width = 0.35;
  let mut rng = rng_from_seed(opts.seed);
  let min_scale = 1.5;
  let mut seed = opts.seed / 7.;
  let primary = packing(
    seed,
    2000000,
    10000,
    2,
    1.,
    (pad, pad, width - pad, height - pad),
    &VCircle::new(width / 2., height / 2., width + height),
    min_scale,
    rng.gen_range(32.0, 256.0),
  );

  let mut circles = Vec::new();
  for c in primary {
    if c.r > 10.0 {
      for c2 in packing(
        seed,
        2000000,
        10000,
        1,
        0.,
        (pad, pad, width - pad, height - pad),
        &c,
        rng.gen_range(0.8, 1.6),
        4.0 + rng.gen_range(0.0, 80.0) * rng.gen_range(0.0, 1.0),
      ) {
        circles.push(c2);
      }
      seed = seed * 1.1 + 0.3;
    } else {
      circles.push(c);
    }
  }

  let colors = vec!["#f90", "#09f"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, &color)| {
      let subset: Vec<VCircle> = circles
        .iter()
        .filter(|c| (ci == 0) == (c.y > height / 2.0))
        .map(|&c| c)
        .collect();
      let routes: Vec<Vec<(f64, f64)>> =
        group_with_kmeans(subset, opts.kmeans_clusters)
          .par_iter()
          .map(|cin| {
            let points: Vec<(f64, f64)> =
              cin.iter().map(|c| (c.x, c.y)).collect();
            let tour = travelling_salesman::simulated_annealing::solve(
              &points,
              time::Duration::seconds(opts.seconds),
            );
            let circles: Vec<VCircle> =
              tour.route.iter().map(|&i| cin[i]).collect();
            let route: Vec<(f64, f64)> = circles
              .par_iter()
              .flat_map(|c| {
                let s = opts.seed + c.x * 3.1 + c.y / 9.8;
                let mut rng = rng_from_seed(s);
                let pow = 1.8;
                let samples = sample_2d_candidates_f64(
                  &|p| {
                    let dx = p.0 - 0.5;
                    let dy = p.1 - 0.5;
                    let d2 = dx * dx + dy * dy;
                    if d2 > 0.25 {
                      0.0
                    } else {
                      1.0
                    }
                  },
                  (8. * c.r) as usize,
                  (20. + (1.8 * c.r).powf(pow)) as usize,
                  &mut rng,
                );
                let candidates = samples
                  .iter()
                  .map(|(x, y)| {
                    (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y)
                  })
                  .collect();
                route_spiral(candidates)
              })
              .collect();
            route
          })
          .collect();

      let mut data = Data::new();
      for route in routes {
        data = render_route_curve(data, route);
      }
      layer(color).add(base_path(color, stroke_width, data))
    })
    .collect()
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

fn group_with_kmeans(samples: Vec<VCircle>, n: usize) -> Vec<Vec<VCircle>> {
  let arr = Array2::from_shape_vec(
    (samples.len(), 2),
    samples.iter().flat_map(|c| vec![c.x, c.y]).collect(),
  )
  .unwrap();

  let (means, clusters) = rkm::kmeans_lloyd(&arr.view(), n);

  let all: Vec<Vec<VCircle>> = means
    .outer_iter()
    .enumerate()
    .map(|(c, _coord)| {
      clusters
        .iter()
        .enumerate()
        .filter(|(_i, &cluster)| cluster == c)
        .map(|(i, _c)| samples[i])
        .collect()
    })
    .collect();

  all
}
