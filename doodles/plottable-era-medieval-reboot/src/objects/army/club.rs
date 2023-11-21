use crate::algo::{
  clipping::regular_clip_polys,
  paintmask::PaintMask,
  polylines::{path_subdivide_to_curve_it, route_translate_rotate},
};
use rand::prelude::*;

pub struct Club {
  pub origin: (f32, f32),
  pub routes: Vec<(usize, Vec<(f32, f32)>)>,
  pub polys: Vec<Vec<(f32, f32)>>,
}

impl Club {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    clr: usize,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];

    let l = rng.gen_range(0.6..1.0) * size;
    let w = 0.2 * size;

    let v0 = rng.gen_range(-0.2..0.0);
    let v1 = rng.gen_range(0.7..0.9);
    let v2 = rng.gen_range(0.4..0.6);
    let v3 = rng.gen_range(0.3..0.5);
    let v4 = rng.gen_range(0.0..0.2);

    let mut route = Vec::new();
    route.push((v0 * l, -v4 * w));
    route.push((v2 * l, -v3 * w));
    route.push((v1 * l, -w));
    route.push((l, 0.0));
    route.push((v1 * l, w));
    route.push((v2 * l, v3 * w));
    route.push((v0 * l, v4 * w));
    route.push((v0 * l, -v4 * w));
    route = path_subdivide_to_curve_it(&route, 0.8);

    let route = route_translate_rotate(&route, origin, angle);
    polys.push(route.clone());
    routes.push((clr, route));

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
