use crate::algo::{
  clipping::{clip_routes_with_colors, regular_clip},
  math1d::mix,
  math2d::{lookup_ridge, strictly_in_boundaries},
  moving_average::moving_average_2d,
  paintmask::PaintMask,
  passage::Passage,
  polylines::Polylines,
};
use noise::*;
use rand::prelude::*;

use super::castle;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct CastleGrounding {
  pub position: (f32, f32),
  pub width: f32,
  pub moats: (bool, bool),
}

pub struct Mountain {
  // meta info for the objects we will need to draw inside mountains
  pub castle: Option<CastleGrounding>,
  pub ridge: Vec<(f32, f32)>,
  pub yhorizon: f32,
  pub width: f32,
  // info for the render time
  pub routes: Polylines,
}

impl Mountain {
  pub fn render<R: Rng>(
    &self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Polylines {
    let out = regular_clip(&self.routes, paint);
    paint.paint_fn(&|(x, y)| {
      let yridge = lookup_ridge(&self.ridge, x);
      yridge < y && y < self.yhorizon
    });
    out
  }
}

pub struct MountainsV2 {
  pub mountains: Vec<Mountain>,
}

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
    let precision = 0.1;

    let mut mountains = vec![];

    for j in 0..count {
      let mut routes: Polylines = Vec::new();
      let mut local_height_map: Vec<f32> = Vec::new();

      let h: f32 = rng.gen_range(3.0..5.0);
      let ampfactor = 0.07 * rng.gen_range(0.5..1.0);
      let ynoisefactor = rng.gen_range(0.01..0.1);
      let yincr =
        2.0 + (rng.gen_range(-1f32..8.0) * rng.gen_range(0.0..1.0)).max(0.0);
      let amp1 = rng.gen_range(0.0..10.0) * rng.gen_range(0.0..1.0);
      let amp2 = rng.gen_range(0.0..8.0) * rng.gen_range(0.0..1.0);
      let amp3 = rng.gen_range(0.0..4.0) * rng.gen_range(0.0..1.0);
      let center = rng.gen_range(0.2..0.8) * width;

      let stopy = mix(
        yhorizon,
        ymax,
        (j as f32 / ((count - 1) as f32)) * 0.8 + 0.2,
      );

      // Build the mountains bottom-up, with bunch of perlin noises
      let mut base_y = height;
      let mut miny = base_y;
      let mut maxy = 0.0;
      let mut first = true;
      let mut layers = 0;

      loop {
        if miny < stopy {
          break;
        }
        layers += 1;

        let mut route = Vec::new();
        let mut x = mountainpadding;
        let mut was_outside = true;
        loop {
          if x > width - mountainpadding {
            break;
          }
          let mut xv = (h - base_y / height) * (x - center);

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

          /*
          y += 0.05
            * amp
            * perlin.get([
              //
              6.6 + seed * 1.3,
              8.3 + xv as f64 * 0.027,
              8.1 + y as f64 * 0.051,
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
        let mut castle_position = smoothed_ridge[0];
        let borderypush = height * 0.5;
        let xmin = 0.1 * width;
        let xmax = 0.9 * width;
        for p in smoothed_ridge.iter() {
          // formula to avoid borders
          let x = p.1;
          let bordering = 2.0 * (p.0 / width - 0.5);
          let y = x + borderypush * bordering * bordering;
          if y < castle_position.1 && xmin < x && x < xmax {
            castle_position = *p;
          }
        }

        // TODO we could vary this based on the mountain shape
        let castle_width = rng.gen_range(0.2..0.3) * width;

        let leftx = castle_position.0 - castle_width / 2.0;
        let righty = castle_position.0 + castle_width / 2.0;

        // TODO: shape the mountain to flatten the area...

        let mut castle_moats = (false, false);

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
      });
    }

    Self { mountains }
  }
}

pub struct Mountains {
  pub clr: usize,
  pub ybase: f32,
  pub ystart: f32,
  pub width: f32,
  pub ridge: Option<Vec<(f32, f32)>>,
}

impl Mountains {
  pub fn init(clr: usize, ybase: f32, ystart: f32, width: f32) -> Self {
    Self {
      clr,
      ybase,
      ystart,
      width,
      ridge: None,
    }
  }

  pub fn ridge(&self) -> Vec<(f32, f32)> {
    self.ridge.clone().unwrap()
  }

  pub fn render<R: Rng>(
    &mut self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let clr = self.clr;
    let ybase = self.ybase;
    let ystart = self.ystart;
    let width = self.width;

    // TODO rework the implementation
    // TODO we may split the idea of front vs back mountains

    let mut routes = vec![];
    let perlin = Perlin::new(rng.gen());
    // mini mountains
    let count = rng.gen_range(2..12);
    let h = ybase - ystart;
    let xincr = 1.0;

    let mut curves = vec![];

    for i in 0..count {
      let y = ybase;
      let divmin = count as f32 * 0.3;
      let divmax = count as f32 * 0.6;
      let yamp = (i as f32 + 1.0) * h / rng.gen_range(divmin..divmax);

      let f1 = rng.gen_range(0.01..0.03) * rng.gen_range(0.0..1.0);
      let amp2 = rng.gen_range(0.0..2.0) * rng.gen_range(0.0..1.0);
      let f2 = rng.gen_range(0.0..0.05) * rng.gen_range(0.0..1.0);
      let amp3 = rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
      let f3 = rng.gen_range(0.0..0.1) * rng.gen_range(0.0..1.0);
      let seed1 = rng.gen_range(0.0..100.0);
      let seed2 = rng.gen_range(0.0..100.0);
      let seed3 = rng.gen_range(0.0..100.0);

      let valuef = |x32, y32| {
        let x = x32 as f64;
        let y = y32 as f64;
        let n = 0.5
          + 0.5
            * perlin.get([
              f1 * x,
              f1 * y,
              amp2
                * perlin.get([
                  f2 * x,
                  seed2 + amp3 * perlin.get([seed3, f3 * x, f3 * y]),
                  f2 * y,
                ])
                + seed1
                + i as f64 * 55.5,
            ]) as f32;
        n
      };

      let (rts, curve) =
        stroke_mountains(paint, 0.0, width, xincr, y, yamp, &valuef, clr);

      routes.extend(rts);
      curves.push(curve);
    }

    let mut ridge = vec![];
    let first = curves[0].clone();
    let len = first.len();
    for i in 0..len {
      let p = first[i];
      let mut max = p.1;
      for curve in curves.iter().skip(1) {
        let y = curve[i].1;
        if y < max {
          max = y;
        }
      }
      ridge.push((p.0, max));
    }
    self.ridge = Some(ridge);

    routes
  }
}

fn stroke_mountains(
  paint: &mut PaintMask,
  xfrom: f32,
  xto: f32,
  xincr: f32,
  ybase: f32,
  yamp: f32,
  valuef: &dyn Fn(f32, f32) -> f32,
  clr: usize,
) -> (Vec<(usize, Vec<(f32, f32)>)>, Vec<(f32, f32)>) {
  let mut routes = vec![];

  // sample the curve with f
  let mut curve = vec![];
  let mut x = xfrom;
  while x < xto {
    let y = ybase - yamp * valuef(x, ybase);
    curve.push((x, y));
    x += xincr;
  }
  if x > xto {
    let y = ybase - yamp * valuef(xto, ybase);
    curve.push((xto, y));
  }

  if curve.len() < 2 {
    return (routes, curve);
  }

  // TODO rework the implementation

  // make the polygons
  let mut polys = vec![];
  let len = curve.len();
  for j in 1..len {
    let i = j - 1;
    let mut poly = vec![];
    let a = curve[i];
    let b = curve[j];
    poly.push(a);
    poly.push(b);
    poly.push((b.0, ybase));
    poly.push((a.0, ybase));
    polys.push(poly);
  }

  routes.push((clr, curve.clone()));

  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }
  (routes, curve.clone())
}
