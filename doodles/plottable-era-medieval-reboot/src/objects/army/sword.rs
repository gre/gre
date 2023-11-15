use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{grow_stroke_zigzag, route_translate_rotate},
};
use rand::prelude::*;

pub fn sword<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  origin: (f32, f32),
  size: f32,
  angle: f32,
  clr: usize,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes: Vec<Vec<(f32, f32)>> = Vec::new();

  let sword_len = rng.gen_range(0.8..1.2) * size;
  let handle_len = 0.12 * size;
  let handle_w = 0.06 * size;
  let hilt_size = 0.2 * size;
  let hilt_w = 0.05 * size;
  let blade_w = 0.08 * size;

  // draw the swords: =||>==--

  let line_dist = 0.3;

  routes.push(grow_stroke_zigzag(
    (0.0, 0.0),
    (handle_len, 0.0),
    handle_w,
    line_dist,
  ));

  routes.push(grow_stroke_zigzag(
    (handle_len, -hilt_size / 2.0),
    (handle_len, hilt_size / 2.0),
    hilt_w,
    line_dist,
  ));

  let mut route = Vec::new();
  route.push((0.0, -blade_w / 2.0));
  route.push((sword_len, 0.0));
  route.push((0.0, blade_w / 2.0));
  routes.push(route);

  // translate routes
  regular_clip(
    &routes
      .iter()
      .map(|route| (clr, route_translate_rotate(&route, origin, angle)))
      .collect(),
    paint,
  )
}
