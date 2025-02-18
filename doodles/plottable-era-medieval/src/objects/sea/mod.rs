use super::{
  army::{
    boat::BoatGlobals,
    boatarmy::{BoatArmy, SpawnHumanArg},
    body::HumanPosture,
    human::{HeadShape, HoldableObject, Human},
    sword::Sword,
  },
  blazon::Blazon,
  castle::chinesedoor::ChineseDoor,
  rock::Rock,
};
use crate::{
  algo::{
    clipping::{clip_routes_with_colors, regular_clip},
    math1d::mix,
    paintmask::PaintMask,
    passage::Passage,
    polylines::{slice_polylines, Polylines},
    renderable::{as_box_renderable, Container},
  },
  global::{GlobalCtx, Special},
  objects::sea::sauron::SauronEye,
};
use rand::prelude::*;
use std::f32::consts::PI;

pub mod beach;
pub mod port;
pub mod sauron;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Sea {
  pub sea_mask: PaintMask,
  pub yhorizon: f32,
  pub blazon: Blazon,
}

pub struct SeaReflectableObject {
  pub routes: Vec<(usize, Vec<(f32, f32)>)>,
  pub ybase: f32,
  pub xcenter: f32,
  pub width: f32,
  pub height: f32,
}

pub struct SeaRendered {
  pub routes: Vec<(usize, Vec<(f32, f32)>)>,
  pub objects: Vec<SeaReflectableObject>,
}

impl Sea {
  pub fn from(paint: &PaintMask, yhorizon: f32, blazon: Blazon) -> Self {
    let mut sea_mask = paint.clone();
    sea_mask.paint_fn(&|(_, y)| y < yhorizon);

    Self {
      yhorizon,
      sea_mask,
      blazon,
    }
  }

  pub fn reflect_shapes<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
    reflectables: &Vec<(usize, Vec<(f32, f32)>)>,
    sea_routes: &SeaRendered,
    probability_per_color: Vec<f32>,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    let prec = 0.5;
    let mut passage = Passage::new(prec, paint.width, paint.height);
    let is_below_sea_level = |(_x, y): (f32, f32)| y > self.yhorizon;
    let reflectables =
      clip_routes_with_colors(&reflectables, &is_below_sea_level, 0.5, 3);

    let stroke_len_base = 0.04 * paint.width;

    let ydistortion = rng.gen_range(0.5..1.0);
    let ydisplacement = rng.gen_range(0.1..0.2) * paint.height;
    let xdisplacement = rng.gen_range(0.1..0.3) * paint.width;

    let max_passage = 3;
    let boundaries = (0.0, 0.0, paint.width, paint.height);

    for obj in &sea_routes.objects {
      let ratio = obj.width / paint.width;
      routes.extend(reflect_shapes(
        rng,
        &obj.routes,
        &mut passage,
        &probability_per_color,
        stroke_len_base * ratio.max(0.4),
        obj.ybase,
        boundaries,
        max_passage,
        xdisplacement * ratio,
        ydisplacement * ratio,
        ydistortion,
      ));

      // shadow under the object
      let w = obj.width;
      let h = obj.height;
      let mut x = obj.xcenter - w / 2.0;
      let xmax = x + w;
      while x < xmax {
        let mut y = obj.ybase; // - h / 2.0;
        let ymax = y + h / 2.0;
        let dx = (x - obj.xcenter) / (w / 2.0);
        while y < ymax {
          let dy = (y - obj.ybase) / (h / 2.0);
          let d = dx * dx + dy * dy;
          if d < 1.0 {
            let v = max_passage + 1;
            passage.set((x, y), v);
          }
          y += prec;
        }
        x += prec;
      }
    }

    routes.extend(reflect_shapes(
      rng,
      &reflectables,
      &mut passage,
      &probability_per_color,
      stroke_len_base,
      self.yhorizon,
      boundaries,
      max_passage,
      xdisplacement,
      ydisplacement,
      ydistortion,
    ));

