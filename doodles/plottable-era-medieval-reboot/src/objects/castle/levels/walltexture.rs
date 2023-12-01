use crate::algo::{
  clipping::regular_clip, paintmask::PaintMask, polylines::Polylines,
};
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn wall_texture<R: Rng>(
  _rng: &mut R,
  paintref: &PaintMask,
  seed: u32,
  clr: usize,
  polygon: &Vec<(f32, f32)>,
  scale: f32,
) -> Polylines {
  if polygon.len() < 3 {
    return vec![];
  }

  // controls the gap between strokes
  let xstep = 2.0 * scale;
  let ystep = 1.0 * scale;
  let strokel = 1.6 * scale;
  let stroke_lfactor = 0.8;

  // control the noise of the stroke length
  let frequency = 1.773;
  let freq2 = frequency / 2.421;
  let intensity = 0.3;

  let perlin = Perlin::new(seed);
  let mut miny = f32::INFINITY;
  let mut maxy = -f32::INFINITY;
  let mut minx = f32::INFINITY;
  let mut maxx = -f32::INFINITY;
  for p in polygon {
    miny = miny.min(p.1);
    maxy = maxy.max(p.1);
    minx = minx.min(p.0);
    maxx = maxx.max(p.0);
  }
  miny = (miny / ystep).floor() * ystep;
  maxy = (maxy / ystep).ceil() * ystep;

  let mut lines = vec![];
  let mut yi = miny;
  let mut alt = false;
  while yi <= maxy {
    let yf = yi as f64 * frequency as f64;
    // let offset_factor = 0.5 * perlin.get([yf, 1.0]) as f32;
    // let lengthfactor = 0.8 + 0.4 * perlin.get([yf, 10.0]) as f32;
    /*
    let l = (amplitude * lengthfactor * light_x_direction.abs())
      .max(0.0)
      .min(1.0);
    let range = if light_x_direction > 0.0 {
      (1.0 - l)..1.0
    } else {
      0.0..l
    };
    */
    let y = yi;

    /*
    let xrep = scale * rng.gen_range(2.6..3.2);
    let yrep = scale * rng.gen_range(1.2..1.6);
    let mut alt = false;
    let mut y = wallheighty + merlonh + yrep;
    loop {
      if y > ybase {
        break;
      }
      let mut x = left.0;
      if alt {
        x += xrep / 2.0;
      }
      loop {
        if x > right.0 {
          break;
        }
        let strokel = scale * rng.gen_range(1.3..1.5);
        let dx = scale * rng.gen_range(-0.2..0.2);
        let dy = scale * rng.gen_range(-0.1..0.1);
        let x1 = (x + dx).max(left.0).min(right.0);
        let x2 = (x + dx + strokel).max(left.0).min(right.0);
        let y1 = y + dy;
        if y1 < ybase && y1 < ybase && rng.gen_bool(0.95) {
          routes.push((clr, vec![(x1, y + dy), (x2, y + dy)]));
        }
        x += xrep;
      }
      y += yrep;
      alt = !alt;
    }
     */

    let mut x = minx;
    if alt {
      x += xstep / 2.0;
    }
    while x <= maxx {
      let xf = x as f64;
      let threshold = perlin.get([45.7892, xf * freq2, yf * freq2]) + intensity;
      if perlin.get([xf * frequency, yf * frequency, 7.532]) < threshold {
        let ydir = perlin.get([xf * frequency, yf * frequency, PI]) as f32;
        let l = strokel
          * (1.0
            + stroke_lfactor
              * perlin.get([xf * frequency, yf * frequency, 0.05 / 7.4])
                as f32);
        let line = vec![(x, y), (x + l.min(xstep), y), (x + l, y + ydir * l)];
        lines.push((clr, line));
      }

      x += xstep;
    }

    yi += ystep;
    alt = !alt;
  }

  let mut mask = paintref.clone_empty();
  mask.paint_polygon(polygon);
  mask.reverse();
  let routes = regular_clip(&lines, &mask);

  /*
  // strokes -> fill -> strokes. will create nice textures!
  let strokew = 0.8;
  let density = 2.0;
  let iterations = 100;
  let bound = (minx, miny, maxx, maxy);
  let mut drawings = paintref.clone_empty();
  for (_clr, route) in routes.iter() {
    drawings.paint_polyline(route, strokew);
  }
  let filling = WormsFilling::rand(rng);
  let routes =
    filling.fill_in_paint(rng, &drawings, clr, density, bound, iterations);
    */

  routes
}
