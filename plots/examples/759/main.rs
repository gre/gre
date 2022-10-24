use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "148.0")]
  pub height: f64,
  #[clap(short, long, default_value = "105.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn make_shape<R: Rng>(
  peaks: &Vec<(f64, f64, f64)>,
  bound: (f64, f64, f64, f64),
  roughness: f64,
  rng: &mut R,
  passage: &mut Passage,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  if peaks.len() == 0 {
    return routes;
  }

  let amp_base = rng.gen_range(5.0, 20.0);
  let iterations = 80;
  let values: Vec<f64> = (0..iterations)
    .map(|_i| rng.gen_range(-1.0, 1.0) * 0.2)
    .collect();

  let mut lines = Vec::new();
  for (i, &(x, y, _a)) in peaks.iter().enumerate() {
    let progress = i as f64 / (peaks.len() - 1) as f64 - 0.5;
    let mut p = (x, y);
    let mut a = -PI / 2.0;
    let mut amp = amp_base;
    let mut route = Vec::new();
    for angle_increment in values.clone() {
      if route.len() > 0 {
        if let Some(q) =
          collide_segment_boundaries(route[route.len() - 1], p, bound)
        {
          route.push(q);
          break;
        }
      }
      route.push(p);
      p = (p.0 + amp * a.cos(), p.1 + amp * a.sin());
      p.0 += rng.gen_range(-1.0, 1.0) * roughness;
      p.1 += rng.gen_range(-1.0, 1.0) * roughness;
      a += angle_increment;
      amp += -progress * angle_increment * 1.0;
    }
    lines.push(route);
  }

  let mainline = lines[lines.len() / 2].clone();
  let node_mod = rng.gen_range(2, 8);
  let node_delta = rng.gen_range(0, node_mod);
  for line in lines {
    let route = line
      .iter()
      .enumerate()
      .map(|(i, &p)| {
        if (i + node_delta) % node_mod == 0 && i < mainline.len() {
          lerp_point(
            p,
            mainline[i],
            0.95 - 0.9 * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
          )
        } else {
          p
        }
      })
      .collect();
    let route = path_subdivide_to_curve(route, 3, 0.7);
    routes.push((0, route));
  }

  for (_c, r) in routes.iter() {
    for &p in r.iter() {
      passage.count(p);
    }
  }

  routes
}

