mod clouds;
mod eagle;
mod sun;

use rand::prelude::*;
use std::f32::consts::PI;

use crate::{
  algo::{packing::VCircle, paintmask::PaintMask},
  global::GlobalCtx,
};

use self::{clouds::cloud_in_circle, eagle::eagle, sun::sun};

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
        rng.gen_range(0.05..1.0) * rng.gen_range(0.0..0.6) * height,
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
    }

    // sun
    routes.extend(sun(paint, sun_color, sun_circle.pos(), sun_circle.r, 0.5));

    routes
  }
}
