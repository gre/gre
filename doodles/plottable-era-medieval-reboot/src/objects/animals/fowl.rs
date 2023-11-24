use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  math2d::p_r,
  paintmask::PaintMask,
  polylines::{path_subdivide_to_curve, shake, Polylines},
  renderable::Renderable,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Fowl {
  pub routes: Polylines,
  pub origin: (f32, f32),
}

impl Fowl {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    scale: f32,
    rot: f32,
  ) -> Self {
    let mut routes = vec![];

    let mut lead = vec![];

    let amp = 0.1;
    for _i in 0..4 {
      lead.push((
        rng.gen_range(-1.0..1.0) * amp,
        -0.6 * rng.gen_range(-1.0..1.0) * amp,
      ));
    }
    let side = rng.gen_range(-1.0..1.0);
    lead.push((-0.3 * side, -0.6));
    lead.push((-0.6 * side, -0.8));
    lead.push((-0.8 * side, -1.0));

    let from = rng.gen_range(0.35..0.5);
    let to = rng.gen_range(0.1..0.25);
    let count = (scale * rng.gen_range(1.0..2.0)) as usize;
    let mut path = Vec::new();
    for (i, l) in lead.iter().enumerate() {
      let percent = i as f32 / ((lead.len() - 1) as f32);
      for _i in 0..count {
        let amp = mix(from, to, percent);
        path.push((
          l.0 + rng.gen_range(-1.0..1.0) * amp,
          l.1 + rng.gen_range(-1.0..1.0) * amp,
        ));
      }
    }
    routes.push(path_subdivide_to_curve(&path, 2, 0.7));

    for i in vec![-1.0, 1.0] {
      let mut path = Vec::new();
      for (amp, a) in vec![(0.0, 0.0), (0.6, 0.0), (0.6, 0.3 * i)] {
        let ang = PI / 2.0 + a + 0.2 * i;
        path.push((amp * ang.cos(), amp * ang.sin()));
      }
      for _i in 0..3 {
        routes.push(shake(path.clone(), 0.005 * scale, rng));
      }
    }

    let dy = 0.6;

    // scale, rotate & translate
    let routes = routes
      .iter()
      .map(|route| {
        (
          clr,
          route
            .iter()
            .map(|&p| {
              let mut p = p_r(p, rot);
              p = (scale * p.0 + origin.0, scale * (p.1 - dy) + origin.1);
              p
            })
            .collect(),
        )
      })
      .collect();

    Self { routes, origin }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for (_, poly) in &self.routes {
      paint.paint_polygon(poly); // TODO it would be best to have proper polygons
    }
    routes
  }
}

impl<R: Rng> Renderable<R> for Fowl {
  fn render(&self, _rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(paint)
  }
  fn yorder(&self) -> f32 {
    self.origin.1
  }
}
