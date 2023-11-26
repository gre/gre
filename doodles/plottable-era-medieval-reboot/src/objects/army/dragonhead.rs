use crate::algo::{
  clipping::{clip_routes_with_colors, regular_clip},
  paintmask::PaintMask,
  polygon::polygon_includes_point,
  polylines::{
    path_subdivide_to_curve_it, route_scale_translate_rotate, shake, Polylines,
  },
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct DragonHead {
  pub routesbg: Polylines,
  pub routes: Polylines,
  pub polysbg: Vec<Vec<(f32, f32)>>,
  pub polys: Vec<Vec<(f32, f32)>>,
}

impl DragonHead {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    yflip: bool,
    texture: &Polylines,
  ) -> Self {
    let ymul = if yflip { -1.0 } else { 1.0 };
    let mut routes = vec![];
    let mut routesbg = vec![];
    let mut polysbg = vec![];
    let mut polys = vec![];

    let scale = size / 50.0;

    let shapes = vec![
      vec![
        (20, 0),
        (0, -27),
        (0, 29),
        (-22, 18),
        (-46, 44),
        (-53, 41),
        (-38, 13),
        (-46, 11),
        (-69, 28),
        (-75, 15),
        (-57, -7),
        (-36, -18),
        (0, -27),
        (20, 0),
      ],
      vec![(-52, 1), (-43, -4), (-46, 1), (-52, 1)],
    ];
    let shapes = shapes
      .iter()
      .map(|path| {
        route_scale_translate_rotate(
          &path
            .iter()
            .map(|&p| (p.0 as f32, ymul * p.1 as f32))
            .collect(),
          (scale, scale),
          origin,
          -angle,
        )
      })
      .collect::<Vec<_>>();

    let mut mainshape = shapes[0].clone();
    mainshape = path_subdivide_to_curve_it(&mainshape, 0.8);
    if size > 4.0 {
      let s = rng.gen_range(0.0..0.1) * rng.gen_range(0.0..1.0);
      mainshape = shake(mainshape, s * size, rng);
      mainshape = path_subdivide_to_curve_it(&mainshape, 0.7);
      if size > 10.0 {
        let s = rng.gen_range(0.0..0.1) * rng.gen_range(0.0..1.0);
        mainshape = shake(mainshape, s * size, rng);
        mainshape = path_subdivide_to_curve_it(&mainshape, 0.6);
      }
    }

    let eyeshape = &shapes[1];

    routesbg.push((clr, mainshape.clone()));
    polysbg.push(mainshape.clone());

    routes.push((clr, eyeshape.clone()));
    polys.push(eyeshape.clone());

    let is_outside = |p: (f32, f32)| !polygon_includes_point(&mainshape, p);
    routesbg.extend(clip_routes_with_colors(&texture, &is_outside, 1.0, 3));

    Self {
      routes,
      routesbg,
      polysbg,
      polys,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut out = vec![];
    out.extend(regular_clip(&self.routes, paint));
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }
    out.extend(regular_clip(&self.routesbg, paint));
    for poly in &self.polysbg {
      paint.paint_polygon(poly);
    }
    out
  }
}
