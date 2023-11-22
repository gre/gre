use super::spear::Spear;
use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  paintmask::PaintMask,
  polylines::{path_to_fibers, Polylines},
};
use rand::prelude::*;
use std::f32::consts::PI;

pub struct Flag {
  pub spear: Spear,
  pub cloth: FlagCloth,
}

impl Flag {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    blazonclr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    flagtoright: bool,
    cloth_height_factor: f32,
    cloth_len_factor: f32,
    spike: bool,
  ) -> Self {
    let spear = Spear::init(clr, origin, size, angle, spike);
    let acos = angle.cos();
    let asin = angle.sin();
    let dir = if flagtoright { -PI / 2.0 } else { PI / 2.0 };
    let pacos = (angle + dir).cos();
    let pasin = (angle + dir).sin();

    let d = size * mix(0.5, 0.2, cloth_height_factor);
    let off = -0.03 * size;
    let o = (
      origin.0 + d * acos + off * pacos,
      origin.1 + d * asin + off * pasin,
    );
    let h = cloth_height_factor * size;
    let l = cloth_len_factor * size;
    let filling = 1.0;
    let oscillating = rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
    let cloth = FlagCloth::init(
      rng,
      blazonclr,
      o,
      angle + dir,
      h,
      l,
      filling,
      oscillating,
    );

    Self { cloth, spear }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let mut out = vec![];
    out.extend(self.cloth.render(paint));
    out.extend(self.spear.render(paint));
    out
  }
}

pub struct FlagCloth {
  pub routes: Polylines,
  pub filling: f32,
}

impl FlagCloth {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    angle: f32,
    h: f32,
    length: f32,
    filling: f32,
    oscillating: f32,
  ) -> Self {
    let mut routes = vec![];

    let acos = (angle).cos();
    let asin = (angle).sin();
    let dacos = (angle + PI / 2.).cos();
    let dasin = (angle + PI / 2.).sin();

    let incr = filling;
    let mut p = origin;
    let mut l = 0.0;
    let mut barebone = vec![];
    let mut widths = vec![];
    let amp1 = 0.7 * h;
    let f1 = mix(rng.gen_range(5.0..15.0), 0.0, oscillating);
    let amp2 = 0.3 * h;
    let f2 = 2.0 * f1;

    while l < length {
      barebone.push(p);

      let n = 0.2
        * oscillating
        * (amp1 * (f1 * l / length).sin() + amp2 * (f2 * l / length).sin());
      p.0 += dacos * n;
      p.1 += dasin * n;

      p.0 += acos * incr;
      p.1 += asin * incr;

      l += incr;

      let mul = 0.7 + 0.4 * n.abs();
      widths.push(h * mul);
    }

    let count = 2 + (h / filling) as usize;
    let fibers = path_to_fibers(&barebone, &widths, count);

    let destr = rng.gen_range(-1.0f32..0.7).max(0.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0);
    let plen = rng.gen_range(0.3..0.5);
    let preverse = rng.gen_bool(0.5);

    let fl = fibers.len();
    for (i, w) in fibers.iter().enumerate() {
      let len = w.len();
      let mut pattern = (i as f32 / (fl - 1) as f32 - 0.5).abs();
      if preverse {
        pattern = 0.5 - pattern;
      }
      let lfactor = (1.0
        - if destr > 0. {
          rng.gen_range(0.0..destr)
        } else {
          0.
        }
        - plen * pattern)
        .max(0.0);
      let rt = w
        .iter()
        .take((len as f32 * lfactor) as usize)
        .cloned()
        .collect::<Vec<_>>();
      routes.push((clr, rt));
    }

    Self { routes, filling }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for (_, poly) in &self.routes {
      paint.paint_polyline(poly, 0.5 * self.filling);
    }
    routes
  }
}
