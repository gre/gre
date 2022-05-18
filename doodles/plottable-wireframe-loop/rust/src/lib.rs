/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Wireframe
 */
mod utils;
use byteorder::*;
use contour::ContourBuilder;
use geojson::Feature;
use kiss3d::camera::*;
use kiss3d::nalgebra::*;
use noise::*;
use rand::prelude::*;
use rand::Rng;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::*;
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
  let mut rng = rng_from_seed(seed);

  // grid used to roughly estimate where there are available white spaces to later place the shapes
  let passage_precision = 3.0;
  let mut global_passage = Passage2DCounter::new(passage_precision, width, width);

  // Randomly chose different parameters
  let pad = 4.0;
  let perlin = Perlin::new();
  let a1 = rng.gen_range(0.5, 4.) + rng.gen_range(0.0, 10.) * rng.gen_range(0.0, 1.0);
  let grid = (14.0 + mix(rng.gen_range(0.0, 20.0), 1.2 * a1, rng.gen_range(0.2, 1.0))) as usize;
  let a2 = rng.gen_range(0.5, 6.);
  let f1 = rng.gen_range(0.5, 2.0) / (4.0 + a1);
  let f2 = f1 * rng.gen_range(0.4, 2.0);
  let f3 = rng.gen_range(0.05, 0.4);
  let is_diff_inks = !opts.primary_name.eq(&opts.secondary_name);
  let blink_mode = if is_diff_inks && rng.gen_bool(0.1) {
    rng.gen_range(1, 2)
  } else {
    0
  };
  let anaglyph_mul = rng.gen_range(0.0, 2.0);
  let with_anaglyph =
    opts.primary_name.eq("FireAndIce") && opts.secondary_name.eq("Poppy Red") && rng.gen_bool(0.9);
  let with_columns = rng.gen_bool(0.23);
  let with_holes = !with_columns && rng.gen_bool(0.2 + 0.7);
  let ymirror_mode =
    rng.gen_bool((if with_holes { 0.5 } else { 0.25 }) * (if with_columns { 0.2 } else { 1.0 }));
  let ymirror_diff_color = is_diff_inks && !with_anaglyph && rng.gen_bool(0.4);
  let xmirror_mode = rng.gen_bool(0.02 * (if with_columns { 0.2 } else { 1.0 }));
  let xmirror_diff_color = is_diff_inks && !with_anaglyph && rng.gen_bool(0.2);
  let circles_diff_color = is_diff_inks
    && (xmirror_mode && xmirror_diff_color
      || ymirror_mode && ymirror_diff_color
      || rng.gen_bool(0.5))
    && rng.gen_bool(0.8);
  let yoffset = if ymirror_mode {
    if rng.gen_bool(0.33) {
      0.0
    } else {
      rng.gen_range(10.0, 20.0)
    }
  } else {
    0.0
  };

  let hole_threshold = if with_holes {
    rng.gen_range(0.2, 0.4)
  } else {
    1.0
  };
  let distortion = rng.gen_range(0.1, 1.0) * rng.gen_range(0.0, 1.0);
  let distortion_frequency_mult = rng.gen_range(0.5, 2.0);
  let angle = if rng.gen_bool(0.7) {
    PI / 4.0
  } else {
    if rng.gen_bool(0.5) {
      PI / 4.0 + 2. * PI * (rng.gen_range(0, 4) as f64) / 4.
    } else {
      2. * PI * (rng.gen_range(0, 8) as f64) / 8.
    }
  };
  let angle2 = if rng.gen_bool(0.4) {
    angle
  } else {
    angle + 2. * PI * (rng.gen_range(0, 4) as f64) / 4.
  };

  let flat1 = rng.gen_range(0.0, 3.0);
  let flat2 = flat1;
  let flat_delta = flat1.min(flat2) * rng.gen_range(-0.5, 0.5);

  let with_y_rotate_motion = a1.max(a2) < 1.0 && rng.gen_bool(0.5) || rng.gen_bool(0.02);
  let radius1 = 0.5 + rng.gen_range(0.0, PI);
  let radius2 = 0.5 + rng.gen_range(0.0, PI) * rng.gen_range(0.0, 1.0);
  let stripes = rng.gen_range(6f32, 14f32);
  let with_x_stripes = is_diff_inks && !with_anaglyph && rng.gen_bool(0.08);
  let with_z_stripes = is_diff_inks && !with_anaglyph && !with_x_stripes && rng.gen_bool(0.08);
  let with_grid_stripes =
    is_diff_inks && !with_anaglyph && !with_x_stripes && !with_z_stripes && rng.gen_bool(0.08);
  let with_y_split_color = is_diff_inks
    && !with_anaglyph
    && !with_x_stripes
    && !with_z_stripes
    && !with_grid_stripes
    && rng.gen_bool(0.05);

  // these function gives a weight on how the "zigzag" shape should sort out the points
  let zz1 = |p: &(f64, f64)| p.0;
  let zz2 = |p: &(f64, f64)| p.1;
  let zz = if rng.gen_bool(0.4) { zz1 } else { zz2 };

  let yfocus = if with_columns && !ymirror_mode {
    0.0
  } else {
    rng.gen_range(-6.0, 0.0)
  };
  let camdist = if with_columns {
    rng.gen_range(-9.0, -6.0)
  } else {
    -6.0
  };

  let mut all_primary = Vec::new();
  let mut all_secondary = Vec::new();
  let mut i = 0;
  // iterate over all the grid cells
  for yi in 0..gridh {
    for xi in 0..gridw {
      let mut primary_routes = Vec::new();
      let mut secondary_routes = Vec::new();
      let index = i;
      let frames = gridw * gridh;
      let progress = index as f64 / (frames as f64);
      let w = grid;
      let h = grid;

      let color_terrain = |p: &Point3<f32>| {
        if with_y_split_color {
          return if p.y > 0.0 { 0 } else { 1 };
        }

        if with_z_stripes {
          let z = (p.z / stripes) % 1.0;
          return if z < 0.5 { 0 } else { 1 };
        }

        if with_x_stripes {
          let x = (p.x / stripes) % 1.0;
          return if x < 0.5 { 0 } else { 1 };
        }

        if with_grid_stripes {
          let x: f32 =
            (2f32 * ((p.x / stripes) % 1f32)).floor() + (2f32 * ((p.z / stripes) % 1f32)).floor();
          return if x == 1.0 { 0 } else { 1 };
        }

        0
      };

      // calc_y gives the y height of a given (x,z)
      let mut precalculated: HashMap<String, f32> = HashMap::new();
      let mut calc_y = |xi: usize, zi: usize| -> f32 {
        let key = format!("{} {}", xi, zi);
        let maybe_value = precalculated.get(&key);
        if maybe_value.is_some() {
          return *maybe_value.unwrap();
        }
        let x = xi as f64;
        let z = zi as f64;
        let p_ref = (20. * z / (grid as f64), 20. * x / (grid as f64));
        let position = p_r(((p_ref.0 - 10.0), (p_ref.1 - 10.0)), progress * 2. * PI);
        let p3d1 = if with_y_rotate_motion {
          (position.0, position.1, seed)
        } else {
          let position = p_r(p_ref, angle);
          project_cylinder_translation(progress, position, radius1, (seed, 0.0))
        };
        let p3d2 = if with_y_rotate_motion {
          (position.0, position.1, 77.7777 + seed)
        } else {
          let position = p_r(p_ref, angle2);
          project_cylinder_translation(progress, position, radius2, (0.0, seed))
        };
        let p3d3 = if with_y_rotate_motion {
          (position.0, position.1, 333.3333 + seed)
        } else {
          let position = p_r(p_ref, angle);
          project_cylinder_translation(progress, position, radius1, (0.0, -seed))
        };

        // two harmonies of noises added together. perlin noise and domain warping techniques.
        let mut n = a1
          * perlin
            .get([
              p3d1.0 * f1,
              p3d1.1 * f1
                + distortion
                  * perlin.get([
                    p3d1.2 * f1 * distortion_frequency_mult,
                    p3d1.1 * f1 * distortion_frequency_mult,
                    seed + p3d1.0 * f1 * distortion_frequency_mult,
                  ]),
              p3d1.2 * f1,
            ])
            .min(flat1 + flat_delta)
            .max(-flat1 + flat_delta);
        n += a2
          * perlin
            .get([
              p3d2.0 * f2
                + distortion
                  * perlin.get([
                    seed + p3d2.1 * f2 * distortion_frequency_mult,
                    p3d2.0 * f2 * distortion_frequency_mult,
                    p3d2.2 * f2 * distortion_frequency_mult,
                  ]),
              p3d2.1 * f2,
              p3d2.2 * f2,
            ])
            .min(flat2 + flat_delta)
            .max(-flat2 + flat_delta);

        if with_holes {
          // another simple perlin noise determines the whole
          let v = perlin.get([p3d3.0 * f3, p3d3.1 * f3, p3d3.2 * f3]);
          n -= if v > hole_threshold { 200.0 } else { 0.0 };
        }

        let ret = n as f32;
        precalculated.insert(key, ret);
        ret
      };

      // for esthetical reason, i remove some polygon of the rectangle (the ones in the front)
      let visible = |x, _y, z| x + z > 2;

      // Paths accumate a bunch of 3D lines that compose the wireframe
      let mut paths = vec![];

      // Add all the "columns" from points that are "in front"
      let mut column_tops = vec![];
      if with_columns {
        for x in 0..(w + 1) {
          for z in 0..(h + 1) {
            if x == 0 || z == 0 || x + z < 5 {
              let y = calc_y(x, z);
              if visible(x, y, z) {
                let main_p = Point3::new(x as f32, y, z as f32);
                column_tops.push(main_p);
              }
            }
          }
        }
      }

      // NB: unlike on plot/448, I have chose to do 3 passes of independant axis (x, y, and diagonals) instead of the technique to "never lift up the pen". This is needed for the color ink simulation of WEBGL to work properly. also, as we make "holes", we wouldn't always benefit of no lift up in any case. AxiDraw plugin can later optimize the tracing.

      // trace x lines
      for x in 0..(w + 1) {
        let xf = x as f32;
        let mut path = Vec::new();
        for z in 0..(h + 1) {
          let y = calc_y(x, z);
          if visible(x, y, z) {
            let zf = z as f32;
            path.push(Point3::new(xf, y, zf));
          } else {
            paths.push(path);
            path = Vec::new();
          }
        }
        paths.push(path);
      }

      // trace z lines
      for z in 0..(h + 1) {
        let zf = z as f32;
        let mut path = Vec::new();
        for x in 0..(w + 1) {
          let y = calc_y(x, z);
          if visible(x, y, z) {
            let xf = x as f32;
            path.push(Point3::new(xf, y, zf));
          } else {
            paths.push(path);
            path = Vec::new();
          }
        }
        paths.push(path);
      }

      // trace diag
      for i in 0..(w + h) {
        let (sx, sy, count) = if i < h {
          (0, h - i, i + 1)
        } else {
          (i - h, 0, 2 * h + 1 - i)
        };
        let mut path = Vec::new();
        for j in 0..count {
          let x = sx + j;
          let z = sy + j;
          let y = calc_y(x, z);
          if visible(x, y, z) {
            path.push(Point3::new(x as f32, y, z as f32));
          } else {
            paths.push(path);
            path = Vec::new();
          }
        }
        paths.push(path);
      }

      let dim = Vector2::new((width - 2. * pad) as f32, (height - 2. * pad) as f32);
      let offset = Vector2::new(pad as f32, pad as f32);

      // if no anaglyph we iterate on [0]
      // if anaglyph we iterate on [1, 2]
      for anaglyph_index in (if with_anaglyph { 1 } else { 0 })..(if with_anaglyph { 3 } else { 1 })
      {
        let camoffset = if anaglyph_index > 0 {
          anaglyph_mul * (0.5 - ((anaglyph_index - 1) as f32))
        } else {
          0.0
        };
        let camera = FirstPerson::new(
          Point3::new(camdist + camoffset, 10.0 + yoffset, camdist - camoffset),
          Point3::new(grid as f32 / 2.0, yfocus + yoffset, grid as f32 / 2.0),
        );

        let mut local_passage = Passage2DCounter::new(0.6, width, height);
        let max_passage = 5;

        // convenient utility to share that logic
        let mut push_route = |route: &Vec<(f64, f64)>, clr: usize| {
          if clr == 0 {
            primary_routes.push(route.clone());
          } else if clr == 1 {
            secondary_routes.push(route.clone());
          } else {
            primary_routes.push(route.clone());
            secondary_routes.push(route.clone());
          }
        };

        // build up columns
        for p in column_tops.iter() {
          let pr = camera.project(&p, &dim);
          let pos = ((offset.x + pr.x) as f64, (offset.y + dim.y - pr.y) as f64);
          let out_of_bounds = pos.0 < 0.0 || pos.1 < 0.0 || pos.0 > width || pos.1 > height;
          if out_of_bounds {
            continue;
          }
          let clr = if with_anaglyph {
            anaglyph_index - 1
          } else {
            color_terrain(&p)
          };
          let route = vec![pos, (pos.0, height - 0.2)];
          // fill the collision map of the columns to avoid chosing this place to put shapes in
          let mut y = pos.1;
          loop {
            if y >= height {
              break;
            }
            let p = (pos.0, y);
            global_passage.count(p);
            local_passage.count(p);
            y += passage_precision;
          }
          push_route(&route, clr);
        }

        // build up the main wireframe
        for points in paths.iter() {
          if points.len() < 2 {
            continue;
          }
          let mut last_clr = 0;
          let mut route: Vec<(f64, f64)> = Vec::new();

          for p in points.iter() {
            let pr = camera.project(&p, &dim);
            let pos = ((offset.x + pr.x) as f64, (offset.y + dim.y - pr.y) as f64);
            let out_of_bounds = pos.0 < 0.0 || pos.1 < 0.0 || pos.0 > width || pos.1 > height;
            let p_c = if route.len() > 0 {
              ((route[0].0 + pos.0) / 2.0, (route[0].1 + pos.1) / 2.0)
            } else {
              pos
            };
            let count = local_passage.count(p_c);
            if !out_of_bounds && count < max_passage {
              let clr = if with_anaglyph {
                anaglyph_index - 1
              } else {
                color_terrain(&p)
              };
              route.push(pos);
              if last_clr != clr && route.len() > 1 {
                push_route(&route, last_clr);
                route = vec![pos];
              }
              last_clr = clr;
            } else {
              if route.len() > 1 {
                push_route(&route, last_clr);
              }
              route = Vec::new();
            }
          }
          if route.len() > 1 {
            push_route(&route, last_clr);
          }
        }
      }

      // for mirroring we just dumbly repeat the shapes
      if ymirror_mode {
        let reverse_y = |r: &Vec<(f64, f64)>| r.iter().map(|&(x, y)| (x, height - y)).collect();
        primary_routes = vec![
          primary_routes.clone(),
          (if ymirror_diff_color {
            secondary_routes.iter()
          } else {
            primary_routes.iter()
          })
          .map(reverse_y)
          .collect(),
        ]
        .concat();
        secondary_routes = vec![
          secondary_routes.clone(),
          (if ymirror_diff_color {
            primary_routes.iter()
          } else {
            secondary_routes.iter()
          })
          .map(reverse_y)
          .collect(),
        ]
        .concat();
      }

      if xmirror_mode {
        let reverse_x = |r: &Vec<(f64, f64)>| r.iter().map(|&(x, y)| (width - x, y)).collect();
        primary_routes = vec![
          primary_routes.clone(),
          (if xmirror_diff_color {
            secondary_routes.iter()
          } else {
            primary_routes.iter()
          })
          .map(reverse_x)
          .collect(),
        ]
        .concat();
        secondary_routes = vec![
          secondary_routes.clone(),
          (if xmirror_diff_color {
            primary_routes.iter()
          } else {
            secondary_routes.iter()
          })
          .map(reverse_x)
          .collect(),
        ]
        .concat();
      }

      for routes in vec![primary_routes.clone(), secondary_routes.clone()].concat() {
        for r in routes {
          global_passage.count(r);
        }
      }

      let dx = xi as f64 * width;
      let dy = yi as f64 * height;
      let remap = |points: &Vec<(f64, f64)>| points.iter().map(|(x, y)| (x + dx, y + dy)).collect();
      let primary: Vec<Vec<(f64, f64)>> = primary_routes.iter().map(remap).collect();
      let secondary: Vec<Vec<(f64, f64)>> = secondary_routes.iter().map(remap).collect();

      if blink_mode > 0 && (index / blink_mode) % 2 > 0 {
        all_primary.push(secondary);
        all_secondary.push(primary);
      } else {
        all_primary.push(primary);
        all_secondary.push(secondary);
      }
      i += 1;
    }
  }

  // now, going to try to find some packing opportunity...
  // impressively it runs pretty fast, but it's not very optimized algo ;)
  let circles = packing(
    seed,
    60000,
    &global_passage,
    rng.gen_range(1, 20),
    rng.gen_range(20, 60),
    rng.gen_range(3.0, 4.0),
    (0.0, 0.0, width, height),
    rng.gen_range(4.0, 18.0),
    width / 2.0,
  );

  let mut global_shapes = Vec::new();

  if circles.len() > 0 {
    let same_shapes = rng.gen_bool(0.5);

    // we are pushing shapes in all the circles place opportunity for each grid cell
    let mut i = 0;
    for yi in 0..gridh {
      for xi in 0..gridw {
        let mut primary_routes = Vec::new();
        let mut secondary_routes = Vec::new();
        let index = i;
        i += 1;
        let frames = gridw * gridh;
        let progress = index as f64 / (frames as f64);
        // local rng is the same rng for all the cells so we are sure we make the same choices. (and yes, we could have calculated this once)
        let mut local_rng = rng_from_seed(seed);

        let mut moon_count = 0;
        let mut shapes = vec![];

        for c in circles.iter() {
          let mut routes = Vec::new();

          let shape = if same_shapes && shapes.len() > 0 {
            shapes[0]
          } else if local_rng
            .gen_bool((if same_shapes { 0.1 } else { 0.28 }) / (moon_count as f64 + 1.0))
            && c.y < 0.4 * height
          {
            moon_count += 1;
            "Moon"
          } else if local_rng.gen_bool(0.25) {
            "ZigZag"
          } else if local_rng.gen_bool(0.2) {
            "Curves"
          } else if local_rng.gen_bool(0.3) {
            "SpiralCircle"
          } else if local_rng.gen_bool(0.15) {
            "Spiral"
          } else {
            "Head"
          };

          shapes.push(shape);

          if shape.eq("Curves") {
            let center = (c.x, c.y);
            let r = c.r;
            for _i in 0..3 {
              let count = 4 + (r * (if with_anaglyph { 3.0 } else { 6.0 })) as usize;
              let mut points = circle_route(center, r, count);
              rng.shuffle(&mut points); // rng is used to vary on each tile
              points.push(center);
              points.insert(0, center);
              points.insert(0, (f64::NAN, f64::NAN));
              routes.push(points);
            }
          } else if shape.eq("ZigZag") {
            let center = (c.x, c.y);
            let r = c.r;
            let count = 10 + (r * 40.0) as usize;
            let mut points = circle_route(center, r, count);
            rng.shuffle(&mut points);
            points.truncate(points.len() / (if with_anaglyph { 4 } else { 2 }));
            points.sort_by(|a, b| (zz(b)).partial_cmp(&zz(a)).unwrap());
            routes.push(points);
          } else if shape.eq("Head") {
            routes = vec![
              routes,
              head(
                seed,
                c.x,
                c.y,
                c.r,
                ((if with_anaglyph { 1.2 } else { 2.0 }) * c.r) as usize,
                (c.x + c.y) / 111.0,
                progress,
              ),
            ]
            .concat();
          } else if shape.eq("SpiralCircle") {
            let mut a = progress * 2.0 * PI;
            let increment = local_rng.gen_range(0.4, 0.7);
            let mut r = c.r;
            let min_r = (local_rng.gen_range(0.1, 0.5)
              + local_rng.gen_range(0.0, 0.2) * (2. * PI * progress).cos())
              * c.r;
            let aincr = local_rng.gen_range(0.0, 0.3);
            loop {
              if r < min_r {
                break;
              }
              let center = (
                c.x + 0.5 * (c.r - r) * a.cos(),
                c.y + 0.5 * (c.r - r) * a.sin(),
              );
              let count = 10 + (r * 10.0) as usize;
              routes.push(circle_route(center, r, count));
              a += aincr;
              r -= increment;
            }
          } else if shape.eq("Spiral") {
            let mut r = c.r;
            let mut a = -progress * 2.0 * PI;
            let mut points = Vec::new();
            let increment = 0.015;
            loop {
              if r < 0.1 {
                break;
              }
              points.push((c.x + r * a.cos(), c.y + r * a.sin()));
              a += 4. / (20. + r.powf(0.5));
              r -= increment;
            }
            routes.push(points);
          } else if shape.eq("Moon") {
            let incr = 0.4;
            let mut delta = 0.0;
            let mut angle = (c.y - height / 2.0).atan2(c.x - width / 2.0);
            if local_rng.gen_bool(0.5) {
              angle += PI / 2.0;
            } else {
              angle -= PI / 2.0;
            }
            angle += 0.1 * (progress * 2.0 * PI).cos();
            let d = (angle.cos(), angle.sin());
            let phase = 0.3
              + local_rng.gen_range(0.0, 1.0) * local_rng.gen_range(0.0, 1.0)
              + local_rng.gen_range(0.0, 0.2) * (progress * 2.0 * PI).sin();
            loop {
              if delta > c.r * phase {
                break;
              }
              let mut a = 0.0;
              let aincr = 0.1 / c.r;
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

          let delta = local_rng.gen_range(0.0, 1.6);
          let clr_rand = local_rng.gen_bool(0.5);
          for route in routes {
            if with_anaglyph {
              secondary_routes.push(route.iter().map(|p| (p.0 - delta, p.1)).collect());
              primary_routes.push(route.iter().map(|p| (p.0 + delta, p.1)).collect());
            } else {
              if circles_diff_color {
                if clr_rand {
                  secondary_routes.push(route);
                } else {
                  primary_routes.push(route);
                }
              } else {
                if is_diff_inks {
                  secondary_routes.push(route);
                } else {
                  primary_routes.push(route);
                }
              }
            }
          }
        }

        if index == 0 {
          global_shapes = shapes;
        }
        let dx = xi as f64 * width;
        let dy = yi as f64 * height;
        let remap =
          |points: &Vec<(f64, f64)>| points.iter().map(|(x, y)| (x + dx, y + dy)).collect();
        let primary: Vec<Vec<(f64, f64)>> = primary_routes.iter().map(remap).collect();
        let secondary: Vec<Vec<(f64, f64)>> = secondary_routes.iter().map(remap).collect();

        if blink_mode > 0 && (index / blink_mode) % 2 > 0 {
          all_primary.push(secondary);
          all_secondary.push(primary);
        } else {
          all_primary.push(primary);
          all_secondary.push(secondary);
        }
      }
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

    let opacity: f64 = 0.6;
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

  let amps_max = a1.max(a2);
  map.insert(
    String::from("Noise Amp"),
    json!(if amps_max < 1.0 {
      "Low"
    } else if amps_max < 6.0 {
      "Medium"
    } else if amps_max < 10.0 {
      "High"
    } else {
      "Very High"
    }),
  );

  map.insert(
    String::from("Noise Frequency"),
    json!(if f1 < 0.08 {
      "Low"
    } else if f1 < 0.15 {
      "Medium"
    } else if f1 < 0.25 {
      "High"
    } else {
      "Very High"
    }),
  );

  map.insert(String::from("Grid Size"), json!(grid));

  if blink_mode > 0 {
    map.insert(String::from("Blink Mode"), json!(blink_mode));
  }
  if with_anaglyph {
    map.insert(String::from("Anaglyph"), json!("Yes"));
  }
  if with_x_stripes {
    map.insert(String::from("X-Stripes"), json!("Yes"));
  }
  if with_z_stripes {
    map.insert(String::from("Z-Stripes"), json!("Yes"));
  }
  if with_grid_stripes {
    map.insert(String::from("Grid-Stripes"), json!("Yes"));
  }
  if with_y_split_color {
    map.insert(String::from("Y-Split Color"), json!("Yes"));
  }
  if with_columns {
    map.insert(String::from("Columns"), json!("Yes"));
  }

  if with_holes {
    map.insert(String::from("Holes"), json!("Yes"));
  }

  if ymirror_mode {
    map.insert(
      String::from("Y Mirror"),
      json!(vec![
        if ymirror_diff_color {
          "Bi Color"
        } else {
          "Same Color"
        },
        if yoffset < 10.0 { "Centered" } else { "" }
      ]
      .join(" ")
      .trim()),
    );
  }

  if xmirror_mode {
    map.insert(
      String::from("X Mirror"),
      json!(json!(if xmirror_diff_color {
        "Bi Color"
      } else {
        "Same Color"
      })),
    );
  }

  if distortion > 0.1 {
    map.insert(
      String::from("Distortion"),
      json!(if distortion < 0.3 {
        "Low"
      } else if amps_max < 0.7 {
        "Medium"
      } else {
        "High"
      }),
    );
  }

  if with_y_rotate_motion {
    map.insert(String::from("Motion"), json!("Y-Rotation"));
  } else {
    map.insert(String::from("Motion"), json!("Directional"));
    let directions = vec![
      "North East",
      "North",
      "North West",
      "West",
      "South West",
      "South",
      "South East",
      "East",
    ];
    let map_direction = |angle| directions[(0.5 + (2. * PI + angle) / (PI / 4.0)) as usize % 8];
    map.insert(String::from("Direction"), json!(map_direction(angle)));
    if !nearly_same(angle, angle2) {
      map.insert(String::from("Direction Bis"), json!(map_direction(angle2)));
    }

    let radius_min = radius1.min(radius2);
    map.insert(
      String::from("Waves Distance"),
      json!(if radius_min > 2.8 {
        "Distant"
      } else if radius_min > 2.0 {
        "Normal"
      } else if radius_min > 1.0 {
        "Close"
      } else {
        "Stretched"
      }),
    );
  }

  if global_shapes.len() > 0 {
    map.insert(String::from("Extra Shapes"), json!(global_shapes.len()));
    let mut deduped = global_shapes.clone();
    deduped.sort();
    deduped.dedup();
    map.insert(String::from("Shapes"), json!(deduped.join(", ")));
  }

  let min_flat = flat1.min(flat2);
  if min_flat < 0.4 {
    map.insert(
      String::from("Flat Terrain"),
      json!(if min_flat > 0.3 {
        "Low"
      } else if min_flat > 0.2 {
        "Normal"
      } else if min_flat > 0.1 {
        "High"
      } else {
        "Important"
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
  passage: &Passage2DCounter,
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
      && !passage.collides(&c)
  };
  scaling_search(f, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  passage: &Passage2DCounter,
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
    if let Some(size) = search_circle_radius(bound, &circles, &passage, x, y, min_scale, max_scale)
    {
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

fn circle_route(center: (f64, f64), r: f64, count: usize) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

fn head(
  seed: f64,
  cx: f64,
  cy: f64,
  r: f64,
  samples: usize,
  seed_delta: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let precision = 0.3;
  let w = (4. * r / precision) as u32;
  let h = (4. * r / precision) as u32;
  let perlin = Perlin::new();
  let f = |(x, y): (f64, f64)| -> f64 {
    let dx: f64 = x - 0.5;
    let dy: f64 = y - 0.5;
    let mut res: f64 = (dx * dx + dy * dy).sqrt();
    let xabs = (x - 0.5).abs();
    let (px, py) = p_r((dx, dy), 2.0 * PI * phase);
    let mut rng = rng_from_seed(seed);
    let f1 = 1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
    let f2 = rng.gen_range(0.8, 2.0) * f1;
    let f3 = rng.gen_range(2.0, 3.0) * f1;
    res += 0.07
      * perlin.get([
        // first level
        f1 * xabs,
        f1 * y + 0.3 * x,
        seed
          + seed_delta
          + 3.
            * perlin.get([
              // 2nd level
              f2 * px,
              f2 * py,
              seed
                + seed_delta
                + 0.2 * x
                + 3.
                  * perlin.get([
                    // 3rd level
                    seed + seed_delta,
                    f3 * xabs,
                    f3 * y,
                  ]),
            ]),
      ]);
    res * 4.5
  };
  let thresholds: Vec<f64> = (0..samples)
    .map(|i| (i as f64) / (samples as f64))
    .collect();
  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, (1.0, 1.0, 4. * r - 1., 4. * r - 1.));
  routes = translate_routes(routes, (cx - 2. * r, cy - 2. * r));
  routes
}

fn contour<F: FnMut((f64, f64)) -> f64>(
  width: u32,
  height: u32,
  mut f: F,
  thresholds: &Vec<f64>,
) -> Vec<Feature> {
  let c = ContourBuilder::new(width, height, true);
  let values = rasterize_1d(width, height, &mut f);
  c.contours(&values, &thresholds).unwrap_or(Vec::new())
}

fn rasterize_1d<F: FnMut((f64, f64)) -> f64>(width: u32, height: u32, mut f: F) -> Vec<f64> {
  (0..height)
    .flat_map(|y| {
      (0..width)
        .map(|x| f((x as f64 / width as f64, y as f64 / height as f64)))
        .collect::<Vec<f64>>()
    })
    .collect::<Vec<f64>>()
}

fn features_to_routes(features: Vec<Feature>, precision: f64) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  for f in features {
    for g in f.geometry {
      let value = g.value;
      match value {
        geojson::Value::MultiPolygon(all) => {
          for poly in all {
            for lines in poly {
              let mut points = lines
                .iter()
                .map(|p| (precision * p[0], precision * p[1]))
                .collect::<Vec<(f64, f64)>>();
              let len = points.len();
              if len < 3 {
                continue;
              }
              if euclidian_dist(points[0], points[len - 1]) <= precision {
                points.push(points[0]);
              }
              routes.push(points);
            }
          }
        }
        _ => {}
      }
    }
  }
  routes
}

fn translate_routes(routes: Vec<Vec<(f64, f64)>>, (tx, ty): (f64, f64)) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    .collect()
}

#[inline]
fn strictly_in_boundaries(p: (f64, f64), boundaries: (f64, f64, f64, f64)) -> bool {
  p.0 > boundaries.0 && p.0 < boundaries.2 && p.1 > boundaries.1 && p.1 < boundaries.3
}

fn crop_route(
  route: &Vec<(f64, f64)>,
  boundaries: (f64, f64, f64, f64),
) -> Option<Vec<(f64, f64)>> {
  if route.len() < 2
    || route
      .iter()
      .all(|&p| !strictly_in_boundaries(p, boundaries))
  {
    return None;
  }
  return Some(route.clone());
}

fn crop_routes(
  routes: &Vec<Vec<(f64, f64)>>,
  boundaries: (f64, f64, f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  return routes
    .iter()
    .filter_map(|route| crop_route(&route, boundaries))
    .collect();
}

#[inline]
fn nearly_same(a: f64, b: f64) -> bool {
  (b - a).abs() < 0.0001
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
  fn collides(self: &Self, c: &VCircle) -> bool {
    let cx = c.x;
    let cy = c.y;
    let r = c.r;
    let r2 = r * r;
    let mut x = cx - r;
    loop {
      if x > cx + r {
        break;
      }
      let mut y = cy - r;
      loop {
        if y > cy + r {
          break;
        }

        let dx = x - cx;
        let dy = y - cy;
        if dx * dx + dy * dy < r2 {
          if self.get((x, y)) > 0 {
            return true;
          }
        }
        y += self.granularity;
      }

      x += self.granularity;
    }
    return false;
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
