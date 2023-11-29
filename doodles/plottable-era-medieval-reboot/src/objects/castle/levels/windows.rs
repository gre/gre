use crate::algo::{
  clipping::clip_routes_with_colors,
  math1d::mix,
  math2d::{euclidian_dist, lerp_point},
  polygon::{polygon_centroid, polygon_includes_point},
  polylines::{path_subdivide_to_curve, Polyline},
  shapes::ovale_route,
};
use rand::prelude::*;

use super::RenderItem;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub enum WindowShape {
  Cross,
  CircleCross,
  Rectangle(f32),
  HalfCurve(f32),
  SquareCross,
}
impl WindowShape {
  pub fn rand<R: Rng>(rng: &mut R) -> WindowShape {
    let i = rng.gen_range(0.0..5.0) * rng.gen_range(0.3..1.0);
    match i as usize {
      0 => WindowShape::Rectangle(rng.gen_range(0.5..1.0)),
      1 => WindowShape::Cross,
      2 => WindowShape::HalfCurve(rng.gen_range(0.0..1.0)),
      3 => WindowShape::CircleCross,
      _ => WindowShape::SquareCross,
    }
  }

  pub fn width(&self, size: f32) -> f32 {
    match self {
      WindowShape::Rectangle(r) => r * size,
      WindowShape::Cross => 0.8 * size,
      WindowShape::HalfCurve(_) => 0.6 * size,
      WindowShape::CircleCross => 1.0 * size,
      WindowShape::SquareCross => 1.0 * size,
    }
  }

  pub fn render(
    &self,
    o: (f32, f32),
    clr: usize,
    size: f32,
    zindex: f32,
    xratio: f32,
  ) -> Vec<RenderItem> {
    let mut items = vec![];
    let mut routes = vec![];

    let h = size;
    let w = size * xratio;

    match self {
      WindowShape::Rectangle(r) => {
        let poly = vec![
          (o.0 - 0.5 * r * w, o.1 - 0.5 * h),
          (o.0 + 0.5 * r * w, o.1 - 0.5 * h),
          (o.0 + 0.5 * r * w, o.1 + 0.5 * h),
          (o.0 - 0.5 * r * w, o.1 + 0.5 * h),
          (o.0 - 0.5 * r * w, o.1 - 0.5 * h),
        ];
        routes.push((clr, poly.clone()));
        items.push(RenderItem::new(routes, vec![poly], zindex));
      }
      WindowShape::Cross => {
        let poly = cross(o, 0.35 * w, 0.5 * h, 0.08 * h);
        routes.push((clr, poly.clone()));
        items.push(RenderItem::new(routes, vec![poly], zindex));
      }
      WindowShape::CircleCross => {
        let circle = ovale_route(o, (0.5 * w, 0.5 * h), 24);
        routes.push((clr, circle.clone()));
        routes.push((clr, cross(o, 0.5 * w, 0.5 * h, 0.05 * h)));
        items.push(RenderItem::new(routes, vec![circle], zindex));
      }
      &WindowShape::HalfCurve(curvy) => {
        let w1 = 0.3 * w;
        let h1 = 0.5 * h;
        let h2 = mix(0.0, -0.5, curvy) * h;

        let mut poly =
          vec![(o.0 - w1, o.1 + h1), (o.0 - w1, o.1 + h2), (o.0, o.1 - h1)];
        poly = path_subdivide_to_curve(&poly, 2, 0.7);
        poly.extend(
          poly
            .iter()
            .rev()
            .map(|&(x, y)| (2. * o.0 - x, y))
            .collect::<Vec<_>>(),
        );
        poly.push(poly[0]);

        routes.push((clr, poly.clone()));
        items.push(RenderItem::new(routes, vec![poly], zindex));
      }
      WindowShape::SquareCross => {
        let poly = vec![
          (o.0 - 0.5 * w, o.1 - 0.5 * h),
          (o.0 + 0.5 * w, o.1 - 0.5 * h),
          (o.0 + 0.5 * w, o.1 + 0.5 * h),
          (o.0 - 0.5 * w, o.1 + 0.5 * h),
          (o.0 - 0.5 * w, o.1 - 0.5 * h),
        ];
        items.push(RenderItem::new(
          vec![
            (clr, poly.clone()),
            (clr, cross(o, 0.5 * h, 0.5 * w, 0.05 * h)),
          ],
          vec![poly.clone()],
          zindex,
        ));
      }
    }

    items
  }
}
pub struct WallWindowParams {
  pub top: f32,
  pub bottom: f32,
  pub pad: f32,
  pub padside: f32,
  pub size: f32,
  pub shape: WindowShape,
  pub columns: usize,
  pub max_rows: usize,
  pub quinconce: bool,
}
impl WallWindowParams {
  pub fn init<R: Rng>(rng: &mut R, scale: f32, available_width: f32) -> Self {
    let size = 2.5 + rng.gen_range(0.0..5.0) * rng.gen_range(0.0..1.0);
    let pad = size * (1.1 + rng.gen_range(0.0..2.0));
    let shape = WindowShape::rand(rng);
    let top = size * rng.gen_range(0.6..2.0);
    let bottom = size * rng.gen_range(1.0..4.0);
    let padside = size * rng.gen_range(1.5..2.5);
    let max_column =
      (available_width - 2.0 * padside) / (1.2 * shape.width(size));
    let mut columns =
      rng.gen_range(-0.01..max_column.max(0.0)).round().max(1.0) as usize;
    if columns > 3 && columns % 2 == 0 && rng.gen_bool(0.8) {
      columns -= 1;
    }
    let quinconce = rng.gen_bool(0.3) || rng.gen_bool(0.8) && columns % 2 == 1;
    let ratio = (available_width / (scale * 24.0)).max(0.1).powf(2.0);
    let max_rows = (1.0 / ratio + rng.gen_range(0.0..1.0)) as usize;
    Self {
      top,
      bottom,
      padside,
      pad,
      size,
      shape,
      columns,
      max_rows,
      quinconce,
    }
  }
}

