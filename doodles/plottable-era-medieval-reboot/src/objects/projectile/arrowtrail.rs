use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, pathlookup::PathLookup,
};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct ArrowTrail {
  pub path: Vec<(f32, f32)>,
  pub clr: usize,
}

impl ArrowTrail {
  pub fn init(clr: usize, path: Vec<(f32, f32)>, percent: f32) -> Self {
    let mut path = path.clone();
    path.reverse();
    let lookup = PathLookup::init(path);
    let l = percent * lookup.length() as f32;
    let path = lookup.slice_before(l);
    Self { path, clr }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Vec<(usize, Vec<(f32, f32)>)> {
    let clr = self.clr;
    let mut routes = vec![];
    routes.push((clr, self.path.clone()));
    routes = regular_clip(&routes, paint);

    for (_, route) in &routes {
      paint.paint_polyline(route, 0.5);
    }

    routes
  }
}
