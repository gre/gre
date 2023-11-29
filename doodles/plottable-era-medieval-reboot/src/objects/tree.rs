use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  paintmask::PaintMask,
  polygon::make_wireframe_from_vertexes,
  polylines::{path_to_fibers, Polylines},
  renderable::Renderable,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Tree {
  pub origin: (f32, f32),
  pub size: f32,
  pub trunk_routes: Polylines,
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub trunk_polys: Vec<Vec<(f32, f32)>>,
  pub inner_crop_polys: Vec<Vec<(f32, f32)>>,
}

impl Tree {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    clr: usize,
    foliage_ratio: f32,
    bush_width_ratio: f32,
    trunk_fill_each: f32,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];
    let mut inner_crop_polys = vec![];

    let bushcy = origin.1 - size * (1. - foliage_ratio);

    let w = size * 0.1;
    let wmin = 0.4 * w;
    let trunkp = vec![
      (origin.0, bushcy),
      (
        origin.0 + rng.gen_range(-0.3..0.3) * w,
        mix(origin.1, bushcy, rng.gen_range(0.5..0.8)),
      ),
      origin,
    ];
    let widths = vec![w, rng.gen_range(wmin..w), w];
    let count = 2 + (w / trunk_fill_each) as usize;
    let trunks = path_to_fibers(&trunkp, &widths, count);
    let trunk_polys =
      make_wireframe_from_vertexes(&trunks[0], &trunks[count - 1]);

    let count_poly = (size / 10.0) as usize + 3;

    for _i in 0..count_poly {
      let amp = rng.gen_range(0.2..0.5) * foliage_ratio * size;
      let delta = foliage_ratio * size - amp;
      let o = (
        origin.0 + rng.gen_range(-0.5..0.5) * rng.gen_range(0.5..1.0) * delta,
        bushcy,
      );
      let mut poly = vec![];
      let mut inner_crop_poly = vec![];
      let count = rng.gen_range(3..12);
      let ampmin = amp * rng.gen_range(0.6..0.9);
      let ampmax = rng.gen_range(ampmin..(1.1 * amp));
      for j in 0..count {
        let a =
          (j as f32 + rng.gen_range(-0.5..0.5)) * 2.0 * PI / (count as f32);
        let m = rng.gen_range(ampmin..ampmax);
        let x = o.0 + bush_width_ratio * m * a.cos();
        let y = o.1 + m * a.sin();
        poly.push((x, y));

        let m = m * rng.gen_range(0.3..0.9);
        let x = o.0 + bush_width_ratio * m * a.cos();
        let y = o.1 + m * a.sin();
        inner_crop_poly.push((x, y));
      }
      poly.push(poly[0]);
      routes.push((clr, poly.clone()));
      polys.push(poly);
      inner_crop_polys.push(inner_crop_poly);
    }

    let mut trunk_routes = vec![];
    for t in trunks {
      trunk_routes.push((clr, t));
    }

    Self {
      origin,
      size,
      routes,
      polys,
      trunk_routes,
      trunk_polys,
      inner_crop_polys,
    }
  }
  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    for poly in self.inner_crop_polys.iter() {
      paint.paint_polygon(poly);
    }

    let mut routes = regular_clip(&self.routes, paint);
    for poly in self.polys.iter() {
      paint.paint_polygon(poly);
    }

    routes.extend(regular_clip(&self.trunk_routes, paint));
    for poly in self.trunk_polys.iter() {
      paint.paint_polygon(poly);
    }

    routes
  }
}

impl<R: Rng> Renderable<R> for Tree {
  fn render(&self, _rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(paint)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
