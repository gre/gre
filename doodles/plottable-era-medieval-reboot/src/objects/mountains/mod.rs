use crate::{
  algo::{
    clipping::{clip_routes_with_colors, regular_clip},
    math1d::mix,
    math2d::{lookup_ridge, strictly_in_boundaries},
    moving_average::{center_vec_2d, moving_average_2d},
    paintmask::PaintMask,
    passage::Passage,
    polygon::polygon_includes_point,
    polylines::Polylines,
  },
  global::GlobalCtx,
};
use noise::*;
use rand::prelude::*;

use self::battlefield::BattlefieldArea;

pub mod battlefield;
pub mod front;
pub mod wall;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct CastleGrounding {
  // pub ridge: Vec<(f32, f32)>,
  pub position: (f32, f32),
  pub width: f32,
  // (from,to) positions of the path to climb possible moats (on left and/or right of castle)
  pub moats: Vec<((f32, f32), (f32, f32))>,
  pub main_door_pos: Option<(f32, f32)>,
  pub scale: f32,
  pub is_on_water: bool,
}

pub struct Mountain {
  precision: f32,
  pub clr: usize,
  // meta info for the objects we will need to draw inside mountains
  pub castle: Option<CastleGrounding>,
  pub ridge: Vec<(f32, f32)>,
  pub smoothed_ridge: Vec<(f32, f32)>,
  pub yhorizon: f32,
  pub width: f32,
  pub has_beach: bool,
  // info for the render time
  pub routes: Polylines,
  pub is_behind: bool,
  pub will_have_the_leader: bool,
}

impl Mountain {
  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let out = regular_clip(&self.routes, paint);
    let yhorizon = self.yhorizon;
    paint.paint_columns_left_to_right(&|x| {
      let yridge = lookup_ridge(&self.ridge, x).min(yhorizon);
      yridge..yhorizon
    });
    out
  }

  pub fn lookup_ridge_index(&self, x: f32) -> usize {
    ((x / self.precision).max(0.0) as usize).min(self.ridge.len() - 1)
  }

  pub fn ridge_pos_for_x(&self, x: f32) -> (f32, f32) {
    let i = self.lookup_ridge_index(x);
    self.ridge[i]
  }

  pub fn ground_stability(&self, (x, y): (f32, f32), width: f32) -> f32 {
    let currenti = self.lookup_ridge_index(x);
    let step = width / 2.0;
    let lefti = self.lookup_ridge_index(x - step);
    let righti = self.lookup_ridge_index(x + step);
    let current = self.smoothed_ridge[currenti];
    let left = self.smoothed_ridge[lefti];
    let right = self.smoothed_ridge[righti];
    let dx = right.0 - left.0;
    if dx <= 0.0 {
      return 0.0;
    }
    let dy = right.1 - left.1;
    // the obj is below
    if y < current.1 || y < left.1 || y < right.1 {
      return 0.0;
    }

    (0.2 * dx.abs() / (0.01 + dy.abs())).min(1.0)
  }

  pub fn slope_for_x(&self, x: f32) -> f32 {
    let step = 1.0;
    let lefti = self.lookup_ridge_index(x - step);
    let righti = self.lookup_ridge_index(x + step);
    let left = self.smoothed_ridge[lefti];
    let right = self.smoothed_ridge[righti];
    let dx = right.0 - left.0;
    let dy = right.1 - left.1;
    if dx <= 0.0 {
      return 0.0;
    }
    dy.atan2(dx)
  }
}

pub struct MountainsV2 {
  pub mountains: Vec<Mountain>,
  pub battlefield: BattlefieldArea,
  pub skip_alt_factor: f64,
}