    regular_clip(&routes, paint)
  }

  pub fn render<R: Rng + 'static>(
    &mut self,
    ctx: &mut GlobalCtx,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> SeaRendered {
    let width = paint.width;
    let height = paint.height;

    let no_boats = ctx.specials.contains(&Special::TrojanHorse);

    let rocks_count = (rng.gen_range(0.0f32..14.0) * rng.gen_range(-0.5..1.0))
      .max(0.0) as usize;
    let boats_count = if no_boats {
      0
    } else {
      let min = if ctx.castle_on_sea { 0.0 } else { -0.3 };
      (rng.gen_range(0.0f32..20.0) * rng.gen_range(min..1.0)).max(0.0) as usize
    };

    // Place rocks
    let mut sea_shapes = Container::new();

    // this mask is used to find location to pack things

    let mut should_set_excalibur = rng.gen_bool(0.1) && ctx.specials.is_empty();

    let tries = 10;
    let mut nb_rocks = 0;
    for _ in 0..rocks_count {
      for _ in 0..tries {
        let x = width * rng.gen_range(0.0..1.0);
        let yp = rng.gen_range(0.0..1.0);
        let y = mix(self.yhorizon, height, yp);
        if self.sea_mask.is_painted((x, y)) {
          continue;
        }

        let size =
          (0.04 + rng.gen_range(0.0..0.04) * rng.gen_range(0.0..1.0)) * width;
        let minx = x - size;
        let maxx = x + size;
        let miny = y - size;
        let maxy = y;
        self.sea_mask.paint_rectangle(minx, miny, maxx, maxy);
        let origin = (x, y);
        let elevation = 1.5 + rng.gen_range(0.0..5.0) * rng.gen_range(0.0..1.0);
        let count_poly =
          (rng.gen_range(0.8..1.2) * (elevation * 5. + 3.)) as usize;

        let rockclr = if rng.gen_bool(0.02) { 1 } else { 0 };

        let excalibur = if should_set_excalibur
          && elevation > 4.0
          && (0.3..0.7).contains(&(x / width))
        {
          should_set_excalibur = false;
          true
        } else {
          false
        };

        let mut rock =
          Rock::init(rng, origin, size, rockclr, count_poly, elevation);

        nb_rocks += 1;

        rock.spawn_on_top(rng, &mut |rng: &mut R, o: (f32, f32), s, a| {
          if !(0.3..0.7).contains(&(o.0 / width)) {
            return None;
          }
          if excalibur {
            ctx.specials.insert(Special::Excalibur);
            let clr = rng.gen_range(0..2);
            Some(as_box_renderable(Sword::init(rng, o, s, a, clr)))
          } else {
            let dy = origin.1 - o.1;
            if rng.gen_bool(0.12) && dy > 0.3 * width {
              let sauron = SauronEye::init(rng, paint, rockclr, 1, o, s);
              ctx.specials.insert(Special::Sauroned);
              return Some(as_box_renderable(sauron));
            }
            None
          }
        });

        sea_shapes.add(rock);
        break;
      }
    }

    if nb_rocks < 2 && ctx.specials.contains(&Special::Chinese) {
      let x = width * rng.gen_range(0.2..0.8);
      let y = mix(self.yhorizon, height, rng.gen_range(0.0..0.5));
      let o = (x, y);
      let h = rng.gen_range(0.1..0.2) * width;
      let w = h * rng.gen_range(1.5..2.0);
      let angle = 0.0;
      let clr = if rng.gen_bool(0.05) { 1 } else { 0 };
      let door = ChineseDoor::init(rng, clr, o, w, h, angle);
      sea_shapes.add(door);
    }

    // Place boats

    let mut should_spawn_supreme_leader = rng.gen_bool(0.3);
    let boatglobs = BoatGlobals::rand(rng, ctx.castle_on_sea);
    let tries = 10;
    let basew = width * rng.gen_range(0.15..0.25);
    for _ in 0..boats_count {
      for _ in 0..tries {
        let x = width * rng.gen_range(0.2..0.8);
        let yp = rng.gen_range(0.1..1.0);
        let y = mix(self.yhorizon, height, yp);
        let w =
          basew * (1.0 + rng.gen_range(-0.4..0.8) * rng.gen_range(0.0..1.0));
        let size = width * mix(0.03, 0.08, yp);
        if self.sea_mask.is_painted((x, y)) {
          continue;
        }
        let minx = x - w / 2.0;
        let maxx = x + w / 2.0;
        let miny = y - w / 6.0;
        let maxy = y + w / 6.0;
        self.sea_mask.paint_rectangle(minx, miny, maxx, maxy);

        let angle = rng.gen_range(-0.2..0.2) * rng.gen_range(0.0..1.0);
        let xflip = if rng.gen_bool(0.8) {
          x > width / 2.0
        } else {
          rng.gen_bool(0.5)
        };

        let color_overrides = if rng.gen_bool(0.05) {
          if rng.gen_bool(0.7) {
            Some(1)
          } else {
            Some(2)
          }
        } else {
          None
        };
        let clr = color_overrides.unwrap_or(0);
        let blazonclr = color_overrides.unwrap_or(2);

        let has_supreme_leader = should_spawn_supreme_leader;
        should_spawn_supreme_leader = false;

        let regular_weapon = if rng.gen_bool(0.5) {
          HoldableObject::Sword
        } else if rng.gen_bool(0.6) {
          HoldableObject::Axe
        } else if rng.gen_bool(0.7) {
          HoldableObject::Club
        } else {
          HoldableObject::Flag
        };

        let leader_weapon = if rng.gen_bool(0.7) {
          regular_weapon
        } else if rng.gen_bool(0.5) {
          HoldableObject::LongSword
        } else {
          HoldableObject::Flag
        };

        let are_paddling = rng.gen_bool(0.8);
        let have_shields = rng.gen_bool(0.8);

        let go_archer_only = rng.gen_bool(0.1);
        let go_archer_lower_index = rng.gen_bool(0.5);
        let archers_split = rng.gen_range(2..10);

        let spawn_human = |rng: &mut R, arg: &SpawnHumanArg| {
          let mut size = arg.size;
          let mut mainclr = clr;
          let mut blazonclr = blazonclr;

          let is_flag_man = arg.index == 0;

          let is_archer = !is_flag_man
            && (go_archer_only
              || go_archer_lower_index
                && arg.index < arg.total / archers_split);

          let is_leader = arg.index == arg.total - 1;

          // one room for leader
          if arg.index == arg.total - 2 {
            return None;
          }

          let mut origin = arg.origin;

          let mut lefthand = if have_shields {
            Some(HoldableObject::Shield)
          } else {
            None
          };
          let mut headshape = HeadShape::HELMET;
          let righthand;
          let posture;
          if is_leader {
            if has_supreme_leader {
              size *= rng.gen_range(1.3..1.6);
              mainclr = 1;
              blazonclr = 1;
            }
            if rng.gen_bool(0.5) {
              lefthand = None;
            }
            righthand = Some(leader_weapon);
            posture = HumanPosture::hand_risen(rng);
          } else if is_archer {
            origin.1 -= size * rng.gen_range(0.0..0.3);
            lefthand = None;
            headshape = HeadShape::NAKED;
            righthand = Some(HoldableObject::LongBow(
              rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
            ));
            posture =
              HumanPosture::from_holding(rng, arg.xflip, lefthand, righthand);
          } else if is_flag_man {
            origin.1 -= size * rng.gen_range(0.0..0.3);
            if rng.gen_bool(0.5) {
              lefthand = None;
            }
            righthand = Some(HoldableObject::Flag);
            posture =
              HumanPosture::from_holding(rng, arg.xflip, lefthand, righthand);
          } else {
            if are_paddling {
              let a = if arg.xflip {
                -PI * rng.gen_range(0.6..0.7)
              } else {
                -PI * rng.gen_range(0.3..0.4)
              };
              righthand = Some(HoldableObject::Paddle(a));
            } else {
              righthand = Some(regular_weapon);
            };
            posture =
              HumanPosture::from_holding(rng, arg.xflip, lefthand, righthand)
          }

          let human = Human::init(
            rng,
            origin,
            size,
            arg.xflip,
            self.blazon,
            mainclr,
            blazonclr,
            posture,
            headshape,
            lefthand,
            righthand,
          );
          Some(human)
        };
        let human_density = rng.gen_range(0.5..1.0);
        let boat = BoatArmy::init(
          rng,
          ctx,
          clr,
          blazonclr,
          (x, y),
          size,
          angle,
          w,
          xflip,
          self.blazon,
          human_density,
          &spawn_human,
          &boatglobs,
        );
        sea_shapes.add(boat);
        break;
      }
    }

    let mut objects = vec![];
    let mut visitor = |rts: &Polylines, ybase: f32| {
      let firstp = rts.get(0).and_then(|(_, r)| r.get(0));
      if let Some(firstp) = firstp {
        let mut minx = firstp.0;
        let mut maxx = firstp.0;
        let mut miny = firstp.1;
        let mut maxy = firstp.1;
        for (_, r) in rts {
          for p in r {
            minx = minx.min(p.0);
            maxx = maxx.max(p.0);
            miny = miny.min(p.1);
            maxy = maxy.max(p.1);
          }
        }
        let width = maxx - minx;
        let height = maxy - miny;
        let xcenter = minx + width / 2.0;
        objects.push(SeaReflectableObject {
          routes: rts.clone(),
          ybase,
          xcenter,
          width,
          height,
        });
      }
    };

    let routes = sea_shapes.render_with_visitor(
      rng,
      ctx,
      paint,
      &|r| !r.sea_reflectable_is_disabled(),
      &mut visitor,
    );

    SeaRendered { routes, objects }
  }
}

