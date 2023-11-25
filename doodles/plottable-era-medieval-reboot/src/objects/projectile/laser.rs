use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{path_to_fibers, Polylines},
};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Laser {
  pub routes: Polylines,
  pub clr: usize,
}

impl Laser {
  pub fn init(clr: usize, path: Vec<(f32, f32)>, size: f32) -> Self {
    let mut routes = Polylines::new();
    let count = (size / 0.5) as usize;
    for p in path_to_fibers(&path, &vec![size, 0.8 * size], count) {
      routes.push((clr, p));
    }
    Self { routes, clr }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Vec<(usize, Vec<(f32, f32)>)> {
    let routes = regular_clip(&self.routes, paint);
    for (_, route) in &routes {
      paint.paint_polyline(route, 0.5);
    }

    routes
  }
}
