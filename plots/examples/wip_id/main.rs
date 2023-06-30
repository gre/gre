use clap::*;
use gre::*;
use noise::*;
use num_complex::Complex;
use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn dunes(
  seed: f64,
  origin: (f64, f64),
  width: f64,
  height: f64,
  pad: f64,
) -> Vec<Vec<(f64, f64)>> {
  let dy = 0.7;
  let mut rng = rng_from_seed(seed);
  let min_route = 8;
  let res1 = (rng.gen_range(0.5, 4.0), rng.gen_range(1.0, 3.0));
  let res2 = (rng.gen_range(0.5, 4.0), rng.gen_range(1.0, 3.0));
  let r1 = rng.gen_range(0.0, 1.0);
  let r2 = rng.gen_range(0.0, 1.0);
  let r3 = rng.gen_range(0.0, 1.0);
  let r4 = rng.gen_range(0.0, 1.0);
  let xyratiof = rng.gen_range(2.0, 4.0);

  let perlin = Perlin::new();

  let low_poly_perlin = |(xr, yr): (f64, f64), x: f64, y: f64, s: f64| {
    // quadradic interpolation between 4 noise points
    let xi = x / xr;
    let yi = y / yr;
    let x1 = xr * xi.floor();
    let y1 = yr * yi.floor();
    let x2 = xr * xi.ceil();
    let y2 = yr * yi.ceil();
    let xp = xi - xi.floor();
    let yp = yi - yi.floor();
    let p1 = perlin.get([x1, y1, s]);
    let p2 = perlin.get([x2, y1, s]);
    let p3 = perlin.get([x2, y2, s]);
    let p4 = perlin.get([x1, y2, s]);
    mix(
      mix(p1 as f64, p2 as f64, xp),
      mix(p4 as f64, p3 as f64, xp),
      yp,
    )
  };

  let mut routes = Vec::new();

  for ci in 0..2 {
    let from = height * 3.;
    let to = -height;
    let mut base_y = from;

    let mut height_map: Vec<f64> = Vec::new();
    loop {
      let precision = rng.gen_range(0.19, 0.21);
      if base_y < to {
        break;
      }
      let is_color = (base_y < height * 0.5) != (ci == 0);
      let mut route = Vec::new();
      let mut x = pad;
      let mut was_outside = true;
      let mut i = 0;
      loop {
        if x > width - pad {
          break;
        }
        let xv = (base_y / height) * (x - width / 2.);
        let amp = mix(0.8, 2.0, r1) * height * (base_y / height);
        let shape = -low_poly_perlin(
          res1,
          mix(0.5, 4.0, r3) * 0.01 * xv,
          mix(0.5, 4.0, r3) * 0.01 * xyratiof * base_y,
          7.7 * seed
            + mix(0.05, 0.2, r2)
              * low_poly_perlin(
                res2,
                mix(0.5, 4.0, r4) * 0.1 * xyratiof * base_y,
                mix(0.5, 4.0, r4) * 0.1 * xv,
                seed / 3.,
              ),
        )
        .abs();
        let displacement =
          mix(
            0.0008,
            0.01,
            smoothstep(-0.2, -0.5, shape).powf(2.0)
              * (base_y / height).max(0.0).min(1.0),
          ) * perlin.get([seed * 9.3, 0.5 * xyratiof * base_y, 0.5 * x]);
        let y = base_y + amp * (shape + displacement);
        let mut collides = false;
        let xi = i; // (x * 10.0) as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] - dy * 0.5 {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let inside = !collides
          && pad < x
          && x < width - pad
          && pad < y
          && y < height - pad;
        if inside {
          if was_outside {
            if route.len() > min_route {
              if is_color {
                routes.push(route);
              }
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push((x + origin.0, y + origin.1));
        } else {
          was_outside = true;
        }
        x += precision;
        i += 1;
      }

      if is_color {
        routes.push(route);
      }

      base_y -= dy;
    }
  }

  routes
}

fn yarnballs(
  seed: f64,
  origin: (f64, f64),
  width: f64,
  height: f64,
  pad: f64,
) -> Vec<(f64, f64)> {
  let mut rng = rng_from_seed(seed);
  let max_scale = rng.gen_range(5.0, 40.0);

  let mut circles = packing(
    3.3 * seed,
    1000000,
    1000,
    2,
    0.0,
    (
      origin.0 + pad,
      origin.1 + pad,
      origin.0 + width - pad,
      origin.1 + height - pad,
    ),
    2.0,
    max_scale,
  );

  let points: Vec<(f64, f64)> = circles.iter().map(|c| (c.x, c.y)).collect();

  let tour = travelling_salesman::simulated_annealing::solve(
    &points,
    time::Duration::seconds(20),
  );

  circles = tour.route.iter().map(|&i| circles[i]).collect();

  let route: Vec<(f64, f64)> = circles
    .par_iter()
    .flat_map(|circle| {
      let s = seed + circle.x * 3.1 + circle.y / 9.8;
      let mut rng = rng_from_seed(s);
      shape_strokes_random(&mut rng, circle)
    })
    .collect();

  route
}

fn mandelglitch(
  seed: f64,
  origin: (f64, f64),
  width: f64,
  height: f64,
  pad: f64,
) -> Vec<Vec<(f64, f64)>> {
  // Prepare all the colors
  let mut rng = rng_from_seed(seed);
  let lightness = 0.5;
  let noise_effect = rng.gen_range(0.0, 0.5);
  let kaleidoscope = rng.gen_bool(0.5);
  let kaleidoscope_mod = rng.gen_range(3, 20);

  // Prepare the generative code

  let mut routes: Vec<Vec<(f64, f64)>> = vec![];

  for _i in 0..2 {
    let s1 = rng.gen_range(0.0, 1.0);
    let s2 = rng.gen_range(0.0, 1.0);
    let s3 = rng.gen_range(0.0, 1.0);
    let s4 = rng.gen_range(0.0, 1.0);
    let s5 = rng.gen_range(0.0, 1.0);
    let s6 = rng.gen_range(0.0, 1.0);
    let s7 = rng.gen_range(0.0, 1.0);
    let s8 = rng.gen_range(0.0, 1.0);
    let s9 = rng.gen_range(0.0, 1.0);
    let mod1 = rng.gen_range(0.0, 1.0);

    let vignette_effect = rng.gen_range(-1f64, 4.0).max(0.0);
    let linear_effect = rng.gen_range(-5.0, 5.0) * rng.gen_range(0.0, 1.0);

    let mut map = WeightMap::new(width, height, 0.5);

    let perlin = Perlin::new();

    let seed = rng.gen_range(0.0, 1000.0);

    let f1 = 0.5 + rng.gen_range(0.5, 6.0) * rng.gen_range(0.1, 1.0);
    let f2 = 0.5 + rng.gen_range(0.5, 6.0) * rng.gen_range(0.1, 1.0);
    let f3 = 0.5 + rng.gen_range(0.5, 6.0) * rng.gen_range(0.1, 1.0);
    let noise_amp: f64 = noise_effect * rng.gen_range(0.2, 1.0);
    let warping = rng.gen_range(0.0, 4.0);

    let xsym = rng.gen_bool(0.5);

    let density_r = rng.gen_range(0.0, 0.6) * rng.gen_range(0.0, 1.0);

    map.fill_fn(|(x, y)| {
      if x < pad || x > width - pad || y < pad || y > height - pad {
        return 0.0;
      }

      let (x, y) = if kaleidoscope {
        kaleidoscope_project(
          (x, y),
          (width / 2.0, height / 2.0),
          2 * kaleidoscope_mod,
        )
      } else {
        (if xsym { (width as f64 - x).min(x) } else { x }, y)
      };

      // dist with center
      let d = (x - width / 2.0).hypot(y - height / 2.0);
      let d = d / (width / 2.0).max(height / 2.0);
      let d = d.powf(0.5) - 0.5;

      let dl = y / height - 0.5;

      let n = perlin.get([
        x / height as f64 * f1,
        y / height as f64 * f1,
        seed
          + warping
            * perlin.get([
              x / height as f64 * f2,
              y / height as f64 * f2,
              7.7 * seed
                + perlin.get([
                  77. + seed / 0.3,
                  x / height as f64 * f3,
                  y / height as f64 * f3,
                ]),
            ]),
      ]);

      let ratio = width / height;
      let mut p = ((ratio - 1.0) / 2.0 + x / width, y / height);

      p.0 += 0.2 * noise_amp.max(0.0) * (n * 10.0).cos();
      p.1 += 0.2 * noise_amp.max(0.0) * (n * 10.0).sin();

      (density_r + 1.0)
        * (7. * shade(p, s1, s2, s3, s4, s5, s6, s7, s8, s9, mod1)
          + vignette_effect * d
          + linear_effect * dl
          - lightness)
    });

    let step = 1.0 + rng.gen_range(0.0, 0.5);
    let rot = PI / rng.gen_range(1.0, 4.0);
    let straight = rng.gen_range(-0.2, 0.8) * rng.gen_range(0.0, 1.0);
    let min_l = 5;
    let max_l = 20;
    let decrease_value = 1.0;
    let min_weight = 1.0;

    let count = 800;
    let search_max = 500;
    let mut bail_out = 0;
    for _i in 0..count {
      let top = map.search_weight_top(&mut rng, search_max, min_weight);
      if top.is_none() {
        bail_out += 1;
        if bail_out > 10 {
          break;
        }
      }
      if let Some(o) = top {
        let angle = perlin.get([
          2. * o.0 / height as f64,
          2. * o.1 / height as f64,
          seed,
        ]);

        if let Some(a) = map.best_direction(o, step, angle, PI, PI / 4.0, 0.0) {
          let route = map.dig_random_route(
            o,
            a,
            step,
            rot,
            straight,
            max_l,
            decrease_value,
          );
          if route.len() >= min_l {
            let mut rt = rdp(&route, 0.05);
            rt = rt
              .iter()
              .map(|p| (p.0 + origin.0, p.1 + origin.1))
              .collect();
            routes.push(rt);
          }
        }
      }
    }
  }

  routes
}

fn cell(
  seed: f64,
  origin: (f64, f64),
  width: f64,
  height: f64,
  pad: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut rng = rng_from_seed(seed);
  match rng.gen_range(0, 4) {
    0 => dunes(seed, origin, width, height, pad),
    1 => vec![yarnballs(seed, origin, width, height, pad)],
    2 => clouds(seed, origin, width, height, pad),
    _ => mandelglitch(seed, origin, width, height, pad),
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;

  let cw = 85.;
  let ch = 55.;
  let pad = 5.;

  let cols = (width / cw).floor() as usize;
  let rows = (height / ch).floor() as usize;

  let offsetx = 0.0;
  let offsety = 0.0;

  let routes = (0..rows)
    .into_par_iter()
    .flat_map(|j| {
      (0..cols).into_par_iter().flat_map(move |i| {
        cell(
          opts.seed / 7.7 + (i + j * cols) as f64 / 0.3,
          (offsetx + i as f64 * cw, offsety + j as f64 * ch),
          cw,
          ch,
          pad,
        )
      })
    })
    .collect::<Vec<Vec<(f64, f64)>>>();

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route_curve(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn scaling_search<F: FnMut(f64) -> bool>(
  mut f: F,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
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
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    bound.0 < c.x - c.r
      && c.x + c.r < bound.2
      && bound.1 < c.y - c.r
      && c.y + c.r < bound.3
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
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
    if let Some(size) =
      search_circle_radius(bound, &circles, x, y, min_scale, max_scale)
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

fn shape_strokes_random<R: Rng>(rng: &mut R, c: &VCircle) -> Vec<(f64, f64)> {
  let pow = rng.gen_range(1.3, 1.5);
  let samples = sample_2d_candidates_f64(
    &|p| {
      let dx = p.0 - 0.5;
      let dy = p.1 - 0.5;
      let d2 = dx * dx + dy * dy;
      if d2 > 0.25 {
        0.0
      } else {
        d2
      }
    },
    (6. * c.r) as usize,
    (40. + c.r.powf(pow)) as usize,
    rng,
  );
  samples
    .iter()
    .map(|(x, y)| (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y))
    .collect()
}

fn kaleidoscope_project(
  p: (f64, f64),
  center: (f64, f64),
  n: usize,
) -> (f64, f64) {
  let (x, y) = p;
  let (cx, cy) = center;

  // Translate the point relative to the center
  let (x, y) = (x - cx, y - cy);

  let angle = 2.0 * PI / n as f64;
  let dist = (x * x + y * y).sqrt();
  let theta = PI + y.atan2(x);
  let sector = (theta / angle).floor();
  let theta_in_sector = theta - sector * angle;
  let new_theta = if sector as u32 % 2 == 0 {
    theta_in_sector
  } else {
    angle - theta_in_sector
  };
  let new_x = dist * new_theta.cos();
  let new_y = dist * new_theta.sin();

  // Translate the point back to its original position
  let (new_x, new_y) = (new_x + cx, new_y + cy);

  (new_x, new_y)
}

fn mandelbrot_glitched(
  x: f64,
  y: f64,
  max_iterations: u32,
  s1: f64,
  s2: f64,
  s3: f64,
  s4: f64,
  s5: f64,
  s6: f64,
  s7: f64,
  s8: f64,
  s9: f64,
) -> f64 {
  let mut p = Complex::new(x, y);
  let init = p;
  let mut iterations = 0;

  for _ in 0..max_iterations {
    let x2 = p.re * p.re;
    let y2 = p.im * p.im;
    let xy = p.re * p.im;

    let a = 1.0 + (s1 - 0.5) * s7 * s9;
    let b = -1.0 + (s2 - 0.5) * s7;
    let c = 0.0 + (s3 - 0.5) * s7 * s9;
    let d = 0.0 + (s4 - 0.5) * s8;
    let e = 0.0 + (s5 - 0.5) * s8;
    let f = 2.0 + (s6 - 0.5) * s8 * s9;

    p.re = a * x2 + b * y2 + c * xy + d;
    p.im = f * xy + e;

    p += init;

    if p.norm_sqr() >= 4.0 {
      break;
    }

    iterations += 1;
  }

  iterations as f64 / max_iterations as f64
}

fn rotate_point(point: (f64, f64), angle: f64) -> (f64, f64) {
  let (x, y) = point;
  let cos_a = angle.cos();
  let sin_a = angle.sin();
  (x * cos_a - y * sin_a, x * sin_a + y * cos_a)
}

fn shade(
  uv: (f64, f64),
  s1: f64,
  s2: f64,
  s3: f64,
  s4: f64,
  s5: f64,
  s6: f64,
  s7: f64,
  s8: f64,
  s9: f64,
  mod1: f64,
) -> f64 {
  let zoom = (0.3 + 6.0 * s7 * s8) * (1.0 + 3.0 * mod1);
  let focus_angle = 4.0 * mod1;
  let focus_amp = 0.4 * s7;
  let mut init = (2.0 * (uv.0 - 0.5) / zoom, 2.0 * (uv.1 - 0.5) / zoom);

  init =
    rotate_point(init, std::f64::consts::PI * (0.5 + 8.0 * s3).floor() / 4.0);
  init.0 -= 0.8;
  init.1 -= 0.0;
  init.0 += focus_amp * focus_angle.cos();
  init.1 += focus_amp * focus_angle.sin();

  let max_iterations = (50. + 500. * s7) as u32;

  let mandelbrot_value = mandelbrot_glitched(
    init.0,
    init.1,
    max_iterations,
    s1,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
  );

  mandelbrot_value
}

struct WeightMap {
  weights: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl WeightMap {
  fn new(width: f64, height: f64, precision: f64) -> WeightMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let weights = vec![0.0; w * h];
    WeightMap {
      weights,
      w,
      h,
      width,
      height,
      precision,
    }
  }
  fn fill_fn(&mut self, f: impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.weights[y * self.w + x] = v;
      }
    }
  }

  // do a simple bilinear interpolation
  fn get_weight(&self, p: (f64, f64)) -> f64 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let w00 = self.weights[y0 * self.w + x0];
    let w01 = self.weights[y0 * self.w + x1];
    let w10 = self.weights[y1 * self.w + x0];
    let w11 = self.weights[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the weights around the point p with a given radius
  fn decrease_weight_gaussian(
    &mut self,
    p: (f64, f64),
    radius: f64,
    value: f64,
  ) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = (x - radius).floor() as usize;
    let y0 = (y - radius).floor() as usize;
    let x1 = (x + radius).ceil() as usize;
    let y1 = (y + radius).ceil() as usize;
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let d = (p.0 - p.0).hypot(p.1 - p.1);
        if d < radius {
          let w = self.weights[y * self.w + x];
          let v = value * (1.0 - d / radius);
          self.weights[y * self.w + x] = w - v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  fn best_direction(
    &self,
    p: (f64, f64),
    step: f64,
    angle: f64,
    max_ang_rotation: f64,
    angle_precision: f64,
    straight_factor: f64,
  ) -> Option<f64> {
    let mut best_ang = None;
    let mut best_weight = 0.0;
    let mut a = -max_ang_rotation;
    while a < max_ang_rotation {
      let ang = a + angle;
      let dx = step * ang.cos();
      let dy = step * ang.sin();
      let np = (p.0 + dx, p.1 + dy);
      if np.0 < 0.0 || np.0 > self.width || np.1 < 0.0 || np.1 > self.height {
        a += angle_precision;
        continue;
      }
      // more important when a is near 0.0 depending on straight factor
      let wmul = (1.0 - straight_factor)
        + (1.0 - a.abs() / max_ang_rotation) * straight_factor;
      let weight = self.get_weight(np) * wmul;
      if weight > best_weight {
        best_weight = weight;
        best_ang = Some(ang);
      }
      a += angle_precision;
    }
    return best_ang;
  }

  fn search_weight_top<R: Rng>(
    &mut self,
    rng: &mut R,
    search_max: usize,
    min_weight: f64,
  ) -> Option<(f64, f64)> {
    let mut best_w = min_weight;
    let mut best_p = None;
    for _i in 0..search_max {
      let x = rng.gen_range(0.0, self.width);
      let y = rng.gen_range(0.0, self.height);
      let p = (x, y);
      let w = self.get_weight(p);
      if w > best_w {
        best_w = w;
        best_p = Some(p);
      }
    }
    return best_p;
  }

  fn dig_random_route(
    &mut self,
    origin: (f64, f64),
    initial_angle: f64,
    step: f64,
    max_ang_rotation: f64,
    straight_factor: f64,
    max_length: usize,
    decrease_value: f64,
  ) -> Vec<(f64, f64)> {
    let mut route = Vec::new();
    let mut p = origin;
    let mut angle = initial_angle;
    for _i in 0..max_length {
      if let Some(ang) = self.best_direction(
        p,
        step,
        angle,
        max_ang_rotation,
        0.2 * max_ang_rotation,
        straight_factor,
      ) {
        angle = ang;
        let prev = p;
        p = (p.0 + step * angle.cos(), p.1 + step * angle.sin());
        route.push(p);
        self.decrease_weight_gaussian(prev, step, decrease_value);
      } else {
        break;
      }
    }

    route
  }
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes(
  input_routes: &Vec<Vec<(f64, f64)>>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<Vec<(f64, f64)>> {
  // locate the intersection where inside and outside cross
  let search = |inside: (f64, f64), outside: (f64, f64), n| {
    let mut a = inside;
    let mut b = outside;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if is_outside(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes.iter() {
    if input_route.len() < 2 {
      continue;
    }

    let mut prev = input_route[0];
    let mut prev_is_outside = is_outside(prev);
    let mut route = vec![];
    if !prev_is_outside {
      // prev is not to crop. we can start with it
      route.push(prev);
    }

    for &p in input_route.iter().skip(1) {
      // we iterate in small steps to detect any interruption
      let static_prev = prev;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let d = (dx * dx + dy * dy).sqrt();
      let vx = dx / d;
      let vy = dy / d;
      let iterations = (d / stepping).ceil() as usize;
      let mut v = 0.0;
      for _i in 0..iterations {
        v = (v + stepping).min(d);
        let q = (static_prev.0 + vx * v, static_prev.1 + vy * v);
        let q_is_outside = is_outside(q);
        if prev_is_outside != q_is_outside {
          // we have a crossing. we search it precisely
          let intersection = if prev_is_outside {
            search(q, prev, dichotomic_iterations)
          } else {
            search(prev, q, dichotomic_iterations)
          };

          if q_is_outside {
            // we close the path
            route.push(intersection);
            if route.len() > 1 {
              // we have a valid route to accumulate
              routes.push(route);
            }
            route = vec![];
          } else {
            // we open the path
            route.push(intersection);
          }
          prev_is_outside = q_is_outside;
        }

        prev = q;
      }

      // prev should be == p
      if !prev_is_outside {
        // prev is not to crop. we can start with it
        route.push(p);
      }
    }

    if route.len() > 1 {
      // we have a valid route to accumulate
      routes.push(route);
    }
  }

  routes
}

struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
}

impl PaintMask {
  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    }
  }

  fn is_painted(&self, point: (f64, f64)) -> bool {
    // check out of bounds
    if point.0 <= 0.0
      || point.0 >= self.width
      || point.1 <= 0.0
      || point.1 >= self.height
    {
      return false;
    }
    let precision = self.precision;
    let width = self.width;
    let x = (point.0 / precision) as usize;
    let y = (point.1 / precision) as usize;
    let wi = (width / precision) as usize;
    self.mask[x + y * wi]
  }

  fn paint_circle(&mut self, circle: &VCircle) {
    let (minx, miny, maxx, maxy) = (
      circle.x - circle.r,
      circle.y - circle.r,
      circle.x + circle.r,
      circle.y + circle.r,
    );
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if euclidian_dist(point, (circle.x, circle.y)) < circle.r {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
}

fn clouds(
  seed: f64,
  origin: (f64, f64),
  width: f64,
  height: f64,
  pad: f64,
) -> Vec<Vec<(f64, f64)>> {
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(seed);

  let mut mask = PaintMask::new(0.1, width, height);
  let mut routes = Vec::new(); // all the paths to draw are stored here

  let in_shape = |p: (f64, f64)| -> bool {
    !mask.is_painted(p) && strictly_in_boundaries(p, bound)
  };

  let does_overlap = |c: &VCircle| {
    in_shape((c.x, c.y))
      && circle_route((c.x, c.y), c.r, 8)
        .iter()
        .all(|&p| in_shape(p))
  };

  let mut all = vec![];

  for _i in 0..rng.gen_range(80, 200) {
    let count = (rng.gen_range(0., 100.)) as usize;
    let min = rng.gen_range(8.0, 12.0);
    let max = min + rng.gen_range(0.0, 40.0) * rng.gen_range(0.0, 1.0);
    let optim = (1. + rng.gen_range(0., 10.) * rng.gen_range(0., 1.)) as usize;
    let ppad = rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0);

    let circles = packing2(
      &mut rng,
      50000,
      count,
      optim,
      ppad,
      bound,
      &does_overlap,
      min,
      max,
    );

    all.extend(circles);
  }

  rng.shuffle(&mut all);

  let pow = rng.gen_range(0.8, 1.2);

  all = all
    .iter()
    .filter(|c| {
      let dx = c.x - width / 2.0;
      let dy = c.y - height / 2.0;
      let d = 0.5 * (dx.abs() + dy.abs() - c.r).max(0.) / (width + height);
      rng.gen_bool(d.powf(pow))
    })
    .cloned()
    .collect();

  all.iter().for_each(|c| {
    let (rts, circles) = cloud_in_circle(&mut rng, &c);
    let rts = clip_routes(&rts, &|p| mask.is_painted(p), 0.3, 7);
    routes.extend(rts);
    for c in circles.clone() {
      mask.paint_circle(&c);
    }
  });

  routes
    .iter()
    .map(|r| {
      r.iter()
        .map(|p| {
          let x = p.0 + origin.0;
          let y = p.1 + origin.1;
          (x, y)
        })
        .collect()
    })
    .collect()
}

fn cloud_in_circle<R: Rng>(
  rng: &mut R,
  circle: &VCircle,
) -> (Vec<Vec<(f64, f64)>>, Vec<VCircle>) {
  // FIXME the clouds have a weird issue on the fact we don't always see the edges

  let mut routes = vec![];

  let mut circles: Vec<VCircle> = vec![];

  let stretchy = rng.gen_range(0.2, 1.0);

  let count = rng.gen_range(16, 80);
  for _i in 0..count {
    let radius = circle.r * rng.gen_range(0.3, 0.5) * rng.gen_range(0.2, 1.0);
    let angle = rng.gen_range(0.0, 2.0 * PI);
    let x = circle.x + angle.cos() * (circle.r - radius);
    let y = circle.y
      + angle.sin() * (circle.r - radius) * rng.gen_range(0.5, 1.0) * stretchy;
    let circle = VCircle::new(x, y, radius);

    let should_crop = |p| circles.iter().any(|c| c.includes(p));

    let mut input_routes = vec![];
    let mut r = radius;
    let dr = rng.gen_range(0.5, 2.0);
    loop {
      if r < 1.0 {
        break;
      }
      let count = (r * 2.0 + 10.0) as usize;
      let amp = rng.gen_range(0.5 * PI, 1.2 * PI);
      let ang = angle
        + PI
          * rng.gen_range(-1.0, 1.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0);
      let start = ang - amp / 2.0;
      let end = ang + amp / 2.0;
      input_routes.push(arc((x, y), r, start, end, count));
      r -= dr;
    }

    routes.extend(crop_routes_with_predicate_rng(
      rng,
      0.0,
      input_routes,
      &should_crop,
      &mut vec![],
    ));

    circles.push(circle);
  }

  (routes, circles)
}

fn arc(
  center: (f64, f64),
  r: f64,
  start: f64,
  end: f64,
  count: usize,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = start + (end - start) * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
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
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist(p, (self.x, self.y)) < self.r
  }
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

fn search_circle_radius2(
  does_overlap: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing2<R: Rng>(
  rng: &mut R,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) =
      search_circle_radius2(&does_overlap, &circles, x, y, min_scale, max_scale)
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

// TODO rework with clip_routes
fn crop_routes_with_predicate_rng<R: Rng>(
  rng: &mut R,
  proba_skip: f64,
  input_routes: Vec<Vec<(f64, f64)>>,
  should_crop: &dyn Fn((f64, f64)) -> bool,
  cutted_points: &mut Vec<(f64, f64)>,
) -> Vec<Vec<(f64, f64)>> {
  let search = |a_, b_, n| {
    let mut a = a_;
    let mut b = b_;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if should_crop(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes {
    if input_route.len() < 2 {
      continue;
    }
    if proba_skip > 0.0 && rng.gen_bool(proba_skip) {
      routes.push(input_route);
      continue;
    }
    let mut prev = input_route[0];
    let mut route = vec![];
    if !should_crop(prev) {
      // prev is not to crop. we can start with it
      route.push(prev);
    } else {
      if !should_crop(input_route[1]) {
        // prev is to crop, but [1] is outside. we start with the exact intersection
        let intersection = search(input_route[1], prev, 7);
        prev = intersection;
        cutted_points.push(intersection);
        route.push(intersection);
      } else {
        cutted_points.push(prev);
      }
    }
    // cut segments with crop logic
    for &p in input_route.iter().skip(1) {
      // TODO here, we must do step by step to detect crop inside the segment (prev, p)

      if should_crop(p) {
        if route.len() > 0 {
          // prev is outside, p is to crop
          let intersection = search(prev, p, 7);
          cutted_points.push(intersection);
          route.push(intersection);
          routes.push(route);
          route = vec![];
        } else {
          cutted_points.push(p);
        }
      } else {
        // nothing to crop
        route.push(p);
      }
      prev = p;
    }
    if route.len() >= 2 {
      routes.push(route);
    }
  }

  routes
}
