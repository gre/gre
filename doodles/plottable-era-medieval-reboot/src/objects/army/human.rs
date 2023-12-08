use super::{
  axe::Axe,
  body::{HumanBody, HumanPosture},
  club::Club,
  flag::Flag,
  head::{head_cyclope, head_square},
  helmet::Helmet,
  longbow::long_bow,
  paddle::Paddle,
  shield::Shield,
  sword::Sword,
};
use crate::{
  algo::{
    clipping::regular_clip, math2d::p_r, paintmask::PaintMask,
    polylines::Polylines, renderable::Renderable,
    wormsfilling::worms_fill_strokes,
  },
  global::GlobalCtx,
  objects::blazon::Blazon,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Human {
  pub body: HumanBody,
  pub shields: Vec<Shield>,
  pub axes: Vec<Axe>,
  pub swords: Vec<Sword>,
  pub paddles: Vec<Paddle>,
  pub flags: Vec<Flag>,
  pub clubs: Vec<Club>,
  pub helmet: Option<Helmet>,
  pub mainclr: usize,
  pub blazonclr: usize,
  pub wormsfillingrendering: Option<f32>,
  pub weapon_routes: Vec<(usize, Vec<(f32, f32)>)>,
  pub head_routes: Vec<(usize, Vec<(f32, f32)>)>,
  pub head_polys: Vec<Vec<(f32, f32)>>,
  pub size: f32,
  pub xflip: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum HoldableObject {
  RaisingUnknown,
  Shield,
  Axe,
  Sword,
  Club,
  LongSword,
  Flag,
  LongBow(/*phase: */ f32),
  Paddle(/* ang */ f32),
  // TODO Lance,
}

#[derive(Clone, Copy, PartialEq)]
pub enum HeadShape {
  NAKED,
  HELMET,
  CYCLOPE,
}

impl Human {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    xflip: bool,
    blazon: Blazon,
    mainclr: usize,
    blazonclr: usize,
    posture: HumanPosture,
    headshape: HeadShape,
    lefthand: Option<HoldableObject>,
    righthand: Option<HoldableObject>,
  ) -> Self {
    let angle = posture.get_rotation();
    let acos = angle.cos();
    let asin = angle.sin();
    let xdir = if xflip { -1.0 } else { 1.0 };

    let humansize = size * 0.5;
    let y = rng.gen_range(-0.1 * size..0.0);
    let p = (0.0, y);
    let p = (p.0 * acos + p.1 * asin, p.1 * acos - p.0 * asin);
    let p = (p.0 * xdir + origin.0, p.1 + origin.1);
    let human = HumanBody::new(p, humansize, posture);

    let mut shields = vec![];
    let mut axes = vec![];
    let mut swords = vec![];
    let mut flags = vec![];
    let mut clubs = vec![];
    let mut paddles = vec![];
    let mut head_routes = vec![];
    let mut head_polys = vec![];
    let mut weapon_routes = vec![];

    for (side, (pos, handangle)) in vec![
      (lefthand, human.hand_left_pos_angle()),
      (righthand, human.hand_right_pos_angle()),
    ] {
      if let Some(obj) = side {
        match obj {
          HoldableObject::Shield => {
            let shield = Shield::init(
              rng,
              mainclr,
              blazonclr,
              pos,
              size * 0.6,
              angle,
              xflip,
              blazon,
            );
            shields.push(shield);
          }
          HoldableObject::Axe => {
            let axeang = handangle - PI / 2.0; // - xdir * rng.gen_range(0.0..1.0);
            let s = 0.4 * size;
            let a = axeang - PI / 2.0;
            let handle = 0.3 * xdir;
            let dx = a.cos() * s * handle;
            let dy = a.sin() * s * handle;
            let o = (pos.0 + dx, pos.1 + dy);
            let axe = Axe::init(rng, mainclr, o, s, axeang, (false, xflip));
            axes.push(axe);
          }
          HoldableObject::Sword => {
            let xdir = if xflip { -1.0 } else { 1.0 };
            let swordang = PI / 2.0 - xdir * rng.gen_range(0.0..2.0);
            let sword = Sword::init(rng, pos, 0.5 * size, swordang, mainclr);
            swords.push(sword);
          }
          HoldableObject::Club => {
            let xdir = if xflip { -1.0 } else { 1.0 };
            let ang = PI / 2.0 - xdir * rng.gen_range(0.0..2.0);
            let club = Club::init(rng, pos, 0.5 * size, ang, mainclr);
            clubs.push(club);
          }
          HoldableObject::LongSword => {
            let xdir = if xflip { -1.0 } else { 1.0 };
            let swordang = PI / 2.0 - xdir * rng.gen_range(0.0..2.0);
            let sword = Sword::init(rng, pos, size, swordang, mainclr);
            swords.push(sword);
          }
          HoldableObject::Flag => {
            let o = (pos.0, pos.1 - 0.4 * size);
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
              true,
            );
            flags.push(flag);
          }
          HoldableObject::LongBow(phase) => {
            let bow = long_bow(mainclr, pos, size * 0.5, -handangle, phase);
            weapon_routes.extend(bow);
          }
          HoldableObject::Paddle(a) => {
            let paddle = Paddle::init(rng, mainclr, pos, size, a);
            paddles.push(paddle);
          }
          HoldableObject::RaisingUnknown => {}
        }
      }
    }

    let (headpos, headangle) = human.head_pos_angle();
    let mut helmet = None;
    match headshape {
      HeadShape::HELMET => {
        helmet = Some(Helmet::init(headpos, headangle, humansize, xflip));
      }
      HeadShape::NAKED => {
        let h = head_square(mainclr, headpos, headangle, humansize);
        head_routes.extend(h.clone());
        for (_, r) in h {
          head_polys.push(r.clone());
        }
      }
      HeadShape::CYCLOPE => {
        let h = head_cyclope(mainclr, headpos, headangle, humansize, xflip);
        head_routes.extend(h.clone());
        for (_, r) in h {
          head_polys.push(r.clone());
        }
      }
    }

    Self {
      body: human,
      shields,
      axes,
      swords,
      paddles,
      clubs,
      flags,
      helmet,
      mainclr,
      blazonclr,
      head_routes,
      head_polys,
      weapon_routes,
      wormsfillingrendering: None,
      size,
      xflip,
    }
  }

  pub fn with_worms_filling_defaults(mut self) -> Self {
    self.wormsfillingrendering = Some(0.022 * self.size);
    self
  }

  pub fn eye_pos(&self) -> (f32, f32) {
    let ((x, y), a) = self.body.head_pos_angle();
    let size = self.body.height;
    let xmul = if self.xflip { -1.0 } else { 1.0 };
    let (dx, dy) = p_r((-0.25 * size, -0.1 * size * xmul), a);
    (x + dx, y + dy)
  }

  fn rendering_pass<R: Rng>(
    &self,
    rng: &mut R,
    paint: &PaintMask,
    routes: &Polylines,
  ) -> Polylines {
    if let Some(w) = self.wormsfillingrendering {
      let density = 3.5;
      let its = (self.body.height * 10.0 + 100.0) as usize;
      worms_fill_strokes(rng, paint, its, w, density, routes)
    } else {
      routes.clone()
    }
  }

  pub fn render_foreground_only<R: Rng>(
    &self,
    rng: &mut R,
    mask: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    for shield in self.shields.iter() {
      routes.extend(shield.render(mask));
    }
    for paddle in self.paddles.iter() {
      routes.extend(paddle.render(mask));
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
    // TODO aren't we supposed to clip this? also to paint the mask actually?!
    self.rendering_pass(rng, mask, &routes)
  }

  pub fn render_background_only<R: Rng>(
    &self,
    rng: &mut R,
    mask: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let human = &self.body;
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

    for axe in self.axes.iter() {
      routes.extend(axe.render(mask));
    }

    routes.extend(human.render(mask, mainclr));
    self.rendering_pass(rng, mask, &routes)
  }

  /*
  pub fn get_renderables_parts<R: Rng>(&self) -> Vec<Box<dyn Renderable<R>>> {
    vec![
      Box::new(HumanForeground(Box::new(self))),
      Box::new(HumanBackground(Box::new(self))),
    ]
  }
  */

  // a standalone rendering version when it's not rendered riding something.
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    mask: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    routes.extend(self.render_foreground_only(rng, mask));
    routes.extend(self.render_background_only(rng, mask));
    let strokew = (self.body.height * 0.2).max(3.0 * mask.precision).min(1.6);
    for (_, route) in routes.iter() {
      mask.paint_polyline(route, strokew);
    }
    routes
  }
}

// TODO we need to expand human into a container that can yield the 2 parts
// but we need to figure out the polyline halo effect...
impl<R: Rng> Renderable<R> for Human {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(rng, paint)
  }

  fn zorder(&self) -> f32 {
    self.body.origin.1
  }

  fn as_human(&self) -> Option<&Human> {
    Some(self)
  }
}

/*
pub struct HumanForeground(Box<Human>);

impl<R: Rng> Renderable<R> for HumanForeground {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.0.render_foreground_only(rng, paint)
  }

  fn zorder(&self) -> f32 {
    self.0.body.origin.1 + 0.2 * self.0.size
  }
}

pub struct HumanBackground(Box<Human>);

impl<R: Rng> Renderable<R> for HumanBackground {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.0.render_background_only(rng, paint)
  }

  fn zorder(&self) -> f32 {
    self.0.body.origin.1 + 0.2 * self.0.size
  }
}
*/
