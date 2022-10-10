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
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;

  // Prepare all the random values
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();
  let min_route = 1;
  let peakfactor = rng.gen_range(-0.001, 0.002) * rng.gen_range(0.0, 1.0);
  let stopy = rng.gen_range(0.2, 0.5) * height;
  let ampfactor = rng.gen_range(0.04, 0.2);
  let ynoisefactor = rng.gen_range(0.1, 0.2);
  let xfreq = 0.005
    + rng.gen_range(0.0, 0.1)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
  let amp2 = rng.gen_range(1.0, 4.0);
  let yincr = 0.4 + rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
  let yrounding = rng.gen_range(0.5, 10.0);
  let precision = rng.gen_range(1.0, 3.0);
  let offsetstrategy = rng.gen_range(0, 5);

  let matchlength = 4.0;

  let passage_precision = rng.gen_range(0.7, 2.8);
  let mut passage = Passage::new(passage_precision, width, height);

  let mut matches = Vec::new();

  // we will use "routes" temporarily first to prepare the lines where to place matches
  let mut routes = Vec::new();
  let mut base_y = height * rng.gen_range(1.0, 5.0);
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
      let xv = (2.0 - base_y / height) * (x - width / 2.);

      let amp = height * ampfactor;
      let displacement = amp * peakfactor * (xv * xv);

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

      y = (y / yrounding).round() * yrounding;

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
      if inside {
        if was_outside {
          if route.len() > min_route {
            routes.push(route);
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
      routes.push(route);
    }
    base_y -= yincr;
  }

  // Border around the postcard
  let border_size = 3;
  let border_dist = 0.8;
  let mut route = Vec::new();
  for i in 0..border_size {
    let d = i as f64 * border_dist;
    route.push((pad + d, pad + d));
    route.push((pad + d, height - pad - d));
    route.push((width - pad - d, height - pad - d));
    route.push((width - pad - d, pad + d));
    route.push((pad + d, pad + d));
    // TODO use matches instead!
  }
  routes.push(route);

  for route in routes.clone() {
    let mut acclen = 0.0;
    let mut prevmatchp = route[0];
    let mut prev = route[0];
    passage.count(prev);
    for &p in route.iter().skip(1) {
      passage.count(p);
      let l = euclidian_dist(prev, p);
      let (x1, y1) = prev;
      let (x2, y2) = p;
      let splits = (l / passage_precision).ceil() as usize;
      let incr = l / (splits as f64 + 1.0);
      for i in 0..(splits + 1) {
        let p = i as f64 / (splits as f64);
        let x = mix(x1, x2, p);
        let y = mix(y1, y2, p);
        passage.count((x, y));
        acclen += incr;
        if acclen >= matchlength {
          let dx = x - prevmatchp.0;
          let dy = y - prevmatchp.1;
          let l = (dx * dx + dy * dy).sqrt();
          matches.push((
            prevmatchp.0,
            prevmatchp.1,
            prevmatchp.0 + matchlength * dx / l,
            prevmatchp.1 + matchlength * dy / l,
          ));
          prevmatchp = (x, y);
          acclen -= matchlength * rng.gen_range(1.0, 1.1);
        }
      }
      prev = p;
    }
  }

  let extrapad = pad + 2.0;

  for _i in 0..rng.gen_range(1, 200000) {
    let x = rng.gen_range(extrapad, width - extrapad);
    let miny = height_map[(x / precision) as usize % height_map.len()];
    let maxy = height - extrapad;
    if miny >= maxy {
      continue;
    }
    let y = rng.gen_range(miny, maxy);
    let v = rng.gen_range(-PI, PI)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);

    let a = if y / height
      > (1.0 + stopy / height) / 2.0 + 0.3 * rng.gen_range(-1.0, 1.0)
    {
      //rng.gen_bool(y / height) {
      v
    } else {
      PI / 2.0 + v
    };
    let dx = a.cos();
    let dy = a.sin();
    let x1 = x - 0.5 * dx * matchlength;
    let x2 = x + 0.5 * dx * matchlength;
    let y1 = y - 0.5 * dy * matchlength;
    let y2 = y + 0.5 * dy * matchlength;
    if x1 < extrapad
      || x2 < extrapad
      || x1 > width - extrapad
      || x2 > width - extrapad
      || y1 < extrapad
      || y2 < extrapad
      || y1 > height - extrapad
      || y2 > height - extrapad
    {
      continue;
    }

    let l = euclidian_dist((x1, y1), (x2, y2));
    let splits = (l / passage_precision).ceil() as usize;
    let mut free_space = true;
    let mut points = Vec::new();
    for i in 0..(splits + 1) {
      let p = i as f64 / (splits as f64);
      let x = mix(x1, x2, p);
      let y = mix(y1, y2, p);
      points.push((x, y));
      if passage.get((x, y)) > 0 {
        free_space = false;
        break;
      }
    }
    if free_space {
      for p in points {
        passage.count(p);
      }
      routes.push(vec![(x1, y1), (x2, y2)]); // tmp
      matches.push((x1, y1, x2, y2));
    }
  }

  let mut toproutes = Vec::new();
  let mut woodroutes = Vec::new();
  let wood_dist = 0.3;
  let top_dist = 0.5;
  let topl = 0.8;
  let disp = 0.1;
  for (x1, y1, x2, y2) in matches {
    let x1 = x1 + rng.gen_range(-disp, disp);
    let x2 = x2 + rng.gen_range(-disp, disp);
    let y1 = y1 + rng.gen_range(-disp, disp);
    let y2 = y2 + rng.gen_range(-disp, disp);
    let diff = (x2 - x1, y2 - y1);
    let v = (diff.0 / matchlength, diff.1 / matchlength);
    let dy = v.0 * top_dist / 2.0;
    let dx = -v.1 * top_dist / 2.0;
    toproutes.push(vec![
      (x1 - dx, y1 - dy),
      (x1 - dx + topl * v.0, y1 - dy + topl * v.1),
      (x1 + dx + topl * v.0, y1 + dy + topl * v.1),
      (x1 + dx, y1 + dy),
      (x1, y1),
      (x1 + topl * v.0, y1 + topl * v.1),
    ]);
    let dy = v.0 * wood_dist / 2.0;
    let dx = -v.1 * wood_dist / 2.0;
    woodroutes.push(vec![
      (x1 - dx + topl * v.0, y1 - dy + topl * v.1),
      (x2 - dx, y2 - dy),
      (x2 + dx, y2 + dy),
      (x1 + dy + topl * v.0, y1 + dy + topl * v.1),
    ]);
  }

  // Make the SVG
  vec![("#f90", woodroutes), ("#a00", toproutes)]
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
