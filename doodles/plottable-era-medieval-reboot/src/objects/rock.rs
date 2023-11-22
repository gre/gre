use std::f32::consts::PI;

use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, polygon::polygons_find_miny,
  polylines::Polylines,
};
use rand::prelude::*;

use super::army::sword::Sword;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Rock {
  pub origin: (f32, f32),
  pub size: f32,
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub inner_crop_polys: Vec<Vec<(f32, f32)>>,
  pub sword: Option<Sword>,
}

impl Rock {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    clr: usize,
    count_poly: usize,
    elevation: f32,
    excalibur: bool,
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
      let count = rng.gen_range(5..10);
      let ampmin = amp * rng.gen_range(0.3..0.8);
      let ampmax = rng.gen_range(ampmin..amp);
      for j in 0..count {
        let a =
          (j as f32 + rng.gen_range(-0.5..0.5)) * 2.0 * PI / (count as f32);
        // FIXME yes there is a bug xD we're remultiply by amp. but the generator worked with that assumption so it's hard to fix now...
        let m = amp * rng.gen_range(ampmin..ampmax);
        let x = o.0 + m * a.cos();
        let y = (o.1 + m * a.sin()).min(origin.1);
        poly.push((x, y));

        let m = m * rng.gen_range(0.3..0.9);
        let x = o.0 + m * a.cos();
        let y = (o.1 + m * a.sin()).min(origin.1);
        inner_crop_poly.push((x, y));
      }
      poly.push(poly[0]);
      routes.push((clr, poly.clone()));
      polys.push(poly);
      inner_crop_polys.push(inner_crop_poly);
    }

    let sword = if excalibur {
      if let Some(y) = polygons_find_miny(&polys, origin.0) {
        let s = 2.0 * size;
        let o = (origin.0, y - rng.gen_range(0.5..0.7) * s);
        let a = -PI / 2.0;
        let clr = rng.gen_range(0..2);
        Some(Sword::init(rng, o, s, a, clr))
      } else {
        None
      }
    } else {
      None
    };

    Self {
      origin,
      size,
      routes,
      polys,
      inner_crop_polys,
      sword,
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

    if let Some(sword) = &self.sword {
      let rts = sword.render(paint);
      // halo around the sword
      for (_, route) in &rts {
        paint.paint_polyline(route, 2.0);
      }
      routes.extend(rts);
    }

    routes
  }
}
