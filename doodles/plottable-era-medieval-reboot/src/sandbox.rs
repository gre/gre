use crate::{
  algo::{paintmask::PaintMask, polylines::Polylines, renderable::Container},
  global::GlobalCtx,
  objects::{
    army::{
      boat::BoatGlobals,
      boatarmy::BoatArmy,
      body::HumanPosture,
      cannon::Cannon,
      catapult::Catapult,
      human::{HeadShape, HoldableObject, Human},
      trebuchet::Trebuchet,
    },
    sky::dragons::dragons,
  },
};
use noise::*;
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn sandbox<R: Rng>(
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  routes: &mut Polylines,
  width: f32,
  height: f32,
) {
  match rng.gen_range(0..5) {
    0 => sandbox_trebuchet(rng, ctx, paint, routes, width, height),
    1 => sandbox_dragons(rng, ctx, paint, routes, width, height),
    2 => sandbox_men(rng, ctx, paint, routes, width, height),
    3 => sandbox_boat(rng, ctx, paint, routes, width, height),
    4 => sandbox_cannon_catapult(rng, ctx, paint, routes, width, height),
    _ => {}
  }
}

fn sandbox_dragons<R: Rng>(
  rng: &mut R,
  _ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  routes: &mut Polylines,
  width: f32,
  height: f32,
) {
  let n = rng.gen_range(10..40);
  routes.extend(dragons(rng, paint, width, height, 0.0, 0.0, n));
}

fn sandbox_men<R: Rng>(
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  routes: &mut Polylines,
  width: f32,
  height: f32,
) {
  let mut container = Container::new();
  let s = rng.gen_range(8.0..20.0);
  let clrmax = rng.gen_range(1..4);
  let delta = rng.gen_range(1..3);
  let obj = if rng.gen_bool(0.5) {
    Some(HoldableObject::Flag)
  } else {
    Some(HoldableObject::Torch)
  };
  for _ in 0..rng.gen_range(100..500) {
    let o = (
      rng.gen_range(0.1..0.9) * width,
      rng.gen_range(0.1..0.95) * height,
    );
    let xflip = rng.gen_bool(0.5);
    let lefthand = obj;
    let righthand: Option<HoldableObject> = None;
    let head = HeadShape::NAKED;
    let posture = HumanPosture::from_holding(rng, xflip, lefthand, righthand);
    let s = s * rng.gen_range(1.0..2.0);
    let clr = rng.gen_range(0..clrmax);
    let bclr = (clr + delta) % 3;
    let human = Human::init(
      rng,
      o,
      s,
      xflip,
      ctx.attackers,
      clr,
      bclr,
      posture,
      head,
      lefthand,
      righthand,
    )
    .with_worms_filling_defaults();
    container.add(human);
  }

  let rts = container.render_with_extra_halo(rng, ctx, paint, 2.0);
  routes.extend(rts.clone());
}

fn sandbox_boat<R: Rng>(
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  routes: &mut Polylines,
  width: f32,
  height: f32,
) {
  let boatconf = BoatGlobals::rand(rng, false);
  let mut container = Container::new();
  let general_width = 2. * rng.gen_range(0.08..0.15) * width;
  for _ in 0..rng.gen_range(200..1000) {
    let o = (
      rng.gen_range(0.1..0.9) * width,
      rng.gen_range(0.1..0.9) * height * rng.gen_range(0.5..1.0),
    );
    let w = general_width * rng.gen_range(0.8..1.2);
    let size = rng.gen_range(0.3..0.4) * w;
    let angle = rng.gen_range(-0.5..0.5)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0);
    let xflip = rng.gen_bool(0.5);
    let blazon = ctx.attackers;

    let mainclr = rng.gen_range(0..3);
    let blazonclr = rng.gen_range(0..3);

    let human_density = rng.gen_range(0.0..1.0);
    let boat = BoatArmy::init(
      rng,
      ctx,
      mainclr,
      blazonclr,
      o,
      size,
      angle,
      w,
      xflip,
      blazon,
      human_density,
      &|rng, arg| {
        let headshape = HeadShape::HELMET;
        let lefthandobj = Some(HoldableObject::Shield);
        let a = if xflip {
          -PI * rng.gen_range(0.6..0.7)
        } else {
          -PI * rng.gen_range(0.3..0.4)
        };
        let righthandobj = Some(HoldableObject::Paddle(a));

        let posture =
          HumanPosture::from_holding(rng, false, lefthandobj, righthandobj);

        let human = Human::init(
          rng,
          arg.origin,
          arg.size,
          arg.xflip,
          blazon,
          mainclr,
          blazonclr,
          posture,
          headshape,
          lefthandobj,
          righthandobj,
        );
        Some(human)
      },
      &boatconf,
    );
    container.add(boat);
  }

  let rts = container.render_with_extra_halo(rng, ctx, paint, 2.0);
  routes.extend(rts.clone());
}

