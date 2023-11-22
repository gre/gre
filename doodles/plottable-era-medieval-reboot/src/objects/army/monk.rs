use super::body::{HumanBody, HumanPosture};
use crate::algo::{
  clipping::regular_clip, math2d::lerp_point, paintmask::PaintMask,
  polylines::route_xreverse_translate_rotate,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Monk {
  pub human: HumanBody,
  pub mainclr: usize,
  pub xflip: bool,
}

impl Monk {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    mainclr: usize,
    carrying: bool,
  ) -> Self {
    let acos = angle.cos();
    let asin = angle.sin();
    let xdir = if xflip { -1.0 } else { 1.0 };

    let humansize = size * 0.5;
    // TODO carrying will be this. but we need non carrying positions.
    let joints = HumanPosture {
      body_angle: -PI / 2.0,
      head_angle: -PI / 2.0,
      shoulder_right_angle: rng.gen_range(0.0..PI / 4.0),
      shoulder_left_angle: rng.gen_range(3.0 * PI / 4.0..PI),
      elbow_right_angle: PI / 2.0 + rng.gen_range(-0.2..0.2),
      elbow_left_angle: PI / 2.0 + rng.gen_range(-0.2..0.2),
      hip_right_angle: PI / 2.0 - 0.5 + rng.gen_range(-0.2..0.2),
      hip_left_angle: PI / 2.0 + 0.5 + rng.gen_range(-0.2..0.2),
      knee_right_angle: PI / 2.0 + rng.gen_range(-0.2..0.2),
      knee_left_angle: PI / 2.0 + rng.gen_range(-0.2..0.2),
      left_arm_bend: 1.0,
      right_arm_bend: 1.0,
      left_leg_bend: 1.0,
      right_leg_bend: 1.0,
    };
    let y = rng.gen_range(-0.1 * size..0.0);
    let p = (0.0, y);
    let p = (p.0 * acos + p.1 * asin, p.1 * acos - p.0 * asin);
    let p = (p.0 * xdir + origin.0, p.1 + origin.1);
    let human = HumanBody::new(p, humansize, joints);

    Self {
      human,
      mainclr,
      xflip,
    }
  }

  pub fn render(&self, mask: &mut PaintMask) -> Vec<(usize, Vec<(f32, f32)>)> {
    let human = &self.human;
    let mainclr = self.mainclr;
    let humansize = human.height;
    let xflip = self.xflip;
    let mut routes = vec![];
    let (headpos, headangle) = human.head_pos_angle();
    let origin = lerp_point(human.hip, human.shoulder, 0.1);
    routes.extend(rect(
      mask,
      mainclr,
      origin,
      headangle,
      (0.25 * humansize, 0.7 * humansize),
      xflip,
    ));
    routes.extend(hat(mask, mainclr, headpos, headangle, humansize, xflip));
    routes.extend(human.render(mask, mainclr));
    routes
  }
}

fn hat(
  paint: &mut PaintMask,
  clr: usize,
  origin: (f32, f32),
  angle: f32,
  size: f32,
  xreverse: bool,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = -0.4 * size;
  let h2 = 0.2 * size;
  routes.push(vec![(-dx, h2), (0.0, h), (dx, h2), (-dx, h2)]);

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  let routes = regular_clip(
    &routes
      .iter()
      .map(|route| {
        (
          clr,
          route_xreverse_translate_rotate(&route, xreverse, origin, ang),
        )
      })
      .collect(),
    paint,
  );

  // consider routes to be polygon for now.
  for (_clr, route) in &routes {
    paint.paint_polygon(&route);
  }

  routes
}

fn rect(
  paint: &mut PaintMask,
  clr: usize,
  origin: (f32, f32),
  angle: f32,
  (sx, sy): (f32, f32),
  xreverse: bool,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = Vec::new();
  let dx = 0.5 * sx;
  let dy = 0.5 * sy;
  routes.push(vec![(-dx, -dy), (dx, -dy), (dx, dy), (-dx, dy), (-dx, -dy)]);

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  let routes = regular_clip(
    &routes
      .iter()
      .map(|route| {
        (
          clr,
          route_xreverse_translate_rotate(&route, xreverse, origin, ang),
        )
      })
      .collect(),
    paint,
  );

  // consider routes to be polygon for now.
  for (_clr, route) in &routes {
    paint.paint_polygon(&route);
  }

  routes
}
