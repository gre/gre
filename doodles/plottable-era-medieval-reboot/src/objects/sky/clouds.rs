use crate::algo::{
  clipping::clip_routes_with_colors, packing::VCircle, paintmask::PaintMask,
  shapes::arc,
};
use rand::prelude::*;
use std::f64::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn cloud_in_circle<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  clr: usize,
  circle: &VCircle,
  base_dr: f64,
  minr: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  let mut circles: Vec<VCircle> = vec![];

  let heightmul = rng.gen_range(0.2..0.6);

  let count = rng.gen_range(40..120);
  for _i in 0..count {
    let radius = circle.r * rng.gen_range(0.3..0.5) * rng.gen_range(0.2..1.0);
    let angle = rng.gen_range(0.0..2.0 * PI);
    let x = circle.x + angle.cos() * (circle.r - radius);
    let y = circle.y
      + angle.sin() * (circle.r - radius) * rng.gen_range(0.5..1.0) * heightmul;
    let circle = VCircle::new(x, y, radius);

    let should_crop = |p| circles.iter().any(|c| c.includes(p));

    let mut input_routes = vec![];
    let mut r = radius;
    let dr = base_dr * rng.gen_range(0.7..1.5);
    loop {
      if r < minr {
        break;
      }
      let count = (r * 2.0 + 10.0) as usize;
      let amp = rng.gen_range(0.5 * PI..1.2 * PI);
      let ang = angle
        + PI
          * rng.gen_range(-1.0..1.0)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0);
      let start = ang - amp / 2.0;
      let end = ang + amp / 2.0;
      input_routes.push((clr, arc((x, y), r, start, end, count)));
      r -= dr;
    }

    routes.extend(clip_routes_with_colors(&input_routes, &should_crop, 0.3, 4));

    circles.push(circle);
  }

  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for c in circles {
    paint.paint_circle(c.x, c.y, c.r);
  }

  routes
}
