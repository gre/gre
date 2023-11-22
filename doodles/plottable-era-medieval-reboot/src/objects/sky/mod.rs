pub mod clouds;
pub mod eagle;
pub mod moon;
pub mod rain;
pub mod star;
pub mod sun;

use self::{
  clouds::cloud_in_circle, eagle::eagle, moon::Moon, rain::Rain, star::Star,
  sun::Sun,
};
use crate::{
  algo::{
    packing::{self, packing, VCircle},
    paintmask::PaintMask,
  },
  global::GlobalCtx,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct MedievalSky {
  pub sun_color: usize,
  pub cloud_color: usize,
  pub eagle_color: usize,
  pub sun_circle: VCircle,
  pub desired_clouds: usize,
  pub desired_eagles: usize,
  pub width: f32,
  pub height: f32,
  pub pad: f32,
  pub night_time: bool,
}

impl MedievalSky {
  pub fn rand<R: Rng>(
    ctx: &mut GlobalCtx,
    rng: &mut R,
    width: f32,
    height: f32,
    pad: f32,
  ) -> Self {
    let sun_circle = VCircle::new(
      width * rng.gen_range(0.4..0.6),
      height * rng.gen_range(0.1..0.4),
      width * rng.gen_range(0.07..0.1),
    );

    let desired_clouds = (rng.gen_range(-0.3f32..1.0)
      * rng.gen_range(0.2..1.0)
      * rng.gen_range(0.0..80.0))
    .max(0.0) as usize;

    let desired_eagles = (rng.gen_range(-0.2f32..1.0)
      * rng.gen_range(0.2..1.0)
      * rng.gen_range(0.0..20.0))
    .max(0.0) as usize;

    MedievalSky {
      sun_color: 1,
      cloud_color: 0,
      eagle_color: 0,
      sun_circle,
      desired_clouds,
      desired_eagles,
      pad,
      width,
      height,
      night_time: ctx.night_time,
    }
  }
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let MedievalSky {
      sun_color,
      cloud_color,
      eagle_color,
      sun_circle,
      desired_clouds,
      desired_eagles,
      width,
      height,
      pad,
      night_time,
    } = *self;
    let mut routes = vec![];

    // TODO we can use a packing in the available space to place the items below the yhorizon.
    // that we we avoid too much collisions with the castle.
    // then we pipe into a perlin noise to have interesting patterns
    //
    // TODO we also should add some straight lines for the sky too like in Era (1)
    // we probably should follow the same noise pattern as the clouds
    // so we can place them consistently together...

    // eagles
    for _i in 0..desired_eagles {
      let sz = rng.gen_range(0.008..0.02) * height;
      let p = pad + sz;
      let origin = (
        p + rng.gen_range(0.0..1.0) * (width - p * 2.0),
        p + rng.gen_range(0.0..0.5) * (height - p * 2.0),
      );
      let rotation = 0.3 * rng.gen_range(0.0..1.0) * rng.gen_range(-PI..PI);
      let xreverse = rng.gen_bool(0.5);
      routes.extend(eagle(
        rng,
        paint,
        origin,
        sz,
        rotation,
        xreverse,
        eagle_color,
      ));
    }

    // clouds
    for _i in 0..desired_clouds {
      let circle = VCircle::new(
        rng.gen_range(0.0..1.0) * width,
        rng.gen_range(0.0..1.0) * rng.gen_range(0.0..0.5) * height,
        (0.02 + 0.1 * rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0))
          * height,
      );
      let base_dr = rng.gen_range(1.0..2.0);
      let minr = 0.5 + rng.gen_range(0.0..circle.r) * rng.gen_range(0.0..0.3);
      routes.extend(cloud_in_circle(
        rng,
        paint,
        cloud_color,
        &circle,
        base_dr,
        minr,
      ));

      // TODO cloud side function of y
      // TODO use WeightMap
    }

    let should_sun_spiral = !night_time && rng.gen_bool(0.7);
    let should_rain = !night_time && !should_sun_spiral && rng.gen_bool(0.8);
    let should_have_stars = night_time && rng.gen_bool(0.8);

    // sun
    let dr = 0.5;
    if night_time {
      let phase = rng.gen_range(0.35..0.65);
      let moon =
        Moon::init(sun_color, sun_circle.pos(), sun_circle.r, dr, phase);
      routes.extend(moon.render(paint));
    } else {
      let spiralrays = if should_sun_spiral {
        Some(rng.gen_range(1.0..4.0))
      } else {
        None
      };
      let sun =
        Sun::init(sun_color, sun_circle.pos(), sun_circle.r, dr, spiralrays);
      routes.extend(sun.render(paint));
    }

    if should_have_stars {
      let count = rng.gen_range(50..500);
      let pad = rng.gen_range(0.0..0.02) * width;
      let min = pad + rng.gen_range(0.005..0.01) * width;
      let max = min + 0.001 * width;
      let starbranches = 2 * rng.gen_range(5..8);
      let circles = packing(
        rng,
        5000,
        count,
        1,
        pad,
        (0.0, 0.0, width, height),
        &|c| !paint.is_painted(c.pos()),
        min,
        max,
      );
      let mind = rng.gen_range(0.3..0.8);
      for c in circles {
        let r = c.r * rng.gen_range(mind..1.0);
        let star = Star::init(rng, sun_color, c.pos(), r, starbranches);

        routes.extend(star.render(paint));
      }
    }

    if should_rain {
      let clr = rng.gen_range(0..2);
      let width = paint.width;
      let height = paint.height;
      let fromlen = rng.gen_range(0.001..0.01) * width;
      let tolen = 2.0 * fromlen;
      let angle = PI / 2.0
        + rng.gen_range(-2.0..2.0)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(0.0..1.0);
      let perlinfreq = rng.gen_range(0.0..10.0) * rng.gen_range(0.0..1.0);
      let perlinamp = rng.gen_range(0.0..3.0) * rng.gen_range(0.0..1.0);
      let layers = rng.gen_range(3..6);
      let iterations = rng.gen_range(4000..8000);
      let rain = Rain::init(
        rng, paint, clr, layers, iterations, width, height, fromlen, tolen,
        angle, perlinfreq, perlinamp,
      );
      routes.extend(rain.render(paint));
    }

    routes
  }
}