pub fn wall_windows(
  params: &WallWindowParams,
  clr: usize,
  zindex: f32,
  polygon: &Vec<(f32, f32)>,
  ratio: f32,
) -> Vec<RenderItem> {
  let mut items = vec![];
  let mut miny = f32::INFINITY;
  let mut maxy = -f32::INFINITY;
  let mut minx = f32::INFINITY;
  let mut maxx = -f32::INFINITY;
  for p in polygon {
    miny = miny.min(p.1);
    maxy = maxy.max(p.1);
    minx = minx.min(p.0);
    maxx = maxx.max(p.0);
  }

  let center = polygon_centroid(&polygon);

  for (clr, route) in clip_routes_with_colors(
    &vec![(clr, vec![(center.0, miny), (center.0, maxy)])],
    &|p| !polygon_includes_point(polygon, p),
    1.0,
    3,
  ) {
    let from = route[0];
    let to = route[route.len() - 1];
    let l = euclidian_dist(from, to);
    let mut d = params.top;
    let maxd = l - params.size - params.bottom;
    let mut i = 0;
    while d < maxd && i < params.max_rows {
      let a = lerp_point(from, to, d / l);
      let b = lerp_point(from, to, (d + params.size) / l);
      let origin = lerp_point(a, b, 0.5);

      let mut xleft = origin.0;
      let mut xright = origin.0;
      let incr = 0.5;
      while polygon_includes_point(polygon, (xleft - incr, origin.1)) {
        xleft -= incr;
      }
      while polygon_includes_point(polygon, (xright + incr, origin.1)) {
        xright += incr;
      }
      xleft += params.padside;
      xright -= params.padside;

      let cols = params.columns;
      let quinconce = params.quinconce;
      for c in 0..cols {
        if !quinconce && i % 2 == 0 || quinconce && c % 2 == i % 2 {
          let m = if cols == 1 {
            0.5
          } else {
            c as f32 / (cols - 1) as f32
          };
          let o = lerp_point((xleft, origin.1), (xright, origin.1), m);
          items.extend(params.shape.render(o, clr, params.size, zindex, ratio));
        }
      }
      /*
      if i % 2 == 0 {
        items.extend(params.shape.render(
          origin,
          clr,
          params.size,
          zindex,
          ratio,
        ));
      }
      */
      d += (params.size + params.pad) / 2.0;
      i += 1;
    }
  }

  // TODO quinconces

  items
}

pub fn cross(
  o: (f32, f32),
  // half the width
  w: f32,
  // half the height
  h: f32,
  // half the thickness
  s: f32,
) -> Polyline {
  vec![
    (o.0 - w, o.1 - s),
    (o.0 - w, o.1 + s),
    (o.0 - s, o.1 + s),
    (o.0 - s, o.1 + h),
    (o.0 + s, o.1 + h),
    (o.0 + s, o.1 + s),
    (o.0 + w, o.1 + s),
    (o.0 + w, o.1 - s),
    (o.0 + s, o.1 - s),
    (o.0 + s, o.1 - h),
    (o.0 - s, o.1 - h),
    (o.0 - s, o.1 - s),
    (o.0 - w, o.1 - s),
  ]
}
