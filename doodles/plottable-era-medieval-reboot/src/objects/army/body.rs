use super::human::HoldableObject;
use crate::algo::{
  clipping::regular_clip, math1d::mix, paintmask::PaintMask,
  polylines::grow_as_rectangle,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
#[derive(Clone, Copy)]
pub struct HumanPosture {
  pub body_angle: f32,
  pub head_angle: f32,
  // shoulders (left, right)
  pub shoulder_right_angle: f32,
  pub shoulder_left_angle: f32,
  // elbows (left, right)
  pub elbow_right_angle: f32,
  pub elbow_left_angle: f32,
  // hips
  pub hip_right_angle: f32,
  pub hip_left_angle: f32,
  // knees (left, right)
  pub knee_right_angle: f32,
  pub knee_left_angle: f32,

  pub left_arm_bend: f32,
  pub left_leg_bend: f32,
  pub right_arm_bend: f32,
  pub right_leg_bend: f32,

  pub origin_on_feet: bool,
}

impl HumanPosture {
  pub fn get_rotation(&self) -> f32 {
    0.0
  }
  pub fn climbing<R: Rng>(rng: &mut R, angle: f32, la: f32) -> Self {
    let la1 = 0.3;
    let la2 = -0.3 + la;
    let shoulder_right_angle = angle + la1;
    let elbow_right_angle = shoulder_right_angle - la2
      + rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0);
    let shoulder_left_angle = angle - la1;
    let elbow_left_angle = shoulder_left_angle
      + la2
      + rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0);

    let left_arm_bend = rng.gen_range(0.0..1.0);
    let right_arm_bend = rng.gen_range(0.7..1.0);

    Self {
      body_angle: angle,
      head_angle: angle,
      shoulder_right_angle,
      shoulder_left_angle,
      elbow_right_angle,
      elbow_left_angle,
      hip_right_angle: PI + angle - la1,
      hip_left_angle: PI + angle + la1,
      knee_right_angle: PI + angle,
      knee_left_angle: PI + angle,
      left_arm_bend,
      right_arm_bend,
      left_leg_bend: rng.gen_range(0.7..1.0),
      right_leg_bend: rng.gen_range(0.7..1.0),
      origin_on_feet: false,
    }
  }
  pub fn from_holding<R: Rng>(
    rng: &mut R,
    xflip: bool,
    lefthand: Option<HoldableObject>,
    righthand: Option<HoldableObject>,
  ) -> Self {
    let xdir = if xflip { -1.0 } else { 1.0 };
    let shoulder_right_angle = -PI / 2.0
      + match righthand {
        Some(HoldableObject::LongBow(phase)) => {
          mix(PI / 4.0, PI / 2.0, phase) * xdir
        }
        Some(HoldableObject::LongSword) | Some(HoldableObject::Sword) => {
          xdir * 4.0
        }
        Some(HoldableObject::RaisingUnknown) => xdir * (PI / 2.0) * 0.3,
        None => xdir * PI * 0.8,
        _ => {
          xdir
            * (PI / 2.0)
            * (1.0 + rng.gen_range(-1.0..1.0) * rng.gen_range(0.5..1.0))
        }
      };
    let elbow_right_angle = match righthand {
      Some(HoldableObject::Paddle(a)) => a,
      _ => {
        shoulder_right_angle
          + rng.gen_range(-0.5..0.5) * rng.gen_range(0.5..1.0)
      }
    };

    let shoulder_left_angle = -PI / 2.0
      + match lefthand {
        None => -xdir * PI * 0.8,
        _ => {
          -xdir
            * (PI / 2.0)
            * (1.0 + rng.gen_range(-1.0..1.0) * rng.gen_range(0.5..1.0))
        }
      };

    let elbow_left_angle =
      shoulder_left_angle + rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0);

    let left_arm_bend = match lefthand {
      Some(HoldableObject::Shield) => 0.2,
      _ => 1.0,
    };

    let right_arm_bend = match righthand {
      Some(HoldableObject::Shield) => 0.2,
      Some(HoldableObject::Axe) => 1.0,
      Some(HoldableObject::Club) => rng.gen_range(0.5..1.0),
      Some(HoldableObject::LongBow(_)) => 1.0,
      Some(HoldableObject::RaisingUnknown) => 1.0,
      None => 0.8,
      _ => 0.4,
    };

    Self {
      body_angle: -PI / 2.0,
      head_angle: -PI / 2.0,
      shoulder_right_angle,
      shoulder_left_angle,
      elbow_right_angle,
      elbow_left_angle,
      hip_right_angle: PI / 2.0 - 0.5,
      hip_left_angle: PI / 2.0 + 0.5,
      knee_right_angle: PI / 2.0,
      knee_left_angle: PI / 2.0,
      left_arm_bend,
      right_arm_bend,
      left_leg_bend: 1.0,
      right_leg_bend: 1.0,
      origin_on_feet: true,
    }
  }

  pub fn hand_risen<R: Rng>(rng: &mut R) -> Self {
    let shoulder_right_angle = -PI / 2.0 + 1.0;
    let elbow_right_angle =
      shoulder_right_angle + rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0);

    let shoulder_left_angle = -PI / 2.0 - 1.0;
    let elbow_left_angle =
      shoulder_left_angle + rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0);

    let left_arm_bend = 1.0;
    let right_arm_bend = 1.0;

    Self {
      body_angle: -PI / 2.0,
      head_angle: -PI / 2.0,
      shoulder_right_angle,
      shoulder_left_angle,
      elbow_right_angle,
      elbow_left_angle,
      hip_right_angle: PI / 2.0 - 0.5,
      hip_left_angle: PI / 2.0 + 0.5,
      knee_right_angle: PI / 2.0,
      knee_left_angle: PI / 2.0,
      left_arm_bend,
      right_arm_bend,
      left_leg_bend: 1.0,
      right_leg_bend: 1.0,
      origin_on_feet: true,
    }
  }

  pub fn sit<R: Rng>(rng: &mut R, ang: f32) -> Self {
    let shoulder_right_angle = -PI / 2.0 + rng.gen_range(0.0..2.0) + ang;
    let elbow_right_angle =
      shoulder_right_angle + rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0);

    let shoulder_left_angle = -PI / 2.0 - rng.gen_range(0.0..2.0) + ang;
    let elbow_left_angle =
      shoulder_left_angle + rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0);

    let left_arm_bend = rng.gen_range(0.8..1.0);
    let right_arm_bend = rng.gen_range(0.8..1.0);

    Self {
      body_angle: -PI / 2.0 + ang,
      head_angle: -PI / 2.0 + ang,
      shoulder_right_angle,
      shoulder_left_angle,
      elbow_right_angle,
      elbow_left_angle,
      hip_right_angle: PI / 2.0 - rng.gen_range(1.8..2.8) + ang,
      hip_left_angle: PI / 2.0 + rng.gen_range(1.8..2.8) + ang,
      knee_right_angle: PI / 2.0 + ang,
      knee_left_angle: PI / 2.0 + ang,
      left_arm_bend,
      right_arm_bend,
      left_leg_bend: 1.0,
      right_leg_bend: 1.0,
      origin_on_feet: true,
    }
  }
}

