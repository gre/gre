use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "148.0")]
  pub width: f64,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn ego<R: Rng>(
  pos: (f64, f64),
  rng: &mut R,
  passage: &mut Passage,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();

  let scale = rng.gen_range(1.8, 3.2);

  let headr = 0.7 * scale;
  let headh = 5.0 * scale;
  let armw = 2.0 * scale;
  let armh = 3.8 * scale;
  let footh = 1.6 * scale;
  let footw = 1.0 * scale;

  let leftarmdy: f64 = rng.gen_range(-5.0, 5.0);
  let rightarmdy: f64 = rng.gen_range(-5.0, 5.0);

  for _j in 0..3 {
    for side in vec![-1.0, 1.0] {
      let mut route = Vec::new();
      let headcenter = (pos.0, pos.1 - headh);
      for _i in 0..6 {
        let a = rng.gen_range(-PI, PI);
        route.push((
          headcenter.0 + headr * a.cos(),
          headcenter.1 + headr * a.sin(),
        ));
      }
      route.push(headcenter);
      route = path_subdivide_to_curve_it(route, 0.75);
      route.push((pos.0, pos.1 - footh));
      route.push((pos.0 - side * footw, pos.1));
      route = shake(route, 0.4, rng);
      route = path_subdivide_to_curve_it(route, 0.8);
      routes.push(route);
    }

    let mut route = Vec::new();
    route.push((
      pos.0 - armw + 0.3 * leftarmdy.abs(),
      pos.1 - armh + leftarmdy + rng.gen_range(-0.1, 0.1) * scale,
    ));
    let steps = 5;
    for i in 0..steps {
      let f = i as f64 / ((steps - 1) as f64);
      route.push((
        pos.0 + rng.gen_range(0.0, 0.5) * scale,
        pos.1 - mix(armh, footh, f),
      ));
    }
    for i in 0..steps {
      let f = i as f64 / ((steps - 1) as f64);
      route.push((
        pos.0 - rng.gen_range(0.0, 0.5) * scale,
        pos.1 - mix(footh, armh, f),
      ));
    }
    route.push((
      pos.0 + armw - 0.3 * rightarmdy.abs(),
      pos.1 - armh + rightarmdy + rng.gen_range(-0.1, 0.1) * scale,
    ));
    route = path_subdivide_to_curve_it(route, 0.8);
    route = shake(route, 0.3, rng);
    route = path_subdivide_to_curve_it(route, 0.7);
    routes.push(route);
  }

  for r in routes.iter() {
    for p in path_subdivide_to_curve(r.clone(), 2, 0.8) {
      // TODO custom code to do all the lines properly
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
  let stopy = rng.gen_range(0.4, 0.6) * height;
  let passage_threshold = 9;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -30.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let precision = 0.21;
  let count = rng.gen_range(2, 8);
  for j in 0..count {
    let peakfactor = rng.gen_range(0.0002, 0.001);
    let ampfactor = rng.gen_range(0.03, 0.04);
    let yincr = 0.5;
    let amp2 = rng.gen_range(1.0, 8.0);
    let ynoisefactor = rng.gen_range(0.05, 0.1);
    let offsetstrategy = rng.gen_range(0, 2);

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
        let xv = (4.01 - base_y / height) * (x - width / 2.);

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
  }

  // calculate a moving average
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
  let highest = smooth_heights[0];
  let pos = (
    highest.0,
    highest.1
      - rng.gen_range(-stopy * 0.2, stopy * 0.5).max(0.0)
        * rng.gen_range(0.0, 1.0),
  );
  for route in ego(pos, &mut rng, &mut passage) {
    routes.push((0, route));
  }

  let radius = rng.gen_range(3.0, 5.0);
  passage.grow_passage(radius);

  let does_overlap = |c: &VCircle| {
    circle_route((c.x, c.y), c.r, 12).iter().all(|&p| {
      passage.get(p) == 0
        && strictly_in_boundaries(p, bound)
        && p.1
          < height_map
            [((p.0 - mountainpadding) / precision) as usize % height_map.len()]
    })
  };

  let ppad = 0.5;
  let min = ppad + rng.gen_range(1.0, 4.0);
  let max = min + rng.gen_range(0.0, 10.0);
  let total_pad = pad + 2.0 + max;
  let circles = packing(
    seed,
    500000,
    1000,
    0,
    ppad,
    (total_pad, total_pad, width - total_pad, height - total_pad),
    &does_overlap,
    min,
    max,
  );

  for c in circles {
    let a = (pos.0 - c.x).atan2(pos.1 - c.y - 5.0);
    let route = heart(c.r, (c.x, c.y), a);
    routes.push((2, route));
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
  let colors = vec!["#000", "#000", "#c00"];
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

fn packing(
  seed: f64,
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

fn shake<R: Rng>(
  path: Vec<(f64, f64)>,
  scale: f64,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  path
    .iter()
    .map(|&(x, y)| {
      let a = rng.gen_range(-PI, PI);
      let amp = rng.gen_range(0.0, scale);
      let dx = amp * a.cos();
      let dy = amp * a.sin();
      (x + dx, y + dy)
    })
    .collect()
}

fn heart(radius: f64, (dx, dy): (f64, f64), ang: f64) -> Vec<(f64, f64)> {
  let spins = radius / 0.35;
  let iterations = (spins * 100.0) as usize;
  let mut route = Vec::new();
  if iterations < 2 {
    return route;
  }
  for i in 0..iterations {
    let p = (i as f64) / (iterations - 1) as f64;
    let r = 0.8 * radius * (p * (1.0 + 1.5 / spins)).min(1.0);
    let t = spins * 2.0 * PI * p;
    let x = r * (t.sin().powf(3.0));
    let mut v = t.cos() - t.cos().powf(4.0) + 0.55;
    v = mix(-1.0, v, smoothstep(-3.0, 0.0, v));

    let y = -r * v;
    let mut p = p_r((x, y), ang);
    p.0 += dx;
    p.1 += dy;
    route.push(p);
  }
  route
}
