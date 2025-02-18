use clap::*;
use geo::prelude::*;
use geo::*;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::{path::Data, Group};

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
  (x1, y1, x2, y2): (f64, f64, f64, f64),
  polys: &Vec<Polygon<f64>>,
  x: f64,
  y: f64,
  angle: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let p = &make_polygon(polygoncount, x, y, size, angle);
    p.exterior().points_iter().all(|c| {
      let (x, y) = c.x_y();
      x1 < x && x < x2 && y1 < y && y < y2
    }) && !poly_collides_in_polys(polys, p)
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
  container: (f64, f64, f64, f64),
  min_scale: f64,
  max_scale: f64,
) -> Vec<Polygon<f64>> {
  let mut polys = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let (x1, y1, x2, y2) = container;
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

fn art(opts: &Opts) -> Vec<Group> {
  let pad = 8.0;
  let width = opts.width;
  let height = opts.height;
  let stroke_width = 0.35;
  let mut rng = rng_from_seed(opts.seed);
  let polygoncount = rng.gen_range(3, 6);

  let routes = packing(
    polygoncount,
    opts.seed,
    200000,
    1000,
    1 + (rng.gen_range(0., 100.) * rng.gen_range(0.0, 1.0)) as usize,
    rng.gen_range(0.0, 0.2),
    (pad, pad, width - pad, height - pad),
    rng.gen_range(0.2, 0.4),
    rng.gen_range(1.0, 64.0),
  )
  .iter()
  .map(|poly| poly.exterior().points_iter().map(|p| p.x_y()).collect())
  .collect::<Vec<_>>();
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
