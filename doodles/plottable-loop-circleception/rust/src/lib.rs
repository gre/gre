/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Plottable Circleception
 */
mod utils;
use byteorder::*;
use geo::algorithm::euclidean_distance::*;
use geo::*;
use noise::*;
use pointy::*;
use rand::prelude::*;
use rand::Rng;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub struct Opts {
  pub seed: f64,
  pub hash: String,
  pub primary_name: String,
  pub secondary_name: String,
  pub dark_mode: bool,
}

fn apply_transform(
  t: Transform<f64>,
  scale: f64,
  rotation: f64,
  translation: (f64, f64),
  width: f64,
  height: f64,
) -> Transform<f64> {
  t.translate(-width / 2.0, -height / 2.0)
    .scale(scale, scale)
    .rotate(rotation)
    .translate(width / 2.0 + translation.0, height / 2.0 + translation.1)
}

enum CircleShape {
  None,
  Simple,
  Half(f64),
  Concentric(f64),
  HalfConcentric(f64),
  Spiral(f64, f64),
  Moon(f64, f64, f64),
  ZigZag(f64),
  Random,
}

#[derive(Clone, Copy)]
enum FrameShape {
  Simple,
  Cordon,
}

fn circle_shape_string(s: CircleShape) -> String {
  String::from(match s {
    CircleShape::None => "None",
    CircleShape::Simple => "Simple",
    CircleShape::Half(_) => "Half",
    CircleShape::Concentric(_) => "Concentric",
    CircleShape::HalfConcentric(_) => "HalfConcentric",
    CircleShape::Spiral(_, _) => "Spiral",
    CircleShape::Moon(_, _, _) => "Moon",
    CircleShape::ZigZag(_) => "ZigZag",
    CircleShape::Random => "Random",
  })
}

fn circle_resolution_infer(radius: f64) -> usize {
  5 + (radius * 12.0) as usize
}

