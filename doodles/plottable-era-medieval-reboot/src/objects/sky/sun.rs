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

pub struct Sun {
  pub routes: Polylines,
  pub spiralrays: Option<f32>,
  pub origin: (f32, f32),
  pub radius: f32,
  pub clr: usize,
  pub prec: f32,
}

impl Sun {
  pub fn init(
    clr: usize,
    origin: (f32, f32),
    radius: f32,
    dr: f32,
    spiralrays: Option<f32>,
  ) -> Self {
    let prec_sun = 0.5;
    let routes = vec![
      (
        clr,
        spiral_optimized(origin.0, origin.1, radius, dr, prec_sun),
      ),
      (
        clr,
        circle_route(origin, radius, (radius * 2. + 8.) as usize),
      ),
    ];
    let prec = 4.0;
    Self {
      routes,
      origin,
      radius,
      spiralrays,
      clr,
      prec,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let origin = self.origin;
    let radius = self.radius;
    let mut routes = regular_clip(&self.routes, paint);
    paint.paint_circle(origin.0, origin.1, radius);
    if let Some(dr) = self.spiralrays {
      let rt = spiral_optimized(
        origin.0,
        origin.1,
        paint.height.max(paint.width),
        dr,
        self.prec,
      );
      let rts = vec![(self.clr, rt)];
      /*
      // exact clipping to avoid glitch with the 2d array
      let is_outside = |p: (f32, f32)| {
        let dx = p.0 - origin.0;
        let dy = p.1 - origin.1;
        dx * dx + dy * dy < radius * radius
      };
      let rts = clip_routes_with_colors(&rts, &is_outside, 0.3, 3);
      */
      routes.extend(regular_clip(&rts, paint));
    }
    routes
  }
}
