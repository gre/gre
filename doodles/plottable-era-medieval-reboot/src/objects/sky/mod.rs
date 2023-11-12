mod clouds;
mod eagle;
mod sun;

use rand::prelude::*;

use crate::algo::{packing::VCircle, paintmask::PaintMask};

use self::sun::sun;

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
}

impl MedievalSky {
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let MedievalSky {
      sun_color,
      cloud_color,
      eagle_color,
      sun_circle,
      desired_clouds,
      desired_eagles,
    } = *self;
    let mut routes = vec![];

    // eagles

    // clouds

    // TODO should we add some straight lines for the sky too like in Era (1) ?

    // sun
    routes.extend(sun(paint, sun_color, sun_circle.pos(), sun_circle.r, 0.5));

    routes
  }
}
