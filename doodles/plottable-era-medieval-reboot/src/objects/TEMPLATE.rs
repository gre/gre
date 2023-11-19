use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, polylines::Polylines,
};
use rand::prelude::*;

pub struct NAME {
  pub routes: Polylines,
}

impl NAME {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];

    Self { routes, polys }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut routes = regular_clip(&self.routes, paint);
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }
    routes
  }
}
