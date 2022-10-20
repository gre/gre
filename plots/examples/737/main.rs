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
  pub width: f64,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn armadillo<R: Rng>(
  origin: (f64, f64),
  scale: f64,
  rot: f64,
  rng: &mut R,
  passage: &mut Passage,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();

  let c = (0.0, 0.0);
  let mut basey = -10.0;

  let a1 = rng.gen_range(0.2, 0.4);
  let a2 = rng.gen_range(0.7, 1.1);
  let m1 = 0.7;
  let m2 = rng.gen_range(1.5, 2.0);
  let mut bodypoints = Vec::new();
  for a in vec![
    -a2 + rng.gen_range(-0.5, 0.5) * 0.1 * scale,
    -a1 + rng.gen_range(-0.5, 0.5) * 0.1 * scale,
    a1 + rng.gen_range(-0.5, 0.5) * 0.1 * scale,
    a2 + rng.gen_range(-0.5, 0.5) * 0.1 * scale,
  ] {
    let mp = 0.5;
    let mut line = Vec::new();
    for (a, m) in vec![
      (a, m1 + rng.gen_range(-0.1, 0.1)),
      (a * 1.1, mix(m1, m2, mp) + rng.gen_range(-0.1, 0.1)),
      (a, m2 + rng.gen_range(-0.1, 0.1)),
    ] {
      let ang = a - PI / 2.0;
      let p = (c.0 + m * ang.cos(), c.1 + m * ang.sin());
      if p.1 > basey {
        basey = p.1;
      }
      line.push(p);
    }
    bodypoints.push(line);
  }

  for l in bodypoints.clone() {
    routes.push(l);
  }
  for i in 0..3 {
    let mut line = Vec::new();
    for l in bodypoints.clone() {
      line.push(l[i]);
    }
    routes.push(line);
  }

  // Make feet
  for (i, l) in bodypoints.iter().enumerate() {
    let origin = l[0];
    let a = PI / 2.0 - 0.5 * (i as f64 - 1.5)
      + rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0);
    let a2 = a + rng.gen_range(-0.4, 0.2) * rng.gen_range(0.0, 1.0);
    for _j in 0..3 {
      let mut route = vec![];
      let o = (
        origin.0 + rng.gen_range(-0.1, 0.1),
        origin.1 + rng.gen_range(-0.1, 0.0),
      );
      route.push(o);
      let amp = 0.3 + rng.gen_range(-0.1, 0.1);
      for (amp, a) in vec![(amp, a), (amp + rng.gen_range(0.1, 0.2), a2)] {
        let p = (o.0 + amp * a.cos(), o.1 + amp * a.sin());
        if p.1 > basey {
          basey = p.1;
        }
        route.push(p);
      }
      routes.push(route);
    }
  }

  // Make head
  let headorigin1 = bodypoints[3][2];
  let headorigin2 = bodypoints[3][1];
  // ears
  for _i in 0..4 {
    let mut line = vec![];
    let o = (
      headorigin1.0 + rng.gen_range(-0.1, 0.3) * 0.1 * scale,
      headorigin1.1 + rng.gen_range(-0.1, 0.4) * 0.05 * scale,
    );
    line.push(o);
    for (amp, a) in vec![(0.3f64, -1.2f64), (0.5, rng.gen_range(-1.8, -1.2))] {
      let p = (o.0 + amp * a.cos(), o.1 + amp * a.sin());
      line.push(p);
    }
    routes.push(line);
  }

  let hmul = rng.gen_range(0.8, 1.2);
  let count = (3.0 + scale) as usize;
  let ang1 = rng.gen_range(0.5, 0.8f64);
  let ang1amp = rng.gen_range(0.1, 0.4);
  let ang2 = ang1 + rng.gen_range(0.0, 0.3) * rng.gen_range(0.0, 1.0);
  let ang2amp = rng.gen_range(0.0, 0.1);
  for i in 0..count {
    let percent = i as f64 / (count as f64 - 1.0);
    let p1 = lerp_point(headorigin1, headorigin2, percent);
    let ang = ang1 + ang1amp * percent;
    let amp = 0.4f64 * hmul;
    let p2 = (p1.0 + amp * ang.cos(), p1.1 + amp * ang.sin());
    let ang = ang2 + ang2amp * percent;
    let amp = amp * rng.gen_range(2.0, 3.0);
    let p3 = (
      headorigin2.0 + amp * ang.cos(),
      headorigin2.1 + amp * ang.sin(),
    );
    routes.push(shake(vec![p1, p2, p3], 0.035 * scale, rng));
  }

  // Make tail
  let a = rng.gen_range(-4.4f64, -3.8);
  let amp = rng.gen_range(0.4, 0.8);
  let params = vec![
    (a, 0.0, 1.0),
    (a, amp, 0.5),
    (
      a + rng.gen_range(-0.3, 0.3),
      amp + rng.gen_range(0.5, 1.0),
      0.0,
    ),
  ];
  let tailoriginbase =
    lerp_point(bodypoints[0][2], bodypoints[0][1], rng.gen_range(0.0, 0.7));
  for _i in 0..3 {
    let tailorigin =
      lerp_point(bodypoints[0][2], bodypoints[0][1], rng.gen_range(0.0, 0.7));
    let mut l = vec![];
    for (ang, amp, mul) in params.clone() {
      let o = lerp_point(tailoriginbase, tailorigin, mul);
      let p = (o.0 + amp * ang.cos(), o.1 + amp * ang.sin());
      l.push(p);
    }
    l = path_subdivide_to_curve(l, 1, 0.7);
    l = shake(l, 0.01 * scale, rng);
    l = path_subdivide_to_curve(l, 1, 0.66);
    routes.push(l);
  }

  // debug line
  // routes.push(vec![(-5.0, basey), (5.0, basey)]);

  // scale, rotate & translate
  let routes = routes
    .iter()
    .map(|route| {
      (
        0,
        route
          .iter()
          .map(|&p| {
            let mut p = p_r(p, rot);
            p = round_point(
              (scale * p.0 + origin.0, scale * (p.1 - basey) + origin.1),
              0.01,
            );
            passage.count(p);
            p
          })
          .collect(),
      )
    })
    .collect();

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let seed = opts.seed;
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  // Prepare all the random values
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let passage_threshold = 9;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -30.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let mut armadillos = Vec::new();

  let precision = 0.2 + rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let count = rng.gen_range(3, 7);
  for j in 0..count {
    let stopy = rng.gen_range(0.1, 0.6) * height;
    let peakfactor = rng.gen_range(-0.001, 0.002)
      * rng.gen_range(-0.5f64, 1.0).max(0.0)
      * rng.gen_range(j as f64 / (count as f64), 1.0);
    let ampfactor = rng.gen_range(0.03, 0.1);
    let ynoisefactor = rng.gen_range(0.02, 0.2);
    let yincr = 0.4 + rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(1.0, 12.0);
    let offsetstrategy = rng.gen_range(0, 5);

    let rounding =
      rng.gen_range(-20.0f64, 8.0).max(0.0) * rng.gen_range(0.1, 1.0);

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
              xv * 0.004111 + 19.9,
              y * 0.0211 + 30.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  55. + seed * 7.3,
                  80.3 + xv * 0.015,
                  y * 0.1 + 111.3,
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
              -seed / 7.7 - 6.66,
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

        if rounding > 0.1 {
          y = (y / rounding).round() * rounding;
        }

        if y < miny {
          miny = y;
        }
        let mut collides = false;
        let xi = ((x - mountainpadding) / precision) as usize;
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

    // calculate a moving average to smooth the stick men positions
    let smooth = 20;
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

    if rng.gen_bool(if j == count - 1 { 0.25 } else { 0.5 })
      && !(armadillos.len() == 0 && j == count - 1)
    {
      continue;
    }
    // place animals
    let splits =
      (1.0 + rng.gen_range(0.0, 32.0) * rng.gen_range(0., 1.)) as usize;
    let extrapad = pad + 6.0;
    let scale_base = 0.5
      + rng.gen_range(0.0, 1.8) * rng.gen_range(0.0, 1.0)
      + 5.0 / (splits as f64 + 1.0);
    for s in 0..splits {
      let p = (rng.gen_range(0.3, 0.7) + s as f64) / (splits as f64);
      let x = extrapad + p * (width - 2.0 * extrapad);
      let xi = ((x - mountainpadding) / precision) as usize;
      let v = smooth_heights[xi];
      let scale = scale_base * rng.gen_range(0.8, 1.2);
      let rot = v.2;
      if strictly_in_boundaries(
        (v.0, v.1),
        (extrapad, extrapad, width - extrapad, height - extrapad),
      ) && rng.gen_bool(0.95)
      {
        armadillos.push(armadillo(
          (v.0, v.1),
          scale,
          rot,
          &mut rng,
          &mut passage,
        ));
      }
    }
  }

  routes = vec![routes, armadillos.concat()].concat();

  let radius = rng.gen_range(3.0, 4.0);
  passage.grow_passage(radius);

  let shape = (3.0
    + rng.gen_range(0.0, 20.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;

  let p = pad + 2.0;
  let extrabound = (p, p, width - p, height - p);

  let overlap = |p| {
    passage.get(p) == 0
      && strictly_in_boundaries(p, extrabound)
      && p.1
        < height_map
          [((p.0 - mountainpadding) / precision) as usize % height_map.len()]
  };

  let does_overlap = |c: (f64, f64, f64)| {
    overlap((c.0, c.1))
      && circle_route((c.0, c.1), c.2, shape)
        .iter()
        .all(|&p| overlap(p))
  };

  let ppad = 1.2;
  let total_pad = radius + pad + ppad;
  let min = ppad + rng.gen_range(0.5, 0.8);
  let max = min
    + rng.gen_range(0.0, 40.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
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
    routes.push((2, circle_route((c.x, c.y), c.r, shape)));
  }

  // Border around the postcard
  let border_size = 8;
  let border_dist = 0.3;
  let mut route = Vec::new();
  for i in 0..border_size {
    let d = i as f64 * border_dist;
    route.push((pad + d, pad + d));
    route.push((pad + d, height - pad - d));
    route.push((width - pad - d, height - pad - d));
    route.push((width - pad - d, pad + d));
    route.push((pad + d, pad + d));
  }
  routes.push((1, route));

  // Make the SVG
  let colors = vec!["black", "#F90", "#09F"];
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

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
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
      let dx = rng.gen_range(-scale, scale);
      let dy = rng.gen_range(-scale, scale);
      (x + dx, y + dy)
    })
    .collect()
}
