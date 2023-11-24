use crate::algo::{
  clipping::regular_clip,
  math1d::mix,
  math2d::{lookup_ridge, strictly_in_boundaries},
  moving_average::moving_average_2d,
  paintmask::PaintMask,
  passage::Passage,
  polylines::Polylines,
};
use noise::*;
use rand::prelude::*;

pub mod front;
pub mod wall;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone, Copy)]
pub struct CastleGrounding {
  pub position: (f32, f32),
  pub width: f32,
  pub moats: (bool, bool),
}

impl CastleGrounding {
  pub fn get_random_target<R: Rng>(&self, rng: &mut R) -> (f32, f32) {
    let (x, y) = self.position;
    (
      x + rng.gen_range(-0.5..0.5) * self.width,
      y + rng.gen_range(-1.0..0.0) * self.width,
    )
  }
}

pub struct Mountain {
  // meta info for the objects we will need to draw inside mountains
  pub castle: Option<CastleGrounding>,
  pub ridge: Vec<(f32, f32)>,
  pub yhorizon: f32,
  pub width: f32,
  pub has_beach: bool,
  // info for the render time
  pub routes: Polylines,
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
}

pub struct MountainsV2 {
  pub mountains: Vec<Mountain>,
}

// TODO the castle position is sometimes weird.
// TODO the castle is still too small at the moment.

impl MountainsV2 {
  pub fn rand<R: Rng>(
    rng: &mut R,
    clr: usize,
    width: f32,
    height: f32,
    yhorizon: f32,
    ymax: f32,
    count: usize,
  ) -> Self {
    let bound = (0.0, 0.0, width, yhorizon);
    let seed = rng.gen_range(0.0..100.0);
    let perlin = Perlin::new(rng.gen());
    let min_route = 2;
    let mountainpadding = 0.0;
    let mut height_map: Vec<f32> = Vec::new();

    let mut passage = Passage::new(0.5, width, height);
    let precision = 0.5;

    let mut mountains = vec![];

    for j in 0..count {
      let jf = j as f32 / ((count - 1) as f32);
      let mut routes: Polylines = Vec::new();
      let mut local_height_map: Vec<f32> = Vec::new();

      let h: f32 = rng.gen_range(3.0..5.0);
      let ampfactor = mix(0.01, 0.1, jf) * rng.gen_range(0.5..1.0);
      let ynoisefactor = rng.gen_range(0.01..0.1);
      let yincr = if j == 0 {
        rng.gen_range(0.5..1.0)
      } else {
        2.0 + (rng.gen_range(-1f32..8.0) * rng.gen_range(0.0..1.0)).max(0.0)
      };
      let amp1 = rng.gen_range(-1.0f32..4.0).max(0.0) * rng.gen_range(0.0..1.0);
      let amp2 = rng.gen_range(-1.0f32..4.0).max(0.0) * rng.gen_range(0.0..1.0);
      let amp3 = rng.gen_range(-1.0f32..2.0).max(0.0) * rng.gen_range(0.0..1.0);
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
          let xv = (h - base_y / height) * (x - center);

          let amp = height * ampfactor;
          let mut y = base_y;

          y += amp2
            * amp
            * perlin.get([
              //
              8.311 + xv as f64 * 0.00511,
              88.1 + y as f64 * ynoisefactor,
              seed * 97.311,
            ]) as f32;

          y += amp1
            * amp
            * perlin.get([
              //
              xv as f64 * 0.007111 + 9.9,
              y as f64 * 0.00311 + 3.1,
              77.
                + seed / 7.3
                + 0.1
                  * perlin.get([
                    //
                    55. + seed * 7.3,
                    80.3 + xv as f64 * 0.0057,
                    y as f64 * 0.06 + 11.3,
                  ]),
            ]) as f32;

          y += amp
            * amp3
            * perlin
              .get([
                //
                xv as f64 * 0.009 + 8.33,
                88.1 + y as f64 * 0.07,
                seed / 7.7 + 6.66,
              ])
              .powf(2.0) as f32;
          /*
           */

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

      let ridge = local_height_map
        .iter()
        .enumerate()
        .map(|(i, &y)| (i as f32 * precision, y))
        .collect();

      let castle = if j == count - 1 {
        // find a location for the castle
        // shape the mountain when needed

        let smooth = (width * 0.1) as usize;
        let smoothed_ridge = moving_average_2d(&ridge, smooth);

        // take an interesting high point
        let mut castle_position = (width / 2.0, height);
        let borderypush = 0.3 * height;
        for p in smoothed_ridge.iter() {
          // formula to avoid borders
          let bordering = 2.0 * (p.0 / width - 0.5).abs();
          let y = p.1 + borderypush * bordering * bordering;
          if y < castle_position.1 && width * 0.2 < p.0 && p.0 < width * 0.8 {
            castle_position = *p;
          }
        }

        if castle_position.1 > yhorizon {
          // TODO in that case, we skip completely the castle?
          castle_position.1 = yhorizon;
        }

        // TODO we could vary this based on the mountain shape
        let castle_width = rng.gen_range(0.2..0.35) * width;

        /*
        let leftx = castle_position.0 - castle_width / 2.0;
        let righty = castle_position.0 + castle_width / 2.0;
        */
        // TODO: shape the mountain to flatten the area...

        let castle_moats = (false, false);

        Some(CastleGrounding {
          position: castle_position,
          width: castle_width,
          moats: castle_moats,
        })
      } else {
        None
      };

      mountains.push(Mountain {
        castle,
        ridge,
        yhorizon,
        routes,
        width,
        has_beach: j == 0,
      });
    }

    Self { mountains }
  }
}
