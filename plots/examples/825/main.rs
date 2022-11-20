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
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

struct Slice {
  routes_above: Vec<(usize, Vec<(f64, f64)>)>,
  routes_below: Vec<(usize, Vec<(f64, f64)>)>,
  a: (f64, f64),
  b: (f64, f64),
}
fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}
fn is_left(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
  ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)) > 0.0
}
fn slice_routes(
  routes: Vec<(usize, Vec<(f64, f64)>)>,
  cuta: (f64, f64),
  cutb: (f64, f64),
) -> Slice {
  let mut routes_above = Vec::new();
  let mut routes_below = Vec::new();

  let mut amin = lerp_point(cuta, cutb, 0.5);
  let mut bmin = amin;
  let mut dista = 99999.0;
  let mut distb = 0.0;

  for (clr, r) in routes.clone() {
    if r.len() < 2 {
      continue;
    }
    let mut prev = r[0];
    let mut route = vec![prev];
    for &p in r.iter().skip(1) {
      if let Some(c) = collides_segment(prev, p, cuta, cutb) {
        let la = euclidian_dist(c, cuta);
        if la > distb {
          distb = la;
          bmin = c;
        }
        if la < dista {
          dista = la;
          amin = c;
        }

        route.push(c);
        if route.len() > 1 {
          if !is_left(cuta, cutb, prev) {
            routes_above.push((clr, route));
          } else {
            routes_below.push((clr, route));
          }
        }
        route = vec![c, p];
      } else {
        route.push(p);
      }
      prev = p;
    }
    if route.len() > 1 {
      if !is_left(cuta, cutb, prev) {
        routes_above.push((clr, route));
      } else {
        routes_below.push((clr, route));
      }
    }
  }

  Slice {
    routes_above,
    routes_below,
    a: amin,
    b: bmin,
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let seed = opts.seed;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();

  let mut routes = Vec::new();

  let mountainpadding = -30.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let min_route = 2;

  let precision = 0.21;
  let count = rng.gen_range(2, 12);
  for _j in 0..count {
    let h = rng.gen_range(3.0, 5.0);
    let peakfactor = rng.gen_range(-0.001, 0.001) * rng.gen_range(0.0, 1.0);
    let ampfactor = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);
    let ynoisefactor = rng.gen_range(0.02, 0.2);
    let yincr = 0.5
      + (rng.gen_range(-2f64, 8.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0))
      .max(0.0)
      .min(2.0);
    let amp1 = rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(0.0, 12.0) * rng.gen_range(0.0, 1.0);
    let amp3 = rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
    let offsetstrategy = rng.gen_range(0, 5);
    let center = rng.gen_range(0.2, 0.8) * width;

    let stopy = height * rng.gen_range(-0.5, 0.8);

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

        let mut xv = (h - base_y / height) * (x - center);

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
        let inside = !collides;
        if inside {
          if was_outside {
            if route.len() > min_route {
              routes.push((0, route));
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push((x, y));
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

  // slice multiple times
  let count = rng.gen_range(40, 60);
  let split = 0.0;
  let slide_effect = rng.gen_range(0.5, 4.0);
  for i in 0..count {
    let x = mix(0.0, 1.0, (i as f64 + 0.5) / (count as f64)) * width;
    let y = height * rng.gen_range(0.5, 0.8);
    let a = PI / 2.0
      + 0.2
        * rng.gen_range(-PI, PI)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
    let dx = a.cos();
    let dy = a.sin();
    let amp = 200.0;
    let left = (x - amp * dx, y - amp * dy);
    let right = (x + amp * dx, y + amp * dy);
    let slice = slice_routes(routes.clone(), left, right);
    let slide = slide_effect
      // * (0.5 - (x / width - 0.5).abs())
      * (if i % 2 == 0 { -1.0 } else { 1.0 });
    let l = euclidian_dist(slice.a, slice.b);
    let v = ((slice.b.0 - slice.a.0) / l, (slice.b.1 - slice.a.1) / l);
    let n = (v.1, -v.0);
    //routes_above.push((clr, vec![amin, bmin]));
    //routes_below.push((clr, vec![amin, bmin]));
    routes = vec![
      translate_routes(
        slice.routes_above,
        (v.0 * slide + n.0 * split, v.1 * slide + n.1 * split),
      ),
      translate_routes(
        slice.routes_below,
        (-v.0 * slide - n.0 * split, -v.1 * slide - n.1 * split),
      ),
    ]
    .concat();
  }

  let mut should_draw_line = |a, b| {
    strictly_in_boundaries(a, bound)
      && strictly_in_boundaries(b, bound)
      && passage.count(a) < 10
  };

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if i == ci {
          data = render_route_when(data, route, &mut should_draw_line);
        }
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

fn translate_routes(
  routes: Vec<(usize, Vec<(f64, f64)>)>,
  (tx, ty): (f64, f64),
) -> Vec<(usize, Vec<(f64, f64)>)> {
  routes
    .iter()
    .map(|(i, route)| {
      (*i, route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    })
    .collect()
}
