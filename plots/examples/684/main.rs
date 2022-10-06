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
  #[clap(short, long, default_value = "100.0")]
  pub width: f64,
  #[clap(short, long, default_value = "150.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
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

fn gargoyle<R: Rng>(
  origin: (f64, f64),
  scale: f64,
  xreverse: bool,
  rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let xmul = if xreverse { -1.0 } else { 1.0 };
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let n = rng.gen_range(3, 5);
  let maxj = rng.gen_range(11, 15);

  let mut dy = 0.0;
  loop {
    if dy < -1.0 {
      break;
    }
    let mut p = (rng.gen_range(0.0, 0.5), dy);
    let mut route = Vec::new();
    route.push((p.0, dy - 1.0));
    route.push(p);
    p.0 += rng.gen_range(2.8, 3.0);
    let mut a: f64 = 0.0;
    for j in 0..rng.gen_range(10, maxj) {
      let amp = 0.5;
      p.0 += amp * a.cos();
      p.1 += amp * a.sin();
      a += 0.1 + (j - n) as f64 / 9.0;
      route.push(p);
    }
    routes.push(shake(route, 0.1, rng));
    dy -= 0.25 / scale;
  }

  // scale & translate
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&p| (xmul * scale * p.0 + origin.0, scale * p.1 + origin.1))
        .collect()
    })
    .collect()
}

fn building<R: Rng>(
  baseleft: (f64, f64),
  baseright: (f64, f64),
  max_y: f64,
  rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let mut creatures = Vec::new();
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let instability = rng.gen_range(0.0, 1.0);
  let randomplacement = rng.gen_range(-10f64, 1.0).max(0.0);
  let mainscale = rng.gen_range(0.5, 2.0);
  let symmetryfactor =
    (1.1 - rng.gen_range(0f64, 1.0) * rng.gen_range(0.0, 1.0)).min(0.999);

  let mut stages = Vec::new();
  for _i in 0..rng.gen_range(4, 30) {
    stages.push(2.0 + rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0));
  }

  let mut xleft = baseleft.0;
  let mut xright = baseright.0;
  let mut left_ybase = baseleft.1;
  let mut right_ybase = baseright.1;

  for stage in stages {
    if xright - xleft < 2.0 || left_ybase < max_y {
      break;
    }
    let mut rx = 0.0;
    loop {
      let x = xleft + rx;
      let ybottom =
        left_ybase + (right_ybase - left_ybase) * rx / (xright - xleft);
      if x > xright {
        break;
      }
      routes.push(vec![(x, ybottom), (x, ybottom - stage)]);
      rx += rng.gen_range(0.3, 0.6);
    }
    left_ybase -= stage;
    right_ybase -= stage;
    let dxl: f64 = rng.gen_range(-5.0, 5.0)
      * rng.gen_range(instability, 1.0)
      * rng.gen_range(instability, 1.0);
    let dxr = if rng.gen_bool(symmetryfactor) {
      -dxl
    } else {
      rng.gen_range(-4f64, 4.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0)
    };
    let scale = mainscale * rng.gen_range(0.8, 1.3);
    let threshold = 2.0;
    if dxl > threshold || rng.gen_bool(randomplacement) {
      creatures.push(gargoyle(
        (xleft + dxl.min(threshold), left_ybase),
        scale,
        true,
        rng,
      ));
    }
    if dxr < -threshold || rng.gen_bool(randomplacement) {
      creatures.push(gargoyle(
        (xright + dxr.max(-threshold), right_ybase),
        scale,
        false,
        rng,
      ));
    }
    xleft += dxl;
    xright += dxr;
    routes.push(shake(
      vec![(xleft, left_ybase), (xright, right_ybase)],
      0.5,
      rng,
    ));
  }

  vec![creatures.concat(), routes].concat()
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();
  let mut passage = Passage::new(0.5, width, height);
  let passage_threshold = 5;

  let min_route = 2;
  let peakfactor = 0.001;
  let stopy = 0.6 * height;
  let ynoisefactor = rng.gen_range(0.02, 0.1);
  let yincr = rng.gen_range(0.5, 2.0);
  let amp2 = rng.gen_range(1.0, 12.0);
  let precision = rng.gen_range(0.1, 0.3);
  let offsetstrategy = rng.gen_range(0, 5);

  let mut routes = Vec::new();

  let mut base_y = height * 5.0;
  let mut miny = height;
  let mut miny_x = 0.0;
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

      let amp = height * 0.05;
      let mut y = base_y;

      if offsetstrategy == 0 {
        y += amp * peakfactor * xv * xv;
      }

      y += -amp
        * perlin
          .get([
            //
            xv * 0.005,
            y * 0.02,
            77.
              + opts.seed / 7.3
              + perlin.get([
                //
                -opts.seed * 7.3,
                8.3 + xv * 0.02,
                y * 0.1,
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
          8.3 + xv * 0.01,
          88.1 + y * ynoisefactor,
          opts.seed * 97.3,
        ]);

      if offsetstrategy == 2 {
        y += amp * peakfactor * xv * xv;
      }

      y += amp
        * perlin.get([
          //
          opts.seed * 9.3 - 77.,
          xv * 0.1,
          y * 0.5,
        ])
        * perlin
          .get([
            //
            xv * 0.02,
            88.1 + y * 0.2,
            -opts.seed / 7.7,
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
          66666. + opts.seed * 1.3,
          88.3 + xv * 0.5,
          88.1 + y * 0.5,
        ]);

      if offsetstrategy == 4 {
        y += amp * peakfactor * xv * xv;
      }

      if y < miny {
        miny = y;
        miny_x = x;
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

  // moving average
  let smooth = 4;
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

  // look left until slope exceeded
  let delta_threshold = 10.0;
  let mut xlefti = (miny_x / precision) as usize;
  let y = smooth_heights[xlefti].1;
  loop {
    if xlefti == 0 {
      break;
    }
    let dy = (smooth_heights[xlefti].1 - y).abs();
    if dy > delta_threshold {
      break;
    }
    xlefti -= 1;
  }
  let mut xrighti = (miny_x / precision) as usize;
  let y = smooth_heights[xrighti].1;
  loop {
    if xrighti == 0 {
      break;
    }
    let dy = (smooth_heights[xrighti].1 - y).abs();
    if dy > delta_threshold {
      break;
    }
    xrighti += 1;
  }

  let xleft = smooth_heights[xlefti];
  let xright = smooth_heights[xrighti];
  let min = xleft.1.min(xright.1);
  let xleft = (xleft.0, min);
  let xright = (xright.0, min);

  routes = vec![routes, building(xleft, xright, pad + 10.0, &mut rng)].concat();

  for i in 0..10 {
    let d = i as f64 * 0.25;
    routes.push(vec![
      (pad + d, pad + d),
      (pad + d, height - pad - d),
      (width - pad - d, height - pad - d),
      (width - pad - d, pad + d),
      (pad + d, pad + d),
    ]);
  }

  let color = "black";
  let mut data = Data::new();
  for route in routes.clone() {
    data = render_route(data, route);
  }
  let mut l = layer(color);
  l = l.add(base_path(color, 0.35, data));
  vec![l]
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
