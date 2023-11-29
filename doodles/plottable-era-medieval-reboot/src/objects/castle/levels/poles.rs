use rand::prelude::*;

use crate::algo::shapes::circle_route;

use super::RenderItem;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub enum PoleKind {
  Flag,
  Circle,
  Cross,
  CrossCircle,
}

impl PoleKind {
  pub fn rand<R: Rng>(rng: &mut R) -> Self {
    let i = rng.gen_range(0.0..4.0) * rng.gen_range(0.3..1.0);
    match i as usize {
      0 => Self::Flag,
      1 => Self::Cross,
      2 => Self::Circle,
      _ => Self::CrossCircle,
    }
  }

  pub fn render(
    &self,
    o: (f32, f32),
    clr: usize,
    size: f32,
    zindex: f32,
  ) -> Option<RenderItem> {
    let crossh = 1.8 * size;
    let crossw = 0.7 * size;

    match self {
      Self::Circle => {
        let r = size * 0.5;
        let center = (o.0, o.1 - r);
        let mut routes = vec![];
        let poly = circle_route(center, r, 32);
        routes.push((clr, poly.clone()));
        Some(RenderItem::new(routes, vec![poly], zindex))
      }
      Self::Cross => {
        let mut routes = vec![];
        routes.push((clr, vec![o, (o.0, o.1 - crossh)]));
        let y = o.1 - crossh + crossw;
        routes
          .push((clr, vec![(o.0 - crossw / 2.0, y), (o.0 + crossw / 2.0, y)]));
        Some(RenderItem::new(routes, vec![], zindex))
      }
      Self::CrossCircle => {
        let r = crossw / 2.0;
        let center = (o.0, o.1 - r);
        let mut routes = vec![];
        let poly = circle_route(center, r, 32);
        routes.push((clr, poly.clone()));
        let o = (o.0, o.1 - 2.0 * r);
        routes.push((clr, vec![o, (o.0, o.1 - crossh)]));
        let y = o.1 - crossh + crossw;
        routes
          .push((clr, vec![(o.0 - crossw / 2.0, y), (o.0 + crossw / 2.0, y)]));
        Some(RenderItem::new(routes, vec![poly], zindex))
      }
      _ => None,
    }
  }
}

#[derive(Clone)]
pub struct SpawnablePole {
  pub pos: (f32, f32),
  pub zorder: f32,
  pub size: f32,
  pub kind: PoleKind,
}

impl SpawnablePole {
  pub fn new(pos: (f32, f32), zorder: f32, size: f32, kind: PoleKind) -> Self {
    Self {
      pos,
      zorder,
      size,
      kind,
    }
  }
}
