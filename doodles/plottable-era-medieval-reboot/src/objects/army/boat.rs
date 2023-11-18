use super::{body::*, helmet::Helmet, shield::Shield};
use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    polylines::{
      path_subdivide_to_curve, route_scale_translate_rotate, Polylines,
    },
  },
  objects::blazon::traits::Blazon,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

// TODO boat need to have people with spears / swords / archers only
// TODO also flags

pub struct Boat {
  x1: f32,
  x2: f32,
  origin: (f32, f32),
  size: f32,
  angle: f32,
  w: f32,
  xflip: bool,
  blazon: Blazon,
}

impl Boat {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    w: f32,
    xflip: bool,
    blazon: Blazon,
  ) -> Self {
    let x1 = -w * rng.gen_range(0.3..0.45);
    let x2 = w * rng.gen_range(0.3..0.4);

    Self {
      x1,
      x2,
      origin,
      size,
      angle,
      w,
      xflip,
      blazon,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    mask: &mut PaintMask,
    clr: usize,
  ) -> Polylines {
    let x1 = self.x1;
    let x2 = self.x2;

    let size = self.size;
    let origin = self.origin;
    let angle = self.angle;
    let w = self.w;
    let xflip = self.xflip;
    let blazon = self.blazon;

    let mut routes = vec![];

    let xdir = if xflip { -1.0 } else { 1.0 };

    let h = size;
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

    routes = routes
      .iter()
      .map(|(clr, route)| {
        (
          *clr,
          route_scale_translate_rotate(route, (xdir, 1.0), origin, angle),
        )
      })
      .collect();

    routes = regular_clip(&routes, mask);

    // FIXME probably better than this clipping. but good for now
    for (_clr, route) in &routes {
      mask.paint_polyline(route, 0.1 * size);
    }

    routes
  }
}

pub fn boat_with_army<R: Rng>(
  rng: &mut R,
  mask: &mut PaintMask,
  clr: usize,
  origin: (f32, f32),
  angle: f32,
  size: f32, // reference size (height of the boat)
  w: f32,
  xflip: bool,
  blazon: Blazon,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = vec![];

  let xdir = if xflip { -1.0 } else { 1.0 };

  let boat = Boat::init(rng, origin, size, angle, w, xflip, blazon);

  let x1 = boat.x1;
  let x2 = boat.x2;

  let humansize = size * 0.5;

  let mut humans = vec![];
  let mut helmets = vec![];
  let mut shields = vec![];
  let mut sticks = vec![];

  let acos = angle.cos();
  let asin = angle.sin();

  let mut x = x1;
  while x < x2 {
    let joints = HumanJointAngles {
      body_angle: -PI / 2.0,
      head_angle: -PI / 2.0,
      shoulder_right_angle: rng.gen_range(0.0..PI / 4.0),
      shoulder_left_angle: rng.gen_range(3.0 * PI / 4.0..PI),
      elbow_right_angle: if xflip { 0.3 - PI } else { 0.3 },
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
    let y = rng.gen_range(-0.1 * size..0.0);
    let p = (x, y);
    let p = (p.0 * acos + p.1 * asin, p.1 * acos - p.0 * asin);
    let p = (p.0 * xdir + origin.0, p.1 + origin.1);
    let human = HumanBody::new(p, humansize, joints);
    humans.push(human);

    let a = if xflip {
      -PI * rng.gen_range(0.6..0.7)
    } else {
      -PI * rng.gen_range(0.3..0.4)
    };
    let stick = Stick::init(p, size, a);
    sticks.push(stick);

    let (headpos, headangle) = human.head_pos_angle();

    let helmet = Helmet::init(headpos, headangle, humansize, xflip);
    helmets.push(helmet);

    let shield_p = human.elbow_right;

    let s = Shield::init(rng, clr, shield_p, size * 0.6, angle, xflip, blazon);

    shields.push(s);

    x += rng.gen_range(0.15..0.25) * size;
  }

  routes = regular_clip(&routes, mask);

  for shield in shields {
    routes.extend(shield.render(mask));
  }

  for stick in sticks {
    routes.extend(stick.render(rng, mask, clr));
  }

  routes.extend(boat.render(rng, mask, clr));

  for helmet in helmets {
    routes.extend(helmet.render(mask, clr));
  }

  for human in humans {
    let human_body = human.render(mask, clr);
    routes.extend(human_body);
  }

  // we also create a halo cropping around castle
  for (_, route) in &routes {
    mask.paint_polyline(route, 1.0);
  }

  routes
}

pub struct Stick {
  origin: (f32, f32),
  size: f32,
  angle: f32,
}

impl Stick {
  pub fn init(origin: (f32, f32), size: f32, angle: f32) -> Self {
    Self {
      origin,
      size,
      angle,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    clr: usize,
  ) -> Polylines {
    let (x, y) = self.origin;
    let size = self.size;
    let angle = self.angle;
    let mut routes = vec![];

    let amp1 = -0.4 * size;
    let amp2 = rng.gen_range(0.4..0.8) * size;
    let stick = vec![
      (x + amp1 * angle.cos(), y + amp1 * angle.sin()),
      (x + amp2 * angle.cos(), y + amp2 * angle.sin()),
    ];
    routes.push((clr, stick));

    routes = regular_clip(&routes, paint);

    routes
  }
}
