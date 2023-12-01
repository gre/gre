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
    clipping::regular_clip,
    math1d::{mix, smoothstep},
    packing::{packing, VCircle},
    paintmask::PaintMask,
    polylines::Polylines,
    shapes::{circle_route, spiral_optimized},
  },
  global::{GlobalCtx, Special},
};
use noise::*;
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct MedievalSky {
  pub clouds: Vec<VCircle>,
  pub sun_color: usize,
  pub cloud_color: usize,
  pub eagle_color: usize,
  pub sun_circle: VCircle,
  pub desired_clouds: usize,
  pub desired_eagles: usize,
  pub width: f32,
  pub height: f32,
  pub pad: f32,
  pub should_moon: bool,
  pub should_sun_spiral: bool,
  pub should_rain: bool,
  pub should_cloud_rays: bool,
  pub stars: Vec<Star>,
  pub routes: Polylines,
}

impl MedievalSky {
  pub fn rand<R: Rng>(
    ctx: &mut GlobalCtx,
    rng: &mut R,
    skysafemask1: &PaintMask,
    skysafemask2: &PaintMask,
    width: f32,
    height: f32,
    pad: f32,
  ) -> Self {
    let sun_circle = VCircle::new(
      width * ctx.sun_xpercentage_pos,
      height * rng.gen_range(0.1..0.3),
      width * rng.gen_range(0.07..0.1),
    );

    let desired_clouds =
      (rng.gen_range(-0.3f32..1.0) * rng.gen_range(0.0..5.0)).max(0.0) as usize;

    let bound1 = skysafemask1.painted_boundaries();
    let bound2 = skysafemask2.painted_boundaries();
    let does_overlap = |c: &VCircle| {
      !skysafemask2.is_painted(c.pos())
        && circle_route(c.pos(), c.r, 8)
          .iter()
          .all(|p| !skysafemask2.is_painted(*p))
    };
    let min_scale = 0.08 * width;
    let max_scale = 0.12 * width;
    let clouds = packing(
      rng,
      1000,
      desired_clouds,
      1,
      0.0,
      bound2,
      &does_overlap,
      min_scale,
      max_scale,
    );

    let desired_eagles = if ctx.specials.contains(&Special::EaglesAttack) {
      rng.gen_range(60..100)
    } else {
      (rng.gen_range(-1.0f32..1.0)
        * rng.gen_range(0.2..1.0)
        * rng.gen_range(0.0..20.0))
      .max(0.0) as usize
    };

    let should_sun_spiral = !ctx.night_time && rng.gen_bool(0.6);
    let should_rain =
      !ctx.night_time && !should_sun_spiral && rng.gen_bool(0.4);
    let should_have_stars = ctx.night_time && rng.gen_bool(0.7);
    // ref to https://greweb.me/plots/1163
    let should_uh_oh_sky = !should_sun_spiral
      && !should_rain
      && !should_have_stars
      && rng.gen_bool(0.5);
    let should_moon = ctx.night_time;

    let mut stars = vec![];
    if should_have_stars {
      let count =
        10 + (rng.gen_range(0.0..300.0) * rng.gen_range(0.0..1.0)) as usize;
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
        bound1,
        &|c| !skysafemask1.is_painted(c.pos()),
        min,
        max,
      );
      let mind = rng.gen_range(0.3..0.8);
      for c in circles {
        let r = c.r * rng.gen_range(mind..1.0);
        let star = Star::init(rng, 1, c.pos(), r, starbranches);
        stars.push(star);
      }
    }

