use std::f32::consts::PI;

use super::{
  army::{
    boatarmy::BoatArmy,
    body::HumanPosture,
    human::{HeadShape, HoldableObject, Human},
    sword::Sword,
  },
  blazon::Blazon,
  rock::Rock,
};
use crate::{
  algo::{
    clipping::{clip_routes_with_colors, regular_clip},
    math1d::mix,
    paintmask::PaintMask,
    passage::Passage,
    polylines::slice_polylines,
    renderable::{as_box_renderable, Renderable},
  },
  global::{GlobalCtx, Special},
  objects::sea::sauron::SauronEye,
};
use rand::prelude::*;
pub mod beach;
pub mod port;
pub mod sauron;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Sea {
  sea_mask: PaintMask,
  boat_color: usize,
  yhorizon: f32,
  blazon: Blazon,
}

impl Sea {
  pub fn from(
    paint: &PaintMask,
    yhorizon: f32,
    boat_color: usize,
    blazon: Blazon,
  ) -> Self {
    let mut sea_mask = paint.clone();
    sea_mask.paint_fn(&|(_, y)| y < yhorizon);

    Self {
      yhorizon,
      sea_mask,
      boat_color,
      blazon,
    }
  }

  pub fn reflect_shapes<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    reflectables: &Vec<(usize, Vec<(f32, f32)>)>,
    probability_par_color: Vec<f32>,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    // TODO idea to improve this:

    // 1: we select the part above yhorizon sea level

    // 2: we store coloring information in a map, with density information...
    // the density is max-ed out by a certain amount to deduplicate information

    // 3: for each grid point we will project a % of the density on the sea, & apply some dash effect & y disp

    let mut passage = Passage::new(0.5, paint.width, paint.height);
    let is_below_sea_level = |(_x, y): (f32, f32)| y > self.yhorizon;
    let reflectables =
      clip_routes_with_colors(&reflectables, &is_below_sea_level, 0.5, 3);

    let ydisplacement = 0.3 * paint.height;
    let xdisplacement = 0.3 * ydisplacement;
    let stroke_len_base = 0.04 * paint.width;

    routes.extend(reflect_shapes(
      rng,
      &reflectables,
      &mut passage,
      probability_par_color,
      stroke_len_base,
      self.yhorizon,
      (0.0, 0.0, paint.width, paint.height),
      3,
      xdisplacement,
      ydisplacement,
    ));

