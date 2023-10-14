use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn rec<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  bound: (f64, f64, f64, f64),
  o: (f64, f64),
  r: f64,
  level: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  // too small
  if r
    < (0.03 + rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0)) * paint.width
  {
    return routes;
  }

  // FIRST, MAKE THE BORDER

  let perlin = Perlin::new();
  let seed = rng.gen_range(-999., 999.);

  let f1 = rng.gen_range(0.01, 0.1);
  let amp2 = rng.gen_range(0.0, 3.0);
  let f2 = rng.gen_range(0.01, 0.1);
  let rminamp = 0.05 + level as f64 * 0.05;
  let ramp = rng.gen_range(0.0, 0.2) * rng.gen_range(0.1, 1.0);

  let splits = (r / rng.gen_range(0.3, 4.0)) as usize + 1;
  let mut vertexes = vec![];
  for i in 0..splits {
    let f = (i as f64) / (splits as f64 - 1.);
    let a = f * PI * 2.0;
    let p = (o.0 + r * a.cos(), o.1 + r * a.sin());
    let r2 = r
      * (1.
        - rminamp
        - ramp
          * (0.5
            + perlin.get([
              f1 * p.0,
              f1 * p.1,
              seed + amp2 * perlin.get([f2 * p.0, f2 * p.1, seed]),
            ])))
      .max(0.);
    let q = (o.0 + r2 * a.cos(), o.1 + r2 * a.sin());
    vertexes.push(vec![p, q]);
  }

  // BORDER POLYGONS
  let mut polys = vec![];
  for i in 0..splits {
    let j = (i + 1) % splits;
    let mut poly = vec![];
    poly.push(vertexes[i][0]);
    poly.push(vertexes[j][0]);
    poly.push(vertexes[j][1]);
    poly.push(vertexes[i][1]);
    polys.push(poly);
  }

  // COLORIZE BORDERS

  let mut map = WeightMap::new(paint.width, paint.height, 0.4);

  let density = 5.0 - level as f64 * 0.4;

  map.fill_fn(&|p| {
    let collides = polys.iter().any(|poly| polygon_includes_point(poly, p));
    if collides {
      density
    } else {
      0.0
    }
  });
  let rot = PI / rng.gen_range(1.0, 3.0);
  let step = 0.6;
  let straight = rng.gen_range(-0.2, 0.2);
  let count = 20000;
  let min_l = 5;
  let max_l = rng.gen_range(10, 80);
  let decrease_value = 1.0;
  let search_max = 500;
  let min_weight = 1.0;
  let mut bail_out = 0;

  for _i in 0..count {
    let top = map.search_weight_top(rng, search_max, min_weight);
    if top.is_none() {
      bail_out += 1;
      if bail_out > 10 {
        break;
      }
    }
    if let Some(o) = top {
      let angle = perlin.get([seed, 0.02 * o.0, 0.02 * o.1]);

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
          let rt = rdp(&route, 0.05);
          routes.push((0, rt));
        }
      }
    }
  }

  // CLIP BORDERS

  let is_outside = |p| !strictly_in_boundaries(p, bound) || paint.is_painted(p);
  let border_routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);

  // We paint what we just added
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }

  // INNER MOUNTAINS

  routes = vec![];
  let min_route = 2;
  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = WeightMap::new(paint.width, paint.height, 0.5);
  let peakfactormajor = rng.gen_range(-0.0001, 0.001);
  let precision = 0.2;
  let count = rng.gen_range(1, 4);
  for j in 0..count {
    let h = rng.gen_range(3.0, 5.0);
    let peakfactor =
      peakfactormajor + rng.gen_range(-0.001, 0.002) * rng.gen_range(0.0, 1.0);
    let ampfactor = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
    let ynoisefactor = rng.gen_range(0.02, 0.2);
    let yincr = 0.7
      + (rng.gen_range(-2f64, 2.0) * rng.gen_range(0.0, 1.0))
        .max(0.0)
        .min(5.0);
    let amp1 = rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(0.0, 12.0) * rng.gen_range(0.0, 1.0);
    let amp3 = rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
    let offsetstrategy = rng.gen_range(0, 5);
    let center = rng.gen_range(0.2, 0.8) * r * 2.0;

    let stopy = o.1 + rng.gen_range(-0.2, 0.8) * r;

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = o.1 + r * 1.5;

    loop {
      if base_y < stopy {
        break;
      }

      let mut route = Vec::new();
      let mut x = 0.0;
      let mut was_outside = true;
      loop {
        if x > 2.0 * r {
          break;
        }
        let xv = (h - base_y / r) * (x - center);

        let amp = r * ampfactor;
        let mut y = base_y;

        if offsetstrategy == 0 {
          y += amp * peakfactor * xv * xv;
        }

        y += amp2
          * amp
          * perlin.get([
            //
            8.311 + xv * 0.00511,
            88.1 + y * ynoisefactor,
            seed * 97.311,
          ]);

        if offsetstrategy == 2 {
          y += amp * peakfactor * xv * xv;
        }

        y += amp1
          * amp
          * perlin
            .get([
              //
              xv * 0.007111 + 9.9,
              y * 0.00311 + 3.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  55. + seed * 7.3,
                  80.3 + xv * 0.017,
                  y * 0.06 + 11.3,
                ]),
            ])
            .max(0.0);

        if offsetstrategy == 1 {
          y += amp * peakfactor * xv * xv;
        }

        y += 0.05
          * amp
          * perlin.get([
            //
            6.6 + seed * 1.3,
            8.3 + xv * 0.207,
            8.1 + y * 0.31,
          ]);

        if offsetstrategy == 4 {
          y += amp * peakfactor * xv * xv;
        }

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

        if offsetstrategy == 3 {
          y += amp * peakfactor * xv * xv;
        }

        let mut collides = false;
        let xi: usize = (x / precision).round() as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] - 0.01 {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let p = (o.0 + x - r, y);

        let inside = !collides
          && euclidian_dist(p, o) < r
          && passage.get_weight(p) < 10.
          && !paint.is_painted(p);

        if inside {
          if was_outside {
            let l = route.len();
            if l >= min_route {
              routes.push((0, route));
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push(p);
          passage.add_weight(p, 1.0);
        } else {
          was_outside = true;
        }

        x += precision;
      }

      let l = route.len();
      if l >= min_route {
        routes.push((0, route));
      }

      base_y -= yincr;
    }
  }

  // CLIP MOUNTAINS

  let is_outside = |p| !strictly_in_boundaries(p, bound) || paint.is_painted(p);
  let mountain_routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);

  let is_under_mountain = |p: (f64, f64)| {
    let xi = ((p.0 - o.0 + r) / precision).round() as usize;
    if xi >= height_map.len() {
      return false;
    }
    p.1 > height_map[xi]
  };

  paint.paint_circle_and_f(o.0, o.1, r, &is_under_mountain);

  // SUN PLACEMENT

  // find the best distance with unpainted part
  let splits = 32;
  let mut dists = vec![];
  for i in 0..splits {
    let a = (i as f64) * PI * 2.0 / (splits as f64);
    let mut v = 0.1;
    loop {
      let r2 = r + v;
      let p = (o.0 + r2 * a.cos(), o.1 + r2 * a.sin());
      if out_of_boundaries(p, bound) {
        break;
      }
      if paint.is_painted(p) {
        break;
      }
      v += 2.0;
    }
    dists.push((a, v));
  }
  rng.shuffle(&mut dists);
  dists.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
  let (a, v) = dists[0];
  let mut sun_routes = vec![];
  if v > 5.0 {
    let h = rng.gen_range(-0.2, if level == 0 { 0. } else { 0.5 })
      * rng.gen_range(0.0, 1.0);
    let c = (o.0 + (r + v * h) * a.cos(), o.1 + (r + v * h) * a.sin());
    let r2 = rng.gen_range(0.3, 0.6) * v;
    let mut routes = vec![
      (1, spiral_optimized(c.0, c.1, r2, 0.5, 0.1)),
      (1, circle_route(c, r2, (r2 * 2. + 8.) as usize)),
    ];

    let is_outside = |p| {
      !strictly_in_boundaries(p, bound)
        || paint.is_painted(p)
        || euclidian_dist(p, o) < r
    };
    routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
    sun_routes = routes;
    let outside_circle = |p| euclidian_dist(p, o) > r;
    paint.paint_circle_and_f(c.0, c.1, r2, &outside_circle);
  }

  // We recursively go one level deeper
  let r2 = r * rng.gen_range(0.6, 0.75);
  let inner = rec(rng, paint, bound, o, r2, level + 1);

  vec![border_routes, mountain_routes, sun_routes, inner].concat()
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bounds = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);

  let mut paint = PaintMask::new(0.5, width, height);

  let y = height * rng.gen_range(0.5, 0.6);

  let routes = rec(
    &mut rng,
    &mut paint,
    bounds,
    (width / 2.0, y),
    0.9 * (width.min(height) - pad) / 2.0,
    0,
  );

  vec!["#000", "#f93"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "297")]
  width: f64,
  #[clap(short, long, default_value = "420")]
  height: f64,
  #[clap(short, long, default_value = "20")]
  pad: f64,
  #[clap(short, long, default_value = "1")]
  seconds: i64,
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

fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  point: (f64, f64),
) -> bool {
  let mut c = false;
  for i in 0..polygon.len() {
    let j = (i + 1) % polygon.len();
    if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
      && (point.0
        < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
          / (polygon[j].1 - polygon[i].1)
          + polygon[i].0)
    {
      c = !c;
    }
  }
  c
}

// TODO more efficient algorithm would be to paint on a mask.

#[derive(Clone)]
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

  fn paint_circle_and_f(
    &mut self,
    cx: f64,
    cy: f64,
    cr: f64,
    f: &dyn Fn((f64, f64)) -> bool,
  ) {
    let (minx, miny, maxx, maxy) = (
      (cx - cr).max(0.),
      (cy - cr).max(0.),
      (cx + cr).min(self.width),
      (cy + cr).min(self.height),
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
        if euclidian_dist(point, (cx, cy)) < cr && f(point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx =
      ((minx - precision).max(0.).min(self.width) / precision) as usize;
    let miny =
      ((miny - precision).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
}

fn polygon_bounds(polygon: &Vec<(f64, f64)>) -> (f64, f64, f64, f64) {
  let mut minx = f64::MAX;
  let mut miny = f64::MAX;
  let mut maxx = f64::MIN;
  let mut maxy = f64::MIN;
  for &(x, y) in polygon {
    minx = minx.min(x);
    miny = miny.min(y);
    maxx = maxx.max(x);
    maxy = maxy.max(y);
  }
  (minx, miny, maxx, maxy)
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes_with_colors(
  input_routes: &Vec<(usize, Vec<(f64, f64)>)>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
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

  for (clrp, input_route) in input_routes.iter() {
    let clr = *clrp;
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
              routes.push((clr, route));
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
      routes.push((clr, route));
    }
  }

  routes
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
  /*
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist(p, (self.x, self.y)) < self.r
  }
  */
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
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y, size)) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing<R: Rng>(
  rng: &mut R,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = vec![];
  let mut tries = Vec::new();
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

/*
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
*/

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
        let variation = ((xi as f64 + (tracks_count as f64 * phase))
          % (tracks_count as f64)
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
  fn fill_fn(&mut self, f: &impl Fn((f64, f64)) -> f64) {
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

  fn add_weight(&mut self, p: (f64, f64), v: f64) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let i = y0 * self.w + x0;
    self.weights[i] += v;
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
