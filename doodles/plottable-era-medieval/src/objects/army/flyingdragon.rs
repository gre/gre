use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  paintmask::PaintMask,
  polylines::{step_polyline, Polyline, Polylines},
  shapes::circle_route_angleoff,
};
use rand::prelude::*;

use super::dragonhead::DragonHead;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct FlyingDragon {
  pub circles: Vec<Polyline>,
  pub dragonhead: Option<DragonHead>,
  pub clr: usize,
}

impl FlyingDragon {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    path: &Vec<(f32, f32)>,
    size: f32,
    step: f32,
    circle_poly_count: usize,
    angleoffset: f32,
  ) -> Self {
    let mut circles = vec![];

    let barebone = step_polyline(&path, step);
    let blen = barebone.len();
    let l = blen as f32 * step;

    let dragonhead = if blen > 0 {
      let mut params: Option<((f32, f32), f32, usize, f32)> = None;
      for i in 0..blen - 1 {
        let o = barebone[i];
        let f = (i as f32) / (l as f32 - 2.0);
        let r = size * 0.5 * mix(0.0, 1.0, f.powf(0.5));
        let angleoff = i as f32 * angleoffset;
        let c = circle_route_angleoff(o, r, circle_poly_count, angleoff);
        if i == blen - 2 {
          params = Some((o, r, circle_poly_count, angleoff));
        }
        circles.push(c.clone());
      }

      let o = barebone[blen - 1];
      let (angle, yflip) = if blen > 1 {
        let p = barebone[blen - 2];
        let yflip = o.0 > p.0;
        let a = (p.1 - o.1).atan2(p.0 - o.0);
        (a, yflip)
      } else {
        (0.0, false)
      };

      let mut texture = vec![];

      if let Some((o, initr, circle_poly_count, angleoff)) = params {
        let mut r = initr;
        let mut aoff = angleoff;
        let diff = step;
        let max = 2.0 * size;
        while r < max {
          r += diff;
          aoff += angleoffset;
          let c = circle_route_angleoff(o, r, circle_poly_count, aoff);
          texture.push((clr, c));
        }
      }

      Some(DragonHead::init(
        rng,
        clr,
        o,
        0.9 * size,
        angle,
        yflip,
        &texture,
      ))
    } else {
      None
    };

    Self {
      circles,
      dragonhead,
      clr,
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut out = vec![];
    // ideally we should .rev() but it doesn't help on the head area
    for c in self.circles.iter() {
      out.extend(regular_clip(&vec![(self.clr, c.clone())], paint));
      paint.paint_polygon(c);
    }
    if let Some(dragonhead) = &self.dragonhead {
      out.extend(dragonhead.render(paint));
    }
    // halo
    for (_, p) in &out {
      paint.paint_polyline(p, 2.0);
    }
    out
  }
}