fn reflect_shapes<R: Rng>(
  rng: &mut R,
  reflectables: &Vec<(usize, Vec<(f32, f32)>)>,
  passage: &mut Passage,
  probability_per_color: &Vec<f32>,
  stroke_len_base: f32,
  ycenter: f32,
  boundaries: (f32, f32, f32, f32),
  max_passage: usize,
  xdisplacement: f32,
  ydisplacement: f32,
  ydistortion: f32,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut new_shapes = Vec::new();

  let min_stroke_length = 0.5 * stroke_len_base;
  let max_stroke_length = stroke_len_base;

  for (clr, route) in reflectables.iter() {
    let probability = probability_per_color[*clr];
    for p in slice_polylines(&route, rng.gen_range(1.0..2.0) * stroke_len_base)
      .iter()
      .flatten()
    {
      if !rng.gen_bool(probability as f64) {
        continue;
      }
      let sx = (min_stroke_length
        + (max_stroke_length - min_stroke_length)
          * rng.gen_range(0f32..1.0).powi(2))
        / 2.0;
      let sy = 0.3 * rng.gen_range(-1.0..1.0) * rng.gen_range(0.0..1.0);
      let x = p.0
        + rng.gen_range(0.0..xdisplacement)
          * rng.gen_range(0.0..1.0)
          * rng.gen_range(-1.0..1.0);
      let y = ycenter
        + (ycenter - p.1) * rng.gen_range((1.0 - ydistortion)..1.0)
        + rng.gen_range(0.0..ydisplacement) * rng.gen_range(-1.0..1.0);
      if y > ycenter && y < boundaries.3 {
        let x1 = (x - sx).max(boundaries.0).min(boundaries.2);
        let x2 = (x + sx).max(boundaries.0).min(boundaries.2);
        if x2 - x1 > min_stroke_length {
          if passage.get((x, y)) > max_passage {
            continue;
          }
          passage.count((x, y));
          new_shapes.push((*clr, vec![(x1, y - sy), (x2, y + sy)]));
        }
      }
    }
  }
  new_shapes
}
