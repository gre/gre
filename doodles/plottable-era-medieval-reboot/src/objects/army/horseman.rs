use crate::algo::{
  clipping::regular_clip,
  math2d::p_r,
  paintmask::PaintMask,
  polylines::{
    grow_stroke_zigzag, path_subdivide_to_curve, route_scale_translate_rotate,
  },
};
use rand::prelude::*;
use std::f64::consts::PI;

use super::{
  body::{HumanBody, HumanJointAngles},
  helmet::helmet,
  shield::shield,
  spear::spear,
  sword::sword,
};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn horse_with_rider<R: Rng>(
  rng: &mut R,
  mask: &mut PaintMask,
  origin: (f64, f64),
  angle: f64,
  size: f64, // reference size (height of the boat)
  xflip: bool,
  mainclr: usize,
  skinclr: usize,
  is_leader: bool,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = vec![];
  let xdir = if xflip { -1.0 } else { 1.0 };

  let x0 = -size * rng.gen_range(0.4..0.5);
  let x1 = -size * rng.gen_range(0.3..0.4);
  let x2 = size * rng.gen_range(0.25..0.35);
  let x3 = size * rng.gen_range(0.4..0.5);
  let yleft = size * rng.gen_range(0.1..0.2);
  let yright = -size * rng.gen_range(0.6..0.8);

  let dy_edge = 0.3;
  // horse body bottom
  let mut route = Vec::new();
  route.push((x0, yleft + dy_edge));
  route.push((x1, 0.0));
  route.push((x2, 0.0));
  route.push((x3 + 0.05 * size, yright + dy_edge + 0.05 * size));
  route = path_subdivide_to_curve(route, 2, 0.8);
  routes.push((mainclr, route));

  // horse body top
  let mut route = Vec::new();
  let y = -0.3 * size;
  route.push((x0, yleft - dy_edge));
  route.push((x1, y));
  route.push((x2, y));
  route.push((x3, yright - dy_edge));
  route = path_subdivide_to_curve(route, 2, 0.8);
  // TODO route will be used to clip people
  routes.push((mainclr, route.clone()));

  // make horse head
  let a = (x3, yright);
  let b = (x3 + rng.gen_range(0.1..0.3) * size, yright + 0.3 * size);
  routes.push((mainclr, grow_stroke_zigzag(a, b, 0.2 * size, 0.5)));
  routes.push((mainclr, vec![a, b]));

  // make horse left feet
  let a = (x1 + 0.1 * size, y + 0.2 * size);
  let b = (x1 + rng.gen_range(-0.2..0.2) * size, y + 0.5 * size);
  routes.push((mainclr, grow_stroke_zigzag(a, b, 0.1 * size, 0.5)));
  routes.push((mainclr, vec![a, b]));

  // make horse right feet
  let a = (x3 - 0.1 * size, y);
  let b = (x3 + rng.gen_range(-0.2..0.1) * size, y + 0.4 * size);
  routes.push((mainclr, grow_stroke_zigzag(a, b, 0.1 * size, 0.5)));
  routes.push((mainclr, vec![a, b]));

  // humans

  let mut foreground_routes = Vec::new();
  //let mask_origin = (3.0 * size, 3.0 * size);
  //let mut foreground_mask =
  //  PaintMask::new(0.5, 2.0 * mask_origin.0, 2.0 * mask_origin.1);

  //let shape1 = rng.gen_range(0.0..1.0);
  //let shape2 = rng.gen_range(0.0..1.0);
  let x = 0.0;
  let joints = HumanJointAngles {
    body_angle: -PI / 2.0,
    head_angle: -PI / 2.0,
    shoulder_right_angle: rng.gen_range(0.0..PI / 4.0),
    shoulder_left_angle: rng.gen_range(3.0 * PI / 4.0..PI),
    elbow_right_angle: 0.3,
    elbow_left_angle: PI / 2.0 + 0.3,
    hip_right_angle: PI / 2.0 - 0.5,
    hip_left_angle: PI / 2.0 + 0.5,
    knee_right_angle: PI / 2.0,
    knee_left_angle: PI / 2.0,

    left_arm_bend: 0.5,
    right_arm_bend: 0.4,
    left_leg_bend: 0.0,
    right_leg_bend: 1.5,
  };
  let humansize = size * 0.5;
  let y = rng.gen_range(-0.1 * size..0.0);
  let human = HumanBody::new((x, y), humansize, joints);

  let human_body = human.render(mask, skinclr);
  routes.extend(human_body);

  let left_hand = human.hand_left_pos_angle();

  let obj_strokes = if is_leader {
    vec![] //flag(rng, mask, left_hand.0, size * 0.5, 4.0 * size, skinclr)
  } else if rng.gen_bool(0.5) {
    sword(rng, mask, left_hand.0, 0.5 * size, left_hand.1, mainclr)
  } else {
    spear(rng, left_hand.0, size * 0.5, left_hand.1, mainclr)
  };
  routes.extend(obj_strokes);

  let (headpos, headangle) = human.head_pos_angle();
  let h = helmet(headpos, headangle, humansize, false, skinclr);
  routes.extend(h);

  let shield_p = human.elbow_right;

  let s = shield(
    rng,
    mask,
    mainclr,
    shield_p,
    size * 0.6,
    0.0,
    //shape1,
    //shape2,
  );

  // FIXME
  // let is_colliding_shield = |point: (f64, f64)| s.includes_point(point);

  //foreground_routes =
  //  clip_routes_with_colors(&foreground_routes, &is_colliding_shield, 1.0, 5);

  foreground_routes.extend(s);

  /*
  for poly in s.polygons.iter() {
    foreground_mask.paint_polygon(
      &poly
        .iter()
        .map(|p| {
          let (x, y) = p;
          let x = x + mask_origin.0;
          let y = y + mask_origin.1;
          (x, y)
        })
        .collect::<Vec<_>>(),
    );

    let has_foreground = |p: (f64, f64)| {
      foreground_mask.is_painted((p.0 + mask_origin.0, p.1 + mask_origin.1))
    };

    routes = clip_routes_with_colors(&routes, &has_foreground, 1.0, 5);
  }
  */

  routes.extend(foreground_routes.clone());

  // translate routes
  routes = routes
    .iter()
    .map(|(clr, route)| {
      let proj =
        route_scale_translate_rotate(route, (xdir, 1.0), origin, angle);
      (*clr, proj)
    })
    .collect();

  let out = regular_clip(&routes, mask);

  for (_clr, rt) in routes {
    mask.paint_polyline(&rt, 1.0);
  }

  out
}
