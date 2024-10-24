use super::{
  poles::PoleKind,
  shapes::{
    bartizan::Bartizan,
    bell::Bell,
    bridges::{Bridges, BridgesParams},
    merlon::Merlon,
    pillars::Pillars,
    roof::{Roof, RoofParams},
    wall::{Wall, WallParams},
    walltransition::WallTransition,
    zigzaggrid::ZigZagGrid,
  },
  Floor, Level, LevelParams, RenderItem,
};
use crate::{
  algo::{
    math1d::mix, math2d::lerp_point, paintmask::PaintMask,
    polylines::Polylines, renderable::Renderable,
    renderitem::multicut_along_line,
  },
  global::GlobalCtx,
  objects::{
    army::{
      body::HumanPosture,
      fire::Fire,
      flag::Flag,
      human::{HeadShape, HoldableObject, Human},
    },
    mountains::CastleGrounding,
    projectile::attack::DefenseTarget,
  },
};
use rand::prelude::*;
use std::{collections::HashMap, f32::consts::PI};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct GlobalCastleProperties {
  pub extra_towers: usize,
  // ybase is where the chapel foundation need to start
  pub ybase: f32,
  // where we absolutely need to stop building
  pub ymax: f32,
  //
  pub grounding: CastleGrounding,
  pub reference_roof_params: RoofParams,
  pub light_x_direction: f32,
  pub first_only_choices_extra: Vec<usize>,
  pub generic_choices_extra: Vec<usize>,
  pub shapes_proba: Vec<f32>,
}
impl GlobalCastleProperties {
  pub fn rand<R: Rng>(
    rng: &mut R,
    ctx: &GlobalCtx,
    grounding: &CastleGrounding,
    ybase: f32,
    ymax: f32,
    extra_towers: usize,
  ) -> Self {
    let reference_roof_params = RoofParams::rand(rng, ctx);
    let dx = grounding.position.0 - ctx.sun_xpercentage_pos * ctx.width;
    let light_x_direction = (dx / (0.1 * ctx.width)).max(-1.0).min(1.0);
    let mut first_only_choices_extra = vec![];
    let mut generic_choices_extra = vec![];

    if grounding.is_on_water {
      first_only_choices_extra.push(8);
    } else if rng.gen_bool(0.02) {
      first_only_choices_extra.push(8);
    }

    if rng.gen_bool(0.02) {
      generic_choices_extra.push(8);
    }

    let mut shapes_proba = vec![];
    for _ in 0..9 {
      shapes_proba.push(rng.gen_range(-0.5f32..1.5).max(0.001).min(0.999));
    }

    Self {
      grounding: grounding.clone(),
      reference_roof_params,
      light_x_direction,
      ybase,
      ymax,
      extra_towers,
      first_only_choices_extra,
      generic_choices_extra,
      shapes_proba,
    }
  }
}

/*
struct Tower {
  origin: (f32, f32),
  castleprops: GlobalCastleProperties,
  width: f32,
}

impl Tower {
  fn new(
    castleprops: &GlobalCastleProperties,
    origin: (f32, f32),
    width: f32,
  ) -> Self {
    Self {
      castleprops: castleprops.clone(),
      origin,
      width,
    }
  }

  fn render_to_render_item(
    &self,
    paint: &mut PaintMask,
    floors: &mut Vec<Floor>,
    ctx: &mut GlobalCtx,
    rng: &mut impl Rng,
    objects: &mut HashMap<usize, Box<dyn Renderable<impl Rng>>>,
  ) -> RenderItem {
    let mut items = rec_build(
      0,
      floors,
      rng,
      ctx,
      paint,
      &self.castleprops,
      self.origin,
      self.width,
      objects,
    );
    items.sort();
    RenderItem::new_from_items(items)
  }
}
*/

