use std::f64::consts::PI;

use crate::clip_routes_with_colors;

pub struct SliceProps {
  pub center: (f64, f64),
  pub angle: f64,
  pub slice_width: f64,
}

fn signed_distance_point_to_line(
  p: (f64, f64),
  center: (f64, f64),
  angle_rad: f64,
) -> f64 {
  let dir_x = angle_rad.cos();
  let dir_y = angle_rad.sin();
  let numerator = (center.0 - p.0) * dir_y - (center.1 - p.1) * dir_x;
  numerator
}

/**
 * takes a bunch of strokes as inputs and cut them all into slices,
 * organized
 */
pub fn slice_into_parts(
  input: &Vec<(usize, Vec<(f64, f64)>)>,
  slice_props: &SliceProps,
) -> Vec<Vec<(usize, Vec<(f64, f64)>)>> {
  let mut parts = Vec::new();

  parts.push(input.clone());

  // slice 0 = the one in the center
  // slice 1 = one on the left
  // slice 2 = one on the right
  // slice 3 = one on the left after the first one
  // slice 4 = one on the right after the first one
  // ....

  // while we find inputs to slice, we will continue slicing.
  // we start with the center and with the props' angle and we shift based on the function shift_for_index

  let mut lowest_dist = 0.0;
  let mut highest_dist = 0.0;
  for (_i, route) in input.iter() {
    for &(x, y) in route.iter() {
      let dist = signed_distance_point_to_line(
        (x, y),
        slice_props.center,
        slice_props.angle,
      );
      if dist > highest_dist {
        highest_dist = dist;
      }
      if dist < lowest_dist {
        lowest_dist = dist;
      }
    }
  }

  let slice_width = slice_props.slice_width;
  let count =
    ((highest_dist.max(lowest_dist.abs()) * 2.0) / slice_width).ceil() as usize;
  for i in 0..count {
    let is_outside = |p: (f64, f64)| {
      let dist =
        signed_distance_point_to_line(p, slice_props.center, slice_props.angle);
      let mut index = (dist / slice_width).round();
      if index > 0.0 {
        index *= 2.0;
      } else {
        index = -2.0 * index + 1.0;
      }
      index as usize != i
    };
    let part = clip_routes_with_colors(input, &is_outside, 1.0, 5);
    parts.push(part);
  }

  parts
}

pub fn shift_a_slice(
  input: &Vec<(usize, Vec<(f64, f64)>)>,
  slice_props: &SliceProps,
  displacement: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  let angle = slice_props.angle + PI / 2.0;
  for (i, route) in input.iter() {
    let mut new_route = Vec::new();
    for &(x, y) in route.iter() {
      let new_x = x + displacement * angle.cos();
      let new_y = y + displacement * angle.sin();
      new_route.push((new_x, new_y));
    }
    routes.push((*i, new_route));
  }
  routes
}
