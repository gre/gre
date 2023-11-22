use crate::algo::{
  clipping::regular_clip, packing::packing, paintmask::PaintMask,
  polylines::Polylines,
};
use noise::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Rain {
  pub routes: Polylines,
}

impl Rain {
  pub fn init<R: Rng>(
    rng: &mut R,
    paint: &PaintMask,
    clr: usize,
    layers: usize,
    iterations: usize,
    width: f32,
    height: f32,
    fromlen: f32,
    tolen: f32,
    angle: f32,
    perlinfreq: f64,
    perlinamp: f32,
  ) -> Self {
    let mut routes = vec![];
    let mut circles = vec![];
    for _ in 0..layers {
      let s = rng.gen_range(fromlen..tolen);
      let min = 0.8 * s;
      let max = 1.2 * s;
      circles.extend(packing(
        rng,
        iterations,
        iterations,
        1,
        0.0,
        (0.0, 0.0, width, height),
        &|c| !paint.is_painted(c.pos()),
        min,
        max,
      ));
    }
    let perlin = Perlin::new(rng.gen());
    let seed = rng.gen_range(0.0..999.0);
    let m = perlinfreq / width as f64;
    for c in circles {
      let a = angle
        + perlinamp * perlin.get([c.x as f64 * m, c.y as f64 * m, seed]) as f32;
      let dx = a.cos() * c.r;
      let dy = a.sin() * c.r;
      routes.push((clr, vec![(c.x - dx, c.y - dy), (c.x + dx, c.y + dy)]));
    }
    Self { routes }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    routes
  }
}
