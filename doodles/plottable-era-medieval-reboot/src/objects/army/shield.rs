use crate::{
  algo::{
    clipping::regular_clip_polys, paintmask::PaintMask,
    polylines::route_translate_rotate,
  },
  objects::blazon::Blazon,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub struct Shield {
  routes: Vec<(usize, Vec<(f32, f32)>)>,
  polygons: Vec<Vec<(f32, f32)>>,
}

impl Shield {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    blazonclr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    _blazon: Blazon, // TODO
  ) -> Self {
    let mx = if xflip { -1.0 } else { 1.0 };
    let mut routes = Vec::new();
    let dx = 0.2 * size;
    let dy = 0.4 * size;
    let mut route = vec![];
    let mut route2 = vec![];
    // todo s1 and s2 to be associated to the blazon somehow
    let s1 = rng.gen_range(0.0..1.0);
    let s2 = rng.gen_range(0.0..1.0);
    for v in vec![
      (0.0, -dy),
      (mx * 0.5 * dx, -dy),
      (mx * dx, -(1.0 - s1 * s1) * dy),
      (mx * dx, 0.0),
      (mx * dx, s2 * dy),
      (0.0, dy),
    ] {
      route.push(v);
      route2.push((-v.0, v.1));
    }
    route2.reverse();
    route.extend(route2);

    route = route_translate_rotate(&route, origin, angle);
    let polygons = vec![route.clone()];
    routes.push((blazonclr, route));

    // TODO either a > or a square, or a circle,... many possibilities
    let tick = rng.gen_range(0.2..0.3);
    let y = rng.gen_range(-0.2..0.0) * dy;
    routes.push((
      blazonclr,
      route_translate_rotate(
        &vec![
          (0.0, -tick * dy + y),
          (mx * tick * dx, y),
          (0.0, tick * dy + y),
        ],
        origin,
        angle,
      ),
    ));

    Self { routes, polygons }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Vec<(usize, Vec<(f32, f32)>)> {
    regular_clip_polys(&self.routes, paint, &self.polygons)
  }
}
