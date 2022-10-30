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
  pub height: f64,
  #[clap(short, long, default_value = "105.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn cell(opts: &Opts) -> Vec<(usize, Vec<(f64, f64)>)> {
  let seed = opts.seed;
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);
  let bound2 = (pad + 2.0, pad + 2.0, width - pad - 2.0, height - pad - 2.0);

  // Prepare all the random values
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let stopy = rng.gen_range(0.1, 0.4) * height;
  let watery = rng.gen_range(0.5, 0.8) * height;
  let passage_threshold = 9;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -30.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let max_trees = rng.gen_range(0.0, 8.0);
  let max_tippis = rng.gen_range(0.0, 20.0);
  let tippis_threshold = rng.gen_range(0.2, 0.4);

  let bigpad = pad + 2.0;
  passage.prepare(|(x, y)| {
    if x < bigpad || y < bigpad || x > width - bigpad || y > height - bigpad {
      1
    } else {
      0
    }
  });

  let precision = 0.2;
  let count = (2.0 + rng.gen_range(0.0, 20.0)) as usize;
  for j in 0..count {
    let peakfactor = rng.gen_range(-0.002, 0.002) * rng.gen_range(0.0, 1.0);
    let ampfactor = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);
    let yincr = 0.5 + rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(2.0, 6.0);
    let ynoisefactor = rng.gen_range(0.0, 0.3) * rng.gen_range(0.0, 1.0);
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
              xv * 0.003111 + 19.9,
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
            8.311 + xv * 0.00511,
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
            xv * 0.03 + 9.33,
            y * 0.5,
          ])
          * perlin
            .get([
              //
              xv * 0.009 + 8.33,
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
            18.3 + xv * 0.1,
            88.1 + y * 0.1,
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
          if y > height_map[xi] || y > watery {
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

    let count = (1.0
      + j as f64
      + rng.gen_range(0.0, max_tippis) * rng.gen_range(0.0, 1.0))
      as usize;
    for _i in 0..count {
      let x = rng.gen_range(5.0 + pad, width - pad - 5.0);
      let xi = ((x - mountainpadding) / precision) as usize;
      let y = height_map[xi] + rng.gen_range(0.5, 5.0);
      if perlin.get([x * 0.04, y * 0.04, 55. + 0.3 * seed]) < tippis_threshold {
        continue;
      }
      if !strictly_in_boundaries((x, y), bound2) {
        continue;
      }
      let h = mix(1.0, 7.0, y / height) * rng.gen_range(0.7, 1.3);
      let w = rng.gen_range(0.2, 0.5) * h;
      let dy = rng.gen_range(-0.2, 0.2);

      routes.push((0, vec![(x - w * 1.5, y - dy), (x + w * 1.5, y + dy)]));
      for _i in 0..3 {
        routes.push((
          0,
          vec![
            (
              x - w + rng.gen_range(-0.1, 0.1),
              y - dy + rng.gen_range(-0.1, 0.1),
            ),
            (
              x + rng.gen_range(0.1, 0.3) * w,
              y + dy - h + rng.gen_range(-0.1, 0.1),
            ),
          ],
        ));
        routes.push((
          0,
          vec![
            (
              x + rng.gen_range(-0.1, 0.1) + w,
              y + rng.gen_range(-0.1, 0.1) + dy,
            ),
            (
              x - rng.gen_range(0.1, 0.3) * w,
              y + rng.gen_range(-0.1, 0.1) + dy - h,
            ),
          ],
        ));
      }
    }

    // Trees
    let count =
      (rng.gen_range(0.0, max_trees) * rng.gen_range(0.0, 1.0)) as usize;
    if count > 0 {
      for _i in 0..count {
        let x = rng.gen_range(5.0 + pad, width - pad - 5.0);
        let xi = ((x - mountainpadding) / precision) as usize;
        let y = height_map[xi];
        let p = (x, y);
        if !strictly_in_boundaries((x, y), bound2) {
          continue;
        }
        let h = 2.0 + rng.gen_range(4.0, 8.0) / (0.8 + j as f64 * 0.5);
        let mut path = vec![
          (p.0, p.1),
          (
            p.0 - rng.gen_range(-1.0, 1.0) * h * 0.2,
            p.1 - rng.gen_range(0.3, 0.6) * h,
          ),
          (p.0 - rng.gen_range(-1.0, 1.0) * h * 0.4, p.1 - h),
        ];
        path = path_subdivide_to_curve(path, 2, 0.66);
        for _j in 0..5 {
          routes.push((0, shake(path.clone(), 0.4, &mut rng)));
        }
        for j in 0..rng.gen_range(4, 12) {
          let x =
            rng.gen_range(0.5 * h, h) * (if j % 2 == 0 { -1.0 } else { 1.0 });
          let mut path = vec![
            (p.0, p.1 - h * rng.gen_range(0.9, 1.0)),
            (p.0 + x * 0.5, p.1 - h - rng.gen_range(-1.0, 1.0) * h * 0.2),
            (p.0 + x, p.1 - h * rng.gen_range(0.3, 1.1)),
          ];
          path = path_subdivide_to_curve(path, 2, 0.66);
          routes.push((0, path));
        }
      }
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

  // sun
  let center = smooth_heights
    [(rng.gen_range(0.4, 0.6) * (smooth_heights.len() as f64)) as usize];

  let approx = 0.05;
  let dr = 0.5;
  let radius = rng.gen_range(10.0, 20.0);
  let c = (center.0, center.1 - rng.gen_range(-0.2, 0.8) * radius);
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

  // particle of sun
  let approx = 0.3;
  let mut route = Vec::new();
  let mut r: f64 = radius + dr;
  let mut a = 0f64;
  let rmul = rng.gen_range(0.0, 100.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);
  let rmul2 = rng.gen_range(-5.0f64, 10.0).max(0.0);
  let ymul = (rng.gen_range(-5.0f64, 15.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0))
  .max(0.0);
  let xmul = if rng.gen_bool(0.5) {
    ymul
  } else {
    (rng.gen_range(-5.0f64, 15.0)
      * rng.gen_range(0., 1.0)
      * rng.gen_range(0., 1.0))
    .max(0.0)
  };
  let space_mod = 1.0 + rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0);
  loop {
    let ar = r;
    let p = round_point((c.0 + ar * a.cos(), c.1 + ar * a.sin()), 0.01);
    let l = route.len();
    let disabled = (rmul * r + p.0 * xmul + p.1 * ymul) % space_mod > 1.0
      || r * rmul2 % 4.0 > 1.0;
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      if disabled
        || p.1 > watery
        || !strictly_in_boundaries(p, bound2)
        || p.1 > height_map[((p.0 - mountainpadding) / precision) as usize]
      {
        if l > 1 {
          routes.push((1, route));
          route = vec![];
        } else if l > 0 {
          route = vec![];
        }
      } else {
        route.push(p);
      }
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r += dr * da / two_pi;
    if r > width {
      break;
    }
  }

  // reflection
  for (clr, route) in routes.clone() {
    let c = clr as f64;
    let disp = (1. + 4.0 * c).max(0.0);
    for p in route.iter() {
      if rng.gen_bool(0.1 + 0.2 * c) {
        let (x, y) = *p;
        let p = (
          x + rng.gen_range(0.0, disp) * rng.gen_range(0.0, 1.0),
          watery
            + (watery - y)
              * (1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0)),
        );
        if strictly_in_boundaries(p, bound2) {
          let dx = rng.gen_range(0.5, 0.8);
          let dy = rng.gen_range(0.0, 0.2);
          let r = vec![(p.0 - dx, p.1 - dy), (p.0 + dx, p.1 + dy)];
          routes.push((clr, r));
        }
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
  let colors = vec!["#000", "#fc0", "#fb0"];
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
