use super::{
  bartizan::Bartizan,
  bell::Bell,
  merlon::Merlon,
  pillars::Pillars,
  poles::PoleKind,
  roof::RoofParams,
  wall::{Wall, WallParams},
  walltransition::WallTransition,
  zigzaggrid::ZigZagGrid,
  Floor, Level, LevelParams, RenderItem,
};
use crate::{
  algo::{paintmask::PaintMask, polylines::Polylines, renderable::Renderable},
  global::GlobalCtx,
  objects::{
    army::{
      body::HumanPosture,
      flag::Flag,
      human::{HeadShape, HoldableObject, Human},
    },
    castle::levels::roof::Roof,
  },
};
use rand::prelude::*;
use std::{collections::HashMap, f32::consts::PI};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

fn rec_build<R: Rng>(
  toplevels: usize,
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  origin: (f32, f32),
  width: f32,
  ybase: f32,
  ymax: f32,
  scale: f32,
  objects: &mut HashMap<usize, Box<dyn Renderable<R>>>,
) -> Vec<RenderItem> {
  // TODO we could give in param a probabilistic config
  // also most of the rng parts of the shape should be params

  /*
  let mut level = SimpleWall::init(rng, ctx, params);
  levels.push(Box::new(level) as Box<dyn Level>);
  */

  let mut splits = vec![];
  if width < 20.0 * scale {
    let count = (rng.gen_range(0.0..3.0) * rng.gen_range(0.0..1.0)) as usize;
    for _ in 0..count {
      splits.push(rng.gen_range(0.2..0.8));
    }
  }

  let floor = Floor::new(origin, width, splits, false);

  let max_levels = rng.gen_range(3..6);

  let regular_tower_width = 6.0 * scale;
  let minw = 0.6 * regular_tower_width;
  let minh = 4.0 * scale;

  let initial_h = origin.1 - ymax;

  // TODO depends on sun pos
  let light_direction = rng.gen_range(-2.0f32..2.0).max(-1.0).min(1.0);
  let mut params = LevelParams {
    tower_seed: rng.gen(),
    level: 0,
    scaleref: scale,
    blazonclr: ctx.defendersclr,
    clr: 0,
    floor,
    max_height: initial_h,
    level_zorder: 0.0,
    preferrable_height: 0.0,
    lowest_y_allowed: ybase,
    light_x_direction: light_direction,
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
  ];

  let forbidden_on_top_of: Vec<Vec<usize>> = vec![
    // 0 roof
    vec![0],
    // 1 wall
    vec![],
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
    vec![6],
    // TODO stairs
  ];
  let roof_choices = vec![0, 3];
  let first_only_choices = (1..2).collect::<Vec<_>>();
  let generic_choices = (0..forbidden_on_top_of.len()).collect::<Vec<_>>();
  let mut forbidden_structure = vec![];

  let mut items = vec![];
  let mut possible_bg_human_positions = vec![];
  let mut possible_pole_positions = vec![];

  // TODO in future i'm not sure we should be driven by max_levels, we could increase with a random size and determine appropriate time to stop?
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
          && params.floor.width < level_max_allowed_width[*i]
      })
      .cloned()
      .collect::<Vec<_>>();
    if choices.is_empty() {
      break;
    }
    let i = rng.gen_range(0..choices.len());
    let levelkind = choices[i];
    forbidden_structure = forbidden_on_top_of[levelkind].clone();

    // RECURSIVE HERE. TO BE OUT OF THE FUNCTION?

    /*
    if toplevels == 0 && l == 0
      || l > 1 && (toplevels == 0 || rng.gen_bool(0.5))
    {
      // on floor 1, we can have the opportinity to split into multiple towers
      let percent = if toplevels == 0 && l == 0 {
        1.01
      } else {
        rng.gen_range(0.7..0.9)
      };
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

        // make the new floors
        let divs = count as f32;
        let m = remains / divs;
        let mut wsum = 0;
        for (weight, zordermul) in splits {
          let width = m * weight as f32;
          let pos = params.floor.pos;
          let origin = (
            pos.0 - allw / 2.0
              + (wsum as f32 + weight as f32 / 2.0) * allw / divs,
            pos.1,
          );
          let tower = rec_build(
            toplevels + 1,
            rng,
            ctx,
            paint,
            origin,
            width,
            ybase,
            ymax,
            scale,
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
          wsum += weight;
        }
        break;
      }
    }
    */

    // a leaf happens if there is not enough space. (TODO: we need to sometimes "close" with a ceil still...?)
    if params.max_height <= minh || params.floor.width <= minw {
      break;
    }
    params.preferrable_height =
      ((rng.gen_range(0.8..1.0) / (max_levels as f32 + 1.0)) * initial_h)
        .min(params.max_height);

    let level: Box<dyn Level> = match levelkind {
      0 => {
        // TODO in future it's shared.
        let roofparams = RoofParams::rand(rng, ctx);
        Box::new(Roof::init(&params, &roofparams))
      }
      1 => {
        let mut wallparams = WallParams::new();
        wallparams.fill_to_lowest_y_allowed = l == 0;

        Box::new(Wall::init(rng, paint, &params, &wallparams))
      }
      2 => Box::new(WallTransition::init(rng, &params, l + 2 == max_levels)),
      3 => Box::new(Merlon::init(rng, &params)),
      4 => Box::new(ZigZagGrid::init(rng, &params)),
      5 => Box::new(Pillars::init(rng, &params)),
      6 => Box::new(Bartizan::init(rng, ctx, paint, &params)),
      7 => Box::new(Bell::init(rng, ctx, &params)),
      _ => panic!("unknown level kind"),
    };

    items.extend(level.render());
    possible_bg_human_positions
      .extend(level.possible_background_human_positions());
    possible_pole_positions.extend(level.possible_pole_positions());

    if let Some(floor) = level.roof_base() {
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

  for spawn in possible_bg_human_positions {
    let blazon = ctx.defenders;
    let blazonclr = ctx.defendersclr;
    let xflip = rng.gen_bool(0.5);
    let lefthand = Some(HoldableObject::Flag);
    let righthand = None;
    let head = HeadShape::NAKED;
    let posture = HumanPosture::from_holding(rng, xflip, lefthand, righthand);
    let angle = 0.0;
    let s = scale * rng.gen_range(3.0..4.0);

    let human = Human::init(
      rng, spawn.pos, s, angle, xflip, blazon, 0, blazonclr, posture, head,
      lefthand, righthand,
    );

    let id = objects.len();
    objects.insert(id, Box::new(human));
    items.push(RenderItem::from_foreign(id, spawn.zorder));
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
      // FIXME this should global on the castle i think
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
      items.push(RenderItem::from_foreign(id, spawn.zorder));
    }
  }

  // TODO work on the destruction of items with the destruction map
  // we need to somehow preserve the items but move them as we go slicing things. might be tricky.

  items.sort();

  items
}

pub fn build_castle<R: Rng>(
  rng: &mut R,
  ctx: &mut GlobalCtx,
  paint: &mut PaintMask,
  origin: (f32, f32),
  width: f32,
  ybase: f32,
  ymax: f32,
  scale: f32,
) -> Polylines {
  let mut objects = HashMap::new();
  let mut items = rec_build(
    0,
    rng,
    ctx,
    paint,
    origin,
    width,
    ybase,
    ymax,
    scale,
    &mut objects,
  );

  // TODO work on the destruction of items with the destruction map

  items.sort();

  let mut routes = vec![];
  for item in &items {
    routes.extend(item.render(paint));
    if let Some(id) = item.foreign_id {
      if let Some(obj) = objects.get(&id) {
        routes.extend(obj.render(rng, paint));
      }
    }
  }

  // halo around the tower
  let halo = 1.0;
  for item in items {
    for poly in &item.polygons {
      paint.paint_polyline(poly, halo);
    }
  }

  routes
}
