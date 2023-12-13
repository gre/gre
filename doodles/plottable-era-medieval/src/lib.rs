mod algo;
mod effects;
mod frame;
mod fxhash;
mod global;
mod objects;
mod palette;
mod performance;
mod sandbox;
mod svgplot;

use algo::clipping::clip_routes_with_colors;
use algo::math1d::mix;
use algo::renderable::*;
use fxhash::*;
use global::GlobalCtx;
use global::Special;
use objects::army::ArmyOnMountain;
use objects::blazon::get_duel_houses;
use objects::castle::Castle;
use objects::mountains::front::FrontMountains;
use objects::mountains::*;
use objects::rock::Rock;
use objects::sea::beach::Beach;
use objects::sea::Sea;
use objects::sky::dragons::dragons;
use objects::sky::MedievalSky;
use palette::Palette;
use palette::GOLD_GEL;
use rand::prelude::*;
use svgplot::*;
use wasm_bindgen::prelude::*;
mod epictitle;
use algo::paintmask::*;
use algo::text::*;
use epictitle::*;
use frame::medieval_frame;
use performance::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[wasm_bindgen]
pub fn render(
  hash: String,
  width: f32,
  height: f32,
  pad: f32,
  precision: f32,
  fontdata: Vec<u8>,
  mask_mode: bool,
  debug: bool,
) -> String {
  let mut perf = PerfRecords::start(debug);

  perf.span("init", &vec![]);
  let mut rng = rng_from_hash(&hash);
  let mut font = load_font(&fontdata);

  let mut paint = PaintMask::new(precision, width, height);

  // Colors
  let (attacker_house, defender_house) = get_duel_houses(&mut rng);
  let palette = Palette::init(&mut rng, attacker_house);
  let mut ctx = GlobalCtx::rand(
    &mut rng,
    &paint,
    width,
    height,
    precision,
    palette.clone(),
    &defender_house,
    &attacker_house,
  );

  // Make the scene
  let mut routes = vec![];

  let mut decoration_routes = vec![];
  let framingw = 0.05 * width;

  perf.span_end("init", &routes);

  let golden_frame =
    palette.inks[1] == GOLD_GEL && rng.gen_bool(0.3) || rng.gen_bool(0.02);

  if !ctx.is_sandbox {
    perf.span("epic title", &decoration_routes);
    let txt = epic_title(&mut rng, &ctx);
    let fontsize = width / 24.0;
    let iterations = 5000;
    let density = 4.0;
    let growpad = 2.0;
    let clr = if golden_frame && rng.gen_bool(0.5) || rng.gen_bool(0.01) {
      1
    } else {
      0
    };
    decoration_routes.extend(draw_font_with_worms_filling(
      &mut rng,
      &mut font,
      &mut paint,
      fontsize,
      (
        pad + framingw + 0.6 * fontsize,
        height - pad - framingw - 1.8 * fontsize,
      ),
      txt.as_str(),
      clr,
      iterations,
      density,
      growpad,
    ));
    perf.span_end("epic title", &decoration_routes);
  }

  if !ctx.is_sandbox || rng.gen_bool(0.2) {
    perf.span("framing", &decoration_routes);
    let clr = if golden_frame { 1 } else { 0 };
    decoration_routes.extend(medieval_frame(
      &mut rng, &ctx, &mut paint, width, height, pad, framingw, clr,
    ));
    perf.span_end("framing", &decoration_routes);
  } else {
    paint.paint_borders(pad);
  }

  if ctx.is_sandbox {
    sandbox::sandbox(
      &mut rng,
      &mut ctx,
      &mut paint,
      &mut routes,
      width,
      height,
    );
  } else {
    // Front elements
    for s in &ctx.specials {
      match s {
        Special::Dragon(n) => {
          perf.span("dragon", &routes);
          routes.extend(dragons(
            &mut rng, &mut paint, width, height, pad, framingw, *n,
          ));
          perf.span_end("dragon", &routes);
        }
        _ => {}
      }
    }

    let mask_with_framing = paint.clone();
    let yhorizon = ctx.yhorizon;

    //  front mountains
    if rng.gen_bool(0.3) {
      perf.span("mountains_front", &routes);
      let ybase = height - pad - framingw;
      let ystart = if ctx.no_sea {
        (rng.gen_range(0.7..1.0) * height).min(ybase - 0.05 * height)
      } else {
        mix(yhorizon, ybase, rng.gen_range(-0.2..0.9))
      };

      let clr = 0;
      let mut mountains = FrontMountains {
        clr,
        ybase,
        ystart,
        width,
      };
      routes.extend(mountains.render(&mut ctx, &mut rng, &mut paint));
      perf.span_end("mountains_front", &routes);
    }

    let sea_data = if !ctx.no_sea {
      perf.span("sea", &vec![]);
      let mut sea = Sea::from(&paint, yhorizon, attacker_house);
      let sea_routes = sea.render(&mut ctx, &mut rng, &mut paint);
      perf.span_end("sea", &sea_routes.routes);
      let mut water = sea.sea_mask.clone();
      water.reverse();
      ctx.effects.water.paint(&water);
      Some((sea, sea_routes))
    } else {
      None
    };

    if ctx.castle_on_sea {
      perf.span("castle", &routes);
      let castlewidth = rng.gen_range(0.2..0.5) * width;
      let x = rng.gen_range(0.3..0.6) * width;
      let scale = 1.0
        + rng.gen_range(-0.2..0.4) * rng.gen_range(0.0..1.0)
        + rng.gen_range(0.0..(1.0 * castlewidth / width));
      let castle = CastleGrounding {
        position: (x, yhorizon),
        width: castlewidth,
        moats: vec![],
        main_door_pos: None,
        scale,
        is_on_water: true,
      };
      let mut items = Container::new();
      let count = rng.gen_range(1..20);
      let skipf = rng.gen_range(0.2..0.8);
      for i in 0..count {
        if rng.gen_bool(skipf) {
          continue;
        }
        let dx = castlewidth
          * if count == 1 {
            0.0
          } else {
            i as f32 / (count - 1) as f32 - 0.5
          };
        let p = (x + dx, yhorizon + rng.gen_range(0.0..0.05) * castlewidth);
        let size = rng.gen_range(0.05..0.1) * castlewidth;
        let elevation = rng.gen_range(0.0..1.0)
          + rng.gen_range(0.0..3.0) * rng.gen_range(0.0..1.0);
        let count_poly =
          (rng.gen_range(0.8..1.2) * (elevation * 5. + 3.)) as usize;
        let rock = Rock::init(&mut rng, p, size, 0, count_poly, elevation);
        items.add(rock);
      }
      routes.extend(items.render(&mut rng, &mut ctx, &mut paint));

      let ymax = pad + framingw + 0.02 * height;
      let extra_towers =
        rng.gen_range(-1.0f32..20.0 * castle.width / width).max(0.0) as usize;
      let castle =
        Castle::init(&mut ctx, &mut rng, &castle, yhorizon, ymax, extra_towers);
      routes.extend(castle.render(&mut ctx, &mut rng, &mut paint));
      perf.span_end("castle", &routes);
    } else {
      perf.span("mountains", &routes);
      let ymax = mix(
        0.0,
        mix(yhorizon, 0.5 * height, rng.gen_range(0.2..0.7)),
        rng.gen_range(if ctx.no_sea { 0.3 } else { 0.5 }..1.0),
      );
      let count =
        2 + (rng.gen_range(0.0..10.0) * rng.gen_range(0.0..1.0)) as usize;
      let first_is_second = ctx.palette.inks[0] == ctx.palette.inks[1];
      let countextra = if rng.gen_bool(if first_is_second { 0.01 } else { 0.2 })
      {
        rng.gen_range(1..10)
      } else {
        0
      };
      let mountains = MountainsV2::rand(
        &mut rng, &ctx, 0, width, height, yhorizon, ymax, count, countextra,
      );
      perf.span_end("mountains", &routes);

      let army: ArmyOnMountain = ArmyOnMountain::init(attacker_house);

      for (i, mountain) in mountains.mountains.iter().enumerate() {
        if mountain.has_beach {
          perf.span("beach", &routes);
          let beach = Beach::init(
            &mut ctx,
            &mut rng,
            &mut paint,
            yhorizon,
            width,
            0,
            0,
            0,
            attacker_house,
          );
          routes.extend(beach.render(&mut ctx, &mut rng, &mut paint));
          perf.span_end("beach", &routes);
        }

        perf.span("attackers", &routes);
        routes.extend(
          army.render(&mut ctx, &mut rng, &mut paint, &mountain, &mountains, i),
        );
        perf.span_end("attackers", &routes);

        perf.span("mountains", &routes);
        routes.extend(mountain.render(&mut paint));
        perf.span_end("mountains", &routes);

        if let Some(castle) = &mountain.castle {
          perf.span("castle", &routes);
          let ymax = pad + framingw + 0.02 * height;
          let extra_towers = rng
            .gen_range(-1.0f32..20.0 * castle.width / width)
            .max(0.0) as usize;
          let castle = Castle::init(
            &mut ctx,
            &mut rng,
            castle,
            yhorizon,
            ymax,
            extra_towers,
          );
          routes.extend(castle.render(&mut ctx, &mut rng, &mut paint));
          perf.span_end("castle", &routes);
        }
      }
    }

    perf.span("sky masks", &routes);
    let mut skysafemask = paint.clone_rescaled(4.0);
    skysafemask.unpaint_borders(pad + framingw);
    skysafemask.paint_fn(|(_x, y)| y > yhorizon);
    let distances = skysafemask.manhattan_distance();
    let mut skysafemask1 = skysafemask.clone();
    skysafemask1.assign_data_lower_than_threshold(
      &distances,
      rng.gen_range(0.05..0.1) * width,
    );
    skysafemask1.paint_borders(pad + framingw);
    let mut skysafemask2 = skysafemask.clone();
    skysafemask2.assign_data_lower_than_threshold(
      &distances,
      rng.gen_range(0.1..0.2) * width,
    );
    skysafemask2.paint_borders(pad + framingw);
    perf.span_end("sky masks", &routes);

    perf.span("sky", &routes);
    let sky = MedievalSky::rand(
      &mut ctx,
      &mut rng,
      &skysafemask1,
      &skysafemask2,
      width,
      height,
      pad,
    );

    let mut sky_routes = sky.render(&mut rng, &mut paint);
    if !ctx.no_sea {
      // prevent sky to glitch inside the sea
      let is_below_horizon = |(_x, y): (f32, f32)| y > yhorizon;
      sky_routes =
        clip_routes_with_colors(&sky_routes, &is_below_horizon, 1.0, 5);
    }
    routes.extend(sky_routes);
    perf.span_end("sky", &routes);

    if let Some((sea, sea_routes)) = sea_data {
      perf.span("reflect_shapes", &routes);
      let probability_par_color = vec![0.1, 0.15, 0.22];

      routes.extend(sea.reflect_shapes(
        &mut rng,
        &mut paint,
        &routes,
        &sea_routes,
        probability_par_color,
      ));
      perf.span_end("reflect_shapes", &routes);

      routes.extend(sea_routes.routes);
    }

    perf.span("projectiles", &routes);
    ctx.render_projectiles(&mut rng, &mut routes, &paint, &mask_with_framing);
    perf.span_end("projectiles", &routes);
  }

  routes.extend(decoration_routes);

  perf.span("finalize", &vec![]);

  ctx.finalize();

  let feature = ctx.to_feature(&routes);
  let feature_json = serde_json::to_string(&feature).unwrap();
  let palette_json: String = palette.to_json();
  let extra_attributes = vec![ctx.effects.to_svg_metafields()];

  let layers = make_layers_from_routes_colors(
    &routes,
    &palette.inks,
    mask_mode,
    2.0 * precision,
  );
  perf.span_end("finalize", &vec![]);

  let svg = make_document(
    hash.as_str(),
    feature_json,
    palette_json,
    width,
    height,
    mask_mode,
    palette.paper.1,
    &layers,
    if debug { Some(perf) } else { None },
    &extra_attributes,
  );

  svg
}
