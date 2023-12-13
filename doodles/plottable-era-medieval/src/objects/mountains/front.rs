use crate::{
  algo::{
    clipping::{clip_routes_with_colors, regular_clip},
    math1d::mix,
    math2d::lookup_ridge,
    paintmask::PaintMask,
    renderable::Container,
  },
  global::GlobalCtx,
  objects::{
    army::{
      body::HumanPosture,
      firecamp::Firecamp,
      human::{HeadShape, HoldableObject, Human},
      rider::Rider,
      spawn_animal,
      trebuchet::Trebuchet,
    },
    blazon::Blazon,
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
    let mut mountainpaint = paint.clone_rescaled(1.0);

    let (curves, mountainrts) = make_mountains(self, rng, &mut mountainpaint);

    let container = spawn_things(self, rng, paint, ctx, &curves);

    let thingsrts = container.render_with_extra_halo(rng, ctx, paint, 1.6);

    let mountainrts = regular_clip(&mountainrts, paint);
    paint.paint(&mountainpaint);

    let mut routes = vec![];
    routes.extend(mountainrts);
    routes.extend(thingsrts);
    routes
  }
}

fn make_mountains<R: Rng>(
  front: &FrontMountains,
  rng: &mut R,
  paint: &mut PaintMask,
) -> (Vec<Vec<(f32, f32)>>, Vec<(usize, Vec<(f32, f32)>)>) {
  let clr = front.clr;
  let ybase = front.ybase;
  let ystart = front.ystart;
  let width = front.width;

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

  (curves, routes)
}

fn spawn_things<R: Rng>(
  front: &FrontMountains,
  rng: &mut R,
  paint: &mut PaintMask,
  ctx: &mut GlobalCtx,
  curves: &Vec<Vec<(f32, f32)>>,
) -> Container<R> {
  let width = front.width;
  let ybase = front.ybase;
  let mainclr = 0;

  let mut exclusion_mask = paint.clone_empty_rescaled(1.0);
  let mut container = Container::new();

  let mut ridge = vec![];
  let first = curves[0].clone();
  let len = first.len();
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
  }

  let mut nb_fire =
    (rng.gen_range(-200.0f64..20.0) * rng.gen_range(0.0..1.0)) as usize;

  let mut nb_man =
    (rng.gen_range(-1.0f64..3.5) * rng.gen_range(0.0..1.0)) as usize;

  let mut nb_trebuchets =
    (rng.gen_range(-1.0f64..4.0) * rng.gen_range(0.0..1.0)) as usize;

  let mut nb_animals = rng.gen_range(-20.0f64..5.0) as usize;

  let sampling = 100;
  for _ in 0..sampling {
    let x = rng.gen_range(0.2..0.8) * width;
    let yoff = rng.gen_range(0.0..1.0);
    let yleft = lookup_ridge(&ridge, x - 0.05 * width);
    let ymid = lookup_ridge(&ridge, x);
    let yright = lookup_ridge(&ridge, x + 0.05 * width);
    let y = mix(ymid.max(yleft).max(yright), ybase, yoff);
    let origin = (x, y);
    if exclusion_mask.is_painted(origin) {
      continue;
    }

    if rng.gen_bool((1. - yoff as f64).powf(2.0)) && nb_man > 0 {
      nb_man -= 1;

      let o = (x, y);
      let size = mix(0.09, 0.14, yoff)
        * width
        * rng.gen_range(0.8..1.0)
        * (if nb_trebuchets == 0 { 1.5 } else { 1.0 });
      let xflip = rng.gen_bool(0.5);
      let blazon = ctx.attackers;
      let mainclr = 0;
      let blazonclr = 2;
      let human = random_human(rng, o, size, xflip, blazon, mainclr, blazonclr);

      if rng.gen_bool(0.8) {
        exclusion_mask.paint_circle(x, y, size);
        container.add(human);
      } else {
        let rider = Rider::init(
          rng, o, size, 0.0, xflip, mainclr, blazonclr, 0.3, 1.0, human,
        );
        exclusion_mask.paint_circle(x, y, 2.0 * size);
        container.add(rider);
      }
    } else if nb_man == 0 {
      if nb_fire > 0 {
        let size = mix(0.01, 0.02, yoff) * width * rng.gen_range(0.8..1.0);
        let smokel = size * rng.gen_range(4.0..12.0);
        let camp = Firecamp::init(rng, ctx, mainclr, origin, size, smokel);
        exclusion_mask.paint_circle(x, y, size);
        container.add(camp);
        nb_fire -= 1;
      } else if nb_animals > 0 {
        let size = mix(0.05, 0.1, yoff) * width * rng.gen_range(0.8..1.0);
        spawn_animal(
          rng,
          &mut container,
          &mut exclusion_mask,
          origin,
          size,
          0.0,
          mainclr,
        );
        nb_animals -= 1;
      } else if nb_trebuchets > 0 {
        nb_trebuchets -= 1;

        let height = mix(0.15, 0.26, yoff) * width * rng.gen_range(0.8..1.0);
        let action_percent = if !ctx.trebuchets_should_shoot {
          0.0
        } else {
          rng.gen_range(0.0..1.0)
        };
        let xflip = rng.gen_bool(0.5);
        let trebuchet =
          Trebuchet::init(rng, origin, height, action_percent, xflip, mainclr);
        exclusion_mask.paint_circle(x, y, height);
        trebuchet.throw_projectiles(ctx);
        container.add(trebuchet);
      }
    }
  }
  container
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
    paint.paint_polyline(poly, 1.0);
  }
  (routes, curve.clone())
}

fn random_human<R: Rng>(
  rng: &mut R,
  origin: (f32, f32),
  size: f32,
  xflip: bool,
  blazon: Blazon,
  mainclr: usize,
  blazonclr: usize,
) -> Human {
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
    rng, origin, size, xflip, blazon, mainclr, blazonclr, posture, headshape,
    lefthand, righthand,
  )
  .with_worms_filling_defaults();

  human
}