fn apply_destruction<R: Rng>(
  items: &mut Vec<RenderItem>,
  rng: &mut R,
  ctx: &mut GlobalCtx,
  castleprops: &GlobalCastleProperties,
) {
  let from;
  let to;
  {
    // iterate through the items to actually precise the avg x and top/bottom y
    let mut miny = ctx.height;
    let mut maxy = 0.0;
    let mut sumx = 0.0;
    let mut countx = 0;
    for item in items.iter() {
      for (_, poly) in &item.routes {
        for p in poly {
          if p.1 < miny {
            miny = p.1;
          }
          if p.1 > maxy {
            maxy = p.1;
          }
          sumx += p.0;
          countx += 1;
        }
      }
    }
    if countx == 0 {
      return;
    }
    sumx /= countx as f32;
    from = (sumx, maxy);
    to = (sumx, miny);
  }

  let mid = lerp_point(from, to, 0.5);
  let destruction = ctx.destruction_map.get_weight(mid);
  if destruction <= 0.001 {
    return;
  }

  let scale = castleprops.grounding.scale.min(0.3 * ctx.width);
  let pushbackbase =
    destruction * 0.1 * rng.gen_range(0.0..scale) * rng.gen_range(0.0..1.0);
  let pushbackrotbase = rng.gen_range(-1.0..1.0) * rng.gen_range(0.0..1.0);
  let pushbackrotmix = rng.gen_range(0.1..0.9);
  let sliding = scale * rng.gen_range(0.0..0.8);
  let increment_base = rng.gen_range(4.0..50.0);
  let o = multicut_along_line(
    rng,
    &items,
    0,
    from,
    to,
    |rng| increment_base * rng.gen_range(1.0..2.0),
    |rng| rng.gen_range(-PI / 2.0..PI / 2.0) * rng.gen_range(0.0..1.0),
    |rng| sliding * rng.gen_range(-1.0..1.0) * rng.gen_range(0.0..1.0),
    |rng| pushbackbase * rng.gen_range(0.5..1.0),
    |rng| 0.1 * mix(pushbackrotbase, rng.gen_range(-1.0..1.0), pushbackrotmix),
  );

  *items = o;
}

