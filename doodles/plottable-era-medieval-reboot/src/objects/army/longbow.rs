use super::{
  body::{HumanBody, HumanJointAngles},
  head::head_square,
};
use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  paintmask::PaintMask,
  polylines::{
    grow_path_zigzag, path_subdivide_to_curve, route_translate_rotate,
  },
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn long_bow(
  clr: usize,
  origin: (f32, f32),
  size: f32,
  angle: f32,
  phase: f32,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes: Vec<Vec<(f32, f32)>> = Vec::new();

  // arc au repos
  let dy = 0.5 * size;
  let dx = 0.5 * dy;
  let bow_w = 0.1 * size;

  let max_allonge = 0.8 * size;
  let allonge = mix(dx, max_allonge, phase);

  let mut route = vec![];
  route.push((-dx, -dy));
  route.push((0.0, 0.0));
  route.push((-dx, dy));
  let bow = path_subdivide_to_curve(&route, 2, 0.8);

  routes.push(grow_path_zigzag(bow, angle, bow_w, 0.3));

  let string = vec![(-dx, -dy), (-allonge, 0.0), (-dx, dy)];

  routes.push(string);

  // translate routes
  let out = routes
    .iter()
    .map(|route| {
      let route = route_translate_rotate(route, origin, angle);
      (clr, route)
    })
    .collect();

  out
}
