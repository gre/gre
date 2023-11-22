use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::Polylines,
  shapes::{circle_route, spiral_optimized},
};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Moon {
  pub routes: Polylines,
  pub phase: f32,
  pub origin: (f32, f32),
  pub radius: f32,
  pub clr: usize,
}

impl Moon {
  pub fn init(
    clr: usize,
    origin: (f32, f32),
    radius: f32,
    dr: f32,
    phase: f32,
  ) -> Self {
    let routes = vec![
      (clr, spiral_optimized(origin.0, origin.1, radius, dr, 0.1)),
      (
        clr,
        circle_route(origin, radius, (radius * 2. + 8.) as usize),
      ),
    ];
    Self {
      routes,
      origin,
      radius,
      phase,
      clr,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let origin = self.origin;
    let radius = self.radius;

    let mut clone = paint.clone();
    clone.paint_circle(
      origin.0 + (self.phase - 0.5) * radius * 4.0,
      origin.1,
      radius,
    );

    let routes = regular_clip(&self.routes, &mut clone);

    for (_, r) in &routes {
      paint.paint_polyline(r, 2.0);
    }

    routes
  }
}