#[derive(Clone, Copy)]
pub struct HumanBody {
  pub posture: HumanPosture,
  pub origin: (f32, f32),
  pub height: f32,
  pub hip: (f32, f32),
  pub shoulder: (f32, f32),
  pub shoulder_right: (f32, f32),
  pub shoulder_left: (f32, f32),
  pub elbow_right: (f32, f32),
  pub elbow_left: (f32, f32),
  pub hip_right: (f32, f32),
  pub hip_left: (f32, f32),
  pub knee_right: (f32, f32),
  pub knee_left: (f32, f32),
  pub foot_right: (f32, f32),
  pub foot_left: (f32, f32),
  pub head: (f32, f32),
}

impl HumanBody {
  pub fn head_pos_angle(&self) -> ((f32, f32), f32) {
    (self.head, self.posture.head_angle)
  }
  pub fn hand_left_pos_angle(&self) -> ((f32, f32), f32) {
    (self.elbow_left, self.posture.elbow_left_angle)
  }
  pub fn hand_right_pos_angle(&self) -> ((f32, f32), f32) {
    (self.elbow_right, self.posture.elbow_right_angle)
  }
  /*
  pub fn foot_left_pos_angle(&self) -> ((f32, f32), f32) {
    (self.knee_left, self.joints.knee_left_angle)
  }
  pub fn foot_right_pos_angle(&self) -> ((f32, f32), f32) {
    (self.knee_right, self.joints.knee_right_angle)
  }
  pub fn get_size(&self) -> f32 {
    self.height
  }
  */

