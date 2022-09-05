use clap::*;
use geo::polygon;
use geo::prelude::Area;
use geo::prelude::BoundingRect;
use geo::prelude::Centroid;
use geo::translate::Translate;
use geo::Polygon;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

// a simple implementation of cutting a convex polygon in 2 with a line
fn cut_polygon(
  poly: &Polygon<f64>,
  a: (f64, f64),
  b: (f64, f64),
) -> Vec<Polygon<f64>> {
  let mut prev: Option<(f64, f64)> = None;
  let mut first = Vec::new();
  let mut second = Vec::new();
  let mut on_first = true;
  for p in poly.exterior().points_iter() {
    let to = p.x_y();
    if let Some(from) = prev {
      let collision = collides_segment(from, to, a, b);
      if let Some(c) = collision {
        first.push(c);
        second.push(c);
        on_first = !on_first;
      }
    }
    if on_first {
      first.push(to);
    } else {
      second.push(to);
    }
    prev = Some(to);
  }
  if second.len() < 2 {
    return vec![poly.clone()];
  }
  return vec![
    Polygon::new(first.into(), vec![]),
    Polygon::new(second.into(), vec![]),
  ];
}

fn rec<R: Rng>(
  rng: &mut R,
  polygon: &Polygon<f64>,
  depth: usize,
) -> Vec<Polygon<f64>> {
  let mut center = polygon.centroid().unwrap();
  let bounds = polygon.bounding_rect().unwrap();
  let w = bounds.width();
  let h = bounds.height();
  let max = 1.001 - 1. / ((depth as f64) + 1.);
  center = center.translate(
    rng.gen_range(0.0, max) * rng.gen_range(-0.5, 0.5) * w,
    rng.gen_range(0.0, max) * rng.gen_range(-0.5, 0.5) * h,
  );
  let ang = rng.gen_range(0.0, 2. * PI);
  let dx = ang.cos();
  let dy = ang.sin();
  let amp = 100.0;
  let a = center.translate(amp * dx, amp * dy).x_y();
  let b = center.translate(-amp * dx, -amp * dy).x_y();
  let mut cut = cut_polygon(polygon, a, b);
  if cut.len() == 1 {
    return vec![polygon.clone()];
  }

  // move the pieces
  cut = cut
    .iter()
    .map(|p| {
      let newcenter = p.centroid().unwrap();
      let dx = newcenter.x() - center.x();
      let dy = newcenter.y() - center.y();
      let dist = (dx * dx + dy * dy).sqrt();
      let amp = depth as f64 * 1.0 + 0.8;
      let poly = p.translate(amp * dx / dist, amp * dy / dist);
      poly
    })
    .collect();

  if depth <= 0 || rng.gen_range(0.0, 1.0) < 0.05 {
    return cut;
  }
  let mut all = Vec::new();
  for poly in cut {
    let inside = rec(rng, &poly, depth - 1);
    for p in inside {
      if p.signed_area() > 0.5 {
        all.push(p);
      }
    }
  }
  return all;
}

fn art(opts: Opts) -> Vec<Group> {
  let colors = vec!["black"];

  let mut rng = rng_from_seed(opts.seed);

  let width = 297.;
  let height = 210.;
  let size = rng.gen_range(100.0, 150.0);
  let x1 = (width - size) / 2.0;
  let x2 = (width + size) / 2.0;
  let y1 = (height - size) / 2.0;
  let y2 = (height + size) / 2.0;

  let poly1 = polygon![
    (x1, y1).into(),
    (x2, y1).into(),
    (x2, y2).into(),
    (x1, y2).into(),
  ];

  let depth = rng.gen_range(4, 8);
  let mut polygons = rec(&mut rng, &poly1, depth);

  let routes: Vec<Vec<(f64, f64)>> = polygons
    .iter()
    .map(|poly| {
      let volume = poly.signed_area();
      let samples = (1.6 * volume) as usize;
      let dim = 200;
      route_spiral(samples_polygon(&poly, samples, dim, &mut rng))
    })
    .collect();

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for poly in polygons.iter() {
        data = render_polygon_stroke(data, poly.clone());
      }
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "1.0")]
  seed: f64,
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
