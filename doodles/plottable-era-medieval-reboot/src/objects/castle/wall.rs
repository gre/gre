use super::decorations::merlon;
use crate::algo::{
  clipping::clip_routes_with_colors,
  math1d::mix,
  paintmask::PaintMask,
  polygon::{polygon_includes_point, polygons_includes_point},
  shapes::arc,
  wormsfilling::WormsFilling,
};
use rand::prelude::*;
use std::f64::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct CastleWall {
  // ybase is where the chapel foundation need to start
  pub ybase: f64,
  // center of the wall base
  pub pos: (f64, f64),
  pub width: f64,
  pub height: f64,
  pub scale: f64,
  pub clr: usize,
  pub dark_wall: bool,
  pub portcullis: bool,
}

impl CastleWall {
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

    let merlonh = scale * rng.gen_range(1.0..2.2);

    let left = (pos.0 - width / 2., ybase);
    let right = (pos.0 + width / 2., ybase);
    let wallheighty = pos.1 - height;

    let mut route = Vec::new();
    polys.push(vec![
      (left.0, ybase),
      (left.0, wallheighty + merlonh),
      (right.0, wallheighty + merlonh),
      (right.0, ybase),
    ]);
    route.push(left);
    route.push((left.0, wallheighty));
    merlon(
      &mut polys,
      &mut route,
      left.0 + 0.01,
      wallheighty,
      right.0 - 0.01,
      wallheighty,
      merlonh,
    );
    route.push(right);
    routes.push((clr, route));

    // wall texture
    if self.dark_wall {
      let its = rng.gen_range(3000..5000);
      let density = rng.gen_range(1.0..3.0);
      routes.extend(WormsFilling::rand(rng).fill(
        rng,
        &|x, y| {
          if polygons_includes_point(&polys, (x, y)) {
            density
          } else {
            0.0
          }
        },
        (
          pos.0 - width / 2.0,
          pos.1 - height,
          pos.0 + width / 2.0,
          ybase,
        ),
        &|_| clr,
        0.5,
        its,
        5,
      ));
    } else {
      let xrep = scale * rng.gen_range(2.6..3.2);
      let yrep = scale * rng.gen_range(1.2..1.6);
      let mut alt = false;
      let mut y = wallheighty + merlonh + yrep;
      loop {
        if y > ybase {
          break;
        }
        let mut x = left.0;
        if alt {
          x += xrep / 2.0;
        }
        loop {
          if x > right.0 {
            break;
          }
          let strokel = scale * rng.gen_range(1.3..1.5);
          let dx = scale * rng.gen_range(-0.2..0.2);
          let dy = scale * rng.gen_range(-0.1..0.1);
          let x1 = (x + dx).max(left.0).min(right.0);
          let x2 = (x + dx + strokel).max(left.0).min(right.0);
          let y1 = y + dy;
          if y1 < ybase && y1 < ybase && rng.gen_bool(0.95) {
            routes.push((clr, vec![(x1, y + dy), (x2, y + dy)]));
          }
          x += xrep;
        }
        y += yrep;
        alt = !alt;
      }
    }

    /*
    if self.destructed_wall {
      (routes, polys) = multicut_along_line(
        rng,
        &routes,
        &polys,
        clr,
        (left.0, wallheighty),
        (right.0, wallheighty),
        |rng| rng.gen_range(8.0, 16.0),
        |rng| {
          0.5
            * rng.gen_range(-PI / 2.0, PI / 2.0)
            * rng.gen_range(0.0, 1.0)
            * rng.gen_range(0.0, 1.0)
        },
        |rng| height * rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
        |rng| rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
        |_rng| 0.0,
      );
    }
    */

    if self.portcullis {
      let x = pos.0;
      let h = width.min(ybase - wallheighty) * rng.gen_range(0.7..0.85);
      let w = h * rng.gen_range(0.3..0.5);
      let r = w / 2.0;

      let door = vec![
        vec![
          (x + w / 2., ybase),
          (x - w / 2., ybase),
          (x - w / 2., ybase - h + r),
        ],
        arc((x, ybase - h + r), r, -PI, 0.0, 32),
        vec![(x + w / 2., ybase - h + r), (x + w / 2., ybase)],
      ]
      .concat();

      let mut grids = vec![];
      let r = rng.gen_range(0.08..0.14) * w;
      let ybottom = mix(ybase, ybase - h, rng.gen_range(0.0..1.0));
      let mut xp = x - w / 2.0;
      let extra = 1.5;
      while xp < x + w / 2.0 {
        let grid = vec![(xp, ybottom + extra), (xp, ybase - h)];
        grids.push((clr, grid));
        xp += r;
      }
      let mut yp = ybase - h;
      while yp < ybottom {
        let grid = vec![(x - w / 2., yp), (x + w / 2., yp)];
        grids.push((clr, grid));
        yp += r;
      }

      // carve into the door
      routes = clip_routes_with_colors(
        &routes,
        &|p| polygon_includes_point(&door, p),
        0.3,
        5,
      );
      // add the door
      routes.push((clr, door.clone()));
      routes.extend(clip_routes_with_colors(
        &grids,
        &|p| !polygon_includes_point(&door, p),
        0.3,
        5,
      ));
    }

    // clip and paint
    let is_outside = |p| paint.is_painted(p);
    let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
    for poly in polys.iter() {
      paint.paint_polygon(poly);
    }

    routes
  }
}
