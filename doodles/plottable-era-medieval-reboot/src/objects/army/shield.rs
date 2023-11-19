use crate::{
  algo::{
    clipping::regular_clip_polys, paintmask::PaintMask,
    polygon::polygons_includes_point, polylines::route_translate_rotate,
  },
  objects::blazon::Blazon,
};
use rand::prelude::*;

pub struct Shield {
  routes: Vec<(usize, Vec<(f32, f32)>)>,
  polygons: Vec<Vec<(f32, f32)>>,
}

impl Shield {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    blazon: Blazon, // TODO
  ) -> Self {
    let mx = if xflip { -1.0 } else { 1.0 };
    let mut routes = Vec::new();
    let dx = 0.2 * size;
    let dy = 0.4 * size;
    let mut route = vec![];
    let mut route2 = vec![];
    for v in vec![
      (0.0, -dy),
      (mx * 0.5 * dx, -dy),
      (
        mx * dx,
        -(1.0 - rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0)) * dy,
      ),
      (mx * dx, 0.0),
      (mx * dx, rng.gen_range(0.0..1.0) * dy),
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

  #[deprecated = "we should use a paint instead"]
  pub fn includes_point(&self, point: (f32, f32)) -> bool {
    polygons_includes_point(&self.polygons, point)
  }
}
