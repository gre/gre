use super::{
  axe::Axe,
  body::{HumanBody, HumanJointAngles},
  club::Club,
  flag::Flag,
  head::head_square,
  helmet::Helmet,
  longbow::long_bow,
  shield::Shield,
  sword::Sword,
};
use crate::{
  algo::{clipping::regular_clip, paintmask::PaintMask},
  objects::blazon::Blazon,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Warrior {
  pub human: HumanBody,
  pub shields: Vec<Shield>,
  pub axes: Vec<Axe>,
  pub swords: Vec<Sword>,
  pub flags: Vec<Flag>,
  pub clubs: Vec<Club>,
  pub helmet: Option<Helmet>,
  pub mainclr: usize,
  pub blazonclr: usize,

  pub weapon_routes: Vec<(usize, Vec<(f32, f32)>)>,

  pub head_routes: Vec<(usize, Vec<(f32, f32)>)>,
  pub head_polys: Vec<Vec<(f32, f32)>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WarriorObject {
  Shield,
  Axe,
  Sword,
  Club,
  LongSword,
  Flag,
  LongBow,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WarriorHead {
  NAKED,
  HELMET,
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
    headshape: WarriorHead,
    lefthandobj: Option<WarriorObject>,
    righthandobj: Option<WarriorObject>,
  ) -> Self {
    let acos = angle.cos();
    let asin = angle.sin();
    let xdir = if xflip { -1.0 } else { 1.0 };

    let humansize = size * 0.5;
    // TODO more various and interesting foot positions
    // TODO postures should be provided by HumanJointAngles and passed in param
    // when we have gesture, we will test bow and axe
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

    let mut shields = vec![];
    let mut axes = vec![];
    let mut swords = vec![];
    let mut flags = vec![];
    let mut clubs = vec![];
    let mut head_routes = vec![];
    let mut head_polys = vec![];
    let mut weapon_routes = vec![];

    for (side, o, (pos, handangle)) in vec![
      (lefthandobj, human.elbow_left, human.hand_left_pos_angle()),
      (
        righthandobj,
        human.elbow_right,
        human.hand_right_pos_angle(),
      ),
    ] {
      if let Some(obj) = side {
        match obj {
          WarriorObject::Shield => {
            let shield =
              Shield::init(rng, mainclr, o, size * 0.6, angle, xflip, blazon);
            shields.push(shield);
          }
          WarriorObject::Axe => {
            // TODO full broken at the moment xd
            let xdir = if xflip { -1.0 } else { 1.0 };
            let axeang = -PI / 2.0 - xdir * rng.gen_range(0.0..1.0);
            let s = 0.5 * size;
            let a = axeang - PI / 2.0;
            let dx = a.cos() * s * 0.5;
            let dy = a.sin() * s * 0.5;
            let o = (o.0 + dx, o.1 + dy);
            let axe = Axe::init(rng, mainclr, o, s, axeang);
            axes.push(axe);
          }
          WarriorObject::Sword => {
            let xdir = if xflip { -1.0 } else { 1.0 };
            let swordang = PI / 2.0 - xdir * rng.gen_range(0.0..2.0);
            let sword = Sword::init(rng, o, 0.5 * size, swordang, mainclr);
            swords.push(sword);
          }
          WarriorObject::Club => {
            let xdir = if xflip { -1.0 } else { 1.0 };
            let ang = PI / 2.0 - xdir * rng.gen_range(0.0..2.0);
            let club = Club::init(rng, o, 0.5 * size, ang, mainclr);
            clubs.push(club);
          }
          WarriorObject::LongSword => {
            let xdir = if xflip { -1.0 } else { 1.0 };
            let swordang = PI / 2.0 - xdir * rng.gen_range(0.0..2.0);
            let sword = Sword::init(rng, o, size, swordang, mainclr);
            swords.push(sword);
          }
          WarriorObject::Flag => {
            let o = (o.0, o.1 - 0.4 * size);
            let flag = Flag::init(
              rng,
              mainclr,
              blazonclr,
              o,
              size,
              -PI / 2.0,
              !xflip,
              0.5,
              1.0,
            );
            flags.push(flag);
          }
          WarriorObject::LongBow => {
            let phase = rng.gen_range(0.0..1.0);
            let bow = long_bow(mainclr, pos, size * 0.5, -handangle, phase);
            weapon_routes.extend(bow);
          }
        }
      }
    }

    let (headpos, headangle) = human.head_pos_angle();
    let mut helmet = None;
    match headshape {
      WarriorHead::HELMET => {
        helmet = Some(Helmet::init(headpos, headangle, humansize, xflip));
      }
      WarriorHead::NAKED => {
        let h = head_square(mainclr, headpos, headangle, humansize);
        head_routes.extend(h.clone());
        for (_, r) in h {
          head_polys.push(r.clone());
        }
      }
    }

    Self {
      human,
      shields,
      axes,
      swords,
      clubs,
      flags,
      helmet,
      mainclr,
      blazonclr,
      head_routes,
      head_polys,
      weapon_routes,
    }
  }

  pub fn render_foreground_only(
    &self,
    mask: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    for shield in self.shields.iter() {
      routes.extend(shield.render(mask));
    }
    for axe in self.axes.iter() {
      routes.extend(axe.render(mask));
    }
    for sword in self.swords.iter() {
      routes.extend(sword.render(mask));
    }
    for club in self.clubs.iter() {
      routes.extend(club.render(mask));
    }
    for flag in self.flags.iter() {
      routes.extend(flag.render(mask));
    }
    routes.extend(regular_clip(&self.weapon_routes, mask));
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
    if let Some(helmet) = helmet {
      routes.extend(helmet.render(mask, mainclr));
    }
    routes.extend(regular_clip(&self.head_routes, mask));
    for poly in &self.head_polys {
      mask.paint_polygon(&poly);
    }
    routes.extend(human.render(mask, mainclr));
    routes
  }

  // a standalone rendering version when it's not rendered riding something.
  pub fn render(&self, mask: &mut PaintMask) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    routes.extend(self.render_foreground_only(mask));
    routes.extend(self.render_background_only(mask));
    for (_, route) in routes.iter() {
      mask.paint_polyline(route, 1.0);
    }
    routes
  }
}
