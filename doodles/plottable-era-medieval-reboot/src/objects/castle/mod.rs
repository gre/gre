use crate::algo::paintmask::PaintMask;
use rand::prelude::*;

use self::{chapel::Chapel, wall::CastleWall, walltower::CastleWallTower};

mod chapel;
mod decorations;
mod wall;
mod walltower;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Castle {
  // ybase is where the chapel foundation need to start
  pub ybase: f64,
  // center of the castle base
  pub pos: (f64, f64),
  pub width: f64,
  pub scale: f64,
  pub clr: usize,
  // other properties
  pub left_tower: bool,
  pub right_tower: bool,
  pub chapel: bool,
  pub dark_chapel: bool,
  pub wall: bool,
  pub destructed_wall: bool,
  pub portcullis: bool,
  pub dark_wall: bool,
  pub wallh: f64,
}

impl Castle {
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let pos = self.pos;
    let width = self.width;
    let scale = self.scale;
    let clr = self.clr;
    let ybase = self.ybase;

    let mut routes = vec![];

    let wallcenter = pos;
    let wallh = self.wallh;

    let towerwidth = scale * rng.gen_range(3.0..6.0);
    let maint_height = scale * rng.gen_range(14.0..32.0);
    let maint_width = scale * rng.gen_range(4.0..8.0);

    for (p, skip) in vec![
      (
        (pos.0 - width / 2. + towerwidth / 2., pos.1),
        !self.left_tower,
      ),
      (
        (pos.0 + width / 2. - towerwidth / 2., pos.1),
        !self.right_tower,
      ),
    ] {
      if skip {
        continue;
      }
      let towerheight = wallh + scale * rng.gen_range(4.0..8.0);
      let tower = CastleWallTower {
        ybase,
        pos: p,
        width: towerwidth,
        height: towerheight,
        scale,
        clr,
      };
      routes.extend(tower.render(rng, paint));
    }

    if self.wall {
      let wall = CastleWall {
        ybase,
        pos: wallcenter,
        width: width - towerwidth * 2.,
        height: wallh,
        scale,
        clr,
        dark_wall: self.dark_wall,
        portcullis: self.portcullis,
      };
      routes.extend(wall.render(rng, paint));
    }

    // chapel
    if self.chapel {
      let chapel = Chapel {
        ybase,
        pos: (wallcenter.0, wallcenter.1 - maint_height),
        width: maint_width,
        height: maint_height,
        scale,
        dark_fill: self.dark_chapel,
        clr,
      };
      routes.extend(chapel.render(rng, paint));
    }

    routes
  }
}
