use clap::*;
use geo::{intersects::Intersects, prelude::BoundingRect, *};
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;
use svg::node::element::{path::Data, Group};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "8.0")]
  seed: f64,
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
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist((self.x, self.y), p) < self.r
  }
}

fn make_polygon(x: f64, y: f64, size: f64, angle: f64) -> Polygon<f64> {
  let count = 3;
  Polygon::new(
    LineString::from(
      (0..count)
        .map(|i| {
          let a = angle + 2. * PI * i as f64 / (count as f64);
          (x + size * a.cos(), y + size * a.sin())
        })
        .collect::<Vec<(f64, f64)>>(),
    ),
    vec![],
  )
}

fn poly_collides_in_polys(
  polys: &Vec<Polygon<f64>>,
  poly: &Polygon<f64>,
) -> bool {
  polys.iter().any(|p| poly.intersects(p))
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

fn search(
  container: &VCircle,
  polys: &Vec<Polygon<f64>>,
  x: f64,
  y: f64,
  angle: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let p = &make_polygon(x, y, size, angle);
    p.exterior()
      .points_iter()
      .all(|c| container.includes(c.x_y()))
      && !poly_collides_in_polys(polys, p)
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
) -> Vec<Polygon<f64>> {
  let mut polys = Vec::new();
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
    let angle = rng.gen_range(0f64, 2. * PI);
    if let Some(size) =
      search(&container, &polys, x, y, angle, min_scale, max_scale)
    {
      tries.push((x, y, size - pad, angle));
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let (x, y, s, a) = tries[0];
        let p = make_polygon(x, y, s, a);
        polys.push(p);
        tries = Vec::new();
      }
    }
    if polys.len() > desired_count {
      break;
    }
  }
  polys
}

fn art(opts: Opts) -> Vec<Group> {
  let width = 297.0;
  let height = 210.0;
  let pad = 10.0;
  let stroke_width = 0.35;

  let routes = packing(
    opts.seed,
    1000000,
    3000,
    8,
    0.1,
    &VCircle::new(width / 2.0, height / 2.0, height / 2.0 - pad),
    0.8,
    30.0,
  )
  .par_iter()
  .map(|poly| {
    let bounds = poly.bounding_rect().unwrap();
    let (x1, y1) = bounds.min().x_y();
    let x2 = x1 + bounds.width();
    let y2 = y1 + bounds.height();
    let f = |p: (f64, f64)| (x1 + p.0 * (x2 - x1), y1 + p.1 * (y2 - y1));
    let mut rng = rng_from_seed(opts.seed + 7.77 * x1 + y1 / 3.);
    let mut candidates = sample_2d_candidates(
      &|p| {
        let q = f(p);
        poly.intersects(&Point::from(q))
      },
      400,
      20 + (0.6 * bounds.width() * bounds.height()) as usize,
      &mut rng,
    );

    candidates = candidates.iter().map(|&p| f(p)).collect();

    route_spiral(candidates)
  })
  .collect::<Vec<_>>();

  let mut layers = Vec::new();

  let color = "black";
  let mut l = layer(color);
  l = l.add(signature(0.8, (180.0, 193.0), color));
  let mut data = Data::new();
  for route in routes {
    data = render_route_curve(data, route);
  }
  l = l.add(base_path(color, stroke_width, data));
  layers.push(l);

  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