  pub fn apply_translation_rotation(&mut self, v: (f32, f32), _rot: f32) {
    self.origin = (self.origin.0 + v.0, self.origin.1 + v.1);
  }

  pub fn new(origin: (f32, f32), height: f32, joints: HumanPosture) -> Self {
    let h = height;
    let j = joints;
    let mut hip = origin;

    if joints.origin_on_feet {
      hip.1 -= 0.5 * h;
    }

    let shoulder = proj_point(hip, j.body_angle, 0.4 * h);

    let shoulder_right =
      proj_point(shoulder, j.shoulder_right_angle, j.right_arm_bend * 0.3 * h);
    let shoulder_left =
      proj_point(shoulder, j.shoulder_left_angle, j.left_arm_bend * 0.3 * h);

    let elbow_right = proj_point(
      shoulder_right,
      j.elbow_right_angle,
      j.right_arm_bend * 0.3 * h,
    );
    let elbow_left =
      proj_point(shoulder_left, j.elbow_left_angle, j.left_arm_bend * 0.3 * h);

    let hip_right =
      proj_point(hip, j.hip_right_angle, j.right_leg_bend * 0.3 * h);
    let hip_left = proj_point(hip, j.hip_left_angle, j.left_leg_bend * 0.3 * h);

    let knee_right =
      proj_point(hip_right, j.knee_right_angle, j.right_leg_bend * 0.3 * h);
    let knee_left =
      proj_point(hip_left, j.knee_left_angle, j.left_leg_bend * 0.3 * h);

    let head = proj_point(shoulder, j.head_angle, 0.3 * h);

    let footd = 0.08 * h;
    let foot_left =
      proj_point(knee_left, joints.knee_left_angle + PI / 2.0, footd);
    let foot_right =
      proj_point(knee_right, joints.knee_right_angle - PI / 2.0, footd);

    Self {
      origin,
      posture: joints,
      height,
      hip,
      shoulder,
      shoulder_right,
      shoulder_left,
      elbow_right,
      elbow_left,
      hip_right,
      hip_left,
      knee_right,
      knee_left,
      foot_right,
      foot_left,
      head,
    }
  }

  pub fn render(
    &self,
    paint: &mut PaintMask,
    clr: usize,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = Vec::new();
    let hip = self.hip;
    let shoulder = self.shoulder;
    let shoulder_right = self.shoulder_right;
    let shoulder_left = self.shoulder_left;
    let elbow_right = self.elbow_right;
    let elbow_left = self.elbow_left;
    let hip_right = self.hip_right;
    let hip_left = self.hip_left;
    let knee_right = self.knee_right;
    let knee_left = self.knee_left;
    let foot_right = self.foot_right;
    let foot_left = self.foot_left;
    let head = self.head;

    routes.push((clr, vec![hip, shoulder, head]));

    let w = 0.06 * self.height;
    if w > 0.5 {
      routes.push((clr, grow_as_rectangle(hip, shoulder, w)));
    }

    routes.push((clr, vec![shoulder, shoulder_right, elbow_right]));
    routes.push((clr, vec![shoulder, shoulder_left, elbow_left]));

    routes.push((clr, vec![hip, hip_right, knee_right, foot_right]));
    routes.push((clr, vec![hip, hip_left, knee_left, foot_left]));

    regular_clip(&routes, paint)
  }
}

fn proj_point(origin: (f32, f32), angle: f32, distance: f32) -> (f32, f32) {
  let (x, y) = origin;
  let s = angle.sin();
  let c = angle.cos();
  (x + distance * c, y + distance * s)
}
