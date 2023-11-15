use crate::algo::{
  clipping::regular_clip_polys, paintmask::PaintMask,
  polylines::route_translate_rotate,
};
use rand::prelude::*;

pub fn shield<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  clr: usize,
  origin: (f32, f32),
  size: f32,
  angle: f32,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = Vec::new();
  let dx = 0.2 * size;
  let dy = 0.4 * size;
  let mut route = vec![];
  let mut route2 = vec![];
  for v in vec![
    (0.0, -dy),
    (0.5 * dx, -dy),
    (
      dx,
      -(1.0 - rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0)) * dy,
    ),
    (dx, 0.0),
    (dx, rng.gen_range(0.0..1.0) * dy),
    (0.0, dy),
  ] {
    route.push(v);
    route2.push((-v.0, v.1));
  }
  route2.reverse();
  route.extend(route2);

  route = route_translate_rotate(&route, origin, angle);
  let polygons = vec![route.clone()];
  routes.push((clr, route));

  let tick = rng.gen_range(0.2..0.3);
  let y = rng.gen_range(-0.2..0.0) * dy;
  routes.push((
    clr,
    route_translate_rotate(
      &vec![(0.0, -tick * dy + y), (tick * dx, y), (0.0, tick * dy + y)],
      origin,
      angle,
    ),
  ));

  regular_clip_polys(&routes, paint, &polygons)
}
