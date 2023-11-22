use crate::algo::{
  clipping::{clip_routes_with_colors, regular_clip},
  math1d::mix,
  paintmask::PaintMask,
  polylines::{route_translate_rotate, Polylines},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Paddle {
  routes: Polylines,
}

impl Paddle {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
  ) -> Self {
    let p0 = 0.0;
    let n = rng.gen_range(0.0..0.15);
    let p1 = -(0.9 + n) * size;
    let p2 = -(1.0 + n) * size;
    let p3 = -1.15 * size;
    let p4 = -1.2 * size;
    let w0 = 0.02 * size;
    let w1 = 0.06 * size;
    let w2 = mix(w0, w1, 0.5);
    let mut stick = vec![
      (p0, w0),
      (p1, w0),
      (p2, w1),
      (p3, w1),
      (p4, w2),
      (p4, -w2),
      (p3, -w1),
      (p2, -w1),
      (p1, -w0),
    ];
    if w0 > 0.2 {
      stick.push((p0, -w0));
      stick.push((p0, w0));
    } else {
      (p1, w0);
    }

    let routes = vec![(clr, stick.clone())];

    // project on the canvas
    let routes = routes
      .iter()
      .map(|(clr, rt)| (*clr, route_translate_rotate(&rt, origin, -angle)))
      .collect::<Vec<_>>();

    // clip when it's in water
    let watery = origin.1 + 0.8 * size;
    let routes = clip_routes_with_colors(&routes, &|o| o.1 > watery, 1.0, 3);

    Self { routes }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for (_, p) in &self.routes {
      paint.paint_polygon(&p); // the routes are the poly
    }
    routes
  }
}
