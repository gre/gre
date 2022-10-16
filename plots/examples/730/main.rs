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
  #[clap(short, long, default_value = "420.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

pub struct Frame {
  index: usize,
  pos: (f64, f64),
  rot: f64,
  size: f64,
}

fn cell(
  seed: f64,
  width: f64,
  height: f64,
  offset: usize,
) -> Vec<Vec<(f64, f64)>> {
  // Prepare all the random values
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let stopy = rng.gen_range(0.1, 0.4) * height;
  let passage_threshold = 9;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mut height_map: Vec<f64> = Vec::new();

  let precision = rng.gen_range(0.2, 0.3);
  let count = rng.gen_range(2, 8);
  for j in 0..3 {
    let peakfactor = rng.gen_range(-0.001, 0.002)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
    let ampfactor = rng.gen_range(0.03, 0.1);
    let ynoisefactor = rng.gen_range(0.02, 0.2);
    let yincr = 0.4 + rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(1.0, 12.0);
    let offsetstrategy = rng.gen_range(0, 5);

    let mut passage = Passage::new(0.5, width, height);
    let stopy = mix(height, stopy, (j as f64 / (count as f64)) * 0.7 + 0.3);

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height * 5.0;
    let mut miny = height;
    loop {
      if miny < stopy {
        break;
      }

      let mut route = Vec::new();
      let mut x = 0.0;
      let mut was_outside = true;
      loop {
        if x > width {
          break;
        }
        let xv = (4.0 - base_y / height) * (x - width / 2.);

        let amp = height * ampfactor;
        let mut y = base_y;

        if offsetstrategy == 0 {
          y += amp * peakfactor * xv * xv;
        }

        y += -amp
          * perlin
            .get([
              //
              xv * 0.004 + 9.9,
              y * 0.02 - 3.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  -seed * 7.3,
                  8.3 + xv * 0.015,
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
            8.3 + xv * 0.008,
            88.1 + y * ynoisefactor,
            seed * 97.3,
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
              xv * 0.015 - 88.33,
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
            6666.6 + seed * 1.3,
            8.3 + xv * 0.5,
            88.1 + y * 0.5,
          ]);

        if offsetstrategy == 4 {
          y += amp * peakfactor * xv * xv;
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
        let inside = !collides && 0. < x && x < width && 0. < y && y < height;
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

    // calculate a moving average to smooth the stick men positions
    let smooth = 16;
    let sf = smooth as f64;
    let mut sum = 0.0;
    let mut acc = Vec::new();
    let mut smooth_heights: Vec<(f64, f64, f64)> = Vec::new();
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

    let gif_frames = 10;
    let gif_ratio = 420. / 504.;
    let size = rng.gen_range(6.0, 10.0);
    let count = rng.gen_range(4, 16);

    // Calculate the "frames" that are all the rectangles to put images frame on

    let mut frames = Vec::new();
    for i in 0..count {
      let x = ((i as f64 + offset as f64 / 10.) / (count as f64)) * width;
      let hindex = (x / precision) as usize;
      let p = smooth_heights[hindex % smooth_heights.len()];
      if p.1 > height {
        continue;
      }
      let rot = p.2 * 0.8;
      let pos = (p.0, p.1);
      frames.push(Frame {
        index: (i + offset) % gif_frames,
        pos,
        rot,
        size,
      });
    }

    for f in frames {
      let get_color =
        image_gif_get_color("images/YoungGrossHoopoe.gif", f.index).unwrap();

      // 4 corners of the image to project
      let x1 = f.pos.0 - f.size / 2.0;
      let x2 = f.pos.0 + f.size / 2.0;
      let y1 = f.pos.1 - 0.9 * f.size / gif_ratio;
      let y2 = f.pos.1 + 0.1 * f.size / gif_ratio;

      // stroke a lot of lines to plot the image
      let res = (f.size / 0.2) as usize;
      for x in 0..res {
        let mut route = Vec::new();
        for y in 0..res {
          let v = (x as f64 / (res as f64), y as f64 / (res as f64));
          let p = (mix(x1, x2, v.0), mix(y1, y2, v.1));
          let q = (p.0 - f.pos.0, p.1 - f.pos.1);
          let p = p_r(q, f.rot);
          let p = (p.0 + f.pos.0, p.1 + f.pos.1);
          let c = get_color(v);
          if c.0 < 0.5 {
            route.push(p);
          } else {
            if route.len() > 0 {
              routes.push(route)
            }
            route = Vec::new();
          }
        }
        if route.len() > 0 {
          routes.push(route);
        }
      }
    }
  }

  // External frame to around the whole piece
  let d = 0.0;
  let pad = 0.0;
  routes.push(vec![
    (pad + d, pad + d),
    (pad + d, height - pad - d),
    (width - pad - d, height - pad - d),
    (width - pad - d, pad + d),
    (pad + d, pad + d),
  ]);

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = 20.0;
  let divx = 2;
  let divy = 5;
  let w = (opts.width - 2.0 * pad) / (divx as f64);
  let h = (opts.height - 2.0 * pad) / (divy as f64);

  let mut all = Vec::new();
  for xi in 0..divx {
    for yi in 0..divy {
      let offset = yi + xi * divy;
      let dx = pad + xi as f64 * w;
      let dy = pad + yi as f64 * h;
      let mut routes = cell(opts.seed, w, h, offset);
      routes = routes
        .iter()
        .map(|route| route.iter().map(|&p| (p.0 + dx, p.1 + dy)).collect())
        .collect();
      all.push(routes);
    }
  }

  let routes = all.concat();
  // Make the SVG
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
