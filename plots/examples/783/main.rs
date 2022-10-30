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

// IDEA
// use the same builder as in slimes
// pack it in the sky
// try to stretch / disform on x to make it more liquid?

fn slime<R: Rng>(
  mut rng: R,
  globp: &Passage,
  opts: SlimeOpts,
) -> Option<SlimeOut> {
  let (cx, cy) = opts.center;
  let amp1pow = opts.amp1pow;
  let freq1 = opts.freq1;
  let amp1 = opts.amp1;
  let freq2 = opts.freq2;
  let amp2 = opts.amp2;
  let freq3 = opts.freq3;
  let amp3 = opts.amp3;
  let max_r = opts.max_r;
  let disp = opts.disp;
  let dispfreq = opts.dispfreq;
  let rotations = opts.rotations;
  let r_increment = opts.r_increment;
  let seed = rng.gen_range(0.0, 1000.0);

  // this passage is used for inter slime collisions
  let mut passage = Passage::new(globp.precision, globp.width, globp.height);
  // this passage is used to not accumulate too much lines
  let mut collision_passage = Passage::new(0.5, globp.width, globp.height);

  let perlin = Perlin::new();
  let mut routes = Vec::new();
  let mut highest_by_angle = vec![0f64; opts.high_map_size];

  let safe_h = opts.safe_h;
  let mut base_r = 0.2;
  let mut end = false;
  loop {
    if base_r > max_r || end {
      break;
    }
    let mut route = Vec::new();
    let angle_delta =
      rng.gen_range(0, rotations as usize) as f64 / rotations * 2.0 * PI;
    let mut a = angle_delta;
    let angle_precision =
      2. * PI / mix(rotations, 1.0 + 30.0 * base_r, opts.snow_effect).round();
    loop {
      if a - angle_delta > 2. * PI + 0.0001 {
        break;
      }
      let hba_index = (highest_by_angle.len() as f64 * ((a) / 2. * PI))
        as usize
        % highest_by_angle.len();

      let mut r = base_r;
      let x = cx + r * a.cos();
      let y = cy + r * a.sin();
      r += amp1
        * base_r
        * (base_r / max_r).powf(amp1pow)
        * perlin.get([
          -seed
            + amp2
              * perlin.get([
                freq2 * x,
                seed * 7.7 - 4.,
                freq2 * y
                  + amp3 * perlin.get([freq3 * x, seed * 2.7 + 11., freq3 * y]),
              ]),
          freq1 * x,
          freq1 * y,
        ]);

      let should_draw = r > highest_by_angle[hba_index] + safe_h;

      if should_draw {
        let mut x = cx + r * a.cos();
        let mut y = cy + r * a.sin();

        x += disp * perlin.get([77. + seed, dispfreq * x, dispfreq * y]);
        y += disp
          * perlin.get([
            99. + seed,
            dispfreq * x,
            dispfreq * y + 2.0 * perlin.get([5.5 * seed, 0.2 * x, 0.1 * y]),
          ]);

        let p = (x, y);
        if globp.get(p) > 0 {
          end = true;
          break;
        }
        passage.count(p);

        highest_by_angle[hba_index] = highest_by_angle[hba_index].max(r);
        route.push(p);
      } else {
        add_route_simplified(&mut routes, &route, &mut collision_passage);
        route = Vec::new();
      }
      a += angle_precision;
    }

    if end {
      break;
    }

    add_route_simplified(&mut routes, &route, &mut collision_passage);

    base_r += r_increment;
  }

  if base_r < opts.min_r {
    return None;
  }

  Some(SlimeOut { passage, routes })
}

struct SlimeOpts {
  center: (f64, f64),
  amp1pow: f64,
  freq1: f64,
  amp1: f64,
  freq2: f64,
  amp2: f64,
  freq3: f64,
  amp3: f64,
  min_r: f64,
  max_r: f64,
  disp: f64,
  dispfreq: f64,
  rotations: f64,
  safe_h: f64,
  snow_effect: f64,
  high_map_size: usize,
  r_increment: f64,
}
struct SlimeOut {
  routes: Vec<Vec<(f64, f64)>>,
  passage: Passage,
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
  let stopy = rng.gen_range(0.2, 0.8) * height;
  let passage_threshold = 9;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -30.0;

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

