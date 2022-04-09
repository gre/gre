use clap::Clap;
use geo::prelude::*;
use geo::*;
use gre::*;
use rand::Rng;
use std::cmp::Ordering;
use svg::node::element::path::Data;
use svg::node::element::Group;

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

fn samples_polygon_edge<R: Rng>(
  polygon: Polygon<f64>,
  samples: f64,
  borderdist: f64,
  rng: &mut R,
) -> (Vec<(f64, f64)>, Vec<usize>) {
  let mut length = 0.;
  let poly: Vec<Point<f64>> =
    polygon.exterior().points_iter().collect();
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

fn sort_point(
  a: &(f64, (f64, f64), usize),
  b: &(f64, (f64, f64), usize),
) -> Ordering {
  ((if a.2 == b.2 { 8. } else { 1. }) * (b.0 - a.0))
    .partial_cmp(&(0.0))
    .unwrap()
}

fn randomize_across_groups<R: Rng>(
  points: Vec<(f64, f64)>,
  groups: Vec<usize>,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  let mut pts: Vec<(f64, (f64, f64), usize)> = points
    .iter()
    .enumerate()
    .map(|(i, &p)| (rng.gen_range(0.0, 1.0), p, groups[i]))
    .collect();
  pts.sort_by(sort_point);
  return pts.iter().map(|p| p.1).collect();
}

fn art(opts: &Opts) -> Vec<Group> {
  let color = "#000";
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;
  let stroke_width = 0.35;
  let borderdist = 4.0;
  let seed = opts.seed;
  let mut layers = Vec::new();
  let mut rng = rng_from_seed(seed);
  let samples_amp = rng.gen_range(3., 5.);
  let voronoi_size = (rng.gen_range(60., 600.)
    * rng.gen_range(0.2, 1.0))
    as usize;
  let truncate = if rng.gen_bool(0.1) {
    ((voronoi_size as f64) * rng.gen_range(0.3, 0.6))
      as usize
  } else {
    voronoi_size
  };
  let r = rng.gen_range(0.8, 4.0);
  let poly_threshold = 0.005;

  let candidates = sample_2d_candidates_f64(
    &|p| {
      let d = euclidian_dist(p, (0.5, 0.5));
      1. - r * d
    },
    800,
    voronoi_size,
    &mut rng,
  );
  let mut polys =
    sample_square_voronoi_polys(candidates, 0.0);

  polys.retain(|poly| {
    poly_bounding_square_edge(poly) > poly_threshold
  });
  rng.shuffle(&mut polys);
  polys.truncate(truncate);

  let routes: Vec<Vec<(f64, f64)>> = polys
    .iter()
    .filter_map(|p| {
      let out_of_bounds =
        p.exterior().points_iter().any(|p| {
          p.x() < 0.0
            || p.y() < 0.0
            || p.x() > 1.0
            || p.y() > 1.0
        });
      if out_of_bounds {
        return None;
      }
      let poly = p.map_coords(|&c| {
        (
          pad + c.0 * (width - 2. * pad),
          pad + c.1 * (height - 2. * pad),
        )
      });
      let samples =
        samples_amp * poly.signed_area().powf(0.5);
      let (points, groups) = samples_polygon_edge(
        poly.clone(),
        samples,
        borderdist,
        &mut rng,
      );
      Some(randomize_across_groups(
        points, groups, &mut rng,
      ))
    })
    .collect();

  let mut l = layer(color);
  let mut data = Data::new();
  let curve = rng.gen_bool(0.3);
  for r in routes.clone() {
    if curve {
      data = render_route_curve(data, r);
    } else {
      data = render_route(data, r);
    }
  }
  l = l.add(base_path(color, stroke_width, data));
  layers.push(l);
  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document =
    base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
