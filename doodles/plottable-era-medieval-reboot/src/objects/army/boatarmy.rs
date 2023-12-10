use super::{
  boat::{Boat, BoatGlobals},
  fire::Fire,
  human::Human,
};
use crate::{
  algo::{
    math2d::angle_mirrored_on_x, paintmask::PaintMask, polylines::Polylines,
    renderable::Renderable,
  },
  global::GlobalCtx,
  objects::{blazon::Blazon, projectile::attack::AttackOrigin},
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
  pub fires: Vec<Fire>,
  pub boat: Boat,
}

pub struct SpawnHumanArg {
  pub origin: (f32, f32),
  pub size: f32,
  pub angle: f32, // in global space (xflip applied)
  pub xflip: bool,
  pub index: usize,
  pub total: usize,
}

impl BoatArmy {
  pub fn init<
    R: Rng,
    SpawnHuman: Fn(&mut R, &SpawnHumanArg) -> Option<Human>,
  >(
    rng: &mut R,
    ctx: &mut GlobalCtx,
    clr: usize,
    blazonclr: usize,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    w: f32,
    xflip: bool,
    blazon: Blazon,
    human_density: f32,
    spawn_human: &SpawnHuman,
    boatglobs: &BoatGlobals,
  ) -> Self {
    let boat = Boat::init(
      rng, origin, size, angle, w, xflip, blazon, clr, blazonclr, boatglobs,
    );

    let xdir = if xflip { -1.0 } else { 1.0 };
    let acos = angle.cos();
    let asin = angle.sin();
    let x1 = boat.x1;
    let x2 = boat.x2;
    let mut x = x1;
    let mut positions = vec![];
    let yhorizondy = (ctx.yhorizon - origin.1).abs();
    let spawn_prob = ((1.0 - 10.0 * yhorizondy / ctx.height) as f64)
      .max(0.00001)
      .min(0.8);
    let mut i = 0;
    if human_density > 0.0 {
      while x < x2 {
        let y = rng.gen_range(-0.1 * size..0.0);
        let p = (x, y);
        let p = (p.0 * acos + p.1 * asin, p.1 * acos - p.0 * asin);
        let p = (p.0 * xdir + origin.0, p.1 + origin.1);
        if i % 3 == 0 && boatglobs.can_spawn_ropes && rng.gen_bool(spawn_prob) {
          ctx.projectiles.add_attack(AttackOrigin::Rope(p, clr));
        }
        positions.push(p);
        x += rng.gen_range(0.15..0.25) * size / human_density;
        i += 1;
      }
    }

    let total = positions.len();
    let a = if !xflip {
      angle
    } else {
      angle_mirrored_on_x(angle)
    };
    let mut humans = vec![];
    let mut fires = vec![];

    if ctx.destruction_map.get_weight(origin) > 0.1
      && boat.with_mast
      && rng.gen_bool(ctx.fire_proba)
    {
      let o = (origin.0, origin.1 - rng.gen_range(2.0..3.0) * size);
      let rad = rng.gen_range(0.5..1.0) * size;
      fires.push(Fire::init(rng, 1, o, rad));
    }

    for (index, &origin) in positions.iter().enumerate() {
      let destruction = ctx.destruction_map.get_weight(origin);
      if destruction > 0.5 {
        if rng.gen_bool(0.2 * ctx.fire_proba) {
          let rad = destruction * rng.gen_range(0.1..0.3) * size;
          fires.push(Fire::init(
            rng,
            1,
            (origin.0, origin.1 - 0.2 * size),
            rad,
          ));
        }
      } else {
        let human = spawn_human(
          rng,
          &SpawnHumanArg {
            origin,
            size,
            angle: a,
            xflip,
            index,
            total,
          },
        );
        if let Some(human) = human {
          human.throw_projectiles(ctx);
          humans.push(human);
        }
      }
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
      fires,
    }
  }

  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    mask: &mut PaintMask,
  ) -> Polylines {
    let clr = self.clr;
    let boat = &self.boat;
    let humans = &self.humans;

    let mut routes = vec![];

    let halo_humans = 0.8;
    let halo_boat = 1.0;

    // FIRES
    for fire in self.fires.iter() {
      let rts = fire.render(ctx, mask);
      routes.extend(rts);
    }

    // HUMANS FOREGROUND
    let mut human_rts = vec![];
    for front in humans.iter() {
      let rts = front.render_foreground_only(rng, ctx, mask);
      human_rts.extend(rts);
    }
    for (_, rt) in &human_rts {
      mask.paint_polyline(rt, halo_humans);
    }
    routes.extend(human_rts);

    // BOAT FOREGROUND
    let main_boat_rts = boat.render_main_only(mask, clr);
    routes.extend(main_boat_rts.clone());

    // HUMANS BACKGROUND
    let mut human_rts = vec![];
    for human in humans.iter() {
      let rts = human.render_background_only(rng, ctx, mask);
      human_rts.extend(rts);
    }
    for (_, rt) in &human_rts {
      mask.paint_polyline(rt, halo_humans);
    }
    routes.extend(human_rts);

    // BOAT BACKGROUND
    let background_boat_rts = boat.render_background_only(rng, mask, clr);
    routes.extend(background_boat_rts.clone());

    // we also create a halo around
    for (_, route) in &main_boat_rts {
      mask.paint_polyline(route, halo_boat);
    }
    for (_, route) in &background_boat_rts {
      mask.paint_polyline(route, halo_boat);
    }

    routes
  }
}

impl<R: Rng> Renderable<R> for BoatArmy {
  fn render(
    &self,
    rng: &mut R,
    ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(rng, ctx, paint)
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
