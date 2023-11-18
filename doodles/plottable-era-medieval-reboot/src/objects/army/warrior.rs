use super::{
  body::{HumanBody, HumanJointAngles},
  helmet::Helmet,
  shield::Shield,
};
use crate::{algo::paintmask::PaintMask, objects::blazon::traits::Blazon};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Warrior {
  pub human: HumanBody,
  pub shield: Option<Shield>,
  pub helmet: Helmet,

  pub mainclr: usize,
  pub blazonclr: usize,
}

impl Warrior {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    blazon: Blazon,
    mainclr: usize,
    blazonclr: usize,
    uses_shield: bool,
  ) -> Self {
    let acos = angle.cos();
    let asin = angle.sin();
    let xdir = if xflip { -1.0 } else { 1.0 };

    let humansize = size * 0.5;
    // TODO more various and interesting foot positions
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
    let p = (0.0, y);
    let p = (p.0 * acos + p.1 * asin, p.1 * acos - p.0 * asin);
    let p = (p.0 * xdir + origin.0, p.1 + origin.1);
    let human = HumanBody::new(p, humansize, joints);

    let shield = if uses_shield {
      let shield_p = human.elbow_right;
      let shield =
        Shield::init(rng, mainclr, shield_p, size * 0.6, angle, xflip, blazon);
      Some(shield)
    } else {
      None
    };

    let (headpos, headangle) = human.head_pos_angle();
    let helmet = Helmet::init(headpos, headangle, humansize, xflip);

    Self {
      human,
      shield,
      helmet,
      mainclr,
      blazonclr,
    }
  }

  pub fn render_foreground_only(
    &self,
    mask: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let shield = &self.shield;
    let mut routes = vec![];
    if let Some(shield) = shield {
      routes.extend(shield.render(mask));
    }
    routes
  }

  pub fn render_background_only(
    &self,
    mask: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let human = &self.human;
    let helmet = &self.helmet;
    let mainclr = self.mainclr;
    let mut routes = vec![];
    routes.extend(helmet.render(mask, mainclr));
    routes.extend(human.render(mask, mainclr));
    routes
  }
}
