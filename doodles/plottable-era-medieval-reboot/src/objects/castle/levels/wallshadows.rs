use crate::algo::{
  clipping::clip_routes_with_colors, math2d::lerp_point,
  polygon::polygon_includes_point, polylines::Polylines,
};
use noise::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn wall_shadow(
  seed: u32,
  clr: usize,
  polygon: &Vec<(f32, f32)>,
  // control the direction of the effect (only on x)
  light_x_direction: f32,
  scale: f32,
  // control the general length of the strokes
  amplitude: f32,
) -> Polylines {
  if polygon.len() < 3 {
    return vec![];
  }

  // controls the gap between strokes
  let stepping = 1.5 * scale;
  // control the noise of the stroke length
  let frequency = 7.173;

  let mut routes = vec![];
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
  miny = (miny / stepping).floor() * stepping;
  maxy = (maxy / stepping).ceil() * stepping;
  let mut yi = miny;
  while yi <= maxy {
    let yf = yi as f64 * frequency as f64;
    let offset_factor = 0.5 * perlin.get([yf, 1.0]) as f32;
    let lengthfactor = 0.8 + 0.4 * perlin.get([yf, 10.0]) as f32;
    let l = (amplitude * lengthfactor * light_x_direction.abs())
      .max(0.0)
      .min(1.0);
    let range = if light_x_direction > 0.0 {
      (1.0 - l)..1.0
    } else {
      0.0..l
    };
    let y = yi + stepping * offset_factor;

    // intersect an horizontal line with the polygon
    let line = vec![(minx, y), (maxx, y)];

    for (clr, route) in clip_routes_with_colors(
      &vec![(clr, line)],
      &|p| !polygon_includes_point(polygon, p),
      1.0,
      3,
    ) {
      // iterate on the strokes to cut a % of it depending on the direction
      let first = route[0];
      let last = route[route.len() - 1];
      //let l = last.0 - first.0;
      let a = lerp_point(first, last, range.start);
      let b = lerp_point(first, last, range.end);

      routes.push((clr, vec![a, b]));
    }

    yi += stepping;
  }

  routes
}