fn sandbox_trebuchet<R: Rng>(
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  routes: &mut Polylines,
  width: f32,
  height: f32,
) {
  let mut container = Container::new();
  let general_s = rng.gen_range(0.08..0.12) * width;
  let xflip = rng.gen_bool(0.5);
  for _ in 0..rng.gen_range(50..200) {
    let o = (
      rng.gen_range(0.15..0.85) * width,
      rng.gen_range(0.2..0.9) * height,
    );
    let size = rng.gen_range(1.0..1.5) * general_s;
    let clr = rng.gen_range(0..3);
    let percent = o.1 / height;
    let trebuchet = Trebuchet::init(rng, o, size, percent, xflip, clr);
    container.add(trebuchet);
  }

  let rts = container.render_with_extra_halo(rng, ctx, paint, 2.0);
  routes.extend(rts.clone());
}

pub fn sandbox_cannon_catapult<R: Rng>(
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  routes: &mut Polylines,
  width: f32,
  height: f32,
) {
  let mut container = Container::new();
  let perlin = Perlin::new(rng.gen());
  let general_s =
    (0.01 + rng.gen_range(0.00..0.01) * rng.gen_range(0.0..1.0)) * width;
  let f = rng.gen_range(1.0..4.0);
  let amp = rng.gen_range(0.0..1.0);
  let f2 = rng.gen_range(0.0..20.0);
  let f3 = rng.gen_range(0.0..20.0);
  let threshold = rng.gen_range(-0.5..0.5);
  for _ in 0..rng.gen_range(100..500) {
    let o = (
      rng.gen_range(0.1..0.9) * width,
      rng.gen_range(0.1..0.9) * height,
    );
    let xflip = perlin.get([
      2.0 * o.0 as f64 / width as f64,
      2.0 * o.1 as f64 / width as f64,
    ]) > 0.;
    let size = rng.gen_range(1.0..2.0) * general_s;
    let clr = ((0.5
      + 0.5
        * perlin.get([
          f2 * o.0 as f64 / width as f64,
          f2 * o.1 as f64 / width as f64,
          2.4213,
        ]))
      * 3.0) as usize;
    let angle = amp
      * perlin.get([
        5. / 7.,
        f * o.0 as f64 / width as f64,
        f * o.1 as f64 / width as f64,
      ]) as f32;

    if perlin.get([
      f3 * o.0 as f64 / width as f64,
      1. / 0.176,
      f3 * o.1 as f64 / width as f64,
    ]) > threshold
    {
      let obj = Cannon::init(rng, clr, o, size, angle, xflip);
      container.add(obj);
    } else {
      let progress = 0.5
        + 0.5
          * perlin.get([
            f * o.0 as f64 / width as f64,
            f * o.1 as f64 / width as f64,
            1. / 9.,
          ]) as f32;
      let obj = Catapult::init(rng, clr, o, 3. * size, angle, xflip, progress);
      container.add(obj);
    }
  }

  let rts = container.render_with_extra_halo(rng, ctx, paint, 1.0);
  routes.extend(rts.clone());
}