    // FIXME: should sea_mask be used instead? meaning that render() would alter it too?
    // depending if we manage to not alter the sea when drawing the above parts
    regular_clip(&routes, paint)
  }

  pub fn render<R: Rng>(
    &self,
    ctx: &mut GlobalCtx,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let width = paint.width;
    let height = paint.height;

    let no_boats = ctx.specials.contains(&Special::TrojanHorse);

    let rocks_count = (rng.gen_range(0.0f32..14.0) * rng.gen_range(-0.5..1.0))
      .max(0.0) as usize;
    let boats_count = if no_boats {
      0
    } else {
      (rng.gen_range(0.0f32..20.0) * rng.gen_range(-0.3..1.0)).max(0.0) as usize
    };

    // Place rocks
    let mut sea_shapes: Vec<Box<dyn Renderable<R>>> = vec![];

    // this mask is used to find location to pack things
    let mut sea_mask = self.sea_mask.clone();

    let mut should_set_excalibur = rng.gen_bool(0.1) && ctx.specials.is_empty();

    let tries = 10;
    sea_shapes.extend(
      (0..rocks_count)
        .filter_map(|_| {
          for _ in 0..tries {
            let x = width * rng.gen_range(0.0..1.0);
            let yp = rng.gen_range(0.0..1.0);
            let y = mix(self.yhorizon, height, yp);
            if !sea_mask.is_painted((x, y)) {
              let size = (0.04
                + rng.gen_range(0.0..0.04) * rng.gen_range(0.0..1.0))
                * width;
              let minx = x - size;
              let maxx = x + size;
              let miny = y - size;
              let maxy = y;
              sea_mask.paint_rectangle(minx, miny, maxx, maxy);
              let origin = (x, y);
              let elevation =
                1.5 + rng.gen_range(0.0..5.0) * rng.gen_range(0.0..1.0);
              let count_poly =
                (rng.gen_range(0.8..1.2) * (elevation * 5. + 3.)) as usize;

              let rockclr = if rng.gen_bool(0.02) { 1 } else { 0 };

              let excalibur = if should_set_excalibur
                && elevation > 4.0
                && (0.3..0.7).contains(&(x / width))
              {
                should_set_excalibur = false;
                true
              } else {
                false
              };

              let mut spawn = |rng: &mut R, o: (f32, f32), s, a| {
                if !(0.3..0.7).contains(&(o.0 / width)) {
                  return None;
                }
                if excalibur {
                  ctx.specials.insert(Special::Excalibur);
                  let clr = rng.gen_range(0..2);
                  Some(as_box_renderable(Sword::init(rng, o, s, a, clr)))
                } else {
                  let dy = origin.1 - o.1;
                  if rng.gen_bool(0.08) && dy > 0.3 * width {
                    let sauron = SauronEye::init(rng, paint, rockclr, 1, o, s);
                    ctx.specials.insert(Special::Sauroned);
                    return Some(as_box_renderable(sauron));
                  } else if dy > 0.15 * width {
                    // opportunity for some other random stuff
                  }
                  None
                }
              };

              let rock = Rock::init(
                rng, origin, size, rockclr, count_poly, elevation, &mut spawn,
              );

              let b: Box<dyn Renderable<R>> = Box::new(rock);
              return Some(b);
            }
          }
          return None;
        })
        .collect::<Vec<_>>(),
    );

    // Place boats

    let tries = 10;
    let basew = width * rng.gen_range(0.15..0.25);
    sea_shapes.extend(
      (0..boats_count)
        .filter_map(|_| {
          for _ in 0..tries {
            let x = width * rng.gen_range(0.2..0.8);
            let yp = rng.gen_range(0.1..1.0);
            let y = mix(self.yhorizon, height, yp);
            let w = basew
              * (1.0 + rng.gen_range(-0.4..0.8) * rng.gen_range(0.0..1.0));
            let size = width * mix(0.03, 0.08, yp);
            if !sea_mask.is_painted((x, y)) {
              let minx = x - w / 2.0;
              let maxx = x + w / 2.0;
              let miny = y - w / 10.0;
              let maxy = y + w / 10.0;
              sea_mask.paint_rectangle(minx, miny, maxx, maxy);

              //for (x, y, w, size) in boat_positions {
              let angle = rng.gen_range(-0.2..0.2) * rng.gen_range(0.0..1.0);
              let xflip = if rng.gen_bool(0.8) {
                x > width / 2.0
              } else {
                rng.gen_bool(0.5)
              };

              // TODO boat need to have people with spears / swords / archers only
              // TODO also flags

              let spawn_human = |rng: &mut R, o, size, angle, xflip| {
                let headshape = HeadShape::HELMET;
                let lefthandobj = Some(HoldableObject::Shield);
                // TODO paddle angle to be organized between people on the boat and sometimes it can be up.
                let a = if xflip {
                  -PI * rng.gen_range(0.6..0.7)
                } else {
                  -PI * rng.gen_range(0.3..0.4)
                };
                let righthandobj = Some(HoldableObject::Paddle(a));
                let posture = HumanPosture::from_holding(
                  rng,
                  false,
                  lefthandobj,
                  righthandobj,
                );

                let human = Human::init(
                  rng,
                  o,
                  size,
                  angle,
                  xflip,
                  self.blazon,
                  0,
                  self.boat_color,
                  posture,
                  headshape,
                  lefthandobj,
                  righthandobj,
                );
                human
              };
              let boat = BoatArmy::init(
                rng,
                self.boat_color,
                (x, y),
                size,
                angle,
                w,
                xflip,
                self.blazon,
                &spawn_human,
              );
              // routes.extend(boat.render(rng, paint));
              //}

              let b: Box<dyn Renderable<R>> = Box::new(boat);
              return Some(b);
            }
          }
          return None;
        })
        .collect::<Vec<_>>(),
    );

    // Render

    // TODO move to a Container
    sea_shapes.sort_by(|a, b| b.yorder().partial_cmp(&a.yorder()).unwrap());

    let mut routes = vec![];
    for s in sea_shapes {
      routes.extend(s.render(rng, paint));
    }
    routes
  }
}

// FIXME this is the old impl
fn reflect_shapes<R: Rng>(
  rng: &mut R,
  reflectables: &Vec<(usize, Vec<(f32, f32)>)>,
  passage: &mut Passage,
  probability_per_color: Vec<f32>,
  stroke_len_base: f32,
  ycenter: f32,
  boundaries: (f32, f32, f32, f32),
  max_passage: usize,
  xdisplacement: f32,
  ydisplacement: f32,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut new_shapes = Vec::new();

  let min_stroke_length = 0.5 * stroke_len_base;
  let max_stroke_length = stroke_len_base;

  for (clr, route) in reflectables.clone() {
    let probability = probability_per_color[clr];
    for p in slice_polylines(&route, rng.gen_range(1.0..2.0) * stroke_len_base)
      .iter()
      .flatten()
    {
      if !rng.gen_bool(probability as f64) {
        continue;
      }
      let sx = (min_stroke_length
        + (max_stroke_length - min_stroke_length)
          * rng.gen_range(0f32..1.0).powi(2))
        / 2.0;
      let sy = 0.3 * rng.gen_range(-1.0..1.0) * rng.gen_range(0.0..1.0);
      let x = p.0
        + rng.gen_range(0.0..xdisplacement)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(-1.0..1.0);
      let y = 2.0 * ycenter - p.1
        + rng.gen_range(0.0..ydisplacement) * rng.gen_range(-1.0..1.0);
      if y > ycenter && y < boundaries.3 {
        let x1 = (x - sx).max(boundaries.0).min(boundaries.2);
        let x2 = (x + sx).max(boundaries.0).min(boundaries.2);
        if x2 - x1 > min_stroke_length {
          // TODO do it with as many point as needed between x1 and x2, if any of these have too much passage, we skip
          if passage.get((x, y)) > max_passage {
            continue;
          }
          passage.count((x, y));
          new_shapes.push((clr, vec![(x1, y - sy), (x2, y + sy)]));
        }
      }
    }
  }
  new_shapes
}
