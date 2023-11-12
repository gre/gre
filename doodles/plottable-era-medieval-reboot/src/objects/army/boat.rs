use super::{body::*, helmet::full_helmet, shield::shield};
use crate::algo::{
  clipping::{clip_routes_with_colors, regular_clip},
  paintmask::PaintMask,
  polylines::{path_subdivide_to_curve, route_scale_translate_rotate},
};
use rand::prelude::*;
use std::f64::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn boat_with_army<R: Rng>(
  rng: &mut R,
  mask: &mut PaintMask,
  clr: usize,
  origin: (f64, f64),
  angle: f64,
  size: f64, // reference size (height of the boat)
  w: f64,
  xflip: bool,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  // TODO rework to mask on the real mask

  let xdir = if xflip { -1.0 } else { 1.0 };

  let h = size;
  let x1 = -w * rng.gen_range(0.3..0.45);
  let x2 = w * rng.gen_range(0.3..0.4);
  let yleft = -h * rng.gen_range(0.6..1.0);
  let yright = -h * rng.gen_range(0.8..1.0);

  let dy_edge = 0.3;
  // boat bottom
  let mut route = Vec::new();
  route.push((-w / 2.0 - dy_edge, yleft + dy_edge));
  route.push((x1, 0.0));
  route.push((x2, 0.0));
  route.push((w / 2.0 + dy_edge, yright + dy_edge));
  route = path_subdivide_to_curve(route, 2, 0.8);
  routes.push((clr, route));

  // boat in between
  let mut route = Vec::new();
  let y = -0.15 * h;
  route.push((-w / 2.0, yleft));
  route.push((x1, y));
  route.push((x2, y));
  route.push((w / 2.0, yright));
  route = path_subdivide_to_curve(route, 2, 0.8);
  // TODO route will be used to clip people
  routes.push((clr, route));

  // boat top
  let mut route = Vec::new();
  let y = -0.3 * h;
  route.push((-w / 2.0 + dy_edge, yleft - dy_edge));
  route.push((x1, y));
  route.push((x2, y));
  route.push((w / 2.0 - dy_edge, yright - dy_edge));
  route = path_subdivide_to_curve(route, 2, 0.8);
  // TODO route will be used to clip people
  routes.push((clr, route.clone()));
  let boat_top = route;

  // make a boat head
  let o = (w / 2.0, yright);
  let mut route = vec![];
  for _i in 0..8 {
    let angle = rng.gen_range(-PI..PI);
    let amp = rng.gen_range(0.1..0.2) * size;
    route.push((o.0 + amp * angle.cos(), o.1 + amp * angle.sin()));
  }
  route.push(route[0]);
  routes.push((clr, route));

  // humans

  let mut foreground_routes = Vec::new();
  let mask_origin = (3.0 * w, 3.0 * h);
  let mut foreground_mask =
    PaintMask::new(0.5, 2.0 * mask_origin.0, 2.0 * mask_origin.1);

  // let shape1 = rng.gen_range(0.0..1.0);
  // let shape2 = rng.gen_range(0.0..1.0);
  let mut x = x1;
  while x < x2 {
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
      left_leg_bend: 1.0,
      right_leg_bend: 1.0,
    };
    let humansize = size * 0.5;
    let y = rng.gen_range(-0.1 * size..0.0);
    let human = HumanBody::new((x, y), humansize, joints);

    let human_body = human.render(&mut foreground_mask, clr);
    // clip human body with boat top
    let is_outside = |p| {
      let (x, y) = p;
      let mut inside = false;
      for i in 0..boat_top.len() - 1 {
        let (x1, y1) = boat_top[i];
        let (x2, y2) = boat_top[i + 1];
        if (y1 < y && y2 > y) || (y1 > y && y2 < y) {
          let x3 = x1 + (x2 - x1) * (y - y1) / (y2 - y1);
          if x3 < x {
            inside = !inside;
          }
        }
      }
      !inside
    };
    let human_body = clip_routes_with_colors(&human_body, &is_outside, 1.0, 6);

    routes.extend(human_body);

    // stick
    let angle = -PI * rng.gen_range(0.3..0.4);
    let amp1 = -0.4 * size;
    let amp2 = rng.gen_range(0.4..0.8) * size;
    let stick = vec![
      (x + amp1 * angle.cos(), y + amp1 * angle.sin()),
      (x + amp2 * angle.cos(), y + amp2 * angle.sin()),
    ];
    routes.push((clr, stick));

    let (headpos, headangle) = human.head_pos_angle();
    let h = full_helmet(
      &mut foreground_mask,
      clr,
      headpos,
      headangle,
      humansize,
      false,
    );
    routes.extend(h);

    let shield_p = human.elbow_right;

    let s = shield(
      rng,
      &mut foreground_mask,
      clr,
      shield_p,
      size * 0.6,
      0.0,
      // TODO implement shapes
      //shape1,
      //shape2,
    );

    // FIXME is this still needed?
    /*
        let is_colliding_shield = |point: (f64, f64)| s.includes_point(point);

        foreground_routes =
          clip_routes_with_colors(&foreground_routes, &is_colliding_shield, 1.0, 5);
          foreground_routes.extend(s.render());
    */
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
    }
    */

    let has_foreground = |p: (f64, f64)| {
      foreground_mask.is_painted((p.0 + mask_origin.0, p.1 + mask_origin.1))
    };

    routes = clip_routes_with_colors(&routes, &has_foreground, 1.0, 5);

    x += rng.gen_range(0.15..0.25) * size;
  }

  routes.extend(foreground_routes.clone());

  // translate routes
  routes = routes
    .iter()
    .map(|(clr, route)| {
      (
        *clr,
        route_scale_translate_rotate(route, (xdir, 1.0), origin, angle),
      )
    })
    .collect();

  regular_clip(&routes, mask)
}
