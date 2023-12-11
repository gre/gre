use crate::{
  algo::{
    packing::packing,
    paintmask::PaintMask,
    polylines::{path_subdivide_to_curve, shake, Polylines},
  },
  objects::army::flyingdragon::FlyingDragon,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn dragons<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  width: f32,
  height: f32,
  pad: f32,
  framingw: f32,
  n: usize,
) -> Polylines {
  let mut routes = vec![];
  for i in 0..n {
    let bx = pad + framingw + 0.2 * width;
    let by = pad + framingw + 0.2 * height;
    let count = rng.gen_range(2..16);
    let mut circles = packing(
      rng,
      500,
      count,
      1,
      0.05 * width,
      (bx, by, width - bx, height - by),
      &|_| true,
      0.01 * width,
      0.1 * width,
    );
    circles.sort_by(|a, b| b.y.partial_cmp(&a.y).unwrap());

    let mut rt = vec![];
    for c in circles {
      rt.push((c.x, c.y));
    }

    while rt.len() < 2 {
      rt.push((
        rng.gen_range(0.33..0.66) * paint.width,
        rng.gen_range(0.2..0.5) * paint.height,
      ));
    }
    for _ in 0..rng.gen_range(1..3) {
      rt = path_subdivide_to_curve(&rt, 1, 0.66);
      let s = rng.gen_range(0.0..0.1) * paint.width;
      rt = shake(rt, s, rng);
    }
    rt = path_subdivide_to_curve(&rt, 1, 0.7);
    rt = path_subdivide_to_curve(&rt, 1, 0.8);

    let size = rng.gen_range(0.04..0.08) * width;
    let step = rng.gen_range(1.0..2.0);
    let count =
      4 + (rng.gen_range(0.0..20.0) * rng.gen_range(0.0..1.0)) as usize;
    let angleoff = rng.gen_range(-0.3..0.3)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(-0.5f32..1.0).max(0.0);
    routes.extend(
      FlyingDragon::init(rng, (i + 2) % 3, &rt, size, step, count, angleoff)
        .render(paint),
    );
  }
  routes
}