fn circle_shape<R: Rng>(
  rng: &mut R,
  shape: &CircleShape,
  circle: &VCircle,
  phase: f64,
  scale: f64,
  animated: bool,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let c = circle;

  match shape {
    CircleShape::None => {}
    CircleShape::Spiral(dir, spins) => {
      let min_stroke = 0.2;
      let mut r = 0.2;
      let mut a = 0f64;
      let mut points = Vec::new();
      let aincr = dir * 0.06;
      let rincr = (0.3 + 0.7 * scale) * aincr.abs() / spins;
      let mut last = (c.x, c.y);
      points.push(last);
      loop {
        if r > c.r {
          break;
        }
        let p = (c.x + r * a.cos(), c.y + r * a.sin());
        if euclidian_dist(last, p) > min_stroke {
          points.push(p);
          last = p;
        }
        a += aincr;
        r += rincr;
      }
      routes.push(points);
    }
    CircleShape::HalfConcentric(dr) => {
      let drscaled = ((0.5 + 0.5 * scale) * dr).max(0.3);
      let mut r = c.r;
      loop {
        if r < c.r * 0.5 {
          break;
        }
        let route = circle_route(
          (c.x, c.y),
          r,
          circle_resolution_infer(r),
          rng.gen_range(0.0, 2.0 * PI),
        );
        routes.push(route);
        r -= drscaled;
      }
    }
    CircleShape::Concentric(dr) => {
      let drscaled = ((0.5 + 0.5 * scale) * dr).max(0.3);
      let mut r = c.r;
      let threshold = 0.04
        + if animated {
          (1.0 - 0.95 * scale) * r
        } else {
          0.0
        };
      loop {
        if r < threshold {
          break;
        }
        let route = circle_route(
          (c.x, c.y),
          r,
          circle_resolution_infer(r),
          rng.gen_range(0.0, 2.0 * PI),
        );
        routes.push(route);
        r -= drscaled;
      }
    }
    CircleShape::Simple => {
      let r = c.r;
      let route = circle_route(
        (c.x, c.y),
        r,
        circle_resolution_infer(r),
        rng.gen_range(0.0, 2.0 * PI),
      );
      routes.push(route);
    }
    CircleShape::Half(incr) => {
      let incrscaled = ((0.5 + 0.5 * scale) * incr).max(0.3);
      let route = circle_route(
        (c.x, c.y),
        c.r,
        circle_resolution_infer(c.r),
        rng.gen_range(0.0, 2.0 * PI),
      );
      let r2 = c.r * c.r;

      let mut v = if animated {
        2.0 * c.r * (scale - 0.5)
      } else {
        0.0
      };
      loop {
        if v >= c.r {
          break;
        }
        let mut f = |l| l * l + v * v < r2;
        if let Some(x) = scaling_search(&mut f, 0.0, c.r) {
          routes.push(vec![(c.x + x, c.y + v), (c.x - x, c.y + v)]);
        }
        v += incrscaled;
      }
      routes.push(route);
    }
    CircleShape::ZigZag(angle) => {
      let dx = angle.cos();
      let dy = angle.sin();
      let zz = |p: &(f64, f64)| p.0 * dx + p.1 * dy;
      let center = (c.x, c.y);
      let r = c.r;
      let count = 4 + (r * 40.0) as usize;
      let mut points = circle_route(center, r, count, rng.gen_range(0.0, 2.0 * PI));
      rng.shuffle(&mut points);
      points.truncate(points.len() / 2);
      points.sort_by(|a, b| (zz(b)).partial_cmp(&zz(a)).unwrap());
      routes.push(points);
    }
    CircleShape::Random => {
      let center = (c.x, c.y);
      let r = c.r;
      let count = 4;
      for i in 0..count {
        let p = i as f64 / (count as f64);
        let count = 4 + (r * (4.0 - 2.0 * p)) as usize;
        let mut points = circle_route(
          center,
          (1.0 - 0.4 * p) * r,
          count,
          rng.gen_range(0.0, 2.0 * PI),
        );
        rng.shuffle(&mut points); // rng is used to vary on each tile
        points.push(center);
        routes.push(points.clone());
      }
    }
    CircleShape::Moon(p, angle, aspeed) => {
      let incr = 0.2 + 0.2 * scale;
      let mut delta = 0.0;
      let pthreshold = if animated {
        0.8 * (*p) + 0.4 * scale
      } else {
        *p
      };
      let nangle = if animated {
        *angle + phase * aspeed
      } else {
        *angle
      };
      let d = (nangle.cos(), nangle.sin());
      loop {
        if delta > c.r * pthreshold {
          break;
        }
        let mut a = 0.0;
        let aincr = 0.2 / c.r;
        let mut route = Vec::new();
        loop {
          if a > 2. * PI {
            break;
          }
          let x = d.0 * delta + c.x + c.r * a.cos();
          let y = d.1 * delta + c.y + c.r * a.sin();
          if euclidian_dist((x, y), (c.x, c.y)) >= c.r - 0.01 {
            if route.len() > 1 {
              routes.push(route);
            }
            route = Vec::new();
          } else {
            route.push((x, y));
          }
          a += aincr;
        }
        if route.len() > 1 {
          routes.push(route);
        }

        delta += incr;
      }
    }
  }

  routes
}

