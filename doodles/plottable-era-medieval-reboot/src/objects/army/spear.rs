use crate::algo::{
  clipping::regular_clip,
  paintmask::PaintMask,
  polylines::{grow_as_rectangle, route_translate_rotate, Polylines},
  renderable::Renderable,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub struct Spear {
  pub routes: Polylines,
  pub polys: Vec<Vec<(f32, f32)>>,
  pub origin: (f32, f32),
}

impl Spear {
  pub fn init(
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    spike: bool,
  ) -> Self {
    let mut routes = vec![];
    let mut polys = vec![];

    let spear_len = size;
    let spear_w = 0.03 * size;
    let blade_w = 0.07 * size;
    let blade_len = 0.15 * size;
    let stick = grow_as_rectangle(
      (-spear_len / 2.0, 0.0),
      (spear_len / 2.0, 0.0),
      spear_w / 2.0,
    );
    let stick = route_translate_rotate(&stick, origin, -angle);
    polys.push(stick.clone());
    routes.push((clr, stick));

    if spike {
      let mut head: Vec<(f32, f32)> = Vec::new();
      head.push((spear_len / 2.0, -blade_w / 2.0));
      head.push((spear_len / 2.0 + blade_len, 0.0));
      head.push((spear_len / 2.0, blade_w / 2.0));
      head.push(head[0]);
      let head = route_translate_rotate(&head, origin, -angle);
      polys.push(head.clone());
      routes.push((clr, head));
    }

    Self {
      routes,
      polys,
      origin,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for poly in &self.polys {
      paint.paint_polygon(poly);
    }
    routes
  }
}

impl<R: Rng> Renderable<R> for Spear {
  fn render(&self, _rng: &mut R, paint: &mut PaintMask) -> Polylines {
    self.render(paint)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
