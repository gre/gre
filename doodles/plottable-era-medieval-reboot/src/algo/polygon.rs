/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
use crate::algo::math2d::*;

use super::polylines::Polyline;

pub fn polygon_bounds(polygon: &Vec<(f32, f32)>) -> (f32, f32, f32, f32) {
  let mut minx = f32::MAX;
  let mut miny = f32::MAX;
  let mut maxx = f32::MIN;
  let mut maxy = f32::MIN;
  for &(x, y) in polygon {
    minx = minx.min(x);
    miny = miny.min(y);
    maxx = maxx.max(x);
    maxy = maxy.max(y);
  }
  (minx, miny, maxx, maxy)
}

pub fn polygon_centroid(polygon: &Vec<(f32, f32)>) -> (f32, f32) {
  let mut sx = 0.0;
  let mut sy = 0.0;
  for p in polygon.iter() {
    sx += p.0;
    sy += p.1;
  }
  let s = polygon.len() as f32;
  (sx / s, sy / s)
}

pub fn polygons_includes_point(
  polygons: &Vec<Vec<(f32, f32)>>,
  p: (f32, f32),
) -> bool {
  for polygon in polygons {
    if polygon_includes_point(polygon, p) {
      return true;
    }
  }
  false
}

pub fn polygon_includes_point(
  polygon: &Vec<(f32, f32)>,
  p: (f32, f32),
) -> bool {
  let mut inside = false;
  let mut j = polygon.len() - 1;
  for i in 0..polygon.len() {
    let pi = polygon[i];
    let pj = polygon[j];
    if (pi.1 > p.1) != (pj.1 > p.1)
      && p.0 < (pj.0 - pi.0) * (p.1 - pi.1) / (pj.1 - pi.1) + pi.0
    {
      inside = !inside;
    }
    j = i;
  }
  inside
}

pub fn cut_polygon(
  poly: &Vec<(f32, f32)>,
  a: (f32, f32),
  b: (f32, f32),
) -> Vec<Vec<(f32, f32)>> {
  let poly = if !same_point(poly[0], poly[poly.len() - 1]) {
    let mut poly = poly.clone();
    poly.push(poly[0]);
    poly
  } else {
    poly.clone()
  };
  let mut prev: Option<(f32, f32)> = None;
  let mut first = Vec::new();
  let mut second = Vec::new();
  let mut on_first = true;
  for p in poly.clone() {
    let to = p;
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
  return vec![first, second];
}

pub fn make_wireframe_from_vertexes(
  vertex1: &Polyline,
  vertex2: &Polyline,
) -> Vec<Vec<(f32, f32)>> {
  // vertex1 and 2 are supposed to be same size
  let len = vertex1.len().min(vertex2.len());
  let mut polys = vec![];
  if len < 2 {
    return polys;
  }
  for i in 0..(len - 1) {
    let j = i + 1;
    let mut poly = vec![];
    poly.push(vertex1[i]);
    poly.push(vertex1[j]);
    poly.push(vertex2[j]);
    poly.push(vertex2[i]);
    polys.push(poly);
  }
  polys
}

pub fn make_tri_wireframe_from_vertexes(
  vertex1: &Polyline,
  vertex2: &Polyline,
) -> Vec<Vec<(f32, f32)>> {
  // vertex1 and 2 are supposed to be same size
  let len = vertex1.len().min(vertex2.len());
  let mut polys = vec![];
  if len < 2 {
    return polys;
  }
  for i in 0..(len - 1) {
    let j = i + 1;
    let mut poly = vec![];
    poly.push(vertex1[i]);
    poly.push(vertex1[j]);
    poly.push(vertex2[j]);
    polys.push(poly);
    let mut poly = vec![];
    poly.push(vertex2[j]);
    poly.push(vertex2[i]);
    poly.push(vertex1[i]);
    polys.push(poly);
  }
  polys
}

pub fn polygons_find_miny(
  polygons: &Vec<Vec<(f32, f32)>>,
  x: f32,
) -> Option<f32> {
  let mut value: Option<f32> = None;

  for polygon in polygons {
    for i in 0..polygon.len() {
      let (x1, y1) = polygon[i];
      let (x2, y2) = polygon[(i + 1) % polygon.len()];

      // Check if the line segment is vertical
      if x1 == x2 {
        continue;
      }

      // Ensure x is between x1 and x2
      if (x1 < x && x < x2) || (x2 < x && x < x1) {
        // Linear interpolation to find y at given x
        let y = y1 + (x - x1) * (y2 - y1) / (x2 - x1);

        value = match value {
          Some(hy) if y < hy => Some(y),
          None => Some(y),
          _ => value,
        };
      }
    }
  }

  value
}
