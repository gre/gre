use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  math2d::{lerp_point, polar_sort_from_center},
  paintmask::PaintMask,
  polylines::{path_subdivide_to_curve, shake, Polylines},
  renderable::Renderable,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Dog {
  pub routes: Polylines,
  pub origin: (f32, f32),
}

impl Dog {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    scale: f32,
    reversex: bool,
    barking: bool,
  ) -> Self {
    let mut routes = vec![];

    let pattes_bas_x = vec![
      rng.gen_range(-0.5..-0.3),
      rng.gen_range(-0.3..-0.1),
      rng.gen_range(-0.1..0.15),
      rng.gen_range(0.1..0.4),
    ];

    let ax = mix(pattes_bas_x[0], pattes_bas_x[1], 0.5);
    let bx = mix(pattes_bas_x[2], pattes_bas_x[3], 0.5);

    let pattes_haut_x = vec![
      mix(ax, pattes_bas_x[0], rng.gen_range(0.2..0.8)),
      mix(ax, pattes_bas_x[1], rng.gen_range(0.2..0.8)),
      mix(bx, pattes_bas_x[2], rng.gen_range(0.2..0.8)),
      mix(bx, pattes_bas_x[3], rng.gen_range(0.2..0.8)),
    ];

    let count = (rng.gen_range(0.5..1.0) * scale) as usize;
    let mut candidates: Vec<(f32, f32)> = (0..count)
      .map(|_i| {
        (
          rng.gen_range(-0.8..0.5) * rng.gen_range(0.5..1.0),
          rng.gen_range(-0.7..-0.2),
        )
      })
      .collect();
    let ybottom = 0.0;
    let ytop = -0.3;
    for i in 0..4 {
      let bottom = (pattes_bas_x[i], ybottom);
      let top = (pattes_haut_x[i], ytop);
      let p = path_subdivide_to_curve(&vec![bottom, top], 1, 0.8);
      let p = shake(p.clone(), 0.4 / scale, rng);
      routes.push(p);
      candidates.push(top);
    }

    let body1 = (rng.gen_range(0.4..0.8), rng.gen_range(-1.0..-0.8));
    let body2 = (rng.gen_range(0.4..0.8), rng.gen_range(-0.9..-0.6));
    candidates.push(body1);
    candidates.push(body2);

    if barking {
      let angbase = -1.5;
      let distr = rng.gen_range(0.0..0.5);
      let angamp = mix(1.0, 1.8, distr);
      let center = lerp_point(body1, body2, 0.5);
      let count = mix(3., 6., distr) as usize;
      for i in 0..count {
        let a = angbase + angamp * (i as f32 + 0.5) / (count as f32);
        let from = rng.gen_range(0.3..0.4);
        let to = from + rng.gen_range(0.4..0.6);
        routes.push(
          vec![from, to]
            .iter()
            .map(|&amp| {
              let x = center.0 + amp * a.cos();
              let y = center.1 + amp * a.sin();
              (x, y)
            })
            .collect(),
        );
      }
    }

    let mut route =
      path_subdivide_to_curve(&polar_sort_from_center(&candidates), 1, 0.8);
    route.push(route[0]);
    routes.push(path_subdivide_to_curve(
      &shake(route.clone(), 0.4 / scale, rng),
      1,
      0.8,
    ));

    let xmul = if reversex { -1.0 } else { 1.0 };

    // scale, rotate & translate
    let routes = routes
      .iter()
      .map(|route| {
        (
          clr,
          route
            .iter()
            .map(|&p| {
              let p = (scale * p.0 * xmul + origin.0, scale * p.1 + origin.1);
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

impl<R: Rng> Renderable<R> for Dog {
  fn render(&self, _rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(paint)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
