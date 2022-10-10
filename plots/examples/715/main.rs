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
  #[clap(short, long, default_value = "105.0")]
  pub width: f64,
  #[clap(short, long, default_value = "148.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "679.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;

  // Prepare all the random values
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let peakfactor = rng.gen_range(0.0, 0.0008) * rng.gen_range(0.0, 1.0);
  let stopy = rng.gen_range(0.4, 0.6) * height;
  let ampfactor = rng.gen_range(0.08, 0.12);
  let ynoisefactor = rng.gen_range(0.05, 0.1);
  let yincr = rng.gen_range(0.4, 0.6);
  let xfreq = rng.gen_range(0.005, 0.01);
  let amp2 = rng.gen_range(1.0, 4.0);
  let precision = rng.gen_range(0.1, 0.3);
  let offsetstrategy = rng.gen_range(0, 5);

  let mut passage = Passage::new(0.5, width, height);
  let passage_threshold = 8;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();
  let mut routes_red = Vec::new();

  // Build the mountains bottom-up, with bunch of perlin noises
  let mut base_y = height * 5.0;
  let mut miny = height;
  let mut height_map: Vec<f64> = Vec::new();
  loop {
    if miny < stopy {
      break;
    }

    let mut route = Vec::new();
    let mut x = pad;
    let mut was_outside = true;
    loop {
      if x > width - pad {
        break;
      }
      let xv = (4.0 - base_y / height) * (x - width / 2.);

      let amp = height * ampfactor;
      let xx = (x - pad) / (width - 2. * pad);
      let xborderd = xx.min(1.0 - xx);
      let displacement = amp * peakfactor * (xv * xv - (xborderd).powf(0.5));

      let mut y = base_y;

      if offsetstrategy == 0 {
        y += displacement;
      }

      y += -amp
        * perlin
          .get([
            //
            xv * xfreq + 9.9,
            y * 0.02 - 3.1,
            77.
              + opts.seed / 7.3
              + perlin.get([
                //
                -opts.seed * 7.3,
                8.3 + xv * 0.015,
                y * 0.1,
              ]),
          ])
          .abs();

      if offsetstrategy == 1 {
        y += displacement;
      }

      y += amp2
        * amp
        * perlin.get([
          //
          8.3 + xv * 0.008,
          88.1 + y * ynoisefactor,
          opts.seed * 97.3,
        ]);

      if offsetstrategy == 2 {
        y += displacement;
      }

      y += amp
        * perlin.get([
          //
          opts.seed * 9.3 + 77.77,
          xv * 0.08 + 9.33,
          y * 0.5,
        ])
        * perlin
          .get([
            //
            xv * 0.015 - 88.33,
            88.1 + y * 0.2,
            -opts.seed / 7.7 - 6.66,
          ])
          .min(0.0);

      if offsetstrategy == 3 {
        y += displacement;
      }

      y += 0.1
        * amp
        * (1.0 - miny / height)
        * perlin.get([
          //
          6666.6 + opts.seed * 1.3,
          8.3 + xv * 0.5,
          88.1 + y * 0.5,
        ]);

      if offsetstrategy == 4 {
        y += displacement;
      }

      if y < miny {
        miny = y;
      }
      let mut collides = false;
      let xi = (x / precision) as usize;
      if xi >= height_map.len() {
        height_map.push(y);
      } else {
        if y > height_map[xi] {
          collides = true;
        } else {
          height_map[xi] = y;
        }
      }
      let inside =
        !collides && pad < x && x < width - pad && pad < y && y < height - pad;
      if inside && passage.get((x, y)) < passage_threshold {
        if was_outside {
          if route.len() > min_route {
            routes.push(route);
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
      routes.push(route);
    }

    base_y -= yincr;
  }

  // We use a "smooth average" algorithm to ignore the sharp edges of the mountains
  let smooth = rng.gen_range(10, 40);
  let sf = smooth as f64;
  let mut sum = 0.0;
  let mut acc = Vec::new();
  let mut smooth_heights = Vec::new();
  for (i, h) in height_map.iter().enumerate() {
    if acc.len() == smooth {
      let avg = sum / sf;
      let xtheoric = (i as f64 - sf / 2.0) * precision;
      smooth_heights.push((xtheoric, avg));
      let prev = acc.remove(0);
      sum -= prev;
    }
    acc.push(h);
    sum += h;
  }
  // We can then highlight the mountain tops with sorting:
  smooth_heights.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

  let mut big_passage = Passage::new(40.0, width, height);

  for s in 0..rng.gen_range(1, 4) {
    let c = smooth_heights[(s * 77) % smooth_heights.len()];
    if big_passage.get(c) > 0 {
      continue;
    }
    big_passage.count(c);
    for i in 0..8 {
      let dy = i as f64 * 0.25;
      let mx = rng.gen_range(0.5, 5.0);
      let my = rng.gen_range(0.3, 1.2);
      let x = c.0 + rng.gen_range(-2.0, 2.0);
      let y = c.1 - dy;
      routes_red.push(vec![(x - mx, y - my), (x + mx, y + my)]);
      let x = c.0 + rng.gen_range(-2.0, 2.0);
      routes_red.push(vec![(x - mx, y + my), (x + mx, y - my)]);
    }

    let r = 4.0;
    let c = (c.0, c.1 - r);
    let mut route = Vec::new();
    let count = 32;
    for i in 0..(count + 1) {
      let p = i as f64 / (count as f64);
      let a = 2.0 * PI * p;
      route.push((c.0 + a.cos() * r * 0.4, c.1 + a.sin() * r * 0.8));
    }
    routes_red.push(route);

    let mut eagles = Vec::new();
    let p = pad + 10.0;
    let skip_factor = 1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
    for i in 0..16 {
      let x = c.0 + i as f64 * 4.0 * rng.gen_range(-1.0, 1.0);
      let y = c.1 - 7.0 - i as f64 * 6.0 - rng.gen_range(-1.0, 1.0);
      if x < p
        || y < p
        || x > width - p
        || y > height - p
        || rng.gen_bool(skip_factor)
      {
        continue;
      }
      let scale = rng.gen_range(0.8, 1.0);
      let rot = 0.2 * rng.gen_range(-PI, PI);
      let xreverse = rng.gen_bool(0.5);
      eagles.push(eagle((x, y), scale, rot, xreverse, &mut rng));
    }
    routes = vec![routes, eagles.concat()].concat();
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
  routes.push(route);

  // Make the SVG
  vec![("black", routes), ("red", routes_red)]
    .iter()
    .map(|(color, routes)| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
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

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }
}

fn eagle<R: Rng>(
  origin: (f64, f64),
  scale: f64,
  rotation: f64,
  xreverse: bool,
  rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let xmul = if xreverse { -1.0 } else { 1.0 };
  let count = 2 + (scale * 3.0) as usize;
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let shaking = scale * 0.1;

  // body
  let bodyw = 5.0;
  let bodyh = 1.5;
  let headcompression = rng.gen_range(0.1, 0.5);
  let headoff = rng.gen_range(0.1, 0.5);
  for i in 0..count {
    let yp = (i as f64 - (count - 1) as f64 * 0.5) / (count as f64);
    let ybase = bodyh * yp;
    let route = shake(
      path_subdivide_to_curve(
        vec![
          (-rng.gen_range(0.4, 0.6) * bodyw, 1.5 * ybase),
          (-0.3 * bodyw, ybase),
          (0.2 * bodyw, ybase),
          (0.45 * bodyw, headcompression * ybase + headoff * bodyh),
        ],
        1,
        0.8,
      ),
      shaking,
      rng,
    );
    routes.push(route);
  }

  let count = 2 + (scale * rng.gen_range(4.0, 6.0)) as usize;

  // wings
  let wingw = 1.4;
  let wingh = 8.0;
  let dx1 = rng.gen_range(-4.0, 4.0) * rng.gen_range(0.0, 1.0);
  let dx2 = if rng.gen_bool(0.8) {
    -dx1
  } else {
    rng.gen_range(-3.0, 3.0)
  };
  let spread1 = 1.0 + rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let spread2 = 1.0 + rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let offset1 = rng.gen_range(-1.0, 0.6) * rng.gen_range(0.0, 1.0);
  let offset2 = rng.gen_range(-1.0, 0.6) * rng.gen_range(0.0, 1.0);
  let interp = 0.5;
  let wing1m = 1.0 - rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
  let wing2m = 1.0 - rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
  let wing2up = rng.gen_bool(0.5);

  for i in 0..count {
    let xp = (i as f64 - (count - 1) as f64 * 0.5) / (count as f64);
    let xbase = wingw * xp;
    let wing1 = rng.gen_range(0.8, 1.1) * wing1m;
    let wing2 =
      rng.gen_range(0.8, 1.1) * wing2m * (if wing2up { -1.0 } else { 1.0 });
    let route = shake(
      path_subdivide_to_curve(
        vec![
          (
            xbase * spread1 + dx1 + wingw * offset1,
            -wingh * 0.5 * wing1,
          ),
          (xbase + dx1 * interp, -wingh * 0.5 * interp * wing1),
          (xbase, 0.0),
          (xbase + dx2 * interp, wingh * 0.5 * interp * wing2),
          (xbase * spread2 + dx2 + wingw * offset2, wingh * 0.5 * wing2),
        ],
        2,
        0.8,
      ),
      shaking,
      rng,
    );
    routes.push(route);
  }

  // scale, rotate & translate
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&p| {
          let p = p_r(p, rotation);
          (xmul * scale * p.0 + origin.0, scale * p.1 + origin.1)
        })
        .collect()
    })
    .collect()
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
