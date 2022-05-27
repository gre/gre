/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Field
 */
mod utils;
use byteorder::*;
use noise::*;
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
}

enum InitialPointDistribution {
  TriangleSpiral,
  Circles,
  DoubleCircles,
  CrossLines,
  Curve,
  GoldSpiral,
  GoldSpiralCircle,
  MillSpiral,
  NestedCircles,
  NestedSquares,
  NestedTriangles,
  Parametric,
  Voronoi,
  XLines,
  YLines,
}

// See https://twitter.com/greweb/status/1524490017531432961
// slide a 2D point on a cylinder in 3D space along with a progress loop
fn project_cylinder_translation(
  // frame index / total frames
  progress: f64,
  // position on a 2D rectangle that have to be loop-translated on X
  point: (f64, f64),
  //radius of the cylinder to use
  radius: f64,
  // allow value to be injected to change the "seed" in the space lookup
  seed: (f64, f64),
) -> (f64, f64, f64) {
  let angle = 2. * PI * progress + point.0 / radius;
  let y = point.1;
  let x = seed.0 + radius * angle.cos();
  let z = seed.1 + radius * angle.sin();
  (x, y, z)
}

pub fn art(opts: &Opts) -> Document {
  let gridw = 4;
  let gridh = 2;
  let width = 70.0;
  let height = 70.0;
  let seed = opts.seed;
  let perlin = Perlin::new();
  let mut rng = rng_from_seed(seed);
  let precision = 0.5;
  let subprecision = 0.24;

  let ang_mod = if rng.gen_bool(0.2) {
    rng.gen_range(3., 7.)
  } else {
    0.
  };
  let a1 = rng.gen_range(0.6, 5.0) + rng.gen_range(0.0, 4.0);
  let a2 = a1 * rng.gen_range(0.6, 2.0);
  let a3 = a1 * rng.gen_range(0.6, 2.0);
  let max_f = rng.gen_range(10.0, 40.0);
  let f1 = rng.gen_range(1.0, max_f) / (a1 + 1.0);
  let f2 = rng.gen_range(1.0, max_f) / (a2 + 2.0);
  let f3 = rng.gen_range(1.0, max_f) / (a3 + 2.0);

  let noise_balance = 0.99 - 0.98 * rng.gen_range(0.0, 1.0);
  let is_diff_inks = !opts.primary_name.eq(&opts.secondary_name);
  let double_primary = is_diff_inks && rng.gen_bool(0.1);

  let round_angle =
    |n: f64, c: f64| 2. * PI * (c as f64 * (1.0 + n / (2. * PI))).round() / (c as f64);

  let cylinder_rot1 = round_angle(rng.gen_range(0.0, 2.0 * PI), 16.);
  let cylinder_rot2 = if rng.gen_bool(0.5) {
    cylinder_rot1
  } else {
    round_angle(rng.gen_range(0.0, 2.0 * PI), 16.)
  };
  let cylinder_rot3 = if rng.gen_bool(0.5) {
    cylinder_rot1
  } else {
    round_angle(rng.gen_range(0.0, 2.0 * PI), 16.)
  };
  let cylinder_radius1 = rng.gen_range(0.01, 0.3) * rng.gen_range(0.05, 1.0);
  let cylinder_radius2 = rng.gen_range(0.01, 0.2) * rng.gen_range(0.05, 1.0);
  let cylinder_radius3 = rng.gen_range(0.01, 0.1) * rng.gen_range(0.05, 1.0);

  let mut all_primary = Vec::new();
  let mut all_secondary = Vec::new();
  let mut i = 0;

  let double_pi = 2. * PI;

  let use_color_group = is_diff_inks && rng.gen_bool(0.25);
  let color_stripe_progress_mul = if is_diff_inks && !use_color_group && rng.gen_bool(0.6) {
    if rng.gen_bool(0.5) {
      1.0
    } else {
      -1.0
    }
  } else {
    0.0
  };
  let color_split_y = if !is_diff_inks || use_color_group && rng.gen_bool(0.8) {
    0.0
  } else {
    rng.gen_range(-100.0, 50f64).min(height / 2.0)
  };
  let color_split_x = if !is_diff_inks || use_color_group && rng.gen_bool(0.8) {
    0.0
  } else {
    rng.gen_range(-100.0, 50f64).min(width / 2.0)
  };
  let color_split_rot_diag = is_diff_inks && !use_color_group && rng.gen_bool(0.3);
  let color_split_rot_progress =
    (color_split_y > 0.0 || color_split_x > 0.0) && !color_split_rot_diag && rng.gen_bool(0.3);

  let color_mod = if is_diff_inks
    && !double_primary
    && !use_color_group
    && color_split_y < 0.0
    && color_split_x < 0.0
  {
    (rng.gen_range(0.0, 80.0) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0)) as usize
  } else {
    0
  };

  let color_blink_mod = if is_diff_inks && !color_split_rot_progress && rng.gen_bool(0.2) {
    if rng.gen_bool(0.1) {
      4
    } else {
      rng.gen_range(1, 3)
    }
  } else {
    0
  };

  let angle_div_grid = if rng.gen_bool(0.3) {
    rng.gen_range(4, 16)
  } else {
    0
  };
  let angle_div_grid_amp = if angle_div_grid > 0 {
    (if rng.gen_bool(0.5) { 1.0 } else { -1.0 }) * rng.gen_range(0.3, 1.2)
  } else {
    0.0
  };
  let angle_div_grid_freq = rng.gen_range(3.0, 8.0);
  let ang_area_disable_effect = rng.gen_bool(0.2);

  let spiral_rot = if rng.gen_bool(0.5) {
    if rng.gen_bool(0.5) {
      1.0
    } else {
      -1.0
    }
  } else {
    0.0
  };

  let circle_ang_delta = rng.gen_range(-PI, PI) * rng.gen_range(0.0, 0.5);
  let circles = if rng.gen_bool(if angle_div_grid > 0 { 0.2 } else { 0.6 }) {
    packing(
      opts.seed,
      200000,
      (1.0 + rng.gen_range(0.0, 40.0)) as usize,
      (1.0 + rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0)) as usize,
      rng.gen_range(0.0, 1.0),
      (0., 0., width, height),
      4.0,
      40.0,
    )
  } else {
    vec![]
  };
  let circle_color_change = is_diff_inks && circles.len() > 0 && rng.gen_bool(0.3);

  let mut total_circles = 0;

  let distribution =
    if circles.len() > 24 - (rng.gen_range(0.0, 22.5) * rng.gen_range(0.0, 1.0)) as usize {
      if rng.gen_bool(0.3) {
        InitialPointDistribution::DoubleCircles
      } else {
        InitialPointDistribution::Circles
      }
    } else if rng.gen_bool(0.13) {
      InitialPointDistribution::NestedSquares
    } else if rng.gen_bool(0.13) {
      InitialPointDistribution::Voronoi
    } else if rng.gen_bool(0.09) {
      InitialPointDistribution::NestedCircles
    } else if rng.gen_bool(0.09) {
      InitialPointDistribution::CrossLines
    } else if rng.gen_bool(0.09) {
      InitialPointDistribution::Parametric
    } else if rng.gen_bool(0.07) {
      InitialPointDistribution::TriangleSpiral
    } else if rng.gen_bool(0.07) {
      InitialPointDistribution::YLines
    } else if rng.gen_bool(0.09) {
      InitialPointDistribution::XLines
    } else if rng.gen_bool(0.08) {
      InitialPointDistribution::Curve
    } else if rng.gen_bool(0.1) {
      InitialPointDistribution::NestedTriangles
    } else if rng.gen_bool(0.18) {
      InitialPointDistribution::MillSpiral
    } else if rng.gen_bool(0.2) {
      InitialPointDistribution::GoldSpiralCircle
    } else {
      InitialPointDistribution::GoldSpiral
    };

  let divisions = match distribution {
    InitialPointDistribution::YLines
    | InitialPointDistribution::XLines
    | InitialPointDistribution::CrossLines => rng.gen_range(3, 24),
    _ => 0,
  };

  let center_effect = if rng.gen_bool(0.1) {
    rng.gen_range(0.2, 1.6)
  } else {
    0.0
  };

  let angle_base = round_angle(rng.gen_range(0.0, 2.0 * PI), 16.);
  let nested_count = rng.gen_range(0.6, 6.5f64).round() as usize;

  let static_initial_points: Vec<((f64, f64), usize)> = match distribution {
    InitialPointDistribution::TriangleSpiral => {
      let origin = (0.0, height * 0.9);
      let d_length = rng.gen_range(4.0, 10.0);
      let length = width;
      let incr = 0.4;
      let mut a: f64 = 0.0;
      let mut p = origin;
      let mut l = length;
      let mut points = Vec::new();
      loop {
        if l < 0.0 {
          break;
        }
        let new_p = (p.0 + l * a.cos(), p.1 + l * a.sin());
        let dx = new_p.0 - p.0;
        let dy = new_p.1 - p.1;
        let d = (dx * dx + dy * dy).sqrt();
        for i in 0..((d / incr).ceil() as usize) {
          let v = (i as f64 * incr) / d;
          points.push(((p.0 + dx * v, p.1 + dy * v), 0));
        }
        p = new_p;
        a -= PI * 2. / 3.;
        l -= d_length;
      }
      points
    }

    InitialPointDistribution::DoubleCircles | InitialPointDistribution::Circles => {
      let all_circles = match distribution {
        InitialPointDistribution::DoubleCircles => vec![
          circles.clone(),
          packing(
            77.7 + opts.seed / 0.3,
            200000,
            (1.0 + rng.gen_range(0.0, 40.0)) as usize,
            (1.0 + rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0))
              as usize,
            rng.gen_range(0.0, 1.0),
            (0., 0., width, height),
            4.0,
            40.0,
          ),
        ],
        _ => vec![circles.clone()],
      };

      let mut initial_points = Vec::new();
      let rmul = rng.gen_range(0.8, 1.0);
      for (gi, circles) in all_circles.iter().enumerate() {
        for (ci, c) in circles.iter().enumerate() {
          let count = (10.0 * c.r) as usize;
          for i in 0..(count + 1) {
            let a = 2. * PI * i as f64 / (count as f64);
            let x = c.x + rmul * c.r * a.cos();
            let y = c.y + rmul * c.r * a.sin();
            initial_points.push((
              (x, y),
              match distribution {
                InitialPointDistribution::DoubleCircles => gi,
                _ => ci,
              },
            ));
          }
          total_circles += 1;
        }
      }
      initial_points
    }

    InitialPointDistribution::YLines => {
      let mut initial_points = Vec::new();
      let count = 2000;
      let sub = (count / divisions).min(320);
      for d in 0..divisions {
        let x = width * (d as f64 + 0.5) / (divisions as f64);
        for s in 0..(sub + 1) {
          let y = height * (s as f64 + 0.5) / (sub as f64);
          initial_points.push(((x, y), d));
        }
      }
      initial_points
    }
    InitialPointDistribution::XLines => {
      let mut initial_points = Vec::new();
      let count = 2000;
      let sub = (count / divisions).min(320);
      for d in 0..divisions {
        let y = height * (d as f64 + 0.5) / (divisions as f64);
        for s in 0..sub {
          let x = width * (s as f64 + 0.5) / (sub as f64);
          initial_points.push(((x, y), d));
        }
      }
      initial_points
    }
    InitialPointDistribution::CrossLines => {
      let mut initial_points = Vec::new();
      let count = 2000;
      let sub = (count / (2 * divisions)).min(320);
      for d in 0..divisions {
        let x = width * (d as f64 + 0.5) / (divisions as f64);
        for s in 0..sub {
          let y = height * (s as f64 + 0.5) / (sub as f64);
          initial_points.push(((x, y), 0));
        }
      }
      for d in 0..divisions {
        let y = height * (d as f64 + 0.5) / (divisions as f64);
        for s in 0..sub {
          let x = width * (s as f64 + 0.5) / (sub as f64);
          initial_points.push(((x, y), 1));
        }
      }
      initial_points
    }
    InitialPointDistribution::NestedCircles => {
      let mut initial_points = Vec::new();
      let base_r = 0.08;
      let incr_r = 0.35;
      for i in 0..nested_count {
        let r = width
          * if nested_count == 1 {
            0.3
          } else {
            base_r + incr_r * (i as f64 / (nested_count as f64 - 1.0))
          };
        let count = (16.0 * r) as usize;
        for j in 0..count {
          let a = 2. * PI * (j as f64 / (count as f64));
          let p = (width / 2.0 + r * a.cos(), height / 2.0 + r * a.sin());
          initial_points.push((p, i));
        }
      }
      initial_points
    }
    InitialPointDistribution::Parametric => {
      let count = (32. * width) as usize;
      let a = rng.gen_range(0.1, 0.5);
      let b = rng.gen_range(0.1, 0.4);
      let c = rng.gen_range(0.1, 0.3);
      let d = rng.gen_range(0.1, 0.3);
      let mut initial_points = Vec::new();
      let parametric = |t: f64| {
        (
          a * (2. * PI * t).cos() + c * (18. * PI * t).cos(),
          b * (2. * PI * t).sin() - d * (10. * PI * t).cos(),
        )
      };
      let mut last_p = (-100.0, -100.0);
      for i in 0..(count + 1) {
        let t = i as f64 / (count as f64);
        let p = parametric(t);
        let new_p = (width * (p.0 + 0.5), height * (p.1 + 0.5));
        if euclidian_dist(new_p, last_p) > 0.35 {
          last_p = new_p;
          initial_points.push((new_p, 0));
        }
      }
      initial_points
    }
    InitialPointDistribution::Curve => {
      let a = rng.gen_range(150.0, 300.0);
      let b = rng.gen_range(150.0, 300.0);
      let c = rng.gen_range(0.2, 10.0);
      let d = rng.gen_range(0.2, 10.0);
      let e = rng.gen_range(0.0, 0.04) * rng.gen_range(0.0, 1.0);
      let count = (32. * width) as usize;
      let mut initial_points = Vec::new();
      for i in 0..(count + 1) {
        let x = width
          * (0.5
            + perlin.get([
              70.334 - opts.seed / 3.0,
              0.3 + i as f64 / a,
              e * perlin.get([-opts.seed, i as f64 * c]),
            ]));
        let y = height
          * (0.5
            + perlin.get([
              i as f64 / b,
              9.1 + 40.3 * opts.seed,
              e * perlin.get([60.1 + opts.seed, i as f64 * d]),
            ]));
        initial_points.push(((x, y), 0));
      }
      initial_points
    }
    InitialPointDistribution::Voronoi => {
      let m = 1.0 - rng.gen_range(0f64, 1.0).powf(1.5);
      let count_mul = mix(1.0, 2.4, m * m);
      let voronoi_size = mix(80.0, 3.0, m) as usize;
      let distrib = match rng.gen_range(0, 5) {
        1 => |p: (f64, f64)| (0.5 - euclidian_dist(p, (0.5, 0.5))).max(0.0),
        2 => |p: (f64, f64)| 0.5 - p.0.min(1. - p.0).min(p.1.min(1. - p.1)),
        3 => |p: (f64, f64)| 0.5 - (p.0 - 0.5).abs(),
        4 => |p: (f64, f64)| 0.5 - (p.1 - 0.5).abs(),
        _ => |_p: (f64, f64)| 1.0,
      };

      let candidates = sample_2d_candidates_f64(&distrib, 20, voronoi_size, &mut rng);

      let pad = 0.05;
      let mut points = Vec::new();
      for c in candidates {
        points.push(voronoi::Point::new(
          pad + (1.0 - 2.0 * pad) * c.0,
          pad + (1.0 - 2.0 * pad) * c.1,
        ));
      }
      let dcel = voronoi::voronoi(points, 1.0);
      let mut out = Vec::new();
      for segment in voronoi::make_line_segments(&dcel) {
        let ap = segment[0];
        let bp = segment[1];
        let mut ax: f64 = ap.x.into();
        let mut bx: f64 = bp.x.into();
        let mut ay: f64 = ap.y.into();
        let mut by: f64 = bp.y.into();
        ax *= width;
        ay *= height;
        bx *= width;
        by *= height;
        let dist = euclidian_dist((ax, ay), (bx, by));
        let count = (dist * count_mul) as usize;
        for i in 0..count {
          let p = (i as f64 + 0.5) / (count as f64);
          let x = mix(ax, bx, p);
          let y = mix(ay, by, p);
          out.push(((x, y), 0));
        }
      }
      out
    }
    _ => vec![],
  };

  let base_length = match distribution {
    InitialPointDistribution::Voronoi => 40.0,
    InitialPointDistribution::Curve => 36.0,
    InitialPointDistribution::Parametric => 32.0,
    InitialPointDistribution::DoubleCircles => 80.0,
    InitialPointDistribution::Circles => 60.0 + 140.0 / (circles.len() as f64),
    InitialPointDistribution::NestedCircles => 1.2 * width / (nested_count as f64),
    InitialPointDistribution::NestedSquares => 1.2 * width / (nested_count as f64),
    InitialPointDistribution::NestedTriangles => 1.2 * width / (nested_count as f64),
    InitialPointDistribution::TriangleSpiral => 40.0,
    InitialPointDistribution::GoldSpiralCircle => 40.0,
    InitialPointDistribution::GoldSpiral => 13.0,
    _ => 18.0,
  } * (rng.gen_range(0.7, 1.5));

  let mut total_ang_area_disable_effect_passage = 0;
  let mut total_ang_area_disable_effect_mul = 0.;

  // iterate over all the grid cells
  for yi in 0..gridh {
    for xi in 0..gridw {
      let progress = i as f64 / ((gridh * gridw) as f64);
      let mut field = |(x, y): (f64, f64)| {
        let p1 = project_cylinder_translation(
          progress,
          p_r((x, y), cylinder_rot1),
          cylinder_radius1,
          (0.0, opts.seed),
        );
        let p2 = project_cylinder_translation(
          progress,
          p_r((x, y), cylinder_rot2),
          cylinder_radius2,
          (0.0, opts.seed),
        );
        let p3 = project_cylinder_translation(
          progress,
          p_r((x, y), cylinder_rot3),
          cylinder_radius3,
          (0.0, opts.seed),
        );

        let ang_noise = perlin.get([
          f1 * p1.0,
          f1 * p1.1,
          f1 * p1.2
            + seed
            + a2
              * perlin.get([
                f2 * p2.0 - seed + a3 * perlin.get([f3 * p3.0, f3 * p3.1, f3 * p3.2]),
                f2 * p2.1,
                f2 * p2.2,
              ]),
        ]);

        let ang_global_noise = perlin.get([
          f1 * x,
          f1 * y,
          seed
            + perlin.get([
              seed + a3 * perlin.get([f3 * y, seed, f3 * x]),
              f2 * x,
              f2 * y,
            ]),
        ]);

        let mut ang = a1 * mix(ang_noise, ang_global_noise, noise_balance);

        if ang_area_disable_effect {
          let mul = perlin.get([-seed - p2.2, f3 * p2.0, f3 * p2.1]).max(0.0);
          ang *= 2.0 * mul;
          total_ang_area_disable_effect_passage += 1;
          total_ang_area_disable_effect_mul += mul;
        }

        ang += angle_base;

        ang += center_effect * (y - 0.5).atan2(x - 0.5);

        if ang_mod > 2.0 {
          let ai = ((ang + 8. * double_pi) % double_pi) / double_pi;
          ang = ((ai * ang_mod).round() / ang_mod) * double_pi;
        }

        if angle_div_grid > 0 {
          let g = (
            (angle_div_grid as f64 * x).floor() / (angle_div_grid as f64),
            (angle_div_grid as f64 * y).floor() / (angle_div_grid as f64),
          );
          ang += angle_div_grid_amp
            * perlin.get([
              angle_div_grid_freq * g.0,
              angle_div_grid_freq * g.1,
              opts.seed,
            ]);
        }

        if circles.len() > 0
          && circle_ang_delta.abs() > 0.1
          && circles
            .iter()
            .any(|other| other.inside((width * x, height * y)))
        {
          ang += circle_ang_delta;
        }

        ang
      };

      let get_color = |x: f64, y: f64, i: usize, j: usize, clr_group: usize| {
        let mut c = 0;
        if color_blink_mod > 0 {
          c += i / color_blink_mod;
        }

        if color_mod > 0 {
          c += j / color_mod;
        }

        if use_color_group {
          c += clr_group;
        }

        if circles.len() > 0
          && circle_color_change
          && circles.iter().any(|other| other.inside((x, y)))
        {
          c += 1;
        }

        let p_c = (x - width / 2.0, y - height / 2.0);

        let p = if color_split_rot_progress {
          p_r(p_c, double_pi * progress)
        } else if color_split_rot_diag {
          p_r(p_c, PI / 4.0)
        } else {
          p_c
        };

        if color_split_y > 0.0 {
          if ((p.1 / color_split_y + progress * color_stripe_progress_mul + 1000.0) % 1.0) < 0.5 {
            c += 1;
          }
        }
        if color_split_x > 0.0 {
          if ((p.0 / color_split_x + progress * color_stripe_progress_mul + 1000.0) % 1.0) < 0.5 {
            c += 1;
          }
        }
        if double_primary {
          c = c % 3;
        }
        c = c % 2;
        c
      };

      let mut rng_local = rng_from_seed(9999. + opts.seed * 7.7);

      let mut primary_routes = Vec::new();
      let mut secondary_routes = Vec::new();

      let pad = 0.2;
      let boundaries = (pad, pad, width - pad, height - pad);

      // inital points with color group
      let mut initial_points: Vec<((f64, f64), usize)> = Vec::new();

      match distribution {
        InitialPointDistribution::MillSpiral => {
          let radius = width * 0.7;
          let count = (50. * radius) as usize;
          let a_incr = 0.05 + rng_local.gen_range(0.0, 0.5) * rng_local.gen_range(0.0, 1.0);
          let d = radius / 20.0;
          let center = (width / 2.0, height / 2.0);
          for i in 0..count {
            let k = i as f64 / (count as f64);
            let a = 2. * PI * ((i as f64) * a_incr + progress * spiral_rot);
            let r = radius * k.sqrt() - 0.5 * d;
            let x = center.0 + r * a.cos();
            let y = center.1 + r * a.sin();
            let p = (x, y);
            initial_points.push((p, 0));
          }
        }
        InitialPointDistribution::GoldSpiralCircle | InitialPointDistribution::GoldSpiral => {
          // Spiral Distribution
          let radius = width
            * (match distribution {
              InitialPointDistribution::GoldSpiralCircle => 0.4,
              _ => 0.7,
            });
          let golden_ratio = (1. + (5f64).sqrt()) / 2.;
          let count = (50. * radius) as usize;
          let d = radius / 20.0;
          let center = (width / 2.0, height / 2.0);
          for i in 0..count {
            let k = i as f64 / (count as f64);
            let a = 2. * PI * ((i as f64) / (golden_ratio * golden_ratio) + progress * spiral_rot);
            let r = radius * k.sqrt() - 0.5 * d;
            let x = center.0 + r * a.cos();
            let y = center.1 + r * a.sin();
            let p = (x, y);
            initial_points.push((p, 0));
          }
        }
        InitialPointDistribution::NestedSquares => {
          let a_incr = rng_local.gen_range(-10.0, 5f64).max(0.0).min(PI / 4.0);
          let base_r = 0.08;
          let incr_r = rng_local.gen_range(0.3, 0.36) - 0.1 * a_incr.sin();
          for i in 0..nested_count {
            let r = width
              * if nested_count == 1 {
                0.3
              } else {
                base_r + incr_r * (i as f64 / (nested_count as f64 - 1.0))
              };
            let a = i as f64 * a_incr;
            let side = 2. * r;
            let side_splits = (4.0 * r) as usize;
            let square_points = 4 * side_splits;
            let topleft = (width / 2. - r, height / 2. - r);
            for j in 0..square_points {
              let d = j / side_splits;
              let rest = j - side_splits * d;
              let side_percent = (rest as f64 + progress) / (side_splits as f64);
              let mut p = match d {
                0 => (topleft.0 + side * side_percent, topleft.1),
                1 => (topleft.0 + side, topleft.1 + side * side_percent),
                2 => (topleft.0 + side * (1. - side_percent), topleft.1 + side),
                _ => (topleft.0, topleft.1 + side * (1. - side_percent)),
              };
              p.0 -= width / 2.0;
              p.1 -= height / 2.0;
              p = p_r(p, a);
              p.0 += width / 2.0;
              p.1 += height / 2.0;
              initial_points.push((p, i));
            }
          }
        }
        InitialPointDistribution::NestedTriangles => {
          let a_incr = rng_local.gen_range(-2.0, 0.3f64).max(0.0);
          let base_r = rng_local.gen_range(0.06, 0.12);
          let incr_r = 0.4 - 0.1 * a_incr;
          let incr = 0.4;
          for i in 0..nested_count {
            let r = width
              * if nested_count == 1 {
                0.3
              } else {
                base_r + incr_r * (i as f64 / (nested_count as f64 - 1.0))
              };
            let a_off = i as f64 * a_incr;
            let l = 2.0 * r;
            let origin = (width / 2.0 - r, height / 2.0 - 0.7 * r);
            let mut a: f64 = 0.0;
            let mut p = origin;
            loop {
              if a > double_pi - 0.001 {
                break;
              }
              let new_p = (p.0 + l * a.cos(), p.1 + l * a.sin());
              let dx = new_p.0 - p.0;
              let dy = new_p.1 - p.1;
              let d = (dx * dx + dy * dy).sqrt();
              for i in 0..((d / incr).ceil() as usize + 1) {
                let v = (i as f64 * incr) / d;
                let x = p.0 + dx * v;
                let y = p.1 + dy * v;
                let rotated = p_r((x - width / 2., y - height / 2.), a_off);
                initial_points.push(((rotated.0 + width / 2., rotated.1 + height / 2.), 0));
              }
              p = new_p;
              a += PI * 2. / 3.;
            }
          }
        }
        _ => {
          initial_points = static_initial_points.clone();
        }
      }

      let mut passage = Passage2DCounter::new(precision, width, height);
      let passage_limit = 4;
      let precision2 = precision * precision;

      for (j, &((x, y), clr_group)) in initial_points.iter().enumerate() {
        let mut p = (x, y);
        if p.0 < boundaries.0
          || p.0 > boundaries.2
          || p.1 < boundaries.1
          || p.1 > boundaries.3
          || passage.count(p) > passage_limit
        {
          continue;
        }
        let mut route = Vec::new();
        let iterations = (base_length / subprecision) as usize;
        if iterations > 6 {
          let mut last_p = (-100.0, -100.0);

          for _i in 0..iterations {
            let normalized = (p.0 / width, p.1 / height);
            let angle = field(normalized);
            let (px, py) = p;
            p = (
              p.0 + subprecision * angle.cos(),
              p.1 + subprecision * angle.sin(),
            );

            let mut collides = false;
            let mut percent_collide = 1f64;

            // glue to boundaries
            if p.0 < boundaries.0 {
              percent_collide = (boundaries.0 - px) / (p.0 - px);
              collides = true;
            } else if p.0 > boundaries.2 {
              percent_collide = (boundaries.2 - px) / (p.0 - px);
              collides = true;
            }
            if p.1 < boundaries.1 {
              let v = (boundaries.1 - py) / (p.1 - py);
              percent_collide = percent_collide.min(v);
              collides = true;
            } else if p.1 > boundaries.3 {
              let v = (boundaries.3 - py) / (p.1 - py);
              percent_collide = percent_collide.min(v);
              collides = true;
            }
            if collides {
              p = (
                px + (p.0 - px) * percent_collide,
                py + (p.1 - py) * percent_collide,
              );
              route.push(p);
              break;
            }

            let x = px;
            let y = py;
            let dx = x - last_p.0;
            let dy = y - last_p.1;
            if dx * dx + dy * dy > precision2 {
              let p = (x, y);
              last_p = p;

              if passage.count(p) > passage_limit {
                break;
              }

              route.push(p);
            }
          }

          if route.len() > 2 {
            let clr = get_color(x, y, i, j, clr_group);
            if clr == 0 {
              primary_routes.push(route);
            } else if clr == 1 {
              secondary_routes.push(route);
            }
          }
        }
      }
      //}

      let dx = xi as f64 * width;
      let dy = yi as f64 * height;
      let remap = |points: &Vec<(f64, f64)>| points.iter().map(|(x, y)| (x + dx, y + dy)).collect();
      let primary: Vec<Vec<(f64, f64)>> = primary_routes.iter().map(remap).collect();
      let secondary: Vec<Vec<(f64, f64)>> = secondary_routes.iter().map(remap).collect();

      all_primary.push(secondary);
      all_secondary.push(primary);
      i += 1;
    }
  }

  let layer_primary = all_primary.concat();
  let layer_secondary = all_secondary.concat();

  let mut inks = Vec::new();

  let layers: Vec<Group> = vec![
    ("#0FF", opts.primary_name.clone(), layer_primary),
    ("#F0F", opts.secondary_name.clone(), layer_secondary),
  ]
  .iter()
  .filter(|(_color, _label, routes)| routes.len() > 0)
  .map(|(color, label, routes)| {
    inks.push(label.clone());
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

  inks.sort();
  if inks.len() == 2 && inks[0].eq(&inks[1]) {
    inks.remove(1);
  }

  let mut map = Map::new();
  map.insert(String::from("Inks Count"), json!(inks.len()));
  map.insert(String::from("Inks"), json!(inks.join(" + ")));

  let amps_max = a1;
  map.insert(
    String::from("Noise Amp"),
    json!(if amps_max < 2.5 {
      "Low"
    } else if amps_max < 5.0 {
      "Medium"
    } else if amps_max < 8.0 {
      "High"
    } else {
      "Very High"
    }),
  );

  let mut anims = Vec::new();
  let stats_disabled = if total_ang_area_disable_effect_passage > 0 {
    total_ang_area_disable_effect_mul / (total_ang_area_disable_effect_passage as f64)
  } else {
    1.0
  };
  if stats_disabled < 0.01 {
    anims.push("None")
  } else {
    if stats_disabled < 0.8 {
      anims.push("Partially");
    }
    anims.push(if noise_balance < 0.2 {
      "Intense"
    } else if noise_balance < 0.4 {
      "Fast"
    } else if noise_balance < 0.75 {
      "Normal"
    } else if noise_balance < 0.94 {
      "Slow"
    } else {
      "Very Slow"
    });
  }

  map.insert(String::from("Noise Animation"), json!(anims.join(" ")));

  map.insert(
    String::from("Noise Frequency"),
    json!(if f1 < 1.4 {
      "Low"
    } else if f1 < 6.0 {
      "Medium"
    } else if f1 < 12.0 {
      "High"
    } else {
      "Very High"
    }),
  );

  if circle_color_change
    || match distribution {
      InitialPointDistribution::DoubleCircles => true,
      InitialPointDistribution::Circles => true,
      _ => false,
    }
  {
    map.insert(
      String::from("Circles Amount"),
      json!(if total_circles < 6 {
        "Low"
      } else if total_circles < 12 {
        "Medium"
      } else if total_circles < 24 {
        "High"
      } else {
        "Huge"
      }),
    );
  }

  map.insert(
    String::from("Distribution"),
    json!(match distribution {
      InitialPointDistribution::Circles => "Circles",
      InitialPointDistribution::CrossLines => "CrossLines",
      InitialPointDistribution::Curve => "Curve",
      InitialPointDistribution::DoubleCircles => "DoubleCircles",
      InitialPointDistribution::GoldSpiral => "GoldSpiral",
      InitialPointDistribution::GoldSpiralCircle => "GoldSpiralCircle",
      InitialPointDistribution::MillSpiral => "MillSpiral",
      InitialPointDistribution::NestedCircles => "NestedCircles",
      InitialPointDistribution::NestedSquares => "NestedSquares",
      InitialPointDistribution::NestedTriangles => "NestedTriangles",
      InitialPointDistribution::Parametric => "Parametric",
      InitialPointDistribution::TriangleSpiral => "TriangleSpiral",
      InitialPointDistribution::Voronoi => "Voronoi",
      InitialPointDistribution::XLines => "XLines",
      InitialPointDistribution::YLines => "YLines",
    }),
  );

  if center_effect > 0.0 {
    map.insert(String::from("Center Effect"), json!("Yes"));
  }

  let mut color_changes = Vec::new();

  if circle_color_change {
    color_changes.push("Circles");
  }
  if color_split_x > 0.0 {
    color_changes.push("X-Split");
  }
  if color_split_y > 0.0 {
    color_changes.push("Y-Split");
  }
  if use_color_group {
    color_changes.push("Group");
  }
  if color_split_rot_progress {
    color_changes.push("Rot");
  }
  if color_split_rot_diag {
    color_changes.push("45deg");
  }
  if color_changes.len() > 0 {
    map.insert(
      String::from("Color Changes"),
      json!(color_changes.join(", ")),
    );
  }
  if color_mod > 0 {
    color_changes.push("Mod");
  }
  if color_blink_mod > 0 {
    map.insert(String::from("Color Blink"), json!(color_blink_mod));
  }

  if ang_mod > 0.0 {
    let round = ang_mod.round();
    let remain = (ang_mod - round).abs();
    map.insert(
      String::from("Angle Mod"),
      json!(if remain < 0.1 {
        format!("Exactly {}", round)
      } else {
        format!("Around {}", round)
      }),
    );
  }

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
  if f64::is_nan(first_p.0) {
    let mut copy = route.clone();
    copy.remove(0);
    return render_route_curve(data, copy);
  }
  let mut d = data.move_to((significant_str(first_p.0), significant_str(first_p.1)));
  for p in route {
    d = d.line_to((significant_str(p.0), significant_str(p.1)));
  }
  return d;
}

pub fn render_route_curve(data: Data, route: Vec<(f64, f64)>) -> Data {
  if route.len() == 0 {
    return data;
  }
  let mut first = true;
  let mut d = data;
  let mut last = route[0];
  for p in route {
    if first {
      first = false;
      d = d.move_to((significant_str(p.0), significant_str(p.1)));
    } else {
      d = d.quadratic_curve_to((
        significant_str(last.0),
        significant_str(last.1),
        significant_str((p.0 + last.0) / 2.),
        significant_str((p.1 + last.1) / 2.),
      ));
    }
    last = p;
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
  fn inside(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist((self.x, self.y), p) < self.r
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
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) = search_circle_radius(bound, &circles, x, y, min_scale, max_scale) {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
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

fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

fn sample_2d_candidates_f64<R: Rng>(
  f: &dyn Fn((f64, f64)) -> f64,
  dim: usize,
  samples: usize,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  let mut candidates = Vec::new();
  for x in 0..dim {
    for y in 0..dim {
      let p = ((x as f64) / (dim as f64), (y as f64) / (dim as f64));
      if f(p) > rng.gen_range(0.0, 1.0) {
        candidates.push(p);
      }
    }
  }
  rng.shuffle(&mut candidates);
  candidates.truncate(samples);
  return candidates;
}
