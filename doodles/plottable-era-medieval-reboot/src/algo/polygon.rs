/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
use crate::algo::math2d::*;

pub fn polygon_bounds(polygon: &Vec<(f64, f64)>) -> (f64, f64, f64, f64) {
  let mut minx = f64::MAX;
  let mut miny = f64::MAX;
  let mut maxx = f64::MIN;
  let mut maxy = f64::MIN;
  for &(x, y) in polygon {
    minx = minx.min(x);
    miny = miny.min(y);
    maxx = maxx.max(x);
    maxy = maxy.max(y);
  }
  (minx, miny, maxx, maxy)
}

pub fn polygons_includes_point(
  polygons: &Vec<Vec<(f64, f64)>>,
  p: (f64, f64),
) -> bool {
  for polygon in polygons {
    if polygon_includes_point(polygon, p) {
      return true;
    }
  }
  false
}

pub fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  p: (f64, f64),
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
  poly: &Vec<(f64, f64)>,
  a: (f64, f64),
  b: (f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  let poly = if !same_point(poly[0], poly[poly.len() - 1]) {
    let mut poly = poly.clone();
    poly.push(poly[0]);
    poly
  } else {
    poly.clone()
  };
  let mut prev: Option<(f64, f64)> = None;
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
