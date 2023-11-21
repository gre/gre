use crate::algo::{
  clipping::regular_clip_polys,
  paintmask::PaintMask,
  polylines::{grow_as_rectangle, grow_stroke_zigzag, route_translate_rotate},
};
use rand::prelude::*;

pub struct Sword {
  pub origin: (f32, f32),
  pub routes: Vec<(usize, Vec<(f32, f32)>)>,
  pub polys: Vec<Vec<(f32, f32)>>,
}

impl Sword {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    clr: usize,
  ) -> Self {
    let mut routes: Vec<Vec<(f32, f32)>> = vec![];
    let mut polys = vec![];

    let sword_len = rng.gen_range(0.8..1.2) * size;
    let handle_len = 0.12 * size;
    let handle_w = 0.06 * size;
    let hilt_size = 0.2 * size;
    let hilt_w = 0.05 * size;
    let blade_w = 0.08 * size;

    // draw the swords: =||>==--

    let line_dist = 0.45;

    routes.push(grow_stroke_zigzag(
      (0.0, 0.0),
      (handle_len, 0.0),
      handle_w,
      line_dist,
    ));
    let poly1 =
      grow_as_rectangle((0.0, 0.0), (handle_len, 0.0), handle_w / 2.0);
    let poly1 = route_translate_rotate(&poly1, origin, angle);
    polys.push(poly1);

    routes.push(grow_stroke_zigzag(
      (handle_len, -hilt_size / 2.0),
      (handle_len, hilt_size / 2.0),
      hilt_w,
      line_dist,
    ));
    let poly2 = grow_as_rectangle(
      (handle_len, -hilt_size / 2.0),
      (handle_len, hilt_size / 2.0),
      hilt_w / 2.0,
    );
    let poly2 = route_translate_rotate(&poly2, origin, angle);
    polys.push(poly2);

    let mut route = Vec::new();
    route.push((0.0, -blade_w / 2.0));
    route.push((sword_len, 0.0));
    route.push((0.0, blade_w / 2.0));
    let poly3 = route_translate_rotate(&route, origin, angle);
    polys.push(poly3);
    routes.push(route);

    // FIXME aren't we doing it twice?!

    let routes = routes
      .iter()
      .map(|route| (clr, route_translate_rotate(&route, origin, angle)))
      .collect::<Vec<_>>();

    Self {
      origin,
      routes,
      polys,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Vec<(usize, Vec<(f32, f32)>)> {
    regular_clip_polys(&self.routes, paint, &self.polys)
  }
}
