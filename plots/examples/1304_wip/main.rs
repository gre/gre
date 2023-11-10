use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "420.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "2")]
  pub divx: usize,
  #[clap(short, long, default_value = "4")]
  pub divy: usize,
  #[clap(short, long, default_value = "2")]
  pub page: usize,
  #[clap(short, long, default_value = "72")]
  pub total: usize,
  #[clap(short, long, default_value = "625.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "")]
  pub testing_seeds: String,
}

fn cell(
  seed: f64,
  width: f64,
  height: f64,
  offset: usize,
  total: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  if offset >= total * 2 {
    return vec![];
  }

  let border = 3.0;
  // Prepare all the random values
  let mut rng = rng_from_seed(seed);

  let gridw = 4;
  let gridh = 2;
  let pager_size = 1.5;
  let pager_pad = border;
  let pager_ratio_scale = 1.0;
  let pgr = |xf, yf| {
    (
      pager_size * xf * pager_ratio_scale + width / 2.0
        - (pager_size * pager_ratio_scale * gridw as f64) / 2.0,
      height + pager_size * yf - pager_pad,
    )
  };

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let fillingbase = WormsFilling::rand(&mut rng);

  let border = 0.5;
  let borderw = 2.0;
  let lmin = 5.0;
  let lmax = 10.0;
  let border_f = |ox: f64, oy: f64| {
    let x = (width - ox).min(ox);
    let y = (height - oy).min(oy);
    let p = (offset as f64 / (total * 2) as f64).min(1.0);
    let phase = if ox < width / 2.0 && x < border + borderw + 1.0
      || oy > height / 2.0 && y < border + borderw + 1.0
    {
      1.0 - p
    } else {
      p
    };

    let rep = |x: f64, m: f64| ((x + m * 99999.) % m - 0.5 * m).abs();

    if x.min(y) - border - borderw < 0.0
      && (x - border - borderw - lmax < 0.0
        || rep(ox - width * phase, width) < borderw / 2.0)
      && (y - border - borderw - lmin < 0.0
        || rep(oy - height * phase, width) < borderw / 2.0)
    {
      4.0
    } else {
      0.0
    }
  };

  for (off, tot, dx, density, folder, fromclr, toclr, hassecondgold) in vec![
    (
      offset,
      total,
      0.33,
      3.0,
      "/Users/gre/Documents/ledger-nanox-crop-anim-2",
      0.7,
      0.1,
      false,
    ),
    (
      offset,
      total,
      -0.33,
      4.0,
      "/Users/gre/Documents/ledger-nanox-crop-anim",
      0.8,
      0.2,
      true,
    ),
  ] {
    let ind = off / tot;
    let mut off = off % tot;
    if ind % 2 == 0 {
      // mirror
      off = tot - 1 - off;
    }
    let index = off % tot + 1;
    let electrics = index;
    let path = format!("{}/{:03}.png", folder, index);

    let get_image = image_get_color(path.as_str()).unwrap();

    let bound = (border, border, width - border, height - border);
    let ratio: f64 = (bound.2 - bound.0) / (bound.3 - bound.1);

    let f = |x: f64, y: f64| {
      let p = (
        ratio * (x - bound.0) / (bound.2 - bound.0) + (1.0 - ratio) * 0.5 + dx,
        (y - bound.1) / (bound.3 - bound.1),
      );
      let centerd = euclidian_dist(p, (0.5, 0.5));
      let mulc = smoothstep(0.6, 0.4, centerd);
      let c = mulc * smoothstep(fromclr, toclr, grayscale(get_image(p)));

      4.0 * c
    };

    let clr = |_rts: &Vec<(f64, f64)>| 0;

    let mut filling = fillingbase.clone();
    filling.step = rng.gen_range(1.0, 3.0);
    filling.min_l = 4;
    filling.max_l = 10;
    routes.extend(filling.fill(&mut rng, &f, bound, &clr, 50));
    filling.step = 0.9;
    filling.max_l = 15;
    routes.extend(filling.fill(&mut rng, &f, bound, &clr, 100));
    filling.step = 0.45;
    filling.max_l = rng.gen_range(15, 30);
    let its = rng.gen_range(200, 500);
    routes.extend(filling.fill(&mut rng, &f, bound, &clr, its));

    if hassecondgold {
      let f = |x, y| {
        let p = (
          ratio * (x - bound.0) / (bound.2 - bound.0)
            + (1.0 - ratio) * 0.5
            + dx,
          (y - bound.1) / (bound.3 - bound.1),
        );
        let (_r, g, _b) = get_image(p);
        density * smoothstep(0.4, 1.0, g).powf(2.0)
      };
      let clr = |rts: &Vec<(f64, f64)>| rts.len() % 2;
      routes.extend(filling.fill(&mut rng, &f, bound, &clr, 50));
    }

    filling.rot = PI / rng.gen_range(2.0, 6.0);
    filling.step = 2.0;
    filling.min_l = 4;
    filling.max_l = 20;
    routes.extend(filling.fill(&mut rng, &f, bound, &clr, electrics));
  }

  let clr = |_rts: &Vec<(f64, f64)>| 0;
  let filling = fillingbase.clone();
  let border = 0.5;
  routes.extend(filling.fill(
    &mut rng,
    &border_f,
    (border, border, width - border, height - border),
    &clr,
    1000,
  ));

  // pager
  let mut pager = Vec::new();
  for xj in vec![0, gridw] {
    pager.push(vec![pgr(xj as f64, 0.0), pgr(xj as f64, gridh as f64)]);
  }
  for yj in vec![0, gridh] {
    pager.push(vec![pgr(0.0, yj as f64), pgr(gridw as f64, yj as f64)]);
  }
  for yi in 0..gridh {
    for xi in 0..gridw {
      let i = gridw * gridh - 1 - (xi + yi * gridw);
      let mask = 2usize.pow(i);
      let fill = offset & mask != 0;
      if fill {
        let lines = 5;
        for l in 0..lines {
          let f = (l as f64 + 0.5) / (lines as f64);
          pager.push(vec![
            pgr(xi as f64, yi as f64 + f),
            pgr(xi as f64 + 1.0, yi as f64 + f),
          ]);
        }
      }
    }
  }

  for r in pager {
    routes.push((1, r));
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = opts.pad;
  let divx = opts.divx;
  let divy = opts.divy;
  let pageoff = opts.page * divx * divy;
  let w = (opts.width) / (divx as f64);
  let h = (opts.height) / (divy as f64);
  let total = opts.total;

  let testing_seeds = Some(
    opts
      .testing_seeds
      .split(",")
      .filter(|s| !s.is_empty())
      .map(|s| s.parse().unwrap())
      .collect::<Vec<f64>>(),
  )
  .and_then(|v| if v.is_empty() { None } else { Some(v) });

  let indexes: Vec<(usize, usize)> = (0..divx)
    .flat_map(|xi| (0..divy).map(|yi| (xi, yi)).collect::<Vec<_>>())
    .collect();

  let all = indexes
    .par_iter()
    .map(|&(xi, yi)| {
      let offset = yi + xi * divy;
      let dx = pad + xi as f64 * w;
      let dy = pad + yi as f64 * h;
      if let Some(seed) = match testing_seeds.clone() {
        None => Some(opts.seed),
        Some(array) => array.get(offset).map(|&o| o),
      } {
        let mut routes =
          cell(seed, w - 2.0 * pad, h - 2.0 * pad, pageoff + offset, total);
        routes = routes
          .iter()
          .map(|(ci, route)| {
            let r: (usize, Vec<(f64, f64)>) =
              (*ci, route.iter().map(|&p| (p.0 + dx, p.1 + dy)).collect());
            r
          })
          .collect();
        return routes;
      }
      vec![]
    })
    .collect::<Vec<_>>();

  let routes = all.concat();
  let colors = vec!["#fff", "gold"];
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
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

#[derive(Clone)]
// homemade implementation of a filling technique that will spawn random worms that eat the space to colorize it!
struct WormsFilling {
  rot: f64,
  step: f64,
  straight: f64,
  min_l: usize,
  max_l: usize,
  decrease_value: f64,
  search_max: usize,
  min_weight: f64,
  freq: f64,
  seed: f64,
}
impl WormsFilling {
  // new
  fn rand<R: Rng>(rng: &mut R) -> Self {
    let seed = rng.gen_range(-999., 999.);
    let rot = PI / rng.gen_range(1.0, 2.0);
    let step = 0.45;
    let straight = rng.gen_range(0.0, 0.1);
    let min_l = 5;
    let max_l = 20;
    let decrease_value = 1.;
    let search_max = 500;
    let min_weight = 1.;
    let freq = 0.05;
    Self {
      rot,
      step,
      straight,
      min_l,
      max_l,
      decrease_value,
      search_max,
      min_weight,
      freq,
      seed,
    }
  }

  fn fill<R: Rng>(
    &self,
    rng: &mut R,
    f: &dyn Fn(f64, f64) -> f64,
    bound: (f64, f64, f64, f64),
    clr: &dyn Fn(&Vec<(f64, f64)>) -> usize,
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let perlin = Perlin::new();
    let w = bound.2 - bound.0;
    let h = bound.3 - bound.1;
    let precision = 0.4;
    if w <= 2. * precision || h <= 2. * precision {
      return routes;
    }
    let mut map = WeightMap::new(w, h, 0.4);

    map.fill_fn(&|p| f(p.0 + bound.0, p.1 + bound.1));

    let seed = self.seed;
    let rot = self.rot;
    let step = self.step;
    let straight = self.straight;
    let min_l = self.min_l;
    let max_l = self.max_l;
    let decrease_value = self.decrease_value;
    let search_max = self.search_max;
    let min_weight = self.min_weight;
    let freq = self.freq;

    let mut bail_out = 0;

    for _i in 0..iterations {
      let top = map.search_weight_top(rng, search_max, min_weight);
      if top.is_none() {
        bail_out += 1;
        if bail_out > 10 {
          break;
        }
      }
      if let Some(o) = top {
        let angle = perlin.get([seed, freq * o.0, freq * o.1]);

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
            let points: Vec<(f64, f64)> = rdp(&route, 0.05);
            // remap
            let rt = points
              .iter()
              .map(|&p| (p.0 + bound.0, p.1 + bound.1))
              .collect::<Vec<_>>();
            let c = clr(&rt);
            routes.push((c, rt));
          }
        }
      }
    }

    routes
  }
}

// data model that stores values information in 2D
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

  // apply a gaussian filter to the weights around the point p with a given radius
  fn decrease_weight_gaussian(
    &mut self,
    p: (f64, f64),
    radius: f64,
    value: f64,
  ) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = ((x - radius).floor().max(0.) as usize).min(self.w);
    let y0 = ((y - radius).floor().max(0.) as usize).min(self.h);
    let x1 = ((x + radius).ceil().max(0.) as usize).min(self.w);
    let y1 = ((y + radius).ceil().max(0.) as usize).min(self.h);
    if x0 >= self.w || y0 >= self.h {
      return;
    }
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

  // FIXME we could optim this by keeping track of tops and not searching too random
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