impl MountainsV2 {
  pub fn rand<R: Rng>(
    rng: &mut R,
    ctx: &GlobalCtx,
    mainclr: usize,
    width: f32,
    height: f32,
    yhorizon: f32,
    ymax: f32,
    count: usize,
    count_behind: usize,
  ) -> Self {
    let bound = (0.0, 0.0, width, yhorizon);
    let seed = rng.gen_range(0.0..100.0);
    let perlin = Perlin::new(rng.gen());
    let min_route = 2;
    let mountainpadding = 0.0;
    let skip_alt_factor = rng.gen_range(-1.0f64..1.5).max(0.001).min(0.99)
      * rng.gen_range(0.0..1.0);
    let mut height_map: Vec<f32> = Vec::new();

    let mut passage = Passage::new(0.5, width, height);
    let precision = 1.0;

    let mut mountains = vec![];

    let secondcolor = (mainclr + 1) % 3;

    let leader_mountain_index = rng.gen_range(0..count);

    for j in 0..count + count_behind {
      let clr = if j < count { mainclr } else { secondcolor };
      let jf = j as f32 / ((count - 1) as f32);
      let mut routes: Polylines = Vec::new();
      let mut local_height_map: Vec<f32> = Vec::new();

      let h: f32 = rng.gen_range(2.0..6.0);
      let ampfactor = mix(0.01, 0.1, jf.min(1.0)) * rng.gen_range(0.5..1.0);
      let ynoisefactor = rng.gen_range(0.01..0.1);
      let yincr = if j == 0 {
        rng.gen_range(0.5..1.0)
      } else {
        2.0 + (rng.gen_range(-1f32..8.0) * rng.gen_range(0.0..1.0)).max(0.0)
      };
      let amp1 = rng.gen_range(-2.0f64..4.0).max(0.0) * rng.gen_range(0.3..1.0);
      let amp2 = rng.gen_range(-1.0f64..3.0).max(0.0) * rng.gen_range(0.3..1.0);
      let amp3 = rng.gen_range(-1.0f64..2.0).max(0.0) * rng.gen_range(0.3..1.0);
      let center = rng.gen_range(0.2..0.8) * width;

      let stopy = mix(yhorizon, ymax, 0.2 + 0.8 * jf);

      // Build the mountains bottom-up, with bunch of perlin noises
      let mut base_y = yhorizon + 0.2 * height;
      let mut miny = base_y;
      let mut maxy = 0.0;
      let mut first = true;

      loop {
        if miny < stopy {
          break;
        }

        let mut route = Vec::new();
        let mut x = mountainpadding;
        let mut was_outside = true;
        loop {
          if x > width - mountainpadding {
            break;
          }
          let xv = ((h - base_y / height) * (x - center)) as f64;

          let amp = (height * ampfactor) as f64;
          let mut y = base_y as f64;

          /*
          y += ((x / width - 0.5).abs() * 120.0) as f64;
          */

          if amp2 > 0. {
            y += amp2
              * amp
              * perlin
                .get([
                  //
                  8.311 + xv * 0.00511,
                  88.1 + y * ynoisefactor,
                  seed * 97.311,
                ])
                .max(0.0);
          }

          if amp1 > 0. {
            y += amp1
              * amp
              * perlin.get([
                //
                xv * 0.007111 + 9.9,
                y * 0.00311 + 3.1,
                77.
                  + seed / 7.3
                  + 0.1
                    * perlin.get([
                      //
                      55. + seed * 7.3,
                      80.3 + xv * 0.0057,
                      y * 0.06 + 11.3,
                    ]),
              ]);
          }

          if amp3 > 0. {
            y += amp
              * amp3
              * perlin
                .get([
                  //
                  xv * 0.009 + 8.33,
                  88.1 + y * 0.07,
                  seed / 7.7 + 6.66,
                ])
                .powf(2.0);
          }

          let y = y as f32;

          if y < miny {
            miny = y;
          }
          if y > maxy {
            maxy = y;
          }
          let mut collides = false;
          let xi = ((x - mountainpadding) / precision).round() as usize;
          if xi >= local_height_map.len() {
            local_height_map.push(y);
          } else {
            if y < local_height_map[xi] {
              local_height_map[xi] = y;
            }
          }

          if xi >= height_map.len() {
            height_map.push(y);
          } else {
            if y > height_map[xi] - 0.01 {
              collides = true;
            } else {
              height_map[xi] = y;
            }
          }
          let inside = !collides && strictly_in_boundaries((x, y), bound);
          passage.get((x, y));
          if inside {
            if was_outside {
              let l = route.len();
              if l >= min_route {
                routes.push((clr, route));
              }
              route = Vec::new();
            }
            was_outside = false;
            route.push((x, y));
            passage.count((x, y));
          } else {
            was_outside = true;
          }

          x += precision;
        }

        let l = route.len();
        if l >= min_route {
          routes.push((clr, route));
        }

        if first {
          first = false;
          // optim: jump directly to the visible area (estimated at the gap between highest mount and the base_y). 5mm security
          let diff = base_y - miny - 5.0;
          base_y -= yincr.max(diff);
        } else {
          base_y -= yincr;
        }
      }

      let ridge: Vec<(f32, f32)> = height_map
        .iter()
        .enumerate()
        .map(|(i, &y)| (i as f32 * precision, y))
        .collect();
      let mut modified_ridge = ridge.clone();

      let castle = if j == count - 1 {
        //  find the flattest area possible that is high enough.
        let tries = rng.gen_range(1..20);
        let mut candidates = vec![];
        for _ in 0..tries {
          // allow a crazy case where the width would be beyond the screen & we literally have a huge castle
          let castle_width = if ctx.full_castle {
            width * 1.5
          } else {
            (0.2 + rng.gen_range(0.0..0.6) * rng.gen_range(0.5..1.0)) * width
          };
          let xcastlepos = rng.gen_range(0.2..0.8) * width;

          let ileft =
            ((xcastlepos - castle_width / 2.0).max(0.0) / precision) as usize;
          let iright = (((xcastlepos + castle_width / 2.0) / precision)
            as usize)
            .min(ridge.len() - 1);

          let mut miny = yhorizon;
          let mut maxy = 0.0;
          for i in ileft..iright {
            let p = ridge[i];
            if p.1 < miny {
              miny = p.1;
            }
            if p.1 > maxy {
              maxy = p.1;
            }
          }
          let castle_position = (xcastlepos, miny);
          candidates.push((castle_position, castle_width, maxy - miny));
        }

        if let Some(&(castlepos, castlewidth, _)) = candidates
          .iter()
          .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        {
          let mut moats = vec![];

          let scale = 1.0
            + rng.gen_range(-0.2..0.4) * rng.gen_range(0.0..1.0)
            + rng.gen_range(0.0..(1.0 * castlewidth / width));

          let holewidth = scale * rng.gen_range(0.03..0.05) * width;
          let holeheight = rng.gen_range(0.03..0.07) * width;
          let maxh = 0.3 * holewidth;
          let pad = 0.1 * width;

          for (xpos, goright) in vec![
            (castlepos.0 + castlewidth / 2.0, true),
            (castlepos.0 - castlewidth / 2.0, false),
          ] {
            if xpos < pad || xpos > width - pad {
              continue;
            }
            let xdir = if goright { 1.0 } else { -1.0 };
            let fromi = (xpos / precision) as usize;
            let x = xpos + xdir * holewidth;
            if x < pad || x > width - pad {
              continue;
            }
            let toi = (x / precision) as usize;
            let l = fromi.abs_diff(toi);
            let fromp = ridge[fromi];
            let top = ridge[toi];
            let mut invalid = false;
            for d in 0..l {
              let i = if goright { fromi + d } else { fromi - d };
              let p = ridge[i];
              if (p.1 - fromp.1).abs() > maxh {
                invalid = true;
                break;
              }
            }

            let gap = l / 8;
            if !invalid && l > gap + 1 {
              // ok, it's a valid moat, now we will dig in the mountain.
              let mut route = vec![];
              route.push((fromp.0, fromp.1 - holeheight));

              let ampcurve = rng.gen_range(0.0..0.6);
              let mut lastp = fromp;
              for d in 0..(l - gap) {
                let i = if goright { fromi + d } else { fromi - d };
                let (x, y) = ridge[i];
                let sf = d as f32 / (l - 1) as f32;
                let dy = holeheight
                  * (1. - ampcurve * (2. * (sf - 0.5).abs()).powf(2.0));
                let y = y + dy;
                route.push((x, y));
                // move the ridge back down
                height_map[i] = y;
                modified_ridge[i] = (x, y);
                lastp = (x, y);
              }

              route.push((lastp.0, lastp.1 - holeheight));

              let is_outside = |p| polygon_includes_point(&route, p);
              routes = clip_routes_with_colors(&routes, &is_outside, 1.0, 3);

              let is_outside = |p: (f32, f32)| lookup_ridge(&ridge, p.0) > p.1;
              routes.extend(clip_routes_with_colors(
                &vec![(clr, route)],
                &is_outside,
                1.0,
                3,
              ));

              moats.push((fromp, top));
            }
          }

          let main_door_pos =
            if rng.gen_bool(if moats.len() > 0 { 0.1 } else { 0.8 }) {
              let xpad = 0.2 * castlewidth;
              let xfrom = castlepos.0 - castlewidth / 2.0 + xpad;
              let xto = castlepos.0 + castlewidth / 2.0 - xpad;
              let xstep = 0.05 * castlewidth;
              let mut x = xfrom;
              let mut best: Option<(f32, f32)> = None;
              while x <= xto {
                let y = lookup_ridge(&ridge, x);
                if y < best.map(|p| p.1).unwrap_or(yhorizon) {
                  best = Some((x, y));
                }
                x += xstep;
              }
              best
            } else {
              None
            };

          Some(CastleGrounding {
            position: castlepos,
            width: castlewidth,
            moats,
            main_door_pos,
            scale,
            is_on_water: false,
          })
        } else {
          None
        }
      } else {
        None
      };

      let rlen = modified_ridge.len();
      let smoothed_ridge =
        center_vec_2d(&moving_average_2d(&modified_ridge, rlen / 16), rlen);

      mountains.push(Mountain {
        precision,
        clr,
        castle,
        ridge: modified_ridge,
        smoothed_ridge,
        yhorizon,
        routes,
        width,
        has_beach: j == 0,
        is_behind: j >= count,
        will_have_the_leader: j == leader_mountain_index,
      });

      // move the ridge up to create some "halo" around mountain. and see more easily the diff layers
      let push = 2.0;
      for h in &mut height_map {
        *h -= push;
      }
    }

    let battlefield =
      BattlefieldArea::rand(rng, ctx, width, height, yhorizon, &mountains);

    Self {
      mountains,
      battlefield,
      skip_alt_factor,
    }
  }
}
