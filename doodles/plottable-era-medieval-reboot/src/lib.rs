mod algo;
mod frame;
mod fxhash;
mod global;
mod objects;
mod palette;
mod performance;
mod svgplot;

use algo::clipping::clip_routes_with_colors;
use algo::math1d::mix;
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
use objects::sky::star::Star;
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

  perf.span("init");
  let mut rng = rng_from_hash(&hash);
  let mut font = load_font(&fontdata);

  // Colors
  let (attacker_house, defender_house) = get_duel_houses(&mut rng);
  let palette = Palette::init(&mut rng, attacker_house);
  let mut ctx = GlobalCtx::rand(
    &mut rng,
    width,
    height,
    precision,
    palette.clone(),
    &defender_house,
    &attacker_house,
  );

  // Make the scene
  let mut routes = vec![];
  let mut paint = PaintMask::new(precision, width, height);

  let mut decoration_routes = vec![];
  let framingw = 0.05 * width;

  perf.span_end("init");

  perf.span("epic title");
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
  perf.span_end("epic title");

  perf.span("framing");
  let golden_frame = palette.inks[1] == GOLD_GEL && rng.gen_bool(0.3);
  let clr = if golden_frame { 1 } else { 0 };
  decoration_routes.extend(medieval_frame(
    &mut rng, &mut paint, width, height, pad, framingw, clr,
  ));
  perf.span_end("framing");
  let mask_with_framing = paint.clone();

  // sandbox when developing
  sandbox(&mut rng, &mut paint, &mut routes, width, height);

  // Front elements
  // maybe we locate the object positions to position it more nicely?
  for s in &ctx.specials {
    match s {
      Special::Dragon(n) => {
        perf.span("dragon");
        for i in 0..*n {
          let mut rt = vec![];
          let c = (0.5 * width, 0.3 * height);
          // TODO less chaotic path. maybe doing a random zig zag?
          // TODO dragon throwing fireballs
          for _ in 0..rng.gen_range(3..5) {
            rt.push((
              c.0 + rng.gen_range(-0.25..0.25) * paint.width,
              c.1 + rng.gen_range(-0.2..0.2) * paint.height,
            ))
          }
          rt = path_subdivide_to_curve(&rt, 1, 0.66);
          rt = shake(rt, 0.1 * paint.width, &mut rng);
          rt = path_subdivide_to_curve(&rt, 1, 0.7);
          rt = shake(rt, 0.1 * paint.width, &mut rng);
          rt = path_subdivide_to_curve(&rt, 1, 0.7);
          rt = shake(rt, 0.1 * paint.width, &mut rng);
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
            FlyingDragon::init(
              &mut rng,
              (i + 2) % 3,
              &rt,
              size,
              step,
              count,
              angleoff,
            )
            .render(&mut paint),
          );
        }
        perf.span_end("dragon");
      }
      _ => {}
    }
  }

  // TODO ? allow some crazier cases where the yhorizon can be 20-40% but with more boats and possible battles in the sea
  let yhorizon = rng.gen_range(0.4..0.7) * height;

  //  mountains
  perf.span("mountains_front");
  // TODO better peaky shapes. not too annoying. can sometimes disappear
  let ystart = mix(yhorizon, height, rng.gen_range(0.0..1.0));
  let ybase = height - pad;
  let clr = 0;
  let mut mountains = FrontMountains {
    clr,
    ybase,
    ystart,
    width,
  };
  routes.extend(mountains.render(&mut ctx, &mut rng, &mut paint));
  perf.span_end("mountains_front");

  perf.span("sea");
  let boat_color = 0;
  let sea = Sea::from(&paint, yhorizon, boat_color, attacker_house);
  let sea_routes = sea.render(&mut ctx, &mut rng, &mut paint);
  perf.span_end("sea");

  perf.span("mountains");
  let ymax = mix(0.0, yhorizon, 0.6);
  let count = rng.gen_range(2..8);
  let mountains =
    MountainsV2::rand(&mut rng, 0, width, height, yhorizon, ymax, count);
  perf.span_end("mountains");

  let army: ArmyOnMountain = ArmyOnMountain::init(attacker_house);

  for (i, mountain) in mountains.mountains.iter().enumerate() {
    if mountain.has_beach {
      perf.span("beach");
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
      perf.span_end("beach");
    }

    perf.span("attackers");
    routes.extend(
      army.render(&mut ctx, &mut rng, &mut paint, &mountain, &mountains, i),
    );
    perf.span_end("attackers");

    perf.span("mountains");
    routes.extend(mountain.render(&mut paint));
    perf.span_end("mountains");

    if let Some(castle) = &mountain.castle {
      perf.span("castle");
      // TODO the randomness is to be done inside a Castle::rand()
      let castle = Castle {
        pos: castle.position,
        width: castle.width,
        scale: 1.0,
        clr: 0,
        ybase: yhorizon,
        wallh: rng.gen_range(0.05..0.1) * height,
        wall: true,
        left_tower: true,
        right_tower: true,
        dark_wall: rng.gen_bool(0.5),
        chapel: rng.gen_bool(0.5),
        dark_chapel: rng.gen_bool(0.5),
        destructed_wall: rng.gen_bool(0.5),
        portcullis: rng.gen_bool(0.5),
        blazon: defender_house,
      };
      routes.extend(castle.render(&mut ctx, &mut rng, &mut paint));
      perf.span_end("castle");
    }
  }

  perf.span("sky");
  let sky = MedievalSky::rand(&mut ctx, &mut rng, width, height, pad);
  // prevent sky to glitch inside the sea
  let is_below_horizon = |(_x, y): (f32, f32)| y > yhorizon;
  let sky_routes = clip_routes_with_colors(
    &sky.render(&mut rng, &mut paint),
    &is_below_horizon,
    1.0,
    5,
  );
  routes.extend(sky_routes);
  perf.span_end("sky");

  perf.span("reflect_shapes");
  let probability_par_color = vec![0.08, 0.1, 0.2];
  routes.extend(sea.reflect_shapes(
    &mut rng,
    &mut paint,
    &routes,
    probability_par_color,
  ));
  perf.span_end("reflect_shapes");

  routes.extend(sea_routes);

  perf.span("projectiles");
  ctx.render_projectiles(&mut rng, &mut routes, &mask_with_framing);
  perf.span_end("projectiles");

  routes.extend(decoration_routes);

  let feature = ctx.to_feature(&routes);
  let feature_json = serde_json::to_string(&feature).unwrap();
  let palette_json: String = palette.to_json();

  let layers = make_layers_from_routes_colors(
    &routes,
    &palette.inks,
    mask_mode,
    2.0 * precision,
  );

  let svg = make_document(
    hash.as_str(),
    feature_json,
    palette_json,
    width,
    height,
    mask_mode,
    palette.paper.1,
    &layers,
    Some(perf),
  );

  svg
}

fn sandbox<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  routes: &mut Polylines,
  width: f32,
  height: f32,
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
}
