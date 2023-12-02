use std::f32::consts::PI;

use crate::{
  algo::{
    clipping::regular_clip, math1d::mix, packing::VCircle,
    paintmask::PaintMask, pathlookup::PathLookup, shapes::circle_route,
    wormsfilling::WormsFilling,
  },
  global::GlobalCtx,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct FireballTrail {
  pub trailmask: PaintMask,
  pub particles: usize,
  pub bound: (f32, f32, f32, f32),
  pub circles: Vec<VCircle>,
  pub clr: usize,
}

impl FireballTrail {
  pub fn init<R: Rng>(
    rng: &mut R,
    referencemask: &PaintMask,
    path_to_object: Vec<(f32, f32)>,
    object_size: f32,
    trailmaxpercent: f32,
    particles: usize,
    strokes: usize,
    clr: usize,
  ) -> Self {
    let mut trailmask = referencemask.clone_empty();

    let sampler = PathLookup::init(path_to_object.clone());

    let mut circles = vec![];
    let circles_count = rng.gen_range(0..particles);

    let l = sampler.length();
    let reduce_disp = 0.6;
    for _ in 0..circles_count {
      let v = mix(
        l,
        0.0,
        rng.gen_range(0.0..trailmaxpercent) * rng.gen_range(0.0..1.0),
      );
      let (x, y) = sampler.lookup_pos(v);
      let a = sampler.lookup_angle(v);
      let apar = a + PI / 2.0;
      let opening = mix(0.0, object_size, v / l);
      let m = rng.gen_range(0.2..0.8);
      let r = mix(0.0, opening / 2.0, m);
      let disp = reduce_disp
        * (opening - r)
        * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
      let x = x + disp * apar.cos();
      let y = y + disp * apar.sin();
      circles.push(VCircle::new(x, y, r));
      trailmask.paint_circle(x, y, r);
    }

    let strokew = 0.8;
    for _ in 0..strokes {
      let v = mix(l, 0.0, rng.gen_range(0.0..trailmaxpercent));
      let offset = rng.gen_range(-0.5..0.5) * object_size;
      let route = sampler.build_path(v..l, offset);
      trailmask.paint_polyline(&route, strokew);
    }

    let bound: (f32, f32, f32, f32) = trailmask.painted_boundaries();

    Self {
      trailmask,
      particles,
      bound,
      circles,
      clr,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let clr = self.clr;
    let mut routes = vec![];

    let its = self.particles as f32 * rng.gen_range(1.0..2.0);

    let filling = WormsFilling::rand(rng);
    routes.extend(filling.fill_in_paint(
      rng,
      &self.trailmask,
      clr,
      1.5,
      self.bound,
      its as usize,
    ));

    for c in &self.circles {
      routes.push((clr, circle_route((c.x, c.y), c.r, 20)));
    }

    routes = regular_clip(&routes, paint);

    for (_, route) in &routes {
      paint.paint_polyline(route, 2.0);
      ctx.effects.hot.paint_polyline(route, 4.0);
    }

    paint.paint(&self.trailmask);

    routes
  }
}
