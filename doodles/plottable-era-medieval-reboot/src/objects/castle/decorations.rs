use crate::algo::math2d::{euclidian_dist, lerp_point};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn wall_shadow<R: Rng>(
  rng: &mut R,
  path: Vec<(f32, f32)>,
  stroke_len: f32,
) -> Vec<Vec<(f32, f32)>> {
  let mut routes = Vec::new();
  if path.len() < 2 {
    return routes;
  }
  let mut prev = path[0];
  let mut current_l = euclidian_dist(prev, path[1]);
  let mut direction = (-1.0, 0.0);
  let mut i = 0;
  let mut l = 0.0;
  loop {
    while l > current_l {
      l -= current_l;
      prev = path[i];
      i += 1;
      if i >= path.len() {
        return routes;
      }
      current_l = euclidian_dist(prev, path[i]);
      let dx = path[i].0 - prev.0;
      let dy = path[i].1 - prev.1;
      direction = (-dy / current_l, dx / current_l);
    }
    let p = lerp_point(prev, path[i], l / current_l);
    let slen = stroke_len * rng.gen_range(0.8..1.2);
    routes.push(vec![
      p,
      (p.0 + slen * direction.0, p.1 + slen * direction.1),
    ]);

    l += rng.gen_range(0.8..1.2) * stroke_len.abs();
  }
}

pub fn merlon(
  polys: &mut Vec<Vec<(f32, f32)>>,
  route: &mut Vec<(f32, f32)>,
  leftx: f32,
  lefty: f32,
  rightx: f32,
  _righty: f32,
  h: f32,
) {
  let mut count = ((rightx - leftx) / h).ceil();
  count = (count / 2.0).floor() * 2.0 + 1.0;
  if count <= 0.0 {
    return;
  }
  let w = (rightx - leftx) / count;
  let mut x = leftx;
  let mut alt = false;
  loop {
    if x > rightx - w / 2.0 {
      break;
    }
    let y = lefty; // TODO interpolate lefty righty
    x += w;
    if alt {
      route.push((x, y + h));
      route.push((x, y));
    } else {
      if route.len() > 1 {
        let last = route[route.len() - 1];
        let minx = last.0;
        let miny = last.1;
        let maxx = x;
        let maxy = y + h;
        polys.push(vec![
          (minx, miny),
          (maxx, miny),
          (maxx, maxy),
          (minx, maxy),
        ]);
      }
      route.push((x, y));
      route.push((x, y + h));
    }
    alt = !alt;
  }
}
