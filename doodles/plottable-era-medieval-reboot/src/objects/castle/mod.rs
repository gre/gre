use crate::{algo::paintmask::PaintMask, global::GlobalCtx};
use rand::prelude::*;

use self::{
  chapel::Chapel, levels::builder::build_castle, wall::CastleWall,
  walltower::CastleWallTower,
};

use super::{
  blazon::Blazon, mountains::CastleGrounding, projectile::attack::DefenseTarget,
};

pub mod chapel;
pub mod chinesedoor;
pub mod chineseroof;
pub mod decorations;
pub mod levels;
pub mod wall;
pub mod walltower;
pub mod watchtower;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Castle {
  // ybase is where the chapel foundation need to start
  pub ybase: f32,
  // where we absolutely need to stop building
  pub ymax: f32,
  // position and scaling
  pub pos: (f32, f32),
  pub width: f32,
  pub scale: f32,
  // stying
  pub clr: usize,
  pub blazonclr: usize,
  pub blazon: Blazon,
  // features
  pub left_tower: bool,
  pub right_tower: bool,
  pub chapel: bool,
  pub dark_chapel: bool,
  pub wall: bool,
  pub destructed_wall: bool,
  pub portcullis: bool,
  pub dark_wall: bool,
  pub wallh: f32,
}

impl Castle {
  pub fn init<R: Rng>(
    ctx: &mut GlobalCtx,
    rng: &mut R,
    castle: &CastleGrounding,
    ybase: f32,
    ymax: f32,
  ) -> Self {
    let width = castle.width;
    let scale = rng.gen_range(0.8..2.0);
    let wallh = rng.gen_range(0.3..0.8) * width;

    Self {
      pos: castle.position,
      width,
      scale,
      clr: 0,
      blazonclr: ctx.defendersclr,
      ybase,
      ymax,
      wallh,
      wall: true,
      left_tower: true,
      right_tower: true,
      dark_wall: rng.gen_bool(0.5),
      chapel: rng.gen_bool(0.5),
      dark_chapel: rng.gen_bool(0.5),
      destructed_wall: rng.gen_bool(0.5),
      portcullis: rng.gen_bool(0.5),
      blazon: ctx.defenders,
    }
  }

  pub fn render<R: Rng>(
    &self,
    ctx: &mut GlobalCtx,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let pos = self.pos;
    let width = self.width;
    let scale = self.scale;
    let ybase = self.ybase;
    let ymax = self.ymax;

    let mut routes = vec![];

    let mut x = 0.0;
    while x < width {
      let w = rng.gen_range(0.05..0.3) * width;
      let rts =
        build_castle(rng, ctx, paint, (x, pos.1), w, ybase, ymax, scale);
      routes.extend(rts);
      x += w * 1.2;
    }

    /*
    let rts =
      build_castle(rng, ctx, paint, (pos.0, pos.1), width, ybase, ymax, scale);
    routes.extend(rts);
    */

    /*
    let clr = self.clr;
    let wallcenter = pos;
    let wallh = self.wallh;

    let towerwidth = scale * rng.gen_range(3.0..6.0);
    let maint_height = scale * rng.gen_range(14.0..32.0);
    let maint_width = scale * rng.gen_range(4.0..8.0);

    for _ in 0..5 {
      ctx.projectiles.add_defense(DefenseTarget::Building((
        pos.0 + rng.gen_range(-0.5..0.5) * width * rng.gen_range(0.0..1.0),
        pos.1
          - 0.5 * wallh
          - rng.gen_range(-0.5..0.5) * wallh * rng.gen_range(0.0..1.0),
      )));
    }

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

    // we also create a halo cropping around castle
    for (_, route) in &routes {
      paint.paint_polyline(route, 1.4);
    }
    */

    routes
  }
}
