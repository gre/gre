use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{route_translate_rotate, translate_rotate, Polylines},
  renderable::Renderable,
};
use rand::prelude::*;

use super::chineseroof::ChineseRoof;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub struct ChineseDoor {
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub roof: ChineseRoof,
  pub origin: (f32, f32),
}

impl ChineseDoor {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    w: f32,
    h: f32,
    angle: f32,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];

    let roofh = rng.gen_range(0.4..0.6) * h;

    let colw = 0.05;
    let colwmul = rng.gen_range(0.45..0.5);
    for xi in 0..2 {
      let x = (xi as f32 - 0.5) * w * colwmul;
      let rect = vec![
        (x - colw * w, -h),
        (x - colw * w, 0.0),
        (x + colw * w, 0.0),
        (x + colw * w, -h),
        (x - colw * w, -h),
      ];
      let rect = route_translate_rotate(&rect, origin, -angle);
      routes.push((clr, rect.clone()));
      polys.push(rect);
    }

    let o = translate_rotate((0.0, -h), origin, -angle);
    let roof = ChineseRoof::init(rng, clr, o, w, roofh, angle);

    Self {
      routes,
      polys,
      roof,
      origin,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut out = vec![];

    out.extend(self.roof.render(paint));

    out.extend(regular_clip(&self.routes, paint));
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }

    out
  }
}

impl<R: Rng> Renderable<R> for ChineseDoor {
  fn render(&self, _rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(paint)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
