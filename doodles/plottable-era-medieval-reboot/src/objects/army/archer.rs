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

pub fn bowman<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  clr: usize,
  origin: (f32, f32),
  size: f32,
  xflip: bool,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let phase = rng.gen_range(0.0..1.0);
  let shoulder_right_angle = mix(0.0, -PI / 4.0, phase);
  let elbow_right_angle = shoulder_right_angle;

  let joints = HumanJointAngles {
    body_angle: -PI / 2.0,
    head_angle: -PI / 2.0,
    shoulder_right_angle,
    shoulder_left_angle: rng.gen_range(3.0 * PI / 4.0..PI),
    elbow_right_angle,
    elbow_left_angle: PI / 2.0 + 0.3,
    hip_right_angle: PI / 2.0 - 0.5,
    hip_left_angle: PI / 2.0 + 0.5,
    knee_right_angle: PI / 2.0,
    knee_left_angle: PI / 2.0,
    left_arm_bend: 0.5,
    right_arm_bend: 1.0,
    left_leg_bend: 1.0,
    right_leg_bend: 1.0,
  };
  let humansize = size * 0.5;
  let xcenter = origin.0;
  let human = HumanBody::new((xcenter, origin.1), humansize, joints);
  let mut new_routes = vec![];

  new_routes.extend(human.render(paint, clr));
  let (headpos, headangle) = human.head_pos_angle();
  let h = head_square(clr, headpos, headangle, humansize);
  new_routes.extend(h);

  let (pos, angle) = human.hand_right_pos_angle();

  let bow = long_bow(clr, pos, size * 0.5, -angle, phase);
  new_routes.extend(bow);

  new_routes = regular_clip(&new_routes, paint);

  for (_clr, route) in &new_routes {
    paint.paint_polyline(route, 0.08 * size);
  }

  new_routes
}

fn long_bow(
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
  let bow = path_subdivide_to_curve(route, 2, 0.8);

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
