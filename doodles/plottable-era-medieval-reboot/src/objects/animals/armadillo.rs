use std::f32::consts::PI;

use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  math2d::{lerp_point, p_r},
  paintmask::PaintMask,
  polylines::{path_subdivide_to_curve, shake, Polylines},
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Armadillo {
  pub routes: Polylines,
}

impl Armadillo {
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    scale: f32,
    rot: f32,
  ) -> Self {
    let mut routes = vec![];

    let c = (0.0, 0.0);
    let mut basey = -10.0;

    let a1 = rng.gen_range(0.2..0.4);
    let a2 = rng.gen_range(0.7..1.1);
    let m1 = 0.7;
    let m2 = rng.gen_range(1.5..2.0);
    let mut bodypoints = Vec::new();
    for a in vec![
      -a2 + rng.gen_range(-0.5..0.5) * 0.1 * scale,
      -a1 + rng.gen_range(-0.5..0.5) * 0.1 * scale,
      a1 + rng.gen_range(-0.5..0.5) * 0.1 * scale,
      a2 + rng.gen_range(-0.5..0.5) * 0.1 * scale,
    ] {
      let mp = 0.5;
      let mut line = Vec::new();
      for (a, m) in vec![
        (a, m1 + rng.gen_range(-0.1..0.1)),
        (a * 1.1, mix(m1, m2, mp) + rng.gen_range(-0.1..0.1)),
        (a, m2 + rng.gen_range(-0.1..0.1)),
      ] {
        let ang = a - PI / 2.0;
        let p = (c.0 + m * ang.cos(), c.1 + m * ang.sin());
        if p.1 > basey {
          basey = p.1;
        }
        line.push(p);
      }
      bodypoints.push(line);
    }

    for l in bodypoints.clone() {
      routes.push(l);
    }
    for i in 0..3 {
      let mut line = Vec::new();
      for l in bodypoints.clone() {
        line.push(l[i]);
      }
      routes.push(line);
    }

    // Make feet
    for (i, l) in bodypoints.iter().enumerate() {
      let origin = l[0];
      let a = PI / 2.0 - 0.5 * (i as f32 - 1.5)
        + rng.gen_range(-1.0..1.0) * rng.gen_range(0.0..1.0);
      let a2 = a + rng.gen_range(-0.4..0.2) * rng.gen_range(0.0..1.0);
      for _j in 0..3 {
        let mut route = vec![];
        let o = (
          origin.0 + rng.gen_range(-0.1..0.1),
          origin.1 + rng.gen_range(-0.1..0.0),
        );
        route.push(o);
        let amp = 0.3 + rng.gen_range(-0.1..0.1);
        for (amp, a) in vec![(amp, a), (amp + rng.gen_range(0.1..0.2), a2)] {
          let p = (o.0 + amp * a.cos(), o.1 + amp * a.sin());
          if p.1 > basey {
            basey = p.1;
          }
          route.push(p);
        }
        routes.push(route);
      }
    }

    // Make head
    let headorigin1 = bodypoints[3][2];
    let headorigin2 = bodypoints[3][1];
    // ears
    for _i in 0..4 {
      let mut line = vec![];
      let o = (
        headorigin1.0 + rng.gen_range(-0.1..0.3) * 0.1 * scale,
        headorigin1.1 + rng.gen_range(-0.1..0.4) * 0.05 * scale,
      );
      line.push(o);
      for (amp, a) in vec![(0.3f32, -1.2f32), (0.5, rng.gen_range(-1.8..-1.2))]
      {
        let p = (o.0 + amp * a.cos(), o.1 + amp * a.sin());
        line.push(p);
      }
      routes.push(line);
    }

    let hmul = rng.gen_range(0.8..1.2);
    let count = (3.0 + scale) as usize;
    let ang1 = rng.gen_range(0.5..0.8f32);
    let ang1amp = rng.gen_range(0.1..0.4);
    let ang2 = ang1 + rng.gen_range(0.0..0.3) * rng.gen_range(0.0..1.0);
    let ang2amp = rng.gen_range(0.0..0.1);
    for i in 0..count {
      let percent = i as f32 / (count as f32 - 1.0);
      let p1 = lerp_point(headorigin1, headorigin2, percent);
      let ang = ang1 + ang1amp * percent;
      let amp = 0.4f32 * hmul;
      let p2 = (p1.0 + amp * ang.cos(), p1.1 + amp * ang.sin());
      let ang = ang2 + ang2amp * percent;
      let amp = amp * rng.gen_range(2.0..3.0);
      let p3 = (
        headorigin2.0 + amp * ang.cos(),
        headorigin2.1 + amp * ang.sin(),
      );
      routes.push(shake(vec![p1, p2, p3], 0.035 * scale, rng));
    }

    // Make tail
    let a = rng.gen_range(-4.4f32..-3.8);
    let amp = rng.gen_range(0.4..0.8);
    let params = vec![
      (a, 0.0, 1.0),
      (a, amp, 0.5),
      (
        a + rng.gen_range(-0.3..0.3),
        amp + rng.gen_range(0.5..1.0),
        0.0,
      ),
    ];
    let tailoriginbase =
      lerp_point(bodypoints[0][2], bodypoints[0][1], rng.gen_range(0.0..0.7));
    for _i in 0..3 {
      let tailorigin =
        lerp_point(bodypoints[0][2], bodypoints[0][1], rng.gen_range(0.0..0.7));
      let mut l = vec![];
      for (ang, amp, mul) in params.clone() {
        let o = lerp_point(tailoriginbase, tailorigin, mul);
        let p = (o.0 + amp * ang.cos(), o.1 + amp * ang.sin());
        l.push(p);
      }
      l = path_subdivide_to_curve(&l, 1, 0.7);
      l = shake(l, 0.01 * scale, rng);
      l = path_subdivide_to_curve(&l, 1, 0.66);
      routes.push(l);
    }

    // scale, rotate & translate
    let routes = routes
      .iter()
      .map(|route| {
        (
          clr,
          route
            .iter()
            .map(|&p| {
              let mut p = p_r(p, rot);
              p = (scale * p.0 + origin.0, scale * (p.1 - basey) + origin.1);
              p
            })
            .collect(),
        )
      })
      .collect();

    Self { routes }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for (_, poly) in &self.routes {
      paint.paint_polygon(poly); // TODO it would be best to have proper polygons
    }
    routes
  }
}
