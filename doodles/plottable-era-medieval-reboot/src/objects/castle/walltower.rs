use super::decorations::{merlon, wall_shadow};
use crate::algo::{
  clipping::regular_clip_polys, math1d::mix, paintmask::PaintMask,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub struct CastleWallTower {
  // ybase is where the chapel foundation need to start
  pub ybase: f64,
  // center of the wall base
  pub pos: (f64, f64),
  pub width: f64,
  pub height: f64,
  pub scale: f64,
  pub clr: usize,
}

impl CastleWallTower {
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let ybase = self.ybase;
    let pos = self.pos;
    let width = self.width;
    let height = self.height;
    let scale = self.scale;
    let clr = self.clr;

    let mut routes = vec![];
    let mut polys = vec![];

    let a = (pos.0 - width / 2., ybase);
    let b = (pos.0 + width / 2., ybase);

    let towerheighty = pos.1 - height;

    let d1 = scale * rng.gen_range(0.0..3.0);
    let h1 = scale * rng.gen_range(3.0..5.0);
    let merlonh = scale * rng.gen_range(1.0..2.2);

    let mut route: Vec<(f64, f64)> = Vec::new();
    route.push(a);
    route.push((a.0, towerheighty));
    route.push((a.0 - d1, towerheighty - d1));
    route.push((a.0 - d1, towerheighty - d1 - h1));
    merlon(
      &mut polys,
      &mut route,
      a.0 - d1,
      towerheighty - d1 - h1,
      b.0 + d1,
      towerheighty - d1 - h1,
      merlonh,
    );
    route.push((b.0 + d1, towerheighty - d1 - h1));
    route.push((b.0 + d1, towerheighty - d1));
    route.push((b.0, towerheighty));
    route.push(b);

    // boundaries of the tower body
    polys.push(vec![
      (a.0, a.1),
      (b.0, b.1),
      (b.0, towerheighty),
      (a.0, towerheighty),
    ]);

    // boundaries of the tower head
    polys.push(vec![
      (a.0, towerheighty),
      (b.0, towerheighty),
      (b.0 + d1, towerheighty - d1),
      (b.0 + d1, towerheighty - d1 - h1 + merlonh),
      (a.0 - d1, towerheighty - d1 - h1 + merlonh),
      (a.0 - d1, towerheighty - d1),
    ]);

    let right_side_path = vec![
      (b.0 + d1, towerheighty - d1 - h1),
      (b.0 + d1, towerheighty - d1),
      (b.0, towerheighty),
      b,
    ];
    for shadow in wall_shadow(rng, right_side_path, scale) {
      routes.push((clr, shadow));
    }
    routes.push((clr, route));

    // windows
    let mut y = towerheighty;
    let w = scale * 0.25;
    let h = scale * rng.gen_range(1.0..1.2);
    loop {
      let x = mix(a.0, b.0, rng.gen_range(0.4..0.6));
      let lowesty = pos.1;
      if y > lowesty - 3.0 * h {
        break;
      }
      routes.push((
        clr,
        vec![
          (x - w, y - h),
          (x + w, y - h),
          (x + w, y + h),
          (x - w, y + h),
          (x - w, y - h),
        ],
      ));
      y += 4.0 * h;
    }

    /*
    let pushbackrotbase = rng.gen_range(-1.0, 1.0);
    let pushbackrotmix = 1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

    let (routes, polys) = multicut_along_line(
      rng,
      &routes,
      &polys,
      clr,
      pos,
      (pos.0, towerheighty),
      |rng| rng.gen_range(4.0, 6.0),
      |rng| {
        rng.gen_range(-PI / 2.0, PI / 2.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0)
      },
      |rng| scale * rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
      |rng| rng.gen_range(0.0, 1.0),
      |rng| 0.1 * mix(pushbackrotbase, rng.gen_range(-1.0, 1.0), pushbackrotmix),
    );
    */

    // clip and paint
    let routes = regular_clip_polys(&routes, paint, &polys);

    /*
    // make ropes behind the construction
    let count = rng.gen_range(3, 16);
    routes.extend(building_ropes(
      rng,
      paint,
      &polys,
      count,
      clr,
      2.0 * width,
      2.0 * height + 50.0,
    ));
    */

    routes
  }
}
