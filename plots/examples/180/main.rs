use std::f64::consts::PI;

use clap::*;
use geo::intersects::Intersects;
use geo::polygon;
use geo::prelude::BoundingRect;
use geo::Point;
use geo::Polygon;
use geo::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
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

fn poly_collides_in_polys(
  polys: &Vec<Polygon<f64>>,
  poly: &Polygon<f64>,
) -> bool {
  polys.iter().any(|p| poly.intersects(p))
}

fn poly_square_scaling_search(
  boundaries: (f64, f64, f64, f64),
  polys: &Vec<Polygon<f64>>,
  x: f64,
  y: f64,
  angle: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let poly = rotated_square_as_polygon(x, y, size, angle);
    let bounds = poly.bounding_rect().unwrap();
    let topleft: Point<f64> = bounds.min().into();
    let bottomright: Point<f64> = topleft
      + point!(
          x: bounds.width(),
          y: bounds.height()
      );
    out_of_boundaries(topleft.x_y(), boundaries)
      || out_of_boundaries(bottomright.x_y(), boundaries)
      || poly_collides_in_polys(polys, &poly)
  };

  let mut from = min_scale;
  let mut to = max_scale;
  loop {
    if overlaps(from) {
      return None;
    }
    if to - from < 0.1 {
      return Some(from);
    }
    let middle = (to + from) / 2.0;
    if overlaps(middle) {
      to = middle;
    } else {
      from = middle;
    }
  }
}

fn art(opts: Opts) -> Vec<Group> {
  let colors = vec!["black"];
  let width = 300.0;
  let height = 240.0;
  let radius = 110.0;
  let bounds = (
    width / 2. - radius,
    height / 2. - radius,
    width / 2. + radius,
    height / 2. + radius,
  );
  let stroke_width = 0.3;
  let upper_limit = 500000;
  let desired_count = 3000;
  let pad = 0.4;
  let min_threshold = 0.8;
  let angle_mod = 10.0;

  let mut polys = Vec::new();
  let mut shapes = Vec::new();
  let mut rng = rng_from_seed(opts.seed);
  for _i in 0..upper_limit {
    let x: f64 = rng.gen_range(bounds.0, bounds.2);
    let y: f64 = rng.gen_range(bounds.1, bounds.3);
    let a: f64 = rng.gen_range(0f64, angle_mod).floor() * 2. * PI / angle_mod;
    if let Some(size) = poly_square_scaling_search(
      bounds,
      &polys,
      x,
      y,
      a,
      min_threshold,
      0.5 * (bounds.2 - bounds.0).max(bounds.3 - bounds.1),
    ) {
      let mult = mix(0.2, 1.0, smoothstep(0.0, 20.0, polys.len() as f64));
      let poly = rotated_square_as_polygon(x, y, mult * size - pad, a);
      polys.push(poly);
      shapes.push(rotated_square_as_polygon(x, y, mult * size - pad, a));
      shapes.push(rotated_square_as_polygon(x, y, mult * size - 2. * pad, a));
    }
    if polys.len() > desired_count {
      break;
    }
  }

  println!("{} shapes", polys.len());

  colors
    .iter()
    .enumerate()
    .map(|(ci, &color)| {
      let mut l = layer(color);
      let mut data = Data::new();
      for (i, poly) in shapes.iter().enumerate() {
        if ci == i % colors.len() {
          data = render_polygon_stroke(data, poly.clone());
        }
      }

      if ci == colors.len() - 1 {
        l = l.add(signature(1.0, (230.0, 216.0), color));
      }
      l = l.add(base_path(color, stroke_width, data));
      l
    })
    .collect()
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
