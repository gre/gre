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
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "148.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  // Prepare all the random values
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let precision = rng.gen_range(0.1, 0.3);
  let mut passage = Passage::new(0.5, width, height);
  let passage_threshold = 5;

  let mut height_map: Vec<f64> = Vec::new();

  let iterations =
    (1. + rng.gen_range(1., 50.) * rng.gen_range(0., 1.)) as usize;

  for _g in 0..iterations {
    let peakfactor = rng.gen_range(-0.005, 0.005)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
    let stopy =
      (0.5 + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0)) * height;
    let ampfactor = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
    let ynoisefactor = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
    let yincr = 1.0 + rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
    let xfreq = rng.gen_range(0.0, 0.04) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(1.0, 6.0) * rng.gen_range(0.0, 1.0);
    let offsetstrategy = rng.gen_range(0, 5);
    let offsetx = rng.gen_range(-0.5, 0.5)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height * 5.0;
    let mut miny = height;
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
        let xv = (4.0 - base_y / height) * (x - width / 2. + offsetx * width);

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
        let in_bounds =
          pad < x && x < width - pad && pad < y && y < height - pad;
        let inside = !collides && in_bounds;
        if inside && passage.get((x, y)) < passage_threshold {
          if was_outside {
            if route.len() > min_route {
              routes.push(route);
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push(round_point((x, y), 0.01));
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
    let smooth = rng.gen_range(20, 40);
    let sf = smooth as f64;
    let mut sum = 0.0;
    let mut acc = Vec::new();
    let mut smooth_heights = Vec::new();
    for (i, h) in height_map.iter().enumerate() {
      if acc.len() == smooth {
        let avg = sum / sf;
        let xtheoric = (i as f64 - sf / 2.0) * precision;
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

  let delta = 0.5 + rng.gen_range(0.5, 2.0) * rng.gen_range(0.0, 1.0);
  let p = pad + border_size as f64 * border_dist - delta;
  let mut x = p;
  let mut route = vec![];
  let mut reverse = false;
  loop {
    if x > width - p {
      break;
    }
    let a = (x, p);
    let b = (x, height - p);
    if reverse {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    x += delta;
    reverse = !reverse;
  }
  routes.push(route);

  // Make the SVG
  vec![("black", routes, 0.35)]
    .iter()
    .map(|(color, routes, stroke_width)| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, *stroke_width, data));
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
