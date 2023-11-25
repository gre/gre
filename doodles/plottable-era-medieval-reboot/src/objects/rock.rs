use std::f32::consts::PI;

use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, polygon::polygons_find_miny,
  polylines::Polylines, renderable::Renderable, wormsfilling::WormsFilling,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Rock<R: Rng> {
  pub origin: (f32, f32),
  pub size: f32,
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub inner_crop_polys: Vec<Vec<(f32, f32)>>,
  pub top: Option<Box<dyn Renderable<R>>>,
  pub clr: usize,
}

impl<R: Rng> Rock<R> {
  pub fn init<
    F: FnMut(&mut R, (f32, f32), f32, f32) -> Option<Box<dyn Renderable<R>>>,
  >(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    clr: usize,
    count_poly: usize,
    elevation: f32,
    spawn_top: &mut F,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];
    let mut inner_crop_polys = vec![];

    for i in 0..count_poly {
      let amp = rng.gen_range(0.2..0.5)
        * size
        * (1.0 - i as f32 / (count_poly as f32 * 2.0));
      let delta = size - amp;
      let o = (
        origin.0 + rng.gen_range(-0.5..0.5) * delta,
        origin.1 - i as f32 * size * elevation / count_poly as f32,
      );
      let mut poly = vec![];
      let mut inner_crop_poly = vec![];
      let count = rng.gen_range(5..15);
      let ampmin = amp * rng.gen_range(0.3..0.8);
      let ampmax = rng.gen_range(ampmin..amp);
      for j in 0..count {
        let a =
          (j as f32 + rng.gen_range(-0.5..0.5)) * 2.0 * PI / (count as f32);
        // NB: yes there is a bug xD we're remultiply by amp. but the generator worked with that assumption so it's hard to fix now...
        let m = amp * rng.gen_range(ampmin..ampmax);
        let x = o.0 + m * a.cos();
        let y = (o.1 + m * a.sin()).min(origin.1);
        poly.push((x, y));

        let m = m * rng.gen_range(0.2..0.8);
        let x = o.0 + m * a.cos();
        let y = (o.1 + m * a.sin()).min(origin.1);
        inner_crop_poly.push((x, y));
      }
      poly.push(poly[0]);
      routes.push((clr, poly.clone()));
      polys.push(poly);
      inner_crop_polys.push(inner_crop_poly);
    }

    let top = if let Some(y) = polygons_find_miny(&polys, origin.0) {
      let s = 2.0 * size;
      let o = (origin.0, y - rng.gen_range(0.5..0.7) * s);
      let a = -PI / 2.0;
      let r = spawn_top(rng, o, s, a);
      r
    } else {
      None
    };

    Self {
      origin,
      size,
      routes,
      polys,
      inner_crop_polys,
      top,
      clr,
    }
  }
  pub fn render(&self, rng: &mut R, paint: &mut PaintMask) -> Polylines {
    for poly in self.inner_crop_polys.iter() {
      paint.paint_polygon(poly);
    }

    let routes = regular_clip(&self.routes, paint);
    let filling = WormsFilling::rand(rng);
    let mut drawing = paint.clone_empty();
    for (_, rt) in routes.iter() {
      let w = 0.4
        + rng.gen_range(0.0..3.0)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0);
      drawing.paint_polyline(rt, w);
      paint.paint_polyline(rt, w + 0.5);
    }
    let bound = drawing.painted_boundaries();
    let its = rng.gen_range(50..1000);
    let mut routes =
      filling.fill_in_paint(rng, &drawing, self.clr, 1.5, bound, its);
    for poly in self.polys.iter() {
      paint.paint_polygon(poly);
    }

    if let Some(o) = &self.top {
      let rts = o.as_ref().render(rng, paint);
      // halo around the sword
      for (_, route) in &rts {
        paint.paint_polyline(route, 2.0);
      }
      routes.extend(rts);
    }

    for poly in self.polys.iter() {
      paint.paint_polyline(poly, 1.2);
    }

    routes
  }
}

impl<R: Rng> Renderable<R> for Rock<R> {
  fn render(&self, rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(rng, paint)
  }
  fn yorder(&self) -> f32 {
    self.origin.1
  }
}
