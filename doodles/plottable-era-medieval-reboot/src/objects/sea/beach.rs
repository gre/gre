use super::port::Port;
use crate::{
  algo::{
    clipping::regular_clip,
    paintmask::PaintMask,
    polylines::Polylines,
    renderable::{Container, Renderable},
    wormsfilling::WormsFilling,
  },
  global::{GlobalCtx, Special},
  objects::{
    army::{
      boat::{Boat, BoatGlobals},
      trojanhorse::TrojanHorse,
    },
    blazon::Blazon,
    palmtree::PalmTree,
  },
};
use noise::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Beach {
  width: f32,
  trees: Vec<PalmTree>,
  port: Option<Port>,
  clr: usize,
  portclr: usize,
  attacker_blazon: Blazon,
  port_boats_count: usize,
  yhorizon: f32,
}

impl Beach {
  pub fn init<R: Rng>(
    _ctx: &mut GlobalCtx,
    rng: &mut R,
    paint: &mut PaintMask,
    yhorizon: f32,
    width: f32,
    clr: usize,
    treeclr: usize,
    portclr: usize,
    attacker_blazon: Blazon,
  ) -> Self {
    let mut avoiding_area = paint.clone_rescaled(2.0);
    let trees_count = (rng.gen_range(-1.0f32..1.0) * rng.gen_range(0.0..30.0))
      .max(0.0) as usize;
    let port_boats_count = (rng.gen_range(-2.0f32..5.5)).max(0.0) as usize;
    let port = if rng.gen_bool(0.3) {
      let size = rng.gen_range(0.1..0.3) * width;
      let origin = (width * rng.gen_range(0.2..0.8), yhorizon);
      avoiding_area.paint_rectangle(
        origin.0 - size / 2.0,
        origin.1 - 10.0,
        origin.0 + size / 2.0,
        origin.1 + 10.0,
      );
      Some(Port::init(portclr, origin, size))
    } else {
      None
    };
    let mut trees = vec![];
    for _ in 0..trees_count {
      let origin = (width * rng.gen_range(0.1..0.9), yhorizon);
      if !avoiding_area.is_painted(origin) {
        let size = rng.gen_range(0.03..0.04) * width;
        avoiding_area.paint_circle(origin.0, origin.1, 0.5 * size);
        trees.push(PalmTree::init(treeclr, origin, size));
      }
    }
    Self {
      width,
      trees,
      port,
      clr,
      portclr,
      attacker_blazon,
      port_boats_count,
      yhorizon,
    }
  }

  pub fn render<R: Rng>(
    &self,
    ctx: &mut GlobalCtx,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Polylines {
    let width = self.width;

    let mut container = Container::new();

    for tree in &self.trees {
      container.add(tree.clone());
    }
    let boatsize: f32 = rng.gen_range(4.0..5.0);
    let (xc, boaty, boatw, distribwidth, angamp) = if let Some(port) = self.port
    {
      container.add(port);
      (
        port.origin.0
          + rng.gen_range(-0.2..0.2) * rng.gen_range(0.0..1.0) * port.size,
        self.yhorizon + boatsize,
        port.size * rng.gen_range(0.3..0.6),
        port.size,
        0.1,
      )
    } else {
      (
        0.5 * width,
        self.yhorizon - 2.0,
        width * 0.1,
        width * 0.8,
        rng.gen_range(0.1..0.3),
      )
    };

    let mut boatglobs = BoatGlobals::rand(rng, false);
    boatglobs.mast_p /= 2.0;
    boatglobs.sailing_p /= 2.0;

    for i in 0..self.port_boats_count {
      let w = boatw * rng.gen_range(0.8..1.2);
      let xflip = rng.gen_bool(0.5);
      let blazon = self.attacker_blazon;
      let origin = (
        xc + rng.gen_range(-0.5..0.5) * distribwidth,
        boaty + (i as f32 + rng.gen_range(0.0..1.0)) * 0.3 * boatsize,
      );
      let ang = rng.gen_range(-1.0..1.0) * angamp;
      let boat = Boat::init(
        rng,
        origin,
        boatsize,
        ang,
        w,
        xflip,
        blazon,
        self.portclr,
        self.portclr,
        &boatglobs,
      );
      container.add(boat);
    }

    if ctx.specials.contains(&Special::TrojanHorse) {
      // TODO ? in that case there should be a lot of people on the beach.
      let origin = (
        width * rng.gen_range(0.3..0.7),
        self.yhorizon - rng.gen_range(0.0..0.04) * paint.height,
      );
      let xflip = rng.gen_bool(0.5);
      let size = rng.gen_range(0.1..0.2) * paint.height;
      let h = TrojanHorse::init(rng, origin, size, xflip, 1);
      container.add(h);
    }

    let mut routes = vec![];

    routes.extend(container.render(rng, ctx, paint));

    routes.extend(beach_rendering(rng, paint, self.clr, self.yhorizon, width));

    routes
  }
}

fn beach_rendering<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  clr: usize,
  yhorizon: f32,
  width: f32,
) -> Polylines {
  let mut drawings = paint.clone_empty();
  let mut routes = vec![];

  let perlin = Perlin::new(rng.gen());
  let amp = rng.gen_range(1.0..5.0);
  let freq = rng.gen_range(3.0..6.0);
  let seed = rng.gen_range(0.0..999.0);
  let pad = 1.0;
  drawings.paint_columns_left_to_right(|x| {
    let n = perlin.get([freq * x as f64 / width as f64, seed]) as f32;
    let y = yhorizon + (amp + pad) * (0.5 + 0.5 * n);
    (yhorizon - pad * (0.5 + 0.5 * n))..y
  });

  let mut filling = WormsFilling::rand(rng);
  filling.step = 0.35;
  filling.max_l = 40;
  let iterations = 10000;
  routes.extend(filling.fill_in_paint(
    rng,
    &drawings,
    clr,
    3.0,
    (0.0, yhorizon - pad, width, yhorizon + amp + pad),
    iterations,
  ));

  routes = regular_clip(&routes, paint);
  paint.paint(&drawings);
  routes
}
