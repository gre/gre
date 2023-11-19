use std::f32::consts::PI;

use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polygon::make_wireframe_from_vertexes,
  polylines::{path_to_fibers, Polylines},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Relic {
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
}

impl Relic {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    filling: f32,
  ) -> Self {
    let angle = angle - PI / 2.0; // pointing up
    let mut routes = vec![];
    let clr = 1;

    let w1 = size;
    let w2 = 0.6 * size;
    let dx = angle.cos();
    let dy = angle.sin();
    let h1 = 0.25 * size;
    let h2 = 0.4 * size;
    let barebone = vec![
      origin,
      (origin.0 + h1 * dx, origin.1 + h1 * dy),
      (origin.0 + h2 * dx, origin.1 + h2 * dy),
    ];
    let widths = vec![w1, w1, w2];
    let count = 2 + (size / filling) as usize;
    let fibers = path_to_fibers(&barebone, &widths, count);
    let vert1 = &fibers[0];
    let vert2 = &fibers[count - 1];
    let polys = make_wireframe_from_vertexes(vert1, vert2);

    for i in 0..vert1.len() {
      let route = vec![vert1[i], vert2[i]];
      routes.push((clr, route));
    }
    for fiber in fibers {
      routes.push((clr, fiber));
    }

    Self { routes, polys }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }
    routes
  }
}
