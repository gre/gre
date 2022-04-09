use std::cmp::Ordering;
use clap::Clap;
use geo::prelude::EuclideanDistance;
use gre::*;
use geo::*;
use rand::Rng;
use svg::node::element::{Group};
use svg::node::element::path::Data;

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(short, long, default_value = "0.0")]
    seed: f64,
    #[clap(short, long, default_value = "100.0")]
    width: f64,
    #[clap(short, long, default_value = "100.0")]
    height: f64,
}


fn samples_polygon_edge<R: Rng> (
  polygon: Polygon<f64>,
  samples: f64,
  borderdist: f64,
  rng: &mut R
) -> (Vec<(f64, f64)>, Vec<usize>) {
  let mut length = 0.;
  let poly: Vec<Point<f64>> = polygon.exterior().points_iter().collect();
  let l = poly.len() - 1;
  let mut dists = Vec::new();
  let mut cx = 0.;
  let mut cy = 0.;
  for i in 0..l {
    let (x1, y1) = poly[i].x_y();
    cx += x1;
    cy += y1;
    let (x2, y2) = poly[i + 1].x_y();
    let dx = x1 - x2;
    let dy = y1 - y2;
    let d = (dx * dx + dy * dy).sqrt();
    length += d;
    dists.push(d);
  }
  cx /= l as f64;
  cy /= l as f64;
  let incr = length / samples;
  let mut points: Vec<(f64, f64)> = Vec::new();
  let mut groups: Vec<usize> = Vec::new();
  for i in 0..l {
    let (x1, y1) = poly[i].x_y();
    let (x2, y2) = poly[i + 1].x_y();
    let d = dists[i];
    let dx = x2 - x1;
    let dy = y2 - y1;
    let inc = incr / d;
    let mut v = 0.0;
    loop {
      if v > 1. {
        break;
      }
      let px = x1 + v * dx;
      let py = y1 + v * dy;
      let ddx = px - cx;
      let ddy = py - cy;
      let dist = (ddx * ddx + ddy * ddy).sqrt();
      let d = rng.gen_range(0.0, borderdist / dist);
      points.push((mix(px, cx, d), mix(py, cy, d)));
      groups.push(i);
      v += inc;
    }
  }
  return (points, groups);
}

fn sort_point (a: &(f64, (f64, f64), usize), b: &(f64, (f64, f64), usize)) -> Ordering {
  ((if a.2 == b.2 { 8. } else { 1. }) *
  (b.0 - a.0)).partial_cmp(&(0.0)).unwrap()
}

fn randomize_across_groups<R: Rng>(points: Vec<(f64, f64)>, groups: Vec<usize>, rng: &mut R) -> Vec<(f64, f64)> {
  let mut pts: Vec<(f64, (f64, f64), usize)> = points
    .iter()
    .enumerate()
    .map(|(i, &p)| (rng.gen_range(0.0, 1.0), p, groups[i]))
    .collect();
  pts.sort_by(sort_point);
  return pts.iter().map(|p| p.1).collect();
}

fn randomize_points_avoid<F: FnMut((f64, f64), (f64, f64)) -> bool>(
  points: Vec<(f64, f64)>,
  collides: &mut F,
  retries: usize
) -> Vec<(f64, f64)> {
  let mut pts: Vec<(f64, f64)> = points.clone();
  let last = points.len() - 1;
  for _r in 0..retries {
    for i in 1..last {
      if collides(pts[i - 1], pts[i]) {
        pts.swap(i, i + 1);
      }
    }
  }
  pts
}

fn rotated_square_as_polygon(x: f64, y: f64, size: f64, angle: f64) -> Polygon<f64> {
  polygon![
      p_r((x-size, y-size), angle).into(),
      p_r((x+size, y-size), angle).into(),
      p_r((x+size, y+size), angle).into(),
      p_r((x-size, y+size), angle).into(),
  ]
}

fn art(opts: &Opts) -> Vec<Group> {
    let color = "#000";
    let width = opts.width;
    let height = opts.height;
    let pad = 8.0;
    let stroke_width = 0.35;
    let samples = 1000.0;
    let avoids_count = 5000;
    let borderdist = 4.0;
    let seed = opts.seed;
    let mut layers = Vec::new();
    let mut rng = rng_from_seed(seed);
    let mut routes = Vec::new();

    let rect = rotated_square_as_polygon(width / 2.0, height / 2.0, width / 2.0 - pad, 0.0);
    let (points, groups) = samples_polygon_edge(rect, samples, borderdist, &mut rng);
    let mut route = randomize_across_groups(points, groups, &mut rng);
    
    let radius = rng.gen_range(1.0, 8.0) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
    
    let centers: Vec<(f64, f64)> = (0..rng.gen_range(1, 20)).map(|_i| (
      mix(pad, width-pad, 0.5 + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0)),
      mix(pad, height-pad, 0.5 + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0))
    )).collect();
    let mut collides = |a: (f64, f64), b: (f64, f64)| -> bool {
      let line = line_string![a.into(), b.into()];
      centers.iter().any(|center| 
        line.euclidean_distance(&Point::new(center.0, center.1)) < radius
      )
    };
    route = randomize_points_avoid(route, &mut collides, avoids_count);
    routes.push(route);

    let mut l = layer(color);
    let mut data = Data::new();
    for r in routes.clone() {
        data = render_route(data, r);
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
