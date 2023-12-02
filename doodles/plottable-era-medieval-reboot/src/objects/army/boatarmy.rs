use super::{boat::Boat, human::Human};
use crate::{
  algo::{paintmask::PaintMask, polylines::Polylines, renderable::Renderable},
  global::GlobalCtx,
  objects::blazon::Blazon,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct BoatArmy {
  pub clr: usize,
  pub origin: (f32, f32),
  pub angle: f32,
  pub size: f32,
  pub w: f32,
  pub xflip: bool,
  pub blazon: Blazon,
  pub humans: Vec<Human>,
  pub boat: Boat,
}

impl BoatArmy {
  pub fn init<
    R: Rng,
    SpawnHuman: Fn(
      &mut R,     // rng
      (f32, f32), // position
      f32,        // size
      f32,        // angle
      bool,       // xflip
    ) -> Human,
  >(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    w: f32,
    xflip: bool,
    blazon: Blazon,
    spawn_human: &SpawnHuman,
  ) -> Self {
    let mut humans = vec![];
    let boat = Boat::init(rng, origin, size, angle, w, xflip, blazon, clr);

    let xdir = if xflip { -1.0 } else { 1.0 };
    let acos = angle.cos();
    let asin = angle.sin();
    let x1 = boat.x1;
    let x2 = boat.x2;
    let mut x = x1;
    while x < x2 {
      let y = rng.gen_range(-0.1 * size..0.0);
      let p = (x, y);
      let p = (p.0 * acos + p.1 * asin, p.1 * acos - p.0 * asin);
      let p = (p.0 * xdir + origin.0, p.1 + origin.1);
      let human = spawn_human(rng, p, size, angle, xflip);
      humans.push(human);
      x += rng.gen_range(0.15..0.25) * size;
    }

    Self {
      clr,
      origin,
      size,
      angle,
      w,
      xflip,
      blazon,
      boat,
      humans,
    }
  }

  pub fn render<R: Rng>(&self, rng: &mut R, mask: &mut PaintMask) -> Polylines {
    let clr = self.clr;
    let boat = &self.boat;
    let humans = &self.humans;

    let mut routes = vec![];

    for front in humans.iter() {
      routes.extend(front.render_foreground_only(rng, mask));
    }

    routes.extend(boat.render(rng, mask, clr));

    for human in humans.iter() {
      routes.extend(human.render_background_only(rng, mask));
    }

    // we also create a halo cropping around castle
    for (_, route) in &routes {
      mask.paint_polyline(route, 1.0);
    }

    routes
  }
}

impl<R: Rng> Renderable<R> for BoatArmy {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(rng, paint)
  }
  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
