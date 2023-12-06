use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    polylines::{grow_as_rectangle, Polylines},
    renderable::Renderable,
  },
  global::GlobalCtx,
};
use noise::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Firecamp {
  pub woods: Polylines,
  pub smokes: Polylines,
  pub origin: (f32, f32),
}

impl Firecamp {
  pub fn init<R: Rng>(
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    smoke_length: f32,
  ) -> Self {
    let (x, y) = origin;

    // Woods
    let mut woods = vec![];
    for i in 0..((size) as usize).max(3).min(16) {
      let dx = size * rng.gen_range(0.7..1.2) / (1.0 + i as f32 * 0.3);
      let dy = size * (i as f32 * 0.2 + 0.3);
      let ytwist = size * rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0);
      let a = (x - dx, y - dy + ytwist);
      let b = (x + dx, y - dy - ytwist);
      woods.push((clr, grow_as_rectangle(a, b, 0.2 * size)));
    }
    woods.shuffle(rng);

    // Smoke
    let mut smokes = vec![];
    let mut smokex = x;
    let xbaseincr = rng.gen_range(-0.2..0.2);
    let mut smokey = y - size;
    let perlin = Perlin::new(rng.gen());
    let seed = rng.gen_range(0.0f64..1000.0);
    let mut w_mul = 0.5 * size;
    let mut incrymul = rng.gen_range(0.5..0.8);
    loop {
      if smokey < y - smoke_length {
        break;
      }
      // let lpercent = (y - smokey) / smoke_length;
      let w = rng.gen_range(0.3..1.1) * w_mul;
      smokes.push((
        clr,
        vec![(smokex - w * 0.5, smokey), (smokex + w * 0.5, smokey)],
      ));
      smokex += size
        * 0.2
        * (xbaseincr
          + 0.8 * (0.1 * smokey).cos()
          + 2.0
            * perlin.get([seed, 0.1 * smokex as f64, 0.1 * smokey as f64])
              as f32);
      smokey -= rng.gen_range(1.0..1.5) * incrymul;
      if rng.gen_bool(0.04) {
        smokey -= 5.0 * incrymul;
      }
      w_mul = (w_mul * 1.06).min(30.0);
      incrymul *= 1.03;
    }

    Self {
      woods,
      smokes,
      origin,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    let mut routes = vec![];
    for wood in &self.woods {
      let rts = regular_clip(&vec![wood.clone()], paint);
      paint.paint_polygon(&wood.1);
      routes.extend(rts);
    }

    let rts = regular_clip(&self.smokes, paint);
    let n = rng.gen_range(1..40);
    for (_, rt) in rts.iter().take(n) {
      paint.paint_polyline(rt, 1.0);
      ctx.effects.hot.paint_polyline(rt, 8.0);
    }
    routes.extend(rts);

    routes
  }
}

impl<R: Rng> Renderable<R> for Firecamp {
  fn render(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(rng, ctx, paint)
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