fn rec_build<R: Rng>(
  rec_level: usize,
  floors: &mut Vec<Floor>,
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  castleprops: &GlobalCastleProperties,
  origin: (f32, f32),
  width: f32,
  objects: &mut HashMap<usize, Box<dyn Renderable<R>>>,
) -> Vec<RenderItem> {
  let ybase = castleprops.ybase;
  let ymax = castleprops.ymax;
  let scale = castleprops.grounding.scale;

  let mut splits = vec![];
  if width < 20.0 * scale {
    let count = (rng.gen_range(0.0..3.0) * rng.gen_range(0.0..1.0)) as usize;
    for _ in 0..count {
      splits.push(rng.gen_range(0.2..0.8));
    }
  }

  let floor = Floor::new(origin, width, splits, true);
  floors.push(floor.clone());

  let initial_h = origin.1 - ymax;
  let max_levels = rng.gen_range(2..14);
  let grow_factor = 1.0 / (max_levels as f32);
  let grow_constant = rng.gen_range(0.0..0.2) * initial_h;

  let regular_tower_width = 6.0 * scale;
  let minw = 0.6 * regular_tower_width;
  let minh = 4.0 * scale;

  let mut params = LevelParams {
    tower_seed: rng.gen(),
    reference_roof_params: castleprops.reference_roof_params.clone(),
    rec_level,
    level: 0,
    scaleref: scale,
    blazonclr: ctx.defendersclr,
    clr: 0,
    floor,
    max_height: initial_h,
    level_zorder: 0.0,
    preferrable_height: 0.0,
    lowest_y_allowed: ybase,
    light_x_direction: castleprops.light_x_direction,
  };

  let level_max_allowed_width = vec![
    Roof::max_allowed_width(scale),
    Wall::max_allowed_width(scale),
    WallTransition::max_allowed_width(scale),
    Merlon::max_allowed_width(scale),
    ZigZagGrid::max_allowed_width(scale),
    Pillars::max_allowed_width(scale),
    Bartizan::max_allowed_width(scale),
    Bell::max_allowed_width(scale),
    Bridges::max_allowed_width(scale),
  ];

  let max_in_a_tower = vec![
    // 0 roof
    1,
    // 1 wall
    usize::MAX,
    // 2 wall transition
    usize::MAX,
    // 3 merlon
    3,
    // 4 zigzag
    3,
    // 5 pillars
    1,
    // 6 bartizans
    5,
    // 7 bell
    2,
    // 8 bridge (special)
    1,
  ];

  let forbidden_on_top_of: Vec<Vec<usize>> = vec![
    // 0 roof
    vec![0],
    // 1 wall
    vec![1],
    // 2 wall transition
    vec![2],
    // 3 merlon
    vec![0, 1, 2, 3, 4, 5, 6, 7],
    // 4 zigzag
    vec![3, 4, 5],
    // 5 pillars
    vec![2, 4, 3, 5, 6, 7],
    // 6 bartizans
    vec![3, 5, 6, 7],
    // 7 bell
    vec![6, 7],
    // 8 bridge (special)
    vec![8],
  ];
  let roof_choices = vec![0, 3];
  let mut first_only_choices = (1..2).collect::<Vec<_>>();
  first_only_choices.extend(castleprops.first_only_choices_extra.clone());

  let mut generic_choices = (0..8).collect::<Vec<_>>();
  generic_choices.extend(castleprops.generic_choices_extra.clone());

  let mut forbidden_structure = vec![];

  let mut items = vec![];
  let mut possible_bg_human_positions = vec![];
  let mut possible_pole_positions = vec![];

  let mut remainings = max_in_a_tower.clone();

  for l in 0..max_levels {
    // We determine the next possible shape to do
    let is_first = l == 0;
    let is_roof = l == max_levels - 1;

    let choices = if is_first {
      first_only_choices.clone()
    } else if is_roof {
      roof_choices.clone()
    } else {
      generic_choices.clone()
    };
    let choices = choices
      .iter()
      .filter(|&i| {
        !forbidden_structure.contains(i)
          && remainings[*i] > 0
          && params.floor.width < level_max_allowed_width[*i]
      })
      .cloned()
      .collect::<Vec<_>>();
    if choices.is_empty() {
      // RECURSIVE BUILD UP
      if rec_level == 0 || rng.gen_bool(0.8) {
        // on floor 1, we can have the opportinity to split into multiple towers
        let percent = rng.gen_range(0.7..0.9);
        let allw = params.floor.width;
        let remains = allw * percent;
        if remains >= 2.0 * regular_tower_width {
          let count = (rng.gen_range(0.8..1.2) * remains / regular_tower_width)
            .max(2.0)
            .min(10.0) as usize;

          // find an interesting distribution of splits
          let mut splits = Vec::new();
          if rng.gen_bool(0.7) && count > 3 {
            splits.push((1, -2.0));
            splits.push((count - 2, -3.0));
            splits.push((1, -2.0));
          } else if rng.gen_bool(0.7) && count > 4 {
            let h = count / rng.gen_range(2..4);
            splits.push((h, -2.0));
            splits.push((count - 2 * h, -3.0));
            splits.push((h, -2.0));
          } else {
            for i in 0..count {
              splits.push((1, -(i as f32) - 1.0));
            }
            splits.shuffle(rng);
          }

          let mut castleprops = castleprops.clone();
          if l > 0 {
            castleprops.first_only_choices_extra = vec![];
            castleprops.ybase = params.floor.pos.1;
            castleprops.grounding = CastleGrounding {
              position: params.floor.pos,
              width: params.floor.width,
              scale: castleprops.grounding.scale,
              is_on_water: false,
              main_door_pos: None,
              moats: vec![],
            };
          }

          // make the new floors
          let divs = count as f32;
          let m = remains / divs;
          let mut wsum = 0;
          for (weight, zordermul) in splits {
            if rng.gen_bool(0.8) {
              let width = m * weight as f32;
              let pos = params.floor.pos;
              let origin = (
                pos.0 - allw / 2.0
                  + (wsum as f32 + weight as f32 / 2.0) * allw / divs,
                pos.1,
              );
              let tower = rec_build(
                rec_level + 1,
                &mut vec![],
                rng,
                ctx,
                paint,
                &castleprops,
                origin,
                width,
                objects,
              );
              // we are remapping the zorder in order to integrate in the bigger picture.
              let tower = tower
                .into_iter()
                .map(|mut item| {
                  item.zorder += 10.0 * zordermul;
                  item
                })
                .collect::<Vec<_>>();

              items.extend(tower);
            }

            wsum += weight;
          }
          break;
        }
      }
    }

    if choices.is_empty() {
      break;
    }

    // find a choice depending on probabilities
    let sum = choices
      .iter()
      .map(|&i| castleprops.shapes_proba[i])
      .sum::<f32>();
    if sum < 0.001 {
      break;
    }
    let mut v = rng.gen_range(0.0..sum);
    let i = choices.iter().position(|&i| {
      let p = castleprops.shapes_proba[i];
      if p > v {
        true
      } else {
        v -= p;
        false
      }
    });

    if i.is_none() {
      break;
    }
    let i = i.unwrap();

    // let i = rng.gen_range(0..choices.len());
    let levelkind = choices[i];
    remainings[levelkind] -= 1;
    forbidden_structure = forbidden_on_top_of[levelkind].clone();

    // a leaf happens if there is not enough space. (TODO: we need to sometimes "close" with a ceil still...?)
    if params.max_height <= minh || params.floor.width <= minw {
      break;
    }
    params.preferrable_height = (grow_constant
      + grow_factor * (params.floor.pos.1 - ymax))
      .min(params.max_height);

    let level: Box<dyn Level> = match levelkind {
      0 => {
        let roofparams =
          RoofParams::from_reference(rng, ctx, &params.reference_roof_params);
        Box::new(Roof::init(rng, &params, &roofparams))
      }
      1 => {
        let mut wallparams = WallParams::new();
        wallparams.fill_to_lowest_y_allowed = l == 0;

        let is_main_floor = l == 0 && rec_level == 0;
        if is_main_floor {
          wallparams.with_door = castleprops.grounding.main_door_pos;
          wallparams.moats = castleprops.grounding.moats.clone();
        }

        Box::new(Wall::init(rng, paint, &params, &wallparams))
      }
      2 => Box::new(WallTransition::init(rng, &params, l + 2 == max_levels)),
      3 => Box::new(Merlon::init(rng, &params)),
      4 => Box::new(ZigZagGrid::init(rng, &params)),
      5 => Box::new(Pillars::init(rng, &params)),
      6 => Box::new(Bartizan::init(rng, ctx, paint, &params)),
      7 => Box::new(Bell::init(rng, ctx, &params)),
      8 => {
        let mut bridgeparams = BridgesParams::new(rng);
        bridgeparams.fill_to_lowest_y_allowed = l == 0;
        Box::new(Bridges::init(rng, paint, &params, &bridgeparams))
      }
      _ => panic!("unknown level kind"),
    };

    items.extend(level.render());
    possible_bg_human_positions
      .extend(level.possible_background_human_positions());
    possible_pole_positions.extend(level.possible_pole_positions());

    for pos in level.possible_ladder_positions() {
      ctx.projectiles.add_defense(DefenseTarget::Ladder(pos));
    }
    for pos in level.possible_rope_attachment_positions() {
      ctx.projectiles.add_defense(DefenseTarget::Rope(pos));
    }

    for s in level.possible_fire_start_positions() {
      let destr = ctx.destruction_map.get_weight(s.pos);
      if destr > 0.1 && rng.gen_bool(ctx.fire_proba) {
        let rad = (rng.gen_range(0.5..1.2) * destr * s.radius).min(10.0);
        let fire = Fire::init(rng, 1, s.pos, rad);
        let id = objects.len();
        objects.insert(id, Box::new(fire));
        items.push(RenderItem::from_foreign(id, s.zorder, s.pos));
      }
    }

    if let Some(floor) = level.roof_base() {
      let middle = lerp_point(floor.pos, params.floor.pos, 0.5);
      floors.push(floor.clone());

      ctx.projectiles.add_defense(DefenseTarget::Building(middle));

      if let Some(y) = level.condamn_build_belowy() {
        params.lowest_y_allowed = y;
      }
      params.max_height = (floor.pos.1 - ymax).max(0.0);
      params.floor = floor;
      params.level_zorder += 1.0;
      params.level += 1;
    } else {
      break;
    }
  }

  if !params.floor.is_closed {
    // closing with a simple line
    let p = params.floor.pos;
    let w = params.floor.width;
    items.push(RenderItem::new(
      vec![(params.clr, vec![(p.0 - w / 2.0, p.1), (p.0 + w / 2.0, p.1)])],
      vec![],
      params.level_zorder + 0.5,
    ))
  }

  for spawn in possible_bg_human_positions {
    if rng.gen_bool(0.5) {
      continue;
    }
    let blazon = ctx.defenders;
    let blazonclr = ctx.defendersclr;
    let xflip = rng.gen_bool(0.5);
    let lefthand = Some(if rng.gen_bool(0.5) {
      HoldableObject::LongBow(rng.gen_range(0.0..1.0))
    } else if ctx.night_time {
      HoldableObject::Torch
    } else {
      HoldableObject::Flag
    });
    let righthand = None;
    let head = HeadShape::NAKED;
    let posture = HumanPosture::from_holding(rng, xflip, lefthand, righthand);
    let s = scale * rng.gen_range(3.0..4.0);

    let human = Human::init(
      rng, spawn.pos, s, xflip, blazon, 0, blazonclr, posture, head, lefthand,
      righthand,
    );

    let id = objects.len();
    objects.insert(id, Box::new(human));
    ctx.projectiles.add_defense(DefenseTarget::Human(spawn.pos));
    items.push(RenderItem::from_foreign(id, spawn.zorder, spawn.pos));
  }

  for spawn in possible_pole_positions {
    if let Some(item) =
      spawn.kind.render(spawn.pos, 0, spawn.size, spawn.zorder)
    {
      items.push(item);
    } else if matches!(spawn.kind, PoleKind::Flag) {
      let cloth_height_factor = rng.gen_range(0.4..0.5);
      let cloth_len_factor = rng.gen_range(0.5..1.0);
      let flagtoleft = true;
      let flag = Flag::init(
        rng,
        0,
        ctx.defendersclr,
        spawn.pos,
        spawn.size * 5.0,
        -PI / 2.0,
        flagtoleft,
        cloth_height_factor,
        cloth_len_factor,
        false,
      );
      let id = objects.len();
      objects.insert(id, Box::new(flag));
      items.push(RenderItem::from_foreign(id, spawn.zorder, spawn.pos));
    }
  }

  if rng.gen_bool(0.3) {
    apply_destruction(&mut items, rng, ctx, castleprops);
  }

  items.sort();

  items
}

