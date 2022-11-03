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

fn wall_shadow<R: Rng>(
  rng: &mut R,
  path: Vec<(f64, f64)>,
  stroke_len: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  if path.len() < 2 {
    return routes;
  }
  let mut prev = path[0];
  let mut current_l = euclidian_dist(prev, path[1]);
  let mut direction = (0.0, 0.0);
  let mut i = 0;
  let mut l = 0.0;
  loop {
    while l > current_l {
      l -= current_l;
      prev = path[i];
      i += 1;
      if i >= path.len() {
        return routes;
      }
      current_l = euclidian_dist(prev, path[i]);
      let dx = path[i].0 - prev.0;
      let dy = path[i].1 - prev.1;
      direction = (-dy / current_l, dx / current_l);
    }
    let p = lerp_point(prev, path[i], l / current_l);
    let slen = stroke_len * rng.gen_range(0.8, 1.5);
    if direction.0.abs() > 0.01 || direction.1.abs() > 0.01 {
      routes.push(vec![
        p,
        (p.0 + slen * direction.0, p.1 + slen * direction.1),
      ]);
    }
    l += rng.gen_range(0.8, 1.2);
  }
}

fn spawn<R: Rng>(
  p: (f64, f64),
  i: usize,
  w: f64,
  rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  match i {
    3 => {
      let h = rng.gen_range(1.2, 1.6) * w;
      // base walls
      routes.push(vec![
        (p.0 - w, p.1 - h),
        (p.0 - w, p.1),
        (p.0 + w, p.1),
        (p.0 + w, p.1 - h),
      ]);
      let w2 = 0.5 * w;
      let h2 = 0.9 * h;
      // square door
      routes.push(vec![
        (p.0 - w2, p.1),
        (p.0 + w2, p.1),
        (p.0 + w2, p.1 - h2),
        (p.0 - w2, p.1 - h2),
        (p.0 - w2, p.1),
      ]);
      // cross in the door
      routes.push(vec![(p.0 + w2, p.1), (p.0 - w2, p.1 - h2)]);
      routes.push(vec![(p.0 - w2, p.1), (p.0 + w2, p.1 - h2)]);
      let h2 = rng.gen_range(1.1, 1.2) * h;
      let w2 = rng.gen_range(0.3, 0.5) * w;
      let h3 = rng.gen_range(1.25, 1.4) * h;
      // window on top
      routes.push(vec![
        (p.0 - w2, p.1 - h2),
        (p.0 + w2, p.1 - h2),
        (p.0, p.1 - h3),
        (p.0 - w2, p.1 - h2),
      ]);
      let y = p.1 - h;
      let h = rng.gen_range(0.7, 1.0) * h;
      let w2 = rng.gen_range(0.3, 0.6);
      let h2 = rng.gen_range(0.6, 0.8) * h;
      // roof
      let route = vec![
        (p.0 - w, y),
        (p.0 - w * w2, y - h2),
        (p.0, y - h),
        (p.0 + w * w2, y - h2),
        (p.0 + w, y),
      ];
      routes.push(route.clone());
      for r in wall_shadow(rng, route, 0.1 * w) {
        routes.push(r);
      }
    }
    1 => {
      let w = 0.5 * w;
      let h = rng.gen_range(1.2, 3.6) * w;
      // base walls
      let r = 0.8;
      let w2 = 0.6;
      // tube
      let route = vec![
        (p.0 - w, p.1 - h - r),
        (p.0 - w, p.1 - h),
        (p.0 - w + r, p.1 - h + r),
        (p.0 - w2, p.1 - h + r),
        (p.0 - w2, p.1),
        (p.0 + w2, p.1),
        (p.0 + w2, p.1 - h + r),
        (p.0 + w - r, p.1 - h + r),
        (p.0 + w, p.1 - h),
        (p.0 + w, p.1 - h - r),
        (p.0 - w, p.1 - h - r),
      ];
      routes.push(route);
      // cords
      for d in vec![-1.0, -0.5, 0.5, 1.0] {
        routes.push(vec![
          (
            p.0 - rng.gen_range(1.0, 1.2) * w * d,
            p.1 + rng.gen_range(0.3, 3.0),
          ),
          (p.0 - rng.gen_range(0.52, 0.7) * w * d, p.1 - h + r),
        ]);
      }
      // cuve shadow
      let y = p.1 - h;
      let h = rng.gen_range(1.8, 2.2) * w;
      let route = vec![(p.0 + w, y), (p.0 + w, y - h)];
      for r in wall_shadow(rng, route, -0.7 * w) {
        routes.push(r);
      }
      // cuve
      routes.push(vec![
        (p.0 + w, y),
        (p.0 + w, y - h),
        (p.0 - w, y - h),
        (p.0 - w, y),
        (p.0 + w, y),
      ]);
      // roof
      let y = y - h;
      let h = w * rng.gen_range(0.8, 1.2);
      let mut l = 0.0;
      loop {
        if l > 2.0 * w {
          break;
        }
        routes.push(vec![(p.0, y - h), (p.0 + w - l, y)]);
        l += rng.gen_range(0.3, 0.7) + l / w;
      }
      routes.push(vec![(p.0 + w, y), (p.0, y - h), (p.0 - w, y)]);
    }
    2 => {
      // cuve shadow
      let y = p.1;
      let h = rng.gen_range(1.8, 2.2) * w;
      let w = 0.5 * w;
      let route = vec![(p.0 + w, y), (p.0 + w, y - h)];
      for r in wall_shadow(rng, route, -0.7 * w) {
        routes.push(r);
      }
      // cuve
      routes.push(path_subdivide_to_curve(
        vec![
          (p.0 + w, y),
          (p.0 + w, y - h),
          (p.0 - w, y - h),
          (p.0 - w, y),
          (p.0 + w, y),
        ],
        2,
        0.9,
      ));
    }
    0 => {
      let h = rng.gen_range(2.5, 3.5) * w;
      let p1 = (p.0, p.1 - h);
      routes.push(vec![p, p1]);
      for d in vec![-1.0, 1.0] {
        routes.push(vec![
          (
            p.0 + d * w * rng.gen_range(0.5, 0.6),
            p.1 + rng.gen_range(1.0, 3.0),
          ),
          p1,
        ]);
      }

      let p2 = (p1.0 + rng.gen_range(0.5, 0.8) * w, p1.1);
      let p3 = (p1.0 - rng.gen_range(0.7, 1.0) * w, p1.1);
      routes.push(vec![p3, p2]);
      routes.push(vec![(p3.0, p3.1 - 0.3), (p2.0, p2.1 - 0.3)]);
      // helices
      for d in vec![-1.0, 0.0, 1.0] {
        routes.push(vec![
          (p2.0 - d * 0.2 * w, p2.1 - 0.6 * w),
          (p2.0 + d * 0.2 * w, p2.1 + 0.6 * w),
        ]);
      }
      // vent
      let count = (w * 2.0) as usize;
      for i in 0..count {
        let f = i as f64 / (count as f64 - 1.0);
        routes.push(vec![p1, (p3.0, p3.1 + f * 0.4 * w)]);
      }
    }
    _ => {}
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
  let stopy = rng.gen_range(0.3, 0.6) * height;
  let passage_threshold = 9;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -10.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let bigpad = pad + 2.0;
  passage.prepare(|(x, y)| {
    if x < bigpad || y < bigpad || x > width - bigpad || y > height - bigpad {
      1
    } else {
      0
    }
  });

  let precision = 0.2;
  let count =
    (2.0 + rng.gen_range(0.0, 20.0) * rng.gen_range(0.2, 1.0)) as usize;
  for j in 0..count {
    let peakfactor = rng.gen_range(-0.001, 0.001) * rng.gen_range(0.0, 1.0);
    let ampfactor = rng.gen_range(0.0, 0.12);
    let yincr = 0.5
      + rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
    let amp1 = rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(0.0, 6.0) * rng.gen_range(0.0, 1.0);
    let amp3 = rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
    let ynoisefactor = rng.gen_range(0.05, 0.1);
    let offsetstrategy = rng.gen_range(0, 5);
    let xc = width * rng.gen_range(0.2, 0.8);

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
        let xv = (4.01 - base_y / height) * (x - xc);

        let amp = height * ampfactor;
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
              xv * 0.007111 + 19.9,
              y * 0.00311 + 30.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  55. + seed * 7.3,
                  80.3 + xv * 0.015,
                  y * 0.06 + 111.3,
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
            66.6 + seed * 1.3,
            18.3 + xv * 0.2,
            88.1 + y * 0.3,
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
              routes.push((0, route));
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
        routes.push((0, route));
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

  // We can then highlight the mountain tops with sorting:
  smooth_heights.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

  let mut spaces = Vec::new();

  let wbase = 4.0 + rng.gen_range(0.0, 6.0) * rng.gen_range(0.0, 1.0);

  let incr_base = rng.gen_range(1, 16);

  let proba = rng.gen_range(0.0, 1.0);

  let mut i = 0;
  loop {
    if i >= smooth_heights.len() {
      break;
    }
    let p = smooth_heights[i % smooth_heights.len()];
    let w = wbase * rng.gen_range(0.8, 1.2);
    let extrapad = rng.gen_range(0.5, 3.0) * rng.gen_range(0.0, 1.0);
    if strictly_in_boundaries(
      (p.0, p.1),
      (
        pad + 10.0,
        pad + 10.0,
        width - pad - 10.0,
        height - pad - 10.0,
      ),
    ) && !spaces.iter().any(|&(left, right)| {
      left < p.0 - w && p.0 - w < right
        || left < p.0 + w && p.0 + w < right
        || p.0 - w < left && left < p.0 + w
        || p.0 - w < right && right < p.0 + w
    }) {
      let input = spaces.len();
      let i = if input > 3 && rng.gen_bool(proba) {
        mix(0.0, 4.0, rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0))
          as usize
      } else {
        input
      };
      for r in spawn((p.0, p.1), i, w, &mut rng) {
        routes.push((0, r.clone()));
        for p in path_subdivide_to_curve_it(r.clone(), 0.8) {
          passage.count(p);
        }
        for p in path_subdivide_to_curve_it(r.clone(), 0.6) {
          passage.count(p);
        }
      }
      spaces.push((p.0 - w - extrapad, p.0 + w + extrapad));
    }
    i += incr_base + rng.gen_range(1, 11);
  }

  // sun
  let l = smooth_heights.len();
  let ci = ((0.5 + rng.gen_range(-0.3, 0.3) * rng.gen_range(0.0, 1.0))
    * (l as f64)) as usize;
  let center = smooth_heights[ci];

  let approx = 0.05;
  let dr = 0.4;
  let radius = rng.gen_range(10.0, 24.0);
  let c = (center.0, mix(center.1, pad, rng.gen_range(0.0, 1.0)));
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r: f64 = radius + 2.0 * dr;
  let mut a = 0f64;
  loop {
    let ar = r.min(radius);
    let p = round_point((c.0 + ar * a.cos(), c.1 + ar * a.sin()), 0.01);
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      if !strictly_in_boundaries(p, bound)
        || p.1 > height_map[((p.0 - mountainpadding) / precision) as usize]
      {
        if route.len() > 1 {
          routes.push((2, route));
        }
        route = vec![];
      } else {
        route.push(p);
      }
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  if route.len() > 1 {
    routes.push((2, route));
  }

  // sky

  let grow = rng.gen_range(1.2, 2.0);
  passage.grow_passage(grow);
  let circleang = if rng.gen_bool(0.5) { 0.5 } else { 0.0 };

  let overlap = |p| {
    passage.get(p) == 0
      && strictly_in_boundaries(p, bound)
      && p.1
        < height_map
          [((p.0 - mountainpadding) / precision) as usize % height_map.len()]
      && euclidian_dist(c, p) > radius
  };

  let does_overlap = |c: (f64, f64, f64)| {
    overlap((c.0, c.1))
      && circle_route((c.0, c.1), c.2, 12, circleang)
        .iter()
        .all(|&p| overlap(p))
  };

  let angbase = rng.gen_range(-PI, PI);
  let delta = rng.gen_range(-0.2, 0.8);
  let amp = rng.gen_range(1.5, 2.5);
  let freq = rng.gen_range(0.0, 0.2);
  let count = 6;
  if count > 0 {
    for i in 0..count {
      let ppad = rng.gen_range(0.4, 1.2);
      let total_pad = grow + pad + ppad;
      let min = ppad + rng.gen_range(0.6, 2.0);
      let max = min + rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
      let circles = packing(
        seed + i as f64 * 1.6,
        100000,
        10000,
        rng.gen_range(0, 4),
        ppad,
        (total_pad, total_pad, width - total_pad, height - total_pad),
        &does_overlap,
        min,
        max,
      );

      for c in circles {
        let ang = angbase
          + amp
            * perlin.get([
              //
              c.x * freq,
              c.y * freq,
              3333. + 7.7 * seed,
            ])
            * (perlin.get([
              c.x * 0.2 * freq,
              c.y * 0.2 * freq,
              4.3 + seed / 0.3,
            ]) + delta)
              .max(0.0);
        let a = (c.x + c.r * ang.cos(), c.y + c.r * ang.sin());
        let b = (c.x - c.r * ang.cos(), c.y - c.r * ang.sin());
        routes.push((1, vec![a, b]));
      }
    }
  }

  // External frame to around the whole piece
  let mut d = 0.0;
  loop {
    if d > 2.0 {
      break;
    }
    routes.push((
      0,
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
  let colors = vec!["#000", "#ccc", "#fb0"];
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

  pub fn prepare<F: Fn((f64, f64)) -> usize>(self: &mut Self, f: F) {
    let mut x = 0.0;
    loop {
      if x >= self.width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= self.height {
          break;
        }
        let index = self.index((x, y));
        self.counters[index] = f((x, y));
        y += self.precision;
      }
      x += self.precision;
    }
  }

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }

  pub fn count_once(self: &mut Self, p: (f64, f64)) {
    let i = self.index(p);
    let v = self.counters[i];
    if v == 0 {
      self.counters[i] = 1;
    }
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

fn circle_route(
  center: (f64, f64),
  r: f64,
  count: usize,
  ang: f64,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * (i as f64 + ang) / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}
