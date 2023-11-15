use super::decorations::*;
use crate::algo::{
  clipping::regular_clip_polys, math1d::mix, paintmask::PaintMask,
  wormsfilling::WormsFilling,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Chapel {
  pub width: f32,
  pub height: f32,
  pub scale: f32,
  pub pos: (f32, f32),
  pub ybase: f32,
  pub clr: usize,
  pub dark_fill: bool,
}

impl Chapel {
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let ybase = self.ybase;
    let pos = self.pos;
    let width = self.width;
    let height = self.height;
    let scale = self.scale;
    let dark_fill = self.dark_fill;
    let clr = self.clr;
    castle_chapel(rng, paint, ybase, pos, width, height, scale, dark_fill, clr)
  }
}

fn castle_chapel<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  // ybase is where the chapel foundation need to start
  ybase: f32,
  // center of the chapel base
  pos: (f32, f32),
  width: f32,
  height: f32,
  scale: f32,
  dark_fill: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = vec![];
  let mut polys = vec![];
  let mut route = Vec::new();

  let roof_height = scale * rng.gen_range(4.0..14.0);

  let x = pos.0 + width / 2.0;
  route.push((x, ybase));
  route.push((x, pos.1 - height));

  if dark_fill {
    let its = rng.gen_range(800..1200);
    routes.extend(WormsFilling::rand(rng).fill(
      rng,
      &|_x, _y| 1.5,
      (
        pos.0 - width / 2.0,
        pos.1 - height,
        pos.0 + width / 2.0,
        ybase,
      ),
      &|_| clr,
      0.5,
      its,
      1,
    ));
  } else {
    for shadow in wall_shadow(rng, route.clone(), -scale) {
      routes.push((clr, shadow));
    }
  }
  let x = pos.0 - width / 2.0;
  route.push((x, pos.1 - height));
  route.push((x, ybase));
  routes.push((clr, route));

  // boundaries of chapel body
  polys.push(vec![
    (pos.0 - width / 2.0, ybase),
    (pos.0 + width / 2.0, ybase),
    (pos.0 + width / 2.0, pos.1 - height),
    (pos.0 - width / 2.0, pos.1 - height),
  ]);

  let w = width * rng.gen_range(0.5..0.55);
  let h = roof_height;
  let y = pos.1 - height;
  routes.push((clr, vec![(pos.0 - w, y), (pos.0, y - h), (pos.0 + w, y)]));

  // boundaries of chapel roof
  polys.push(vec![(pos.0 - w, y), (pos.0, y - h), (pos.0 + w, y)]);
  let mut l = 0.0;
  loop {
    if l > 2.0 * w {
      break;
    }
    routes.push((clr, vec![(pos.0, y - h), (pos.0 + w - l, y)]));
    l += scale * rng.gen_range(0.7..1.0) + l / w;
  }

  // cross
  let x = pos.0;
  let y = y - h - 2.0;
  routes.push((clr, vec![(x - scale * 0.8, y), (x + scale * 0.8, y)]));
  routes.push((clr, vec![(x, y - scale * 1.0), (x, y + scale * 2.0)]));

  // window
  let x = pos.0;
  let y = mix(pos.1 - height, pos.1, rng.gen_range(0.2..0.3));
  let w = scale * 0.4;
  let h = scale * 0.6;
  routes.push((
    clr,
    vec![
      (x - w, y - h),
      (x + w, y - h),
      (x + w, y + h),
      (x - w, y + h),
      (x - w, y - h),
    ],
  ));

  /*
  let pushbackbase =
    rng.gen_range(0.0..0.04) * rng.gen_range(0.0..1.0) * height;
  let pushbackrotbase = rng.gen_range(-1.0..1.0);
  let pushbackrotmix = rng.gen_range(0.1..0.9);
  let sliding = scale * rng.gen_range(0.5..2.0);
  let (routes, polys) = multicut_along_line(
    rng,
    &routes,
    &polys,
    clr,
    pos,
    (pos.0, pos.1 - height),
    |rng| rng.gen_range(2.0, 10.0),
    |rng| rng.gen_range(-PI / 2.0, PI / 2.0) * rng.gen_range(0.0, 1.0),
    |rng| sliding * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0),
    |rng| pushbackbase * rng.gen_range(0.5, 2.0),
    |rng| 0.1 * mix(pushbackrotbase, rng.gen_range(-1.0, 1.0), pushbackrotmix),
  );
  */

  // clip and paint
  // let is_outside = |p| paint.is_painted(p);

  let routes = regular_clip_polys(&routes, paint, &polys);

  /*
  // make ropes behind the construction
  let count = rng.gen_range(3..16);
  routes.extend(building_ropes(
    rng,
    paint,
    &polys,
    count,
    clr,
    2.0 * width,
    height + 50.0,
  ));
  */

  routes
}
