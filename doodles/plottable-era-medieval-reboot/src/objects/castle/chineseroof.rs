use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{path_subdivide_to_curve, route_translate_rotate, Polylines},
};
use rand::prelude::*;

pub struct ChineseRoof {
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
}

impl ChineseRoof {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    w: f32,
    h: f32,
    angle: f32,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];

    let x0 = -0.5 * w;
    let x1 = -0.3 * w;
    let x2 = -0.2 * w;
    let x3 = -0.1 * w;
    let x4 = 0.0;
    let y0 = 0.0;
    let y1 = -0.1 * h;
    let y2 = -0.2 * h;
    let y3 = -rng.gen_range(0.2..0.4) * h;
    let ymax2 = -0.5 * h;
    let ymax1 = -h;

    let mut route = vec![];

    route.push((x1, y0));
    route.push((x0, y1));
    route.push((x0, ymax1));
    route.push((x1, y2));
    route.push((x2, y2));
    route.push((x2, ymax2));
    route.push((x3, y3));
    route.push((x4, y3));

    route.extend(route.iter().rev().map(|p| (-p.0, p.1)).collect::<Vec<_>>());

    route = route_translate_rotate(&route, origin, -angle);

    route = path_subdivide_to_curve(&route, 1, 0.8);

    route.push(route[0]);

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
