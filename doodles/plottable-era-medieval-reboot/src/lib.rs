mod algo;
mod effects;
mod frame;
mod fxhash;
mod global;
mod objects;
mod palette;
mod performance;
mod svgplot;

use algo::clipping::clip_routes_with_colors;
use algo::math1d::mix;
use algo::packing::packing;
use algo::polylines::path_subdivide_to_curve;
use algo::polylines::shake;
use algo::polylines::Polylines;
use fxhash::*;
use global::GlobalCtx;
use global::Special;
use objects::army::flyingdragon::FlyingDragon;
use objects::army::ArmyOnMountain;
use objects::blazon::get_duel_houses;
use objects::castle::Castle;
use objects::mountains::front::FrontMountains;
use objects::mountains::*;
use objects::sea::beach::Beach;
use objects::sea::Sea;
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

  perf.span("epic title", &decoration_routes);
  let txt = epic_title(&mut rng, &ctx);
  let fontsize = width / 26.0;
  let iterations = 5000;
  let density = 4.0;
  let growpad = 2.0;
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
    0,
    iterations,
    density,
    growpad,
  ));
  perf.span_end("epic title", &decoration_routes);

  perf.span("framing", &decoration_routes);
  let golden_frame = palette.inks[1] == GOLD_GEL && rng.gen_bool(0.3);
  let clr = if golden_frame { 1 } else { 0 };
  decoration_routes.extend(medieval_frame(
    &mut rng, &ctx, &mut paint, width, height, pad, framingw, clr,
  ));
  perf.span_end("framing", &decoration_routes);

  // sandbox when developing
  sandbox(&mut rng, &mut paint, &mut routes, width, height);

  // Front elements
  for s in &ctx.specials {
    match s {
      Special::Dragon(n) => {
        perf.span("dragon", &routes);
        routes.extend(dragon(
          &mut rng, &mut paint, width, height, pad, framingw, *n,
        ));
        perf.span_end("dragon", &routes);
      }
      _ => {}
    }
  }

  let mask_with_framing = paint.clone();

  let yhorizon = if ctx.no_sea {
    height
  } else {
    // TODO tweak
    rng.gen_range(0.5..0.8) * height
  };
  //  mountains
  perf.span("mountains_front", &routes);
  let ystart = if ctx.no_sea {
    rng.gen_range(0.7..1.0) * height
  } else {
    mix(yhorizon, height, rng.gen_range(0.0..1.0))
  };
  let ybase = height - pad;
  let clr = 0;
  let mut mountains = FrontMountains {
    clr,
    ybase,
    ystart,
    width,
  };
  routes.extend(mountains.render(&mut ctx, &mut rng, &mut paint));
  perf.span_end("mountains_front", &routes);

  let sea_data = if !ctx.no_sea {
    perf.span("sea", &vec![]);
    let boat_color = 0;
    let mut sea = Sea::from(&paint, yhorizon, boat_color, attacker_house);
    let sea_routes = sea.render(&mut ctx, &mut rng, &mut paint);
    perf.span_end("sea", &sea_routes);
    let mut water = sea.sea_mask.clone();
    water.reverse();
    ctx.effects.water.paint(&water);
    Some((sea, sea_routes))
  } else {
    None
  };

  if ctx.castle_on_sea {
    // TODO make a structure, pilori, etc.. or sometimes just rocks.
    // TODO multiple castle, we would loop through them. they would be placed on the sea but we would enforce there is no unit behind.
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
    };
    perf.span("castle", &routes);
    let ymax = pad + framingw + 0.02 * height;
    let extra_towers =
      rng.gen_range(-1.0f32..20.0 * castle.width / width).max(0.0) as usize;
    let castle =
      Castle::init(&mut ctx, &mut rng, &castle, yhorizon, ymax, extra_towers);
    routes.extend(castle.render(&mut ctx, &mut rng, &mut paint));
    perf.span_end("castle", &routes);
  } else {
    perf.span("mountains", &routes);
    let ymax = mix(0.0, yhorizon, rng.gen_range(0.4..0.6));
    let count = rng.gen_range(2..8);
    let first_is_second = ctx.palette.inks[0] == ctx.palette.inks[1];
    let countextra = if rng.gen_bool(if first_is_second { 0.01 } else { 0.2 }) {
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
        let extra_towers =
          rng.gen_range(-1.0f32..20.0 * castle.width / width).max(0.0) as usize;
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
  let mut skysafemask = paint.clone();
  skysafemask.unpaint_borders(pad + framingw);
  skysafemask.paint_fn(|(_x, y)| y > yhorizon);
  skysafemask.dilate_manhattan(rng.gen_range(0.0..0.1) * width);
  let mut skysafemask1 = skysafemask.clone();
  skysafemask1.paint_borders(pad + framingw);
  skysafemask.dilate_manhattan(rng.gen_range(0.0..0.2) * width);
  let mut skysafemask2 = skysafemask.clone();
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
    let probability_par_color = vec![0.08, 0.1, 0.2];
    routes.extend(sea.reflect_shapes(
      &mut rng,
      &mut paint,
      &routes,
      probability_par_color,
    ));
    perf.span_end("reflect_shapes", &routes);

    routes.extend(sea_routes);
  }

  perf.span("projectiles", &routes);
  ctx.render_projectiles(&mut rng, &mut routes, &paint, &mask_with_framing);
  perf.span_end("projectiles", &routes);

  routes.extend(decoration_routes);

  perf.span("finalize", &vec![]);

  //  routes.extend(debug_weight_map(&ctx.destruction_map, 2, 0.0, 1.0));

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

fn sandbox<R: Rng>(
  _rng: &mut R,
  _paint: &mut PaintMask,
  _routes: &mut Polylines,
  _width: f32,
  _height: f32,
) {
  /*
  for _ in 0..50 {
    let mut rt = vec![];
    let c = (
      rng.gen_range(0.3..0.7) * width,
      rng.gen_range(0.3..0.7) * height,
    );
    for _ in 0..rng.gen_range(2..7) {
      rt.push((
        c.0 + rng.gen_range(-0.2..0.2) * paint.width,
        c.1 + rng.gen_range(-0.2..0.2) * paint.height,
      ))
    }
    rt = path_subdivide_to_curve(&rt, 1, 0.6);
    rt = shake(rt, 0.2 * paint.width, rng);
    rt = path_subdivide_to_curve(&rt, 1, 0.7);
    rt = shake(rt, 0.1 * paint.width, rng);
    rt = path_subdivide_to_curve(&rt, 1, 0.8);
    let clr = rng.gen_range(0..3);
    let size = rng.gen_range(0.05..0.1) * width;
    let step = rng.gen_range(1.0..2.0);
    let count =
      4 + (rng.gen_range(0.0..20.0) * rng.gen_range(0.0..1.0)) as usize;
    let angleoff = rng.gen_range(-0.3..0.3)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(-0.5f32..1.0).max(0.0);
    routes.extend(
      FlyingDragon::init(rng, clr, &rt, size, step, count, angleoff)
        .render(paint),
    );
  }
  */

  /*
  for _ in 0..500 {
    let h = rng.gen_range(10.0..20.0);
    let w = h * rng.gen_range(1.5..2.0);
    let ang = rng.gen_range(-3.0..3.)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0);
    // - std::f32::consts::PI / 2.0;
    let o = (
      rng.gen_range(0.1..0.9) * width,
      rng.gen_range(0.1..0.9) * height,
    );
    let clr = rng.gen_range(0..3);
    // let xflip = rng.gen_bool(0.5);
    // let cloth_height_factor = rng.gen_range(0.3..0.6);
    // let cloth_len_factor = rng.gen_range(0.3..1.0);
    let rts = Star::init(rng, clr, o, 20.0).render(paint);
    routes.extend(rts.clone());
    for (_, rt) in rts.iter() {
      paint.paint_polyline(&rt, 1.0);
    }
  }
  */

  /*
  for _ in 0..10 {
    // let h = rng.gen_range(10.0..20.0);
    // let w = h * rng.gen_range(1.5..2.0);
    let ang = rng.gen_range(-3.0..3.)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0);
    // - std::f32::consts::PI / 2.0;
    let o = (
      rng.gen_range(0.1..0.9) * width,
      rng.gen_range(0.1..0.9) * height,
    );
    //let clr = rng.gen_range(0..3);
    let xflip = rng.gen_bool(0.5);
    // let cloth_height_factor = rng.gen_range(0.3..0.6);
    // let cloth_len_factor = rng.gen_range(0.3..1.0);
    let lefthand = Some(objects::army::human::HoldableObject::Flag);
    let righthand: Option<objects::army::human::HoldableObject> = None;
    let head = HeadShape::NAKED;
    let posture = HumanPosture::from_holding(rng, xflip, lefthand, righthand);
    let s = rng.gen_range(5.0..30.0);
    let mut human = Human::init(
      rng,
      o,
      s,
      ang,
      xflip,
      objects::blazon::Blazon::Dragon,
      0,
      2,
      posture,
      head,
      lefthand,
      righthand,
    )
    .with_worms_filling_defaults();
    let rts = human.render(rng, paint);
    routes.extend(rts.clone());
    for (_, rt) in rts.iter() {
      paint.paint_polyline(&rt, 1.0);
    }
  }
  */
}

/*
fn debug_weight_map(
  weightmap: &WeightMap,
  clr: usize,
  from: f32,
  to: f32,
) -> Polylines {
  let mut routes = vec![];

  let mut x = 0.0;
  while x < weightmap.width {
    let mut y = 0.0;
    while y < weightmap.height {
      let w = weightmap.get_weight((x, y));
      let v = smoothstep(from, to, w);
      if v > 0.0 {
        let xc = x + 0.5 * weightmap.precision;
        let yc = y + 0.5 * weightmap.precision;
        let s = v * 0.5 * weightmap.precision;

        let mut r = s;
        while r > 0.0 {
          routes.push((
            clr,
            vec![
              (xc - r, yc - r),
              (xc + r, yc - r),
              (xc + r, yc + r),
              (xc - r, yc + r),
              (xc - r, yc - r),
            ],
          ));
          r -= 0.2;
        }
      }
      y += weightmap.precision;
    }
    x += weightmap.precision;
  }

  routes
}
*/

fn dragon<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  width: f32,
  height: f32,
  pad: f32,
  framingw: f32,
  n: usize,
) -> Polylines {
  let mut routes = vec![];
  for i in 0..n {
    let bx = pad + framingw + 0.2 * width;
    let by = pad + framingw + 0.2 * height;
    let count = rng.gen_range(2..16);
    let mut circles = packing(
      rng,
      500,
      count,
      1,
      0.05 * width,
      (bx, by, width - bx, height - by),
      &|_| true,
      0.01 * width,
      0.1 * width,
    );
    circles.sort_by(|a, b| b.y.partial_cmp(&a.y).unwrap());

    let mut rt = vec![];
    for c in circles {
      rt.push((c.x, c.y));
    }

    while rt.len() < 2 {
      rt.push((
        rng.gen_range(0.33..0.66) * paint.width,
        rng.gen_range(0.2..0.5) * paint.height,
      ));
    }
    for _ in 0..rng.gen_range(1..3) {
      rt = path_subdivide_to_curve(&rt, 1, 0.66);
      let s = rng.gen_range(0.0..0.1) * paint.width;
      rt = shake(rt, s, rng);
    }
    rt = path_subdivide_to_curve(&rt, 1, 0.7);
    rt = path_subdivide_to_curve(&rt, 1, 0.8);

    let size = rng.gen_range(0.04..0.08) * width;
    let step = rng.gen_range(1.0..2.0);
    let count =
      4 + (rng.gen_range(0.0..20.0) * rng.gen_range(0.0..1.0)) as usize;
    let angleoff = rng.gen_range(-0.3..0.3)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(0.0..1.0)
      * rng.gen_range(-0.5f32..1.0).max(0.0);
    routes.extend(
      FlyingDragon::init(rng, (i + 2) % 3, &rt, size, step, count, angleoff)
        .render(paint),
    );
  }
  routes
}