pub fn build_castle<R: Rng>(
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  castleprops: &GlobalCastleProperties,
) -> Polylines {
  let halo = 1.4;
  let mut routes = vec![];

  let mut levels = vec![];

  // Build the main castle
  {
    let mut objects = HashMap::new();
    let mut items = rec_build(
      0,
      &mut levels,
      rng,
      ctx,
      paint,
      castleprops,
      castleprops.grounding.position,
      castleprops.grounding.width,
      &mut objects,
    );
    items.sort();
    for item in &items {
      routes.extend(item.render(paint));
      if let Some(foreign) = &item.foreign {
        if let Some(obj) = objects.get_mut(&foreign.id) {
          obj.apply_translation_rotation(foreign.translation, foreign.rotation);
          routes.extend(obj.render(rng, ctx, paint));
        }
      }
    }

    // we also create a halo cropping around castle
    for (_, route) in &routes {
      paint.paint_polyline(route, halo);
    }
  }

  // try to get floor 1 and use it as ref
  let (refpos, refwidth) = levels
    .get(1)
    .map(|floor| {
      let pos = floor.pos;
      let width = floor.width;
      (pos, width)
    })
    .unwrap_or((castleprops.grounding.position, castleprops.grounding.width));

  let mut castleprops = castleprops.clone();
  castleprops.first_only_choices_extra = vec![];
  castleprops.ybase = refpos.1;
  castleprops.grounding = CastleGrounding {
    position: refpos,
    width: refwidth,
    scale: castleprops.grounding.scale,
    is_on_water: false,
    main_door_pos: None,
    moats: vec![],
  };

  // build extra towers
  for _ in 0..castleprops.extra_towers {
    let mut objects = HashMap::new();

    let width = rng.gen_range(0.2..0.5) * refwidth;
    let mut origin = refpos;
    origin.0 += rng.gen_range(-0.5..0.5) * (refwidth - width);

    let mut items = rec_build(
      1,
      &mut vec![],
      rng,
      ctx,
      paint,
      &castleprops,
      origin,
      width,
      &mut objects,
    );
    items.sort();

    for item in &items {
      routes.extend(item.render(paint));
      if let Some(foreign) = &item.foreign {
        if let Some(obj) = objects.get_mut(&foreign.id) {
          obj.apply_translation_rotation(foreign.translation, foreign.rotation);
          routes.extend(obj.render(rng, ctx, paint));
        }
      }
    }

    // we also create a halo cropping around castle
    for (_, route) in &routes {
      paint.paint_polyline(route, halo);
    }
  }

  routes
}
