use clap::*;
use geo::prelude::*;
use geo::*;
use gre::*;
use rand::Rng;
use rayon::prelude::*;
use std::f64::consts::PI;
use svg::node::element::{path::Data, Group};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "24.0")]
  seed: f64,
  #[clap(short, long, default_value = "700.0")]
  width: f64,
  #[clap(short, long, default_value = "500.0")]
  height: f64,
}

fn make_polygon(
  count: usize,
  x: f64,
  y: f64,
  size: f64,
  angle: f64,
) -> Polygon<f64> {
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
  polygoncount: usize,
  container: &Polygon<f64>,
  polys: &Vec<Polygon<f64>>,
  x: f64,
  y: f64,
  angle: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let p = &make_polygon(polygoncount, x, y, size, angle);
    p.exterior().points().all(|c| container.contains(&c))
      && !poly_collides_in_polys(polys, p)
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  polygoncount: usize,
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  container: &Polygon<f64>,
  min_scale: f64,
  max_scale: f64,
) -> Vec<Polygon<f64>> {
  let mut polys = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let bound = container.bounding_rect().unwrap();
  let (x1, y1) = bound.min().x_y();
  let x2 = x1 + bound.width();
  let y2 = y1 + bound.height();
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    let angle = rng.gen_range(0f64, 2. * PI);
    if let Some(size) = search(
      polygoncount,
      container,
      &polys,
      x,
      y,
      angle,
      min_scale,
      max_scale,
    ) {
      tries.push((x, y, size - pad, angle));
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let (x, y, s, a) = tries[0];
        let p = make_polygon(polygoncount, x, y, s, a);
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

fn rec(
  depth: usize,
  polygoncount: usize,
  seed: f64,
  container: &Polygon<f64>,
  a: f64,
  b: f64,
) -> Vec<Vec<(f64, f64)>> {
  let m = container
    .bounding_rect()
    .unwrap()
    .width()
    .max(container.bounding_rect().unwrap().height());
  if depth <= 0 || m < 2.0 {
    return vec![container.exterior().points().map(|p| p.x_y()).collect()];
  }
  let mut rng = rng_from_seed(seed);
  let nextpolygoncount = if rng.gen_bool(0.5) {
    polygoncount
  } else {
    rng.gen_range(3, 6)
  };
  let pad = a + b * (depth % 3) as f64;
  let mut routes: Vec<Vec<(f64, f64)>> = packing(
    polygoncount,
    seed,
    100000,
    1000,
    1 + (rng.gen_range(0., 80.) * rng.gen_range(0.0, 1.0)) as usize,
    pad,
    container,
    0.2 + pad,
    m.max(0.3 + pad),
  )
  .par_iter()
  .flat_map(|poly| {
    rec(depth - 1, nextpolygoncount, 7.6 + seed * 3.3, &poly, a, b)
  })
  .collect();
  routes.push(container.exterior().points().map(|p| p.x_y()).collect());
  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = 20.0;
  let width = opts.width;
  let height = opts.height;
  let stroke_width = 2.0;
  let mut rng = rng_from_seed(opts.seed);
  let polygoncount = rng.gen_range(3, 6);

  let container = Polygon::new(
    vec![
      (pad, pad),
      (width - pad, pad),
      (width - pad, height - pad),
      (pad, height - pad),
    ]
    .into(),
    vec![],
  );

  let a = rng.gen_range(3.0, 8.0);
  let b = rng.gen_range(1.0, 2.5);

  let routes = rec(
    rng.gen_range(2, 8),
    polygoncount,
    opts.seed,
    &container,
    a,
    b,
  );
  let mut layers = Vec::new();

  let color = "black";
  let mut l = layer(color);
  let mut data = Data::new();
  for route in routes {
    data = render_route(data, route);
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
