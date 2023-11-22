use crate::algo::{
  clipping::regular_clip,
  math2d::lerp_point,
  paintmask::PaintMask,
  polylines::{
    path_subdivide_to_curve_it, route_translate_rotate, shake, Polylines,
  },
  shapes::{circle_route, spiral_optimized_with_initial_angle},
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct BelierHead {
  pub routesbg: Polylines,
  pub routes: Polylines,
  pub polysbg: Vec<Vec<(f32, f32)>>,
  pub polys: Vec<Vec<(f32, f32)>>,
}

impl BelierHead {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
  ) -> Self {
    let flipx = |p: (f32, f32)| {
      // TODO we should use the angle to flip the x correctly but not a problem for now.
      let mut dx = p.0 - origin.0;
      if xflip {
        dx = -dx;
      }
      (origin.0 + dx, p.1)
    };
    let mapflipx =
      |route: Vec<(f32, f32)>| route.into_iter().map(flipx).collect::<Vec<_>>();

    let mut routes = vec![];
    let mut routesbg = vec![];
    let mut polysbg = vec![];
    let mut polys = vec![];

    let asin = angle.sin();
    let acos = angle.cos();

    let a2 = angle + PI / 2.0;
    let a2sin = a2.sin();
    let a2cos = a2.cos();

    // belier spiral
    let spiralxoff = 0.1 * size;
    let spiraloff = -0.3 * size;
    let spiralrad = 0.5 * size;

    let o = (
      origin.0 + spiralxoff * acos + spiraloff * a2cos,
      origin.1 + spiralxoff * asin + spiraloff * a2sin,
    );
    let dr = (spiralrad * rng.gen_range(0.3..0.5)).max(0.7);
    let initial = angle + PI / 4.0;
    routes.push((
      clr,
      mapflipx(spiral_optimized_with_initial_angle(
        o.0, o.1, spiralrad, initial, dr, 0.1, true,
      )),
    ));
    let circle_poly = mapflipx(circle_route(o, spiralrad, 20));
    polys.push(circle_poly);

    // belier head
    let headoff = 0.2 * size;
    let o = (origin.0 + headoff * a2cos, origin.1 + headoff * a2sin);
    let w = rng.gen_range(0.4..0.5) * size;
    let h = 0.25 * size;
    let poly = vec![
      //
      (-1.5 * w, -h),
      (-w, h),
      (0., 1.5 * h),
      (w, h),
      (w, -h),
    ];
    let poly = shake(poly, rng.gen_range(0.0..0.1) * size, rng);
    let a = -angle + PI * rng.gen_range(0.25..0.45);
    let poly = mapflipx(route_translate_rotate(&poly, o, a));
    let p1 = lerp_point(poly[0], poly[1], 0.5);
    let p2 = lerp_point(poly[3], poly[4], 0.5);
    routesbg.push((clr, vec![lerp_point(p1, p2, 0.2), p1]));
    routes.push((clr, circle_route(lerp_point(p1, p2, 0.5), 0.05 * size, 8)));
    let mut poly = path_subdivide_to_curve_it(&poly, 0.8);
    poly.push(poly[0]);
    routesbg.push((clr, poly.clone()));
    polysbg.push(poly);

    /*
    let o = origin;
    let mut route = vec![];
    let count = 5 + (size * 0.8) as usize;
    for _i in 0..count {
      let angle = rng.gen_range(-PI..PI);
      let amp = rng.gen_range(0.2..0.5) * size;
      route.push((o.0 + amp * angle.cos(), o.1 + amp * angle.sin()));
    }
    route.push(route[0]);
    routes.push((clr, route));
    */

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
