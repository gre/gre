use std::f64::consts::PI;

use clap::*;
use geo::intersects::Intersects;
use geo::polygon;
use geo::prelude::BoundingRect;
use geo::prelude::Contains;
use geo::Point;
use geo::Polygon;
use geo::*;
use gre::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn add(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
  (a.0 + b.0, a.1 + b.1)
}

fn rotated_square_as_polygon(
  x: f64,
  y: f64,
  size: f64,
  angle: f64,
) -> Polygon<f64> {
  polygon![
    p_r((x - size, y - size), angle).into(),
    p_r((x + size, y - size), angle).into(),
    p_r((x + size, y + size), angle).into(),
    p_r((x - size, y + size), angle).into(),
  ]
}

fn rotated_tri_as_polygon(
  x: f64,
  y: f64,
  size: f64,
  angle: f64,
) -> Polygon<f64> {
  polygon![
    add(p_r((0.0, -size), angle), (x, y)).into(),
    add(p_r((0.0, -size), angle + 2. * PI / 3.0), (x, y)).into(),
    add(p_r((0.0, -size), angle - 2. * PI / 3.0), (x, y)).into(),
  ]
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

fn scaling_search_in_container<F: FnMut(f64, f64, f64, f64) -> Polygon<f64>>(
  mut make_shape: F,
  container: &Polygon<f64>,
  polys: &Vec<Polygon<f64>>,
  x: f64,
  y: f64,
  angle: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let poly = make_shape(x, y, size, angle);
    container.contains(&poly) && !poly_collides_in_polys(polys, &poly)
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn poly_accumulate<F: FnMut(f64, f64, f64, f64) -> Polygon<f64>>(
  mut make_shape: F,
  seed: f64,
  iterations: usize,
  desired_count: usize,
  pad: f64,
  container: &Polygon<f64>,
  min_scale: f64,
  a: f64,
) -> Vec<Polygon<f64>> {
  let mut polys = Vec::new();
  let mut rng = rng_from_seed(seed);
  let bounds = container.bounding_rect().unwrap();
  let topleft: Point<f64> = bounds.min().into();
  let bottomright: Point<f64> = topleft
    + point!(
        x: bounds.width(),
        y: bounds.height()
    );
  let max_scale = (bounds.width()).max(bounds.height());
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(topleft.x(), bottomright.x());
    let y: f64 = rng.gen_range(topleft.y(), bottomright.y());
    if let Some(size) = scaling_search_in_container(
      &mut make_shape,
      &container,
      &polys,
      x,
      y,
      a,
      min_scale,
      max_scale,
    ) {
      let poly = make_shape(x, y, size - pad, a);
      polys.push(poly);
    }
    if polys.len() > desired_count {
      break;
    }
  }

  polys
}

fn art(opts: Opts) -> Vec<Group> {
  let width = 300.0;
  let height = 240.0;
  let pad = 20.0;
  let bounds = (pad, pad, width - pad, height - pad);
  let stroke_width = 0.3;

  let bounds_container = polygon![
    (bounds.0, bounds.1).into(),
    (bounds.2, bounds.1).into(),
    (bounds.2, bounds.3).into(),
    (bounds.0, bounds.3).into(),
  ];

  let primaries = poly_accumulate(
    &rotated_tri_as_polygon,
    opts.seed,
    500000,
    2000,
    0.6,
    &bounds_container,
    1.0,
    0.0,
  );

  let secondaries = primaries
    .par_iter()
    .filter(|p| p.bounding_rect().unwrap().width() > 2.0)
    .map(|p| {
      poly_accumulate(
        &rotated_square_as_polygon,
        opts.seed,
        100000,
        2000,
        0.2,
        &p,
        0.8,
        0.0,
      )
    })
    .collect::<Vec<_>>()
    .concat();

  let mut layers = Vec::new();

  let color = "black";
  let mut l = layer(color);
  let mut data = Data::new();
  for poly in secondaries {
    data = render_polygon_stroke(data, poly.clone());
  }
  l = l.add(base_path(color, stroke_width, data));
  l = l.add(signature(0.8, (258.0, 216.0), color));
  layers.push(l);

  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_24x30_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
