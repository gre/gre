use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  paintmask::PaintMask,
  polylines::{path_subdivide_to_curve, route_translate_rotate, Polylines},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Hut {
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
}

impl Hut {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];

    let x1 = -0.5 * size;
    let x2 = 0.5 * size;
    let y2 = -0.6 * size;
    let y3 = -size;
    let dy = rng.gen_range(0.0..0.1) * size;
    let mut route = vec![(x1, 0.0)];
    let p1 = vec![(x1, y2), (0.5 * x1, mix(y2, y3, 0.5) + dy), (0.0, y3)];
    let mut p1 = path_subdivide_to_curve(&p1, 2, 0.7);
    let mut p2 = p1.iter().map(|p| (-p.0, p.1)).collect::<Vec<_>>();
    p2.reverse();
    p1.pop();
    route.extend(p1);
    route.extend(p2);
    route.push((x2, 0.0));
    let route = route_translate_rotate(&route, origin, -angle);
    routes.push((clr, route.clone()));
    polys.push(route);

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
