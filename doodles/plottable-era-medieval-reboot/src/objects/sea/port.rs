use rand::prelude::*;

use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, polylines::Polylines,
  renderable::Renderable,
};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone, Copy)]
pub struct Port {
  pub origin: (f32, f32),
  pub size: f32,
  pub clr: usize,
}

impl Port {
  pub fn init(clr: usize, origin: (f32, f32), size: f32) -> Self {
    Self { origin, size, clr }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    clr: usize,
  ) -> Polylines {
    let size = self.size;
    let (ox, oy) = self.origin;
    let mut routes = Polylines::new();

    let from = ox - size / 2.0;
    let to = ox + size / 2.0;
    let mut x = from;
    let no_peak_area_x = ox + rng.gen_range(-1.0..1.0) * size;
    let no_peak_area_w = rng.gen_range(0.0..1.0) * size;
    let no_peak_area = no_peak_area_x..(no_peak_area_x + no_peak_area_w);
    let incr = rng.gen_range(0.5..1.0);
    let normalh = rng.gen_range(0.5..1.0);
    let peakh = rng.gen_range(4.0..7.0);
    let eachx = rng.gen_range(4.0..10.0);
    let peakw = rng.gen_range(0.8..1.8);
    let floor = rng.gen_range(1.0..2.0);
    let floor2 = rng.gen_range(1.0..5.0);
    let floor2w = size * rng.gen_range(0.3..0.9);
    let floor2x =
      ox + 0.9 * rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0) * size;
    let eachxfloor2 = rng.gen_range(2.0..10.0);
    let floor2wi = rng.gen_range(0.1..1.0) * eachxfloor2;
    let mut alt = false;
    while x < to {
      let up = if (x % eachx) < peakw && !no_peak_area.contains(&x) {
        peakh
      } else {
        normalh
      };
      let down = if 2.0 * (floor2x - x).abs() < floor2w
        && (x % eachxfloor2) < floor2wi
      {
        floor2
      } else {
        floor
      };
      let mut path = vec![(x, oy - up), (x, oy + down)];
      if alt {
        path.reverse();
      }
      routes.push((clr, path));
      x += incr;
      alt = !alt;
    }
    routes.push((clr, vec![(from, oy), (to, oy)]));
    routes.push((clr, vec![(from, oy - normalh), (to, oy - normalh)]));
    routes.push((clr, vec![(from, oy + floor), (to, oy + floor)]));

    routes = regular_clip(&routes, paint);

    for (_clr, route) in &routes {
      paint.paint_polyline(route, 0.8);
    }

    routes
  }
}

impl<R: Rng> Renderable<R> for Port {
  fn render(&self, rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(rng, paint, self.clr)
  }

  fn zorder(&self) -> f32 {
    self.origin.1 + self.size / 2.0
  }
}
