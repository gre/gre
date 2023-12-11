use crate::{global::GlobalCtx, objects::army::human::Human};

use super::{paintmask::PaintMask, polylines::Polylines};
use rand::prelude::*;
use std::cmp::Ordering;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub trait Renderable<R: Rng> {
  fn render(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines;

  fn zorder(&self) -> f32;

  // fn as_any(&self) -> &dyn Any;
  fn as_human(&self) -> Option<&Human> {
    None
  }

  fn apply_translation_rotation(&mut self, _v: (f32, f32), _rot: f32) {}
}

struct RenderableYOrd<R: Rng> {
  inner: Box<dyn Renderable<R>>,
}

impl<R: Rng> PartialEq for RenderableYOrd<R> {
  fn eq(&self, other: &Self) -> bool {
    self.inner.zorder() == other.inner.zorder()
  }
}

impl<R: Rng> Eq for RenderableYOrd<R> {}

impl<R: Rng> PartialOrd for RenderableYOrd<R> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.inner.zorder().partial_cmp(&self.inner.zorder())
  }
}

impl<R: Rng> Ord for RenderableYOrd<R> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap_or(Ordering::Equal)
  }
}

pub fn as_box_renderable<R: Rng, T: Renderable<R> + 'static>(
  item: T,
) -> Box<dyn Renderable<R>> {
  Box::new(item) as Box<dyn Renderable<R>>
}

pub struct Container<R: Rng> {
  elements: Vec<RenderableYOrd<R>>,
}
impl<R: Rng> Container<R> {
  pub fn new() -> Self {
    Self { elements: vec![] }
  }
  pub fn add<T: Renderable<R> + 'static>(&mut self, item: T) {
    self.push(Box::new(item) as Box<dyn Renderable<R>>);
  }
  pub fn push(&mut self, inner: Box<dyn Renderable<R>>) {
    self.elements.push(RenderableYOrd { inner });
    self.elements.sort();
  }
  pub fn extend(&mut self, other: Container<R>) {
    self.elements.extend(other.elements);
    self.elements.sort();
  }

  /*
  // TODO a better impl would be to make humans returning two renderables and we would natively sort these, so we wouldn't need this function...
  pub fn render_with_humans(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = vec![];
    let mut human_routes = vec![];

    let human_halo = 0.8;

    for e in &self.elements {
      if let Some(human) = e.inner.as_human() {
        human_routes.extend(human.render_foreground_only(rng, paint));
      } else {
        routes.extend(e.inner.render(rng, ctx, paint));
      }
    }
    for (_, route) in &human_routes {
      paint.paint_polyline(route, human_halo);
    }
    routes.extend(human_routes);

    human_routes = vec![];
    for e in &self.elements {
      if let Some(human) = e.inner.as_human() {
        human_routes.extend(human.render_background_only(rng, paint));
      }
    }
    for (_, route) in &human_routes {
      paint.paint_polyline(route, human_halo);
    }
    routes.extend(human_routes);

    routes
  }
  */

  pub fn render_with_extra_halo(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
    halo: f32,
  ) -> Polylines {
    let mut routes = vec![];
    for e in &self.elements {
      let rts = e.inner.render(rng, ctx, paint);
      for (_, route) in &rts {
        paint.paint_polyline(route, halo);
      }
      routes.extend(rts);
    }
    routes
  }
}

impl<R: Rng> Renderable<R> for Container<R> {
  fn render(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = vec![];
    for e in &self.elements {
      routes.extend(e.inner.render(rng, ctx, paint));
    }
    routes
  }

  fn zorder(&self) -> f32 {
    self
      .elements
      .iter()
      .max()
      .map(|o| o.inner.zorder())
      .unwrap_or_default()
  }

  // kinda a hack for now..

  fn apply_translation_rotation(&mut self, pos: (f32, f32), angle: f32) {
    for e in &mut self.elements {
      e.inner.apply_translation_rotation(pos, angle);
    }
  }
}
