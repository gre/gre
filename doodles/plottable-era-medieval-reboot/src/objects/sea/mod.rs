use super::army::boat::boat_with_army;
use crate::algo::{
  clipping::{clip_routes_with_colors, regular_clip},
  paintmask::PaintMask,
  passage::{self, Passage},
  polylines::slice_polylines,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Sea {
  sea_mask: PaintMask,
  boat_color: usize,
  yhorizon: f64,
}

impl Sea {
  pub fn from(paint: &PaintMask, yhorizon: f64, boat_color: usize) -> Self {
    let mut sea_mask = paint.clone();
    sea_mask.paint_fn(&|(_, y)| y < yhorizon);

    // FIXME how are we going to project here the above part that isn't drawn yet?
    // TODO figure out areas where there is a sea,
    // that way we can determine where to place possible boats
    // this is where we will be able to also project the mountains

    // TODO some algo that need to do some kind of flood fill to locate the big areas

    Self {
      yhorizon,
      sea_mask,
      boat_color,
    }
  }

  pub fn reflect_shapes<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    reflectables: &Vec<(usize, Vec<(f64, f64)>)>,
    probability_par_color: Vec<f64>,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    // TODO implement

    // 1: we select the part above yhorizon sea level

    // 2: we store coloring information in a map, with density information...
    // the density is max-ed out by a certain amount to deduplicate information

    // 3: for each grid point we will project a % of the density on the sea, & apply some dash effect & y disp

    let mut passage = Passage::new(0.5, paint.width, paint.height);
    let is_below_sea_level = |(_x, y): (f64, f64)| y > self.yhorizon;
    let reflectables =
      clip_routes_with_colors(&reflectables, &is_below_sea_level, 0.5, 3);

    let ydisplacement = 0.5 * paint.height;
    let xdisplacement = 0.3 * ydisplacement;

    routes.extend(reflect_shapes(
      rng,
      &reflectables,
      &mut passage,
      0.3,
      3.0,
      self.yhorizon,
      (0.0, 0.0, paint.width, paint.height),
      5,
      xdisplacement,
      ydisplacement,
    ));

    // FIXME: should sea_mask be used instead? meaning that render() would alter it too?
    // depending if we manage to not alter the sea when drawing the above parts
    regular_clip(&routes, paint)
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let width = paint.width;
    let height = paint.height;

    if rng.gen_bool(0.8) {
      let origin = (
        width * rng.gen_range(0.3..0.7),
        self.yhorizon + height * rng.gen_range(0.0..0.1),
      );
      let size = width * 0.05;
      let angle = rng.gen_range(-0.1..0.1) * rng.gen_range(0.0..1.0);
      let w = size * rng.gen_range(4.0..5.0);
      let xflip = rng.gen_bool(0.5);
      routes.extend(boat_with_army(
        rng,
        paint,
        self.boat_color,
        origin,
        angle,
        size,
        w,
        xflip,
      ));
    }

    routes
  }
}

// FIXME this is the old impl
fn reflect_shapes<R: Rng>(
  rng: &mut R,
  reflectables: &Vec<(usize, Vec<(f64, f64)>)>,
  // TODO use passage to not have too much density
  passage: &mut Passage,
  probability: f64,
  stroke_len_base: f64,
  ycenter: f64,
  boundaries: (f64, f64, f64, f64),
  max_passage: usize,
  xdisplacement: f64,
  ydisplacement: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut new_shapes = Vec::new();

  let min_stroke_length = 0.5 * stroke_len_base;
  let max_stroke_length = stroke_len_base;

  let exact_balance = 0.9;

  for (clr, route) in reflectables.clone() {
    for route in
      slice_polylines(&route, rng.gen_range(1.0..2.0) * stroke_len_base)
    {
      if !rng.gen_bool(exact_balance * probability) {
        continue;
      }
      let dispy = rng.gen_range(0.0..0.2 * ydisplacement)
        * rng.gen_range(0.0..1.0)
        * rng.gen_range(0.0..1.0);
      let projection = route
        .iter()
        .map(|p| {
          let x = p.0;
          let mut y = 2.0 * ycenter - p.1;
          y += dispy;
          (x, y)
        })
        .collect();
      new_shapes.push((clr, projection));
    }

    for p in route {
      if !rng.gen_bool((1.0 - exact_balance) * probability) {
        continue;
      }
      let sx = (min_stroke_length
        + (max_stroke_length - min_stroke_length)
          * rng.gen_range(0f64..1.0).powi(2))
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
