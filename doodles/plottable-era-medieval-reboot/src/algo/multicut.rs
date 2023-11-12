use crate::algo::clipping::*;
use crate::algo::math2d::*;
use crate::algo::polygon::*;
use rand::prelude::*;
use std::f64::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn multicut_along_line<R: Rng>(
  rng: &mut R,
  routes_in: &Vec<(usize, Vec<(f64, f64)>)>,
  polys_in: &Vec<Vec<(f64, f64)>>,
  clr: usize,
  from: (f64, f64),
  to: (f64, f64),
  mut increment_f: impl FnMut(&mut R) -> f64,
  mut angle_delta_f: impl FnMut(&mut R) -> f64,
  mut sliding_f: impl FnMut(&mut R) -> f64,
  mut pushback_f: impl FnMut(&mut R) -> f64,
  mut pushback_rotation_f: impl FnMut(&mut R) -> f64,
) -> (Vec<(usize, Vec<(f64, f64)>)>, Vec<Vec<(f64, f64)>>) {
  let mut routes = routes_in.clone();
  let mut polys = polys_in.clone();
  let initial = increment_f(rng) / 2.0;
  let mut d = initial;
  let l = euclidian_dist(from, to);
  let dx = to.0 - from.0;
  let dy = to.1 - from.1;
  let a = dy.atan2(dx);
  while d < l - initial {
    let p = lerp_point(from, to, d / l);
    let ang = a + PI / 2.0 + angle_delta_f(rng);
    let sliding = sliding_f(rng);
    let pushback = pushback_f(rng);
    let pushback_rotation = pushback_rotation_f(rng);
    let o = binary_cut_and_slide(
      &routes,
      &polys,
      p,
      ang,
      sliding,
      pushback,
      pushback_rotation,
      clr,
    );
    routes = o.0;
    polys = o.1;
    d += increment_f(rng);
  }
  (routes, polys)
}

pub fn binary_cut_and_slide(
  routes_in: &Vec<(usize, Vec<(f64, f64)>)>,
  polys_in: &Vec<Vec<(f64, f64)>>,
  center: (f64, f64),
  ang: f64,
  sliding: f64,
  pushback: f64,
  pushback_rotation: f64,
  clr: usize,
) -> (Vec<(usize, Vec<(f64, f64)>)>, Vec<Vec<(f64, f64)>>) {
  let mut routes = vec![];
  let mut polys = vec![];

  let dx = ang.cos();
  let dy = ang.sin();
  let amp = 1000.0;
  let a = (center.0 + amp * dx, center.1 + amp * dy);
  let b = (center.0 - amp * dx, center.1 - amp * dy);

  let is_left =
    |(x, y)| (x - center.0) * (b.1 - a.1) - (y - center.1) * (b.0 - a.0) > 0.0;

  let is_right = |p| !is_left(p);

  let project = |(x, y), leftmul| {
    let local = (x - center.0, y - center.1);
    let local = p_r(local, pushback_rotation * leftmul);

    (
      center.0 + local.0 + (sliding * dx - pushback * dy) * leftmul,
      center.1 + local.1 + (sliding * dy + pushback * dx) * leftmul,
    )
  };

  for poly in polys_in.clone() {
    let out = cut_polygon(&poly, a, b);
    for p in out {
      let mut c = (0., 0.);
      for point in p.iter() {
        c.0 += point.0;
        c.1 += point.1;
      }
      let len = p.len() as f64;
      c = (c.0 / len, c.1 / len);

      let leftmul = if is_left(c) { 1.0 } else { -1.0 };

      let p = p.iter().map(|&p| project(p, leftmul)).collect();

      polys.push(p);
    }
  }

  let mut left_routes = clip_routes_with_colors(&routes_in, &is_right, 0.3, 4);
  let mut right_routes = clip_routes_with_colors(&routes_in, &is_left, 0.3, 4);

  let out_of_polys = |p| !polygons_includes_point(polys_in, p);

  let cut_routes =
    clip_routes_with_colors(&vec![(clr, vec![a, b])], &out_of_polys, 0.3, 4);

  left_routes.extend(cut_routes.clone());
  right_routes.extend(cut_routes.clone());

  let data = vec![(1.0, left_routes), (-1.0, right_routes)];
  for (leftmul, rts) in data {
    for (clr, rt) in rts {
      let newrt = rt.iter().map(|&p| project(p, leftmul)).collect();
      routes.push((clr, newrt));
    }
  }

  (routes, polys)
}