pub fn art(opts: &Opts) -> Document {
  let gridw = 4;
  let gridh = 2;
  let width = 70.0;
  let height = 70.0;
  let seed = opts.seed;
  let mut rng = rng_from_seed(seed);

  let mut with_frame: Option<FrameShape> = if rng.gen_bool(0.5) {
    Some(if rng.gen_bool(0.6) {
      FrameShape::Simple
    } else {
      FrameShape::Cordon
    })
  } else {
    None
  };

  let n = rng.gen_range(0usize, 1000);
  let mut special_circle = if n < 20 {
    CircleShape::None
  } else if n < 300 {
    CircleShape::Random
  } else if n < 400 {
    CircleShape::Half(rng.gen_range(0.3, 0.5))
  } else if n < 600 {
    CircleShape::Concentric(rng.gen_range(0.3, 0.5))
  } else if n < 700 {
    CircleShape::ZigZag(rng.gen_range(0.0, 2.0 * PI))
  } else if n < 800 {
    CircleShape::Moon(0.8, rng.gen_range(0.0, 2.0 * PI), rng.gen_range(-2.0, 2.0))
  } else if n < 920 {
    CircleShape::Spiral(
      if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
      rng.gen_range(6.0, 14.0),
    )
  } else if n < 980 {
    CircleShape::Simple
  } else {
    CircleShape::HalfConcentric(rng.gen_range(0.3, 0.5))
  };

  let n = rng.gen_range(0usize, 1000);
  let mut general_circle: CircleShape = if n < 500 {
    CircleShape::Simple
  } else if n < 600 {
    CircleShape::Concentric(rng.gen_range(0.5, 1.0))
  } else if n < 720 {
    CircleShape::Half(rng.gen_range(0.3, 0.5))
  } else if n < 880 {
    CircleShape::HalfConcentric(rng.gen_range(0.3, 0.5))
  } else if n < 970 {
    CircleShape::Spiral(
      if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
      rng.gen_range(6.0, 12.0),
    )
  } else {
    CircleShape::Moon(0.8, rng.gen_range(0.0, 2.0 * PI), rng.gen_range(-1.0, 1.0))
  };

  let dezooming = rng.gen_bool(0.6);

  let mut all_primary = Vec::new();
  let mut all_secondary = Vec::new();
  let mut i = 0;

  let mut scale = 0.0;
  let mut rotation = 0.0;
  let mut translation = (0.0, 0.0);

  let pager_size = 1.0;
  let pager_pad = 1.0;
  let pager_ratio_scale = 1.0;
  let pgr = |xf, yf| {
    (
      pager_size * xf * pager_ratio_scale + pager_pad,
      height + pager_size * (yf - 2.0) - pager_pad,
    )
  };
  let pgr_topleft = pgr(0.0, 0.0);
  let pgr_bottomright = pgr(gridw as f64, gridh as f64);
  let pgr_boundaries = (
    pgr_topleft.0,
    pgr_topleft.1,
    pgr_bottomright.0,
    pgr_bottomright.1,
  );

  let frame_pad = match with_frame {
    Some(FrameShape::Cordon) => rng.gen_range(2.0, 4.0),
    Some(_) => rng.gen_range(0.1, 1.0),
    _ => 0.0,
  } + (rng.gen_range(-1f64, 4.0) * rng.gen_range(0.1, 1.0)).max(0.0);
  let packing_boundaries_inside = (frame_pad, frame_pad, width - frame_pad, height - frame_pad);

  let pad = 0.2;
  let boundaries = (pad, pad, width - pad, height - pad);
  let frame = vec![
    (pad, pad),
    (width - pad, pad),
    (width - pad, height - pad),
    (pad, height - pad),
    (pad, pad),
  ];
  let outside_frame = vec![
    (-frame_pad, -frame_pad),
    (width + frame_pad, -frame_pad),
    (width + frame_pad, height + frame_pad),
    (-frame_pad, height + frame_pad),
    (-frame_pad, -frame_pad),
  ];

  // we are "bruteforcing" a valid transformation. one that can keep the next frame "inside"
  for _i in 0..20 {
    scale = mix(0.8, 0.2, rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0));
    rotation = (rng.gen_range(-0.5, PI * 0.7) * rng.gen_range(0.0, 1.0))
      .max(0.0)
      .min(PI / 2.0)
      * if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
    translation = (
      rng.gen_range(-20.0, 20.0) * rng.gen_range(0.0, 1.0),
      rng.gen_range(-20.0, 20.0) * rng.gen_range(0.0, 1.0),
    );
    let t = apply_transform(
      Transform::default(),
      scale,
      rotation,
      translation,
      width,
      height,
    );
    let a = t * Pt::new(0.0, 0.0);
    let b = t * Pt::new(width, 0.0);
    let c = t * Pt::new(0.0, height);
    let d = t * Pt::new(width, height);
    if strictly_in_boundaries((a.x(), a.y()), packing_boundaries_inside)
      && strictly_in_boundaries((b.x(), b.y()), packing_boundaries_inside)
      && strictly_in_boundaries((c.x(), c.y()), packing_boundaries_inside)
      && strictly_in_boundaries((d.x(), d.y()), packing_boundaries_inside)
    {
      break;
    }
  }

  let first_transform = apply_transform(
    Transform::default(),
    scale,
    rotation,
    translation,
    width,
    height,
  );

  let circles_pad = 0.1 + rng.gen_range(0.0, 4.0) * rng.gen_range(0.2, 1.0);
  let min_circle = circles_pad + rng.gen_range(1.0, 4.0) * rng.gen_range(0.3, 1.0);
  let max_circle = min_circle + rng.gen_range(0.0, 40.0) * rng.gen_range(0.01, 1.0);
  let max_circle_first = max_circle + rng.gen_range(10.0, 40.0);

  let excluding_polygon = Polygon::new(
    outside_frame
      .iter()
      .map(|&(x, y)| {
        let pt = first_transform * Pt::new(x, y);
        (pt.x(), pt.y())
      })
      .collect(),
    vec![],
  );

  let mut circles = packing(
    opts.seed,
    100000,
    500,
    (rng.gen_range(1.0, 30.0) * rng.gen_range(0.0, 1.0)) as usize,
    circles_pad,
    packing_boundaries_inside,
    &excluding_polygon,
    min_circle,
    max_circle,
    max_circle_first,
  );
  circles.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
  let circleslen = circles.len();
  if circleslen > 0 {
    let c = circles[0];
    let special_circle_extra_pad_percent = rng.gen_range(0.7, 1.0);
    circles[0] = VCircle::new(c.x, c.y, special_circle_extra_pad_percent * c.r);
  } else {
    special_circle = CircleShape::None;
  }

  if circleslen < 2 {
    general_circle = CircleShape::None;
  }

  let with_cordon = match special_circle {
    CircleShape::Moon(_, _, _) => false,
    CircleShape::None => false,
    CircleShape::Random => rng.gen_bool(0.9),
    _ => rng.gen_bool(0.3),
  };

  let with_everything_animates = match general_circle {
    CircleShape::None => false,
    CircleShape::Simple => false,
    CircleShape::HalfConcentric(_) => false,
    CircleShape::Spiral(_, _) => false,
    _ => rng.gen_bool(0.3),
  };

  // stats by angle to see if there are circles on all directions
  let angle_slices = 8;
  let mut counter = vec![0usize; angle_slices];
  let center_next_frame = first_transform * Pt::new(width / 2.0, height / 2.0);
  let twopi = 2.0 * PI;
  for c in circles.iter() {
    let i = (angle_slices as f64
      * (((twopi + (c.y - center_next_frame.y()).atan2(c.x - center_next_frame.x())) / twopi)
        % 1.0)) as usize;
    counter[i] += 1;
  }

  // if some circles are not in a given angle, we will force the framing to happen
  if counter.iter().any(|&v| v == 0) {
    with_frame = Some(if frame_pad < 1.0 {
      FrameShape::Simple
    } else {
      FrameShape::Cordon
    });
  }

  let frame_cordon_mul = rng.gen_range(0.2, 1.0);

  // iterate over all the grid cells
  for yi in 0..gridh {
    for xiv in 0..gridw {
      let xi = if yi == 0 { xiv } else { gridw - xiv - 1 };
      let progress = if dezooming {
        i as f64 / ((gridh * gridw) as f64)
      } else {
        1.0 - i as f64 / ((gridh * gridw) as f64)
      };

      let mut passage = Passage2DCounter::new(0.5, width, height);

      let mut primary_routes = Vec::new();
      let mut secondary_routes = Vec::new();
      let mut special_circles = Vec::new();

      let mut s = 1.0;

      // NB i never understood why 1.2
      let sm = 1.0 / mix(scale, 1.0, progress.powf(1.25));
      s *= sm;
      let mut t = Transform::default()
        .translate(
          -width / 2.0 - translation.0 * (1.0 - progress),
          -height / 2.0 - translation.1 * (1.0 - progress),
        )
        .rotate(mix(-rotation, 0.0, progress))
        .scale(sm, sm)
        .translate(width / 2.0, height / 2.0);

      let clip_route = |input: Vec<(f64, f64)>| {
        let mut routes = Vec::new();
        let mut route = Vec::new();
        let mut last_down = None;
        let mut last_up = None;
        for p in input {
          if strictly_in_boundaries(p, boundaries) && !strictly_in_boundaries(p, pgr_boundaries) {
            if let Some(last) = last_up {
              let mut a = frame[0];
              let first_collision = frame.iter().skip(1).find_map(|&b| {
                let r = collides_segment(a, b, last, p);
                a = b;
                r
              });
              if let Some(c) = first_collision {
                route.push(c);
              }
            }
            route.push(p);
            last_down = Some(p);
            last_up = None;
          } else {
            if let Some(last) = last_down {
              let mut a = frame[0];
              let first_collision = frame.iter().skip(1).find_map(|&b| {
                let r = collides_segment(a, b, last, p);
                a = b;
                r
              });
              if let Some(c) = first_collision {
                route.push(c);
              }
            }
            let l = route.len();
            if l > 0 {
              if l > 1 {
                routes.push(route);
              }
              route = Vec::new();
            }
            last_down = None;
            last_up = Some(p);
          }
        }
        if route.len() > 1 {
          routes.push(route);
        }
        routes
      };

      /*
      if circleslen > 0 {
        special_circles.push(circles[0]);
      }*/

      for inception in 0..32 {
        if s < 0.005 {
          break;
        }
        let next_t = apply_transform(t, scale, rotation, translation, width, height);

        let phase = inception as f64 + progress;

        let local_circles: Vec<_> = circles
          .iter()
          .enumerate()
          .filter_map(|(i, c)| {
            let r = c.r * s;
            if r < 0.03 {
              return None;
            }
            let newp = t * Pt::new(c.x, c.y);
            let x = newp.x();
            let y = newp.y();
            if r < 0.4 && passage.count((x, y)) > 1 {
              return None;
            }
            let next = VCircle::new(x, y, r);
            if i == 0 {
              special_circles.push(next);
            }
            Some(next)
          })
          .collect();

        if let Some(frame_shape) = with_frame {
          let f = frame
            .iter()
            .map(|&(x, y)| {
              let newp = t * Pt::new(x, y);
              (newp.x(), newp.y())
            })
            .collect();

          let routes = match frame_shape {
            FrameShape::Simple => {
              vec![f]
            }
            FrameShape::Cordon => {
              let tracks_count = 1 + (8.0 * s) as usize;
              let width = frame_cordon_mul * 4.0 * s;
              let noiseamp = width;
              cordon(
                f,
                width,
                noiseamp,
                width / 2.0,
                tracks_count,
                true,
                2.0 + s,
                0.0,
              )
            }
          };

          for route in routes {
            primary_routes.push(clip_route(route));
          }
        }

        let mut local_rng = rng_from_seed(seed);

        if local_circles.len() > 0 {
          let c = local_circles[0];
          let shape = circle_shape(&mut local_rng, &special_circle, &c, phase, s, true);
          for c in local_circles.iter().skip(1) {
            let routes = circle_shape(
              &mut local_rng,
              &general_circle,
              c,
              phase,
              s,
              with_everything_animates,
            );
            for route in routes {
              primary_routes.push(clip_route(route));
            }
          }
          for route in shape {
            secondary_routes.push(clip_route(route));
          }
        }

        t = next_t;
        s *= scale;
      }

      let mut pager = Vec::new();
      for xj in vec![0, gridw] {
        //0..(gridw + 1) {
        pager.push(vec![pgr(xj as f64, 0.0), pgr(xj as f64, gridh as f64)]);
      }
      for yj in vec![0, gridh] {
        //0..(gridh + 1) {
        pager.push(vec![pgr(0.0, yj as f64), pgr(gridw as f64, yj as f64)]);
      }
      let lines = 4;
      for i in 0..lines {
        let f = (i as f64 + 0.5) / (lines as f64);
        pager.push(vec![
          pgr(xi as f64 + f, yi as f64),
          pgr(xi as f64 + f, yi as f64 + 1.0),
        ]);
      }
      secondary_routes.push(pager);

      if special_circles.len() > 1 && with_cordon {
        let from_mul = match special_circle {
          CircleShape::Spiral(_, _) => 0.0,
          CircleShape::Random => 0.5,
          _ => 1.0,
        };
        let mut last = special_circles[0];
        for &c in special_circles.iter() {
          let dx = c.x - last.x;
          let dy = c.y - last.y;
          let l = (dx * dx + dy * dy).sqrt();
          if l > 0.4 {
            let pad = 2.0 * s;
            let v1 = from_mul * (last.r - pad) / l;
            let v2 = 1.0 - (c.r - pad) / l;
            let p1 = (last.x + v1 * dx, last.y + v1 * dy);
            let p2 = (last.x + v2 * dx, last.y + v2 * dy);
            let tracks_count = (l / 6.0) as usize + 1;
            let width = l / 20.0;
            let noiseamp = width;
            let corner_pad = 0.0;
            let routes = cordon(
              vec![p1, p2],
              width,
              noiseamp,
              corner_pad,
              tracks_count,
              false,
              1.0,
              0.0,
            );
            for route in routes {
              secondary_routes.push(clip_route(route));
            }
          }
          last = c;
        }
      }

      let dx = xi as f64 * width;
      let dy = yi as f64 * height;
      let remap = |points: &Vec<(f64, f64)>| points.iter().map(|(x, y)| (x + dx, y + dy)).collect();
      let primary: Vec<Vec<(f64, f64)>> = primary_routes.concat().iter().map(remap).collect();
      let secondary: Vec<Vec<(f64, f64)>> = secondary_routes.concat().iter().map(remap).collect();

      all_primary.push(primary);
      all_secondary.push(secondary);
      i += 1;
    }
  }

  let layer_primary = all_primary.concat();
  let layer_secondary = all_secondary.concat();

  let layers: Vec<Group> = vec![
    ("#0FF", opts.primary_name.clone(), layer_primary),
    ("#F0F", opts.secondary_name.clone(), layer_secondary),
  ]
  .iter()
  .filter(|(_color, _label, routes)| routes.len() > 0)
  .map(|(color, label, routes)| {
    let mut l = Group::new()
      .set("inkscape:groupmode", "layer")
      .set("inkscape:label", label.clone())
      .set("fill", "none")
      .set("stroke", color.clone())
      .set("stroke-linecap", "round")
      .set("stroke-width", 0.35);

    let opacity: f64 = 0.65;
    let opdiff = 0.15 / (routes.len() as f64);
    let mut trace = 0f64;
    for route in routes.clone() {
      trace += 1f64;
      let data = render_route(Data::new(), route);
      l = l.add(
        Path::new()
          .set(
            "opacity",
            (1000. * (opacity - trace * opdiff)).floor() / 1000.0,
          )
          .set("d", data),
      );
    }

    l
  })
  .collect();

  let mut map = Map::new();

  map.insert(
    String::from("Special Shape"),
    json!(circle_shape_string(special_circle)),
  );

  map.insert(
    String::from("General Shape"),
    json!(circle_shape_string(general_circle)),
  );

  map.insert(
    String::from("Theme"),
    json!(String::from(if opts.dark_mode { "Dark" } else { "Light" })),
  );

  if let Some(frame) = with_frame {
    map.insert(
      String::from("Frame"),
      json!(String::from(match frame {
        FrameShape::Simple => "Simple",
        FrameShape::Cordon => "Cordon",
      })),
    );
  }

  if with_everything_animates && circleslen > 0 {
    map.insert(
      String::from("Everything animates"),
      json!(String::from("Yes")),
    );
  }

  map.insert(
    String::from("Inception"),
    json!(String::from(if scale < 0.33 {
      "Low"
    } else if scale < 0.5 {
      "Medium"
    } else if scale < 0.7 {
      "High"
    } else {
      "Very High"
    })),
  );

  let rotabs = rotation.abs();
  map.insert(
    String::from("Rotation"),
    json!(String::from(if rotabs < 0.01 {
      "None"
    } else if rotabs < 0.1 * PI {
      "Low"
    } else if rotabs < 0.3 * PI {
      "Medium"
    } else if rotabs < 0.44 * PI {
      "High"
    } else {
      "Angular"
    })),
  );

  let tr = translation.0.abs().max(translation.1.abs());
  map.insert(
    String::from("Translation"),
    json!(String::from(if tr < 1.0 {
      "None"
    } else if tr < 4.0 {
      "Low"
    } else if tr < 8.0 {
      "Medium"
    } else if tr < 16.0 {
      "High"
    } else {
      "Very High"
    })),
  );

  if with_cordon {
    map.insert(String::from("With Cordon"), json!(String::from("Yes")));
  }

  map.insert(
    String::from("General Shape Amount"),
    json!(String::from(if circleslen < 1 {
      "None"
    } else if circleslen < 8 {
      "Low"
    } else if circleslen < 60 {
      "Medium"
    } else if circleslen < 400 {
      "High"
    } else {
      "Very High"
    })),
  );

  map.insert(
    String::from("Zoom Direction"),
    json!(String::from(if dezooming { "Inside" } else { "Outside" })),
  );

  let traits = Value::Object(map);

  let fwidth = gridw as f64 * width;
  let fheight = gridh as f64 * height;

  let mut document = svg::Document::new()
    .set("data-hash", opts.hash.to_string())
    .set("data-traits", traits.to_string())
    .set("viewBox", (0, 0, fwidth, fheight))
    .set("width", format!("{}mm", fwidth))
    .set("height", format!("{}mm", fheight))
    .set("style", "background:white")
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("xmlns", "http://www.w3.org/2000/svg");
  for l in layers {
    document = document.add(l);
  }
  document
}

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
  let opts = val.into_serde().unwrap();
  let doc = art(&opts);
  let str = doc.to_string();
  return str;
}

fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
  if route.len() == 0 {
    return data;
  }
  let first_p = route[0];
  let mut d = data.move_to((significant_str(first_p.0), significant_str(first_p.1)));
  for p in route {
    d = d.line_to((significant_str(p.0), significant_str(p.1)));
  }
  return d;
}

#[inline]
fn significant_str(f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

fn rng_from_seed(s: f64) -> impl Rng {
  let mut bs = [0; 16];
  bs.as_mut().write_f64::<BigEndian>(s).unwrap();
  let mut rng = SmallRng::from_seed(bs);
  // run it a while to have better randomness
  for _i in 0..50 {
    rng.gen::<f64>();
  }
  return rng;
}

#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f64,
  y: f64,
  r: f64,
}

impl VCircle {
  fn new(x: f64, y: f64, r: f64) -> Self {
    VCircle { x, y, r }
  }
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

fn scaling_search<F: FnMut(f64) -> bool>(mut f: F, min_scale: f64, max_scale: f64) -> Option<f64> {
  let mut from = min_scale;
  let mut to = max_scale;
  loop {
    if !f(from) {
      return None;
    }
    if to - from < 0.1 {
      return Some(from);
    }
    let middle = (to + from) / 2.0;
    if !f(middle) {
      to = middle;
    } else {
      from = middle;
    }
  }
}

fn search_circle_radius(
  bound: (f64, f64, f64, f64),
  circles: &Vec<VCircle>,
  excluding_polygon: &Polygon<f64>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let f = |size| {
    let c = VCircle::new(x, y, size);
    bound.0 < c.x - c.r
      && c.x + c.r < bound.2
      && bound.1 < c.y - c.r
      && c.y + c.r < bound.3
      && excluding_polygon.euclidean_distance(&Point::new(x, y)) > size
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(f, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  excluding_polygon: &Polygon<f64>,
  min_scale: f64,
  max_scale: f64,
  max_scale_first: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    let is_new = circles.len() == 0;
    let optsize = if is_new {
      3 * (optimize_size + 2)
    } else {
      optimize_size
    };
    if let Some(size) = search_circle_radius(
      bound,
      &circles,
      &excluding_polygon,
      x,
      y,
      min_scale,
      if is_new { max_scale_first } else { max_scale },
    ) {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optsize {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
}

pub struct Passage2DCounter {
  granularity: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage2DCounter {
  pub fn new(granularity: f64, width: f64, height: f64) -> Self {
    let wi = (width / granularity).ceil() as usize;
    let hi = (height / granularity).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage2DCounter {
      granularity,
      width,
      height,
      counters,
    }
  }
  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.granularity).ceil() as usize;
    let hi = (self.height / self.granularity).ceil() as usize;
    let xi = ((x / self.granularity).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.granularity).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }
  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }
  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    self.counters[self.index(p)]
  }
}

fn circle_route(center: (f64, f64), r: f64, count: usize, start_angle: f64) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = start_angle + 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

#[inline]
fn strictly_in_boundaries(p: (f64, f64), boundaries: (f64, f64, f64, f64)) -> bool {
  p.0 > boundaries.0 && p.0 < boundaries.2 && p.1 > boundaries.1 && p.1 < boundaries.3
}

fn cordon(
  path: Vec<(f64, f64)>,
  width: f64,
  noiseamp: f64,
  corner_pad: f64,
  tracks_count: usize,
  reconnect: bool,
  freq_mul: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let precision = 0.5;
  let r = precision;
  let mut pindex = 0;
  let mut p = path[pindex];
  let perlin = Perlin::new();
  let mut tracks = Vec::new();
  for _xi in 0..tracks_count {
    tracks.push(Vec::new());
  }
  for &next in path.iter().skip(1) {
    let dx = next.0 - p.0;
    let dy = next.1 - p.1;
    let a = dy.atan2(dx);
    let mut i = 0.0;
    let acos = a.cos();
    let asin = a.sin();
    let mut dist = (dx * dx + dy * dy).sqrt();
    if pindex != 0 {
      dist -= corner_pad;
      p.0 += corner_pad * acos;
      p.1 += corner_pad * asin;
    }
    if pindex == path.len() - 1 {
      dist -= corner_pad;
    }
    loop {
      if i >= dist {
        p = next;
        break;
      }
      p.0 += r * acos;
      p.1 += r * asin;
      for xi in 0..tracks_count {
        let variation = ((xi as f64 + (tracks_count as f64 * phase)) % (tracks_count as f64)
          - ((tracks_count - 1) as f64 / 2.0))
          / (tracks_count as f64);
        let mut delta = variation * width;
        let noisefreq = freq_mul * (0.1 + 0.2 * (0.5 - variation.abs()));
        delta += noiseamp
          * perlin.get([
            //
            noisefreq * p.0,
            noisefreq * p.1,
            10.0 * xi as f64,
          ]);
        let a2 = a + PI / 2.0;
        let q = (p.0 + delta * a2.cos(), p.1 + delta * a2.sin());
        tracks[xi].push(q);
      }
      i += r;
    }
    pindex += 1;
  }
  for track in tracks {
    let mut track_copy = track.clone();
    if reconnect {
      track_copy.push(track[0]);
    }
    routes.push(track_copy);
  }
  routes
}

// collides segments (p0,p1) with (p2,p3)
fn collides_segment(
  p0: (f64, f64),
  p1: (f64, f64),
  p2: (f64, f64),
  p3: (f64, f64),
) -> Option<(f64, f64)> {
  let s10_x = p1.0 - p0.0;
  let s10_y = p1.1 - p0.1;
  let s32_x = p3.0 - p2.0;
  let s32_y = p3.1 - p2.1;
  let d = s10_x * s32_y - s32_x * s10_y;
  if d.abs() < 0.000001 {
    return None;
  }
  let s02_x = p0.0 - p2.0;
  let s02_y = p0.1 - p2.1;
  let s_numer = s10_x * s02_y - s10_y * s02_x;
  if (s_numer < 0.) == (d > 0.) {
    return None;
  }
  let t_numer = s32_x * s02_y - s32_y * s02_x;
  if (t_numer < 0.) == (d > 0.) {
    return None;
  }
  if (s_numer > d) == (d > 0.) || (t_numer > d) == (d > 0.) {
    return None;
  }
  let t = t_numer / d;
  return Some((p0.0 + t * s10_x, p0.1 + t * s10_y));
}
