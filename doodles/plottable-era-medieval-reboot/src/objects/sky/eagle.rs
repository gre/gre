use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{path_subdivide_to_curve, route_scale_translate_rotate, shake},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn eagle<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  origin: (f64, f64),
  sz: f64,
  rotation: f64,
  xreverse: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let scale = sz / 5.0;
  let xmul = if xreverse { -1.0 } else { 1.0 };
  let count = 2 + (scale * 3.0) as usize;
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let shaking = scale * 0.1;

  // body
  let bodyw = 5.0;
  let bodyh = 1.5;
  let headcompression = rng.gen_range(0.1..0.5);
  let headoff = rng.gen_range(0.1..0.5);
  for i in 0..count {
    let yp = (i as f64 - (count - 1) as f64 * 0.5) / (count as f64);
    let ybase = bodyh * yp;
    let route = shake(
      path_subdivide_to_curve(
        vec![
          (-rng.gen_range(0.4..0.6) * bodyw, 1.5 * ybase),
          (-0.3 * bodyw, ybase),
          (0.2 * bodyw, ybase),
          (0.45 * bodyw, headcompression * ybase + headoff * bodyh),
        ],
        1,
        0.8,
      ),
      shaking,
      rng,
    );
    routes.push(route);
  }

  let count = 2 + (scale * rng.gen_range(4.0..6.0)) as usize;

  // wings
  let wingw = 1.4;
  let wingh = 8.0;
  let dx1 = rng.gen_range(-4.0..4.0) * rng.gen_range(0.0..1.0);
  let dx2 = if rng.gen_bool(0.8) {
    -dx1
  } else {
    rng.gen_range(-3.0..3.0)
  };
  let spread1 = 1.0 + rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
  let spread2 = 1.0 + rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
  let offset1 = rng.gen_range(-1.0..0.6) * rng.gen_range(0.0..1.0);
  let offset2 = rng.gen_range(-1.0..0.6) * rng.gen_range(0.0..1.0);
  let interp = 0.5;
  let wing1m = 1.0 - rng.gen_range(0.0..0.5) * rng.gen_range(0.0..1.0);
  let wing2m = 1.0 - rng.gen_range(0.0..0.5) * rng.gen_range(0.0..1.0);
  let wing2up = rng.gen_bool(0.5);

  for i in 0..count {
    let xp = (i as f64 - (count - 1) as f64 * 0.5) / (count as f64);
    let xbase = wingw * xp;
    let wing1 = rng.gen_range(0.8..1.1) * wing1m;
    let wing2 =
      rng.gen_range(0.8..1.1) * wing2m * (if wing2up { -1.0 } else { 1.0 });
    let route = shake(
      path_subdivide_to_curve(
        vec![
          (
            xbase * spread1 + dx1 + wingw * offset1,
            -wingh * 0.5 * wing1,
          ),
          (xbase + dx1 * interp, -wingh * 0.5 * interp * wing1),
          (xbase, 0.0),
          (xbase + dx2 * interp, wingh * 0.5 * interp * wing2),
          (xbase * spread2 + dx2 + wingw * offset2, wingh * 0.5 * wing2),
        ],
        2,
        0.8,
      ),
      shaking,
      rng,
    );
    routes.push(route);
  }

  let mut circles = vec![];
  let border = 1.2;

  // scale, rotate & translate
  let out = routes
    .iter()
    .map(|route| {
      (
        clr,
        route_scale_translate_rotate(
          &route,
          (xmul * scale, scale),
          origin,
          rotation,
        ),
      )
    })
    .collect();
  let out = regular_clip(&out, paint);
  for (x, y, r) in circles {
    paint.paint_circle(x, y, r);
  }
  out
}