    let mut routes = vec![];
    if should_uh_oh_sky {
      let pad = rng.gen_range(0.5..2.0);
      let min = pad + rng.gen_range(0.8..1.0);
      let max = min + rng.gen_range(0.5..2.0);
      let circles = packing(
        rng,
        50000,
        1000,
        1,
        pad,
        bound1,
        &|c| !skysafemask1.is_painted(c.pos()),
        min,
        max,
      );
      let f = rng.gen_range(0.0..0.3)
        * rng.gen_range(0.0..1.0)
        * rng.gen_range(0.0..1.0);

      let perlin = Perlin::new(rng.gen());
      let clr = rng.gen_range(0..2);
      for c in circles {
        let v = perlin.get([c.x as f64 * f, c.y as f64 * f * 3.0]) as f32
          - smoothstep(pad, bound1.3, c.y) * 0.2
          + rng.gen_range(-0.1..0.1);
        if v > 0.0 {
          if v > 0.35 {
            let r = c.r * 1.1;
            routes.push((clr, spiral_optimized(c.x, c.y, r, 1.3 - v, 0.01)));
            routes.push((clr, circle_route((c.x, c.y), c.r, 32)));
          } else {
            let r = mix(0.2, c.r, smoothstep(0.0, 0.3, v));
            routes.push((clr, vec![(c.x - r, c.y), (c.x + r, c.y)]));
            if v > 0.15 {
              routes.push((clr, vec![(c.x, c.y - r), (c.x, c.y + r)]));
            }
          }
        }
      }
    }

    let should_cloud_rays = !should_rain
      && !should_sun_spiral
      && stars.is_empty()
      && rng.gen_bool(0.8);

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
      clouds,
      should_moon,
      should_sun_spiral,
      should_rain,
      should_cloud_rays,
      stars,
      routes,
    }
  }
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let sun_color = self.sun_color;
    let cloud_color = self.cloud_color;
    let eagle_color = self.eagle_color;
    let sun_circle = self.sun_circle;
    let desired_eagles = self.desired_eagles;
    let pad = self.pad;
    let width = self.width;
    let height = self.height;
    let should_moon = self.should_moon;
    let should_sun_spiral = self.should_sun_spiral;
    let should_rain = self.should_rain;
    let should_cloud_rays = self.should_cloud_rays;
    let stars = &self.stars;

    let mut routes = vec![];

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
    let mut cloud_paint = paint.clone_empty();
    for circle in &self.clouds {
      let mut sub = vec![];
      for _ in 0..rng.gen_range(1..16) {
        sub.push(VCircle::new(
          circle.x
            + circle.r * rng.gen_range(-1.0..1.0) * rng.gen_range(0.5..1.0),
          circle.y
            + circle.r * rng.gen_range(-1.0..1.0) * rng.gen_range(0.0..1.0),
          circle.r * rng.gen_range(0.3..1.0),
        ));
      }
      sub.sort_by(|a, b| a.r.partial_cmp(&b.r).unwrap());
      for c in sub {
        let base_dr = rng.gen_range(1.0..2.0);
        let minr = 0.5 + rng.gen_range(0.0..circle.r) * rng.gen_range(0.0..0.3);
        routes.extend(cloud_in_circle(
          rng,
          paint,
          cloud_color,
          &c,
          base_dr,
          minr,
        ));
        let maxr = rng.gen_range(0.0..16.0) * rng.gen_range(0.0..1.0);
        cloud_paint.paint_circle(
          c.x + c.r * rng.gen_range(-maxr..maxr) * rng.gen_range(0.5..1.0),
          c.y + c.r * rng.gen_range(-1.0..1.0) * rng.gen_range(0.0..1.0),
          c.r
            * (1.0
              + rng.gen_range(0.0..maxr)
                * rng.gen_range(0.0..1.0)
                * rng.gen_range(0.0..1.0)),
        );
      }
    }

    // sun
    let dr = 0.5;
    if should_moon {
      let phase = 0.5
        + rng.gen_range(0.02..0.3) * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
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

    for star in stars {
      routes.extend(star.render(paint));
    }

    if should_cloud_rays {
      cloud_paint.reverse();
      cloud_paint.paint(paint);
      let mut rts = vec![];
      let mut y = 0.0;
      let incr = rng.gen_range(0.006..0.012) * width;
      let clr = if rng.gen_bool(0.3) { 0 } else { 1 };
      while y < height {
        rts.push((clr, vec![(0.0, y), (width, y)]));
        y += incr;
      }
      routes.extend(regular_clip(&rts, &cloud_paint));
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

    routes.extend(regular_clip(&self.routes, paint));

    routes
  }
}
