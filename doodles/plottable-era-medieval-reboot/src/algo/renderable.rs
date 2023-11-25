use super::{paintmask::PaintMask, polylines::Polylines};
use rand::prelude::*;
use std::cmp::Ordering;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub trait Renderable<R: Rng> {
  fn render(&self, rng: &mut R, paint: &mut PaintMask) -> Polylines;
  fn yorder(&self) -> f32;
}

// FIXME can a Renderable be an implicit Container? like it wants to emit sub renderable... because we want the human shields to be ordered separately.

struct RenderableYOrd<R: Rng> {
  inner: Box<dyn Renderable<R>>,
}

impl<R: Rng> PartialEq for RenderableYOrd<R> {
  fn eq(&self, other: &Self) -> bool {
    self.inner.yorder() == other.inner.yorder()
  }
}

impl<R: Rng> Eq for RenderableYOrd<R> {}

impl<R: Rng> PartialOrd for RenderableYOrd<R> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.inner.yorder().partial_cmp(&self.inner.yorder())
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
}

impl<R: Rng> Renderable<R> for Container<R> {
  fn render(&self, rng: &mut R, paint: &mut PaintMask) -> Polylines {
    let mut routes = vec![];
    for e in &self.elements {
      routes.extend(e.inner.render(rng, paint));
    }
    routes
  }
  fn yorder(&self) -> f32 {
    self
      .elements
      .iter()
      .max()
      .map(|o| o.inner.yorder())
      .unwrap_or_default()
  }
}
