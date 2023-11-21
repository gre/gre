use crate::algo::{
  clipping::{clip_routes_with_colors, regular_clip},
  paintmask::PaintMask,
  polygon::polygon_includes_point,
  polylines::{route_scale_translate_rotate, shake, Polylines},
  shapes::{circle_route, spiral_optimized},
};
use rand::prelude::*;

pub struct Renault4L {
  pub routesbg: Polylines,
  pub routes: Polylines,
  pub polysbg: Vec<Vec<(f32, f32)>>,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub size: f32,
}

impl Renault4L {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
  ) -> Self {
    let mut routes = vec![];
    let mut routesbg = vec![];
    let mut polysbg = vec![];
    let mut polys = vec![];

    let scale = size / 50.0;

    let shapes = vec![
      vec![
        (-47, -40),
        (-8, -39),
        (-5, -35),
        (9, -34),
        (18, -20),
        (44, -18),
        (51, -12),
        (50, 0),
        (43, -8),
        (32, -9),
        (26, 0),
        (-23, 0),
        (-26, -7),
        (-42, -8),
        (-43, -2),
        (-51, -2),
        (-47, -40),
      ],
      vec![(-14, -31), (5, -31), (11, -21), (-14, -21), (-14, -31)],
      vec![(-34, 2), (38, 2)],
    ];
    let shapes = shapes
      .iter()
      .map(|path| {
        route_scale_translate_rotate(
          &shake(
            path.iter().map(|&p| (p.0 as f32, p.1 as f32)).collect(),
            1.0,
            rng,
          ),
          (scale, scale),
          origin,
          angle,
        )
      })
      .collect::<Vec<_>>();

    let circlepos = &shapes[2];
    let mainshape = &shapes[0];
    let windowshape = &shapes[1];
    let r = 8. * scale;
    let c0 = circlepos[0];
    let c1 = circlepos[1];
    let circle1 = circle_route(c0, r, 16);
    let circle2 = circle_route(c1, r, 16);
    let wheels = vec![
      (clr, circle1.clone()),
      (clr, spiral_optimized(c0.0, c0.1, r, 0.8, 0.1)),
      (clr, circle2.clone()),
      (clr, spiral_optimized(c1.0, c1.1, r, 0.8, 0.1)),
    ];

    let mut dy = 0.0;
    let dr = rng.gen_range(0.5..1.2);
    let mut lines = vec![];
    while dy < size {
      let y = origin.1 - dy;
      let x1 = origin.0 - size;
      let x2 = origin.0 + size;
      lines.push((clr, vec![(x1, y), (x2, y)]));
      dy += dr;
    }
    let is_outside = |p: (f32, f32)| !polygon_includes_point(&mainshape, p);
    routesbg.extend(clip_routes_with_colors(&lines, &is_outside, 1.0, 3));

    routes.push((clr, windowshape.clone()));
    polys.push(windowshape.clone());

    routesbg.extend(wheels);
    polysbg.push(circle1.clone());
    polysbg.push(circle2.clone());

    routesbg.push((clr, mainshape.clone()));
    polysbg.push(mainshape.clone());

    Self {
      routes,
      routesbg,
      polysbg,
      polys,
      size,
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
    for route in &self.polysbg {
      paint.paint_polyline(route, 0.05 * self.size);
    }
    out
  }
}
