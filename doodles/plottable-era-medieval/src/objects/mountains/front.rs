use crate::{
  algo::{
    clipping::clip_routes_with_colors, math1d::mix, math2d::lookup_ridge,
    paintmask::PaintMask,
  },
  global::{GlobalCtx, Special},
  objects::{
    army::{
      body::HumanPosture,
      human::{HeadShape, HoldableObject, Human},
      trebuchet::Trebuchet,
    },
    castle::chinesedoor::ChineseDoor,
  },
};
use noise::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct FrontMountains {
  pub clr: usize,
  pub ybase: f32,
  pub ystart: f32,
  pub width: f32,
}

impl FrontMountains {
  pub fn render<R: Rng>(
    &mut self,
    ctx: &mut GlobalCtx,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let clr = self.clr;
    let ybase = self.ybase;
    let ystart = self.ystart;
    let width = self.width;

    let mut paint_before = paint.clone();
    let mut routes = vec![];

    let perlin = Perlin::new(rng.gen());
    let count = rng.gen_range(2..12);
    let h = ybase - ystart;
    let xincr = 2.0;

    let mut curves = vec![];

    for i in 0..count {
      let y = ybase;
      let divmin = count as f32 * 0.3;
      let divmax = count as f32 * 0.6;
      let yamp = ((i as f32 + 1.0) * h / rng.gen_range(divmin..divmax)).min(h);

      let f1 = rng.gen_range(0.01..0.03) * rng.gen_range(0.0..1.0);
      let amp2 = rng.gen_range(0.0..2.0) * rng.gen_range(0.0..1.0);
      let f2 = rng.gen_range(0.0..0.05) * rng.gen_range(0.0..1.0);
      let amp3 = rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
      let f3 = rng.gen_range(0.0..0.1) * rng.gen_range(0.0..1.0);
      let seed1 = rng.gen_range(0.0..100.0);
      let seed2 = rng.gen_range(0.0..100.0);
      let seed3 = rng.gen_range(0.0..100.0);

      let valuef = |x32, y32| {
        let x = x32 as f64;
        let y = y32 as f64;
        let n = 0.5
          + 0.5
            * perlin.get([
              f1 * x,
              f1 * y,
              amp2
                * perlin.get([
                  f2 * x,
                  seed2 + amp3 * perlin.get([seed3, f3 * x, f3 * y]),
                  f2 * y,
                ])
                + seed1
                + i as f64 * 55.5,
            ]) as f32;
        n
      };

      let (rts, curve) =
        stroke_mountains(paint, 0.0, width, xincr, y, yamp, &valuef, clr);

      routes.extend(rts);
      curves.push(curve);
    }

    // TODO full rework of positioning.
    // add people
    // add rand objects...
    // use the Container
    // animals

    if ctx.specials.contains(&Special::Chinese) {
      // FIXME BUGGY positioning. we want it on the mountain. need to do proper lookup like in the other mountains

      let o = (
        rng.gen_range(0.4..0.6) * width,
        rng.gen_range(0.8..0.9) * paint.height,
      );
      let h = rng.gen_range(0.1..0.15) * width;
      let w = h * rng.gen_range(1.5..2.0);
      let angle = 0.0;
      let door = ChineseDoor::init(rng, clr, o, w, h, angle);
      routes.extend(door.render(paint));
    }

    let mut trebuchet_candidates = vec![];
    let trebuchet_pad = width * 0.2;
    let trebuchet_ythreshold = paint.height * 0.9;

    let mut ridge = vec![];
    let first = curves[0].clone();
    let len = first.len();
    let trebuchet_mod = 1 + len / 20;
    for i in 0..len {
      let p = first[i];
      let mut max = p.1;
      for curve in curves.iter().skip(1) {
        let y = curve[i].1;
        if y < max {
          max = y;
        }
      }
      ridge.push((p.0, max));

      if max < trebuchet_ythreshold
        && i % trebuchet_mod == 0
        && trebuchet_pad < p.0
        && p.0 < width - trebuchet_pad
        && (ctx.specials.contains(&Special::Trebuchets) || rng.gen_bool(0.2))
      {
        // TODO some small glich, we need to push down if we have mountain curve that makes trebuchet off the screen
        let y = mix(max, ybase, rng.gen_range(0.2..0.6));
        trebuchet_candidates.push((p.0, y));
      }
    }

    trebuchet_candidates.shuffle(rng);

    let trebuchets_max =
      (rng.gen_range(-1.0f32..3.5) * rng.gen_range(0.5..1.0)).max(0.0) as usize;

    for &o in trebuchet_candidates.iter().take(trebuchets_max) {
      let height = rng.gen_range(0.17..0.22) * width;
      let action_percent = if !ctx.trebuchets_should_shoot {
        0.0
      } else {
        rng.gen_range(0.0..1.0)
      };
      let xflip = rng.gen_bool(0.5);
      let clr = 0;
      let trebuchet =
        Trebuchet::init(rng, o, height, action_percent, xflip, clr);
      routes.extend(trebuchet.render(&mut paint_before));
      trebuchet.throw_projectiles(ctx);
    }

    let x = rng.gen_range(0.2..0.8) * width;
    let y = mix(lookup_ridge(&ridge, x), ybase, 0.05);
    if y < paint.height * 0.85 {
      let o = (x, y);
      let size = rng.gen_range(0.08..0.12) * width;
      let xflip = rng.gen_bool(0.5);
      let blazon = ctx.attackers;
      let mainclr = 0;
      let blazonclr = 2;
      let objs = (0..2)
        .map(|_| {
          let items = vec![
            None,
            Some(HoldableObject::Flag),
            Some(HoldableObject::Torch),
            Some(HoldableObject::Axe),
            Some(HoldableObject::Shield),
            Some(HoldableObject::Sword),
            Some(HoldableObject::Club),
            Some(HoldableObject::LongBow(rng.gen_range(0.0..1.0))),
          ];
          let i = items.len() as f32 * rng.gen_range(0.0f32..1.0).powf(1.2);
          items[i as usize % items.len()]
        })
        .collect::<Vec<_>>();
      let lefthand = objs[0];
      let righthand = objs[1];

      let headshape = HeadShape::NAKED;
      let posture = if rng.gen_bool(0.8) {
        HumanPosture::hand_risen(rng)
      } else {
        HumanPosture::sit(rng, 0.0)
      };
      let human = Human::init(
        rng, o, size, xflip, blazon, mainclr, blazonclr, posture, headshape,
        lefthand, righthand,
      )
      .with_worms_filling_defaults();
      routes.extend(human.render(rng, ctx, &mut paint_before));
    }

    paint.paint(&paint_before);

    paint.paint_polyline(&ridge, 1.0);

    routes
  }
}

fn stroke_mountains(
  paint: &mut PaintMask,
  xfrom: f32,
  xto: f32,
  xincr: f32,
  ybase: f32,
  yamp: f32,
  valuef: &dyn Fn(f32, f32) -> f32,
  clr: usize,
) -> (Vec<(usize, Vec<(f32, f32)>)>, Vec<(f32, f32)>) {
  let mut routes = vec![];

  // sample the curve with f
  let mut curve = vec![];
  let mut x = xfrom;
  while x < xto {
    let y = ybase - yamp * valuef(x, ybase);
    curve.push((x, y));
    x += xincr;
  }
  if x > xto {
    let y = ybase - yamp * valuef(xto, ybase);
    curve.push((xto, y));
  }

  if curve.len() < 2 {
    return (routes, curve);
  }

  // make the polygons
  let mut polys = vec![];
  let len = curve.len();
  for j in 1..len {
    let i = j - 1;
    let mut poly = vec![];
    let a = curve[i];
    let b = curve[j];
    poly.push(a);
    poly.push(b);
    poly.push((b.0, ybase));
    poly.push((a.0, ybase));
    polys.push(poly);
  }

  routes.push((clr, curve.clone()));

  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }
  (routes, curve.clone())
}