fn cell(opts: &Opts) -> Vec<(usize, Vec<(f64, f64)>)> {
  let seed = opts.seed;
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  // Prepare all the random values
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let stopy = rng.gen_range(0.1, 0.5) * height;
  let passage_threshold = 9;

  let roughness = 0.2 * rng.gen_range(-0.5f64, 0.5).max(0.0);

  let shapes_distance_lines = rng.gen_range(0.5, 1.0);
  let max_shape_per_center = 4;

  let nb_centers =
    1 + (rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0)) as usize;
  let centers: Vec<f64> = (0..nb_centers)
    .map(|i| (i + 1) as f64 / (nb_centers + 1) as f64)
    .collect();
  let mut count_per_center = vec![0; nb_centers];

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -30.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let precision = 0.21;
  let count = rng.gen_range(8, 12);
  for j in 0..count {
    let peakfactor = rng.gen_range(-0.001, 0.002)
      * rng.gen_range(-0.5f64, 1.0).max(0.0)
      * rng.gen_range(j as f64 / (count as f64), 1.0);
    let ampfactor = rng.gen_range(0.01, 0.2) * rng.gen_range(0.1, 1.0);
    let ynoisefactor = rng.gen_range(0.02, 0.2) * rng.gen_range(0.2, 1.0);
    let xmul = rng.gen_range(0.5, 2.0);
    let yincr = 0.5 + rng.gen_range(0.0, 5.0) * rng.gen_range(0.2, 1.0);
    let amp2 = rng.gen_range(0.0, 20.0) * rng.gen_range(0.2, 1.0);
    let offsetstrategy = rng.gen_range(0, 5);

    let stopy =
      mix(height, stopy, (j as f64 / ((count - 1) as f64)) * 0.7 + 0.3);

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height * 5.0;
    let mut miny = height;
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
        let xv = xmul * (4.01 - base_y / height) * (x - width / 2.);

        let amp = height * ampfactor;
        let mut y = base_y;

        if offsetstrategy == 0 {
          y += amp * peakfactor * xv * xv;
        }

        y += -amp
          * perlin
            .get([
              //
              xv * 0.005111 + 19.9,
              y * 0.00111 + 30.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  55. + seed * 7.3,
                  80.3 + xv * 0.015,
                  y * 0.2 + 111.3,
                ]),
            ])
            .abs();

        if offsetstrategy == 1 {
          y += amp * peakfactor * xv * xv;
        }

        y += amp2
          * amp
          * perlin.get([
            //
            8.311 + xv * 0.00811,
            88.1 + y * ynoisefactor,
            seed * 97.311,
          ]);

        if offsetstrategy == 2 {
          y += amp * peakfactor * xv * xv;
        }

        y += amp
          * perlin.get([
            //
            seed * 9.3 + 77.77,
            xv * 0.08 + 9.33,
            y * 0.5,
          ])
          * perlin
            .get([
              //
              xv * 0.015 + 8.33,
              88.1 + y * 0.2,
              seed / 7.7 + 6.66,
            ])
            .min(0.0);

        if offsetstrategy == 3 {
          y += amp * peakfactor * xv * xv;
        }

        y += 0.1
          * amp
          * (1.0 - miny / height)
          * perlin.get([
            //
            66.6 + seed * 1.3,
            18.3 + xv * 0.501,
            88.1 + y * 0.503,
          ]);

        if offsetstrategy == 4 {
          y += amp * peakfactor * xv * xv;
        }

        if y < miny {
          miny = y;
        }
        let mut collides = false;
        let xi = ((x - mountainpadding) / precision).round() as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let inside = !collides && strictly_in_boundaries((x, y), bound);
        if inside && passage.get((x, y)) < passage_threshold {
          if was_outside {
            if route.len() > min_route {
              routes.push((1, route));
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

      if route.len() > min_route {
        routes.push((1, route));
      }

      base_y -= yincr;
    }

    /*
    if selected_mountain != j {
      continue;
    }
    */

    // calculate a moving average to smooth the stick men positions
    let smooth = 40;
    let sf = smooth as f64;
    let mut sum = 0.0;
    let mut acc = Vec::new();
    let mut smooth_heights: Vec<(f64, f64, f64)> = Vec::new();
    for (i, h) in height_map.iter().enumerate() {
      if acc.len() == smooth {
        let avg = sum / sf;
        let xtheoric = mountainpadding + (i as f64 - sf / 2.0) * precision;

        let l = smooth_heights.len();
        let b = (xtheoric, avg, 0.0);
        let a = if l > 2 { smooth_heights[l - 2] } else { b };
        let rot = -PI / 2.0 + (b.0 - a.0).atan2(b.1 - a.1);
        let p = (xtheoric, avg, rot);
        smooth_heights.push(p);
        let prev = acc.remove(0);
        sum -= prev;
      }
      acc.push(h);
      sum += h;
    }

    smooth_heights.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let count = (rng.gen_range(0., 8.) * rng.gen_range(0.0, 1.0)) as usize;
    if count == 0 {
      continue;
    }
    for _j in 0..count {
      let index = rng.gen_range(0, centers.len());
      count_per_center[index] += 1;
      if count_per_center[index] > max_shape_per_center {
        continue;
      }
      let center = centers[index];
      let x =
        (center + rng.gen_range(-0.1, 0.1) * rng.gen_range(0.0, 1.0)) * width;
      let w = (0.1
        + rng.gen_range(-0.05, 0.15)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0))
        * width;

      let mut peaks = Vec::new();
      let divisions = (w / shapes_distance_lines) as usize;
      for i in 0..divisions {
        let px = x + w * ((i as f64) / (divisions - 1) as f64 - 0.5);
        let py = height_map
          [((px - mountainpadding) / precision) as usize % height_map.len()];
        if py > height - pad - 5.0 {
          continue;
        }
        peaks.push((px, py, 0.0));
      }

      if peaks.len() < divisions {
        continue;
      }

      routes = vec![
        routes,
        make_shape(&peaks, bound, roughness, &mut rng, &mut passage),
      ]
      .concat();
    }
  }

  let radius = rng.gen_range(3.0, 4.0);
  passage.grow_passage(radius);

  let does_overlap = |p| {
    passage.get(p) == 0
      && strictly_in_boundaries(p, bound)
      && p.1
        < height_map
          [((p.0 - mountainpadding) / precision) as usize % height_map.len()]
  };

  let ppad = 1.2;
  let total_pad = pad + ppad + radius;
  let min = ppad + rng.gen_range(0.5, 0.8);
  let max = min + rng.gen_range(0.0, 0.8);
  let circles = packing(
    seed,
    500000,
    1000,
    rng.gen_range(0, 4),
    ppad,
    (total_pad, total_pad, width - total_pad, height - total_pad),
    &does_overlap,
    min,
    max,
  );

  for c in circles {
    let count = (c.r * 2.0 + 8.0) as usize;
    routes.push((2, circle_route((c.x, c.y), c.r, count)));
  }

  // External frame to around the whole piece
  let mut d = 0.0;
  loop {
    if d > 2.0 {
      break;
    }
    routes.push((
      1,
      vec![
        (pad + d, pad + d),
        (pad + d, height - pad - d),
        (width - pad - d, height - pad - d),
        (width - pad - d, pad + d),
        (pad + d, pad + d),
      ],
    ));
    d += 0.3;
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let routes = cell(opts);

  // Make the SVG
  let colors = vec!["#000", "#000", "#000"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (c, route) in routes.clone() {
        if c == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
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

#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }

  pub fn count_once(self: &mut Self, p: (f64, f64)) {
    let i = self.index(p);
    let v = self.counters[i];
    if v == 0 {
      self.counters[i] = 1;
    }
  }

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }

  pub fn grow_passage(self: &mut Self, radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters: Vec<usize> = self.counters.iter().cloned().collect();
    let mut mask = Vec::new();
    // TODO, in future for even better perf, I will rewrite this
    // working directly with index integers instead of having to use index() / count_once()
    let mut x = -radius;
    loop {
      if x >= radius {
        break;
      }
      let mut y = -radius;
      loop {
        if y >= radius {
          break;
        }
        if x * x + y * y < radius * radius {
          mask.push((x, y));
        }
        y += precision;
      }
      x += precision;
    }

    let mut x = 0.0;
    loop {
      if x >= width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= height {
          break;
        }
        let index = self.index((x, y));
        if counters[index] > 0 {
          for &(dx, dy) in mask.iter() {
            self.count_once((x + dx, y + dy));
          }
        }
        y += precision;
      }
      x += precision;
    }
  }
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y)) && !circles.iter().any(|other| c.collides(other))
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
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
      search_circle_radius(&does_overlap, &circles, x, y, min_scale, max_scale)
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn path_subdivide_to_curve_it(
  path: Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path;
  }
  let mut route = Vec::new();
  let mut first = path[0];
  let mut last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if looped {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for i in 1..(l - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if looped {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if looped {
    route.push(first);
  }
  route
}

fn path_subdivide_to_curve(
  path: Vec<(f64, f64)>,
  n: usize,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}