  let precision = 0.21;
  let count =
    (2.0 + rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0)) as usize;
  for j in 0..count {
    let peakfactor = rng.gen_range(-0.0002, 0.0005);
    let ampfactor = rng.gen_range(0.03, 0.08);
    let yincr = 0.5;
    let amp2 = rng.gen_range(2.0, 12.0);
    let ynoisefactor = rng.gen_range(0.05, 0.15);
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

  let radius = rng.gen_range(2.0, 5.0);
  passage.grow_passage(radius);

  let max_slimes = 300;
  let max_search = 100000;
  let snow_effect = 0.0;
  let high_map_size = rng.gen_range(6000.0, 10000.0) as usize;
  let amp1pow = rng.gen_range(0.5, 2.0);
  let amp_factor = rng.gen_range(0.0, 1.0);
  let freq1 = rng.gen_range(0.03, 0.06) * (1. - amp_factor);
  let amp1 = 0.1 + 0.4 * amp_factor;
  let freq2 = rng.gen_range(0.02, 0.06);
  let amp2 = rng.gen_range(2.0, 4.0);
  let freq3 = rng.gen_range(0.4, 0.6);
  let amp3 = rng.gen_range(0.0, 0.1);
  let min_r = rng.gen_range(1.0, 2.0);
  let max_r = min_r + rng.gen_range(10.0, 100.0) * rng.gen_range(0.0, 1.0);
  let rotations = (400f64 + 2. * max_r).floor();
  let disp = rng.gen_range(1.0, 6.0);
  let safe_h = rng.gen_range(-8.0, 2.0) * rng.gen_range(0.0, 1.0);
  let dispfreq = rng.gen_range(0.1, 0.3);
  let padding = rng.gen_range(0.6, 2.0);

  let mut center = (width / 2.0, height / 2.0);
  for i in 0..max_slimes {
    let r_increment = rng.gen_range(0.4, 0.6);
    let r = slime(
      &mut rng,
      &passage,
      SlimeOpts {
        center,
        amp1pow,
        freq1,
        amp1,
        freq2,
        amp2,
        freq3,
        amp3,
        max_r,
        min_r,
        disp,
        dispfreq,
        rotations,
        safe_h,
        snow_effect,
        high_map_size,
        r_increment,
      },
    );

    if let Some(r) = r {
      let mut local_passage = r.passage;
      let p = padding * (1.0 + 1.0 / (i as f64 + 1.0));
      local_passage.grow_passage(p);
      passage = passage.add(&local_passage);
      for route in r.routes {
        routes.push((2, route));
      }
    }

    let r = passage.search_space(&mut rng, min_r, max_r, pad, max_search);

    if let Some(p) = r {
      center = p;
    } else {
      break;
    }
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
  let colors = vec!["#000", "#000", "#aaa"];
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

  pub fn add(self: &Self, other: &Self) -> Self {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters = self
      .counters
      .iter()
      .enumerate()
      .map(|(i, v)| v + other.counters[i])
      .collect();
    Passage {
      precision,
      width,
      height,
      counters,
    }
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
  pub fn search_space<R: Rng>(
    self: &Self,
    rng: &mut R,
    min_r: f64,
    max_r: f64,
    pad: f64,
    max_search: usize,
  ) -> Option<(f64, f64)> {
    for j in 0..max_search {
      let optim_r = mix(min_r, max_r, 1.0 / (1.0 + j as f64 * 0.01));
      let minx = pad + optim_r;
      let miny = pad + optim_r;
      let maxx = self.width - pad - optim_r;
      let maxy = self.height - pad - optim_r;

      if minx >= maxx || miny >= maxy {
        break;
      }

      let p = (rng.gen_range(minx, maxx), rng.gen_range(miny, maxy));
      if self.get(p) == 0
        && self.get((p.0 - optim_r, p.1)) == 0
        && self.get((p.0 + optim_r, p.1)) == 0
        && self.get((p.0, p.1 - optim_r)) == 0
        && self.get((p.0, p.1 + optim_r)) == 0
      {
        return Some(p);
      }
    }

    None
  }
}

fn add_route_simplified(
  routes: &mut Vec<Vec<(f64, f64)>>,
  route: &Vec<(f64, f64)>,
  passage: &mut Passage,
) {
  if route.len() < 2 {
    return;
  }

  // simplify the path
  let mut simplified = Vec::new();
  let mut last = route[0];
  simplified.push(last);

  let l = route.len();
  let threshold = 0.12;
  for i in 1..l {
    let p = route[i];
    let dx = last.0 - p.0;
    let dy = last.1 - p.1;
    let d = dx * dx + dy * dy;
    let t = if i == l - 1 { 0.0 } else { threshold };
    if d > t {
      simplified.push(route[i]);
      last = p;
    }
  }

  if simplified.len() < 2 {
    return;
  }
  // split the path using passage if there are too much density
  let mut route = Vec::new();
  for p in simplified {
    if passage.count(p) < 10 {
      route.push(p);
    } else {
      let l = route.len();
      if l > 1 {
        routes.push(route);
        route = Vec::new();
      } else if l > 0 {
        route = Vec::new();
      }
    }
  }
  let l = route.len();
  if l > 1 {
    routes.push(route);
  }
}
