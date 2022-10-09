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

  // Prepare all the random values
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let peakfactor = rng.gen_range(-0.0005, 0.0) * rng.gen_range(0.0, 1.0);
  let stopy = rng.gen_range(0.1, 0.3) * height;
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
  let smooth = 50;
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

  // Algorith will now try to find the best bridge placement
  // Here are configurations:
  let tries = 100;
  let jump = 5; // how much "tops" to skip between tries. to not consider just the best tops.

  // - maximize the area under the bridge (the reason we make a bridge is there is a precipice)
  let min_area_per_dx = rng.gen_range(15.0, 30.0);

  // - try to find a certain length of the bridge:
  let min_bridge_width = 0.1 * width;
  let max_bridge_width = 0.7 * width;

  // - have an horizontal slope as much as possible:
  let min_ratio_threshold = 20.0; // dx/dy ratio of the slope to start accepting

  for t in 0..tries {
    let i = (t * jump) % smooth_heights.len();
    let a = smooth_heights[i];
    let maybe_b = smooth_heights.iter().find(|&&b| {
      let d = (a.0 - b.0).abs();
      if min_bridge_width < d && d < max_bridge_width {
        let dx = (a.0 - b.0).abs();
        let dy = (a.1 - b.1).abs();
        if dy < 1.0 || dx / dy > min_ratio_threshold {
          let left = if a.0 < b.0 { a } else { b };
          let right = if a.0 > b.0 { a } else { b };
          let leftxi = (left.0 / precision) as usize;
          let rightxi = (right.0 / precision) as usize;
          let mut area = 0.0;
          let l = (rightxi - leftxi) as f64;
          for xi in leftxi..rightxi {
            let xp = (xi - leftxi) as f64 / l;
            let liney = mix(left.1, right.1, xp);
            let dy = height_map[xi] - liney;
            if dy < 0.0 {
              area += -dy * dy; // square of the era if it's traversing the bridge
            } else {
              area += dy;
            }
          }
          area *= precision;
          if area / dx > min_area_per_dx {
            return true;
          }
        }
      }
      return false;
    });

    // We may have found our bridge, from a to b:
    if let Some(&b) = maybe_b {
      // Build the Bridge Structure
      let doubledy = 0.7;
      let bridge_height_min = rng.gen_range(2.0, 6.0);
      let bridge_height_amp = rng.gen_range(10.0, 50.0);
      // bridge double line
      routes.push(vec![a, b]);
      routes.push(vec![(a.0, a.1 - doubledy), (b.0, b.1 - doubledy)]);
      let xcenter = (a.0 + b.0) / 2.0;
      let splits = ((b.0 - a.0).abs() / 5.0) as usize; // nb of triangles of the bridge
      let mut route = Vec::new(); // triangles of the bridge structure
      let mut route2 = Vec::new(); // curve of the arche
      let mut route3 = Vec::new(); // second curve (doubled)
      for i in 0..splits {
        let p = i as f64 / ((splits - 1) as f64);
        let x = mix(a.0, b.0, p);
        let y = mix(a.1, b.1, p);
        let dy = bridge_height_min
          + bridge_height_amp
            * (2.0 * ((x - xcenter) / (b.0 - a.0)).abs()).powf(2.0);
        let y2 = (y + dy)
          .min(height - pad)
          .min(height_map[(x / precision) as usize] + rng.gen_range(0.0, 10.0));
        route.push((x, y));
        route.push((x, y2));
        route2.push((x, y2));
        route3.push((x, y2 + doubledy));
      }
      routes.push(route);
      routes.push(route2);
      routes.push(route3);

      // Add our train
      let headx = mix(a.0, b.0, rng.gen_range(0.1, 0.9));
      let trainh = rng.gen_range(1.0, 2.0); // height scale reference for train
      let basetrainy = |x| -1.0 + mix(a.1, b.1, (x - a.0) / (b.0 - a.0));
      let carriage_dist = 0.5;
      let lines_dist = 0.2;

      // railway
      let x1 = headx;
      let x2 = headx - trainh * 3.0;
      let y1 = basetrainy(x1);
      let y2 = basetrainy(x2);
      // base
      let mut dy = 0.0;
      loop {
        if dy > trainh {
          break;
        }
        routes.push(vec![(x1, y1 - dy), (x2, y2 - dy)]);
        dy += lines_dist;
      }
      // chimney
      let mut dx = 0.0;
      let chimneyx = mix(x1, x2, rng.gen_range(0.2, 0.3));
      let chimneyw = rng.gen_range(0.4, 0.6) * trainh;
      let chimneyh = rng.gen_range(0.6, 0.8) * trainh;
      let chimneyytop = basetrainy(chimneyx) - trainh - chimneyh;
      loop {
        if dx > chimneyw {
          break;
        }
        let x = chimneyx - chimneyw / 2.0 + dx;
        let y = basetrainy(x);
        routes.push(vec![(x, y - trainh), (x, y - trainh - chimneyh)]);
        dx += lines_dist;
      }

      // coal carriage

      let w = trainh * 1.8;
      let h = 0.7 * trainh;
      let x1 = x2 - carriage_dist;
      let x2 = x1 - w;
      let y1 = basetrainy(x1);
      let y2 = basetrainy(x2);
      // base
      let mut dy = 0.0;
      loop {
        if dy > h {
          break;
        }
        routes.push(vec![(x1, y1 - dy), (x2, y2 - dy)]);
        dy += lines_dist;
      }
      // coal
      let mut dy = 0.0;
      let coalx = mix(x1, x2, 0.5);
      let coalw = rng.gen_range(0.7, 0.8) * w;
      let coalh = rng.gen_range(0.2, 0.3) * trainh;
      loop {
        if dy > coalh {
          break;
        }
        let x1 = coalx - coalw / 2.0;
        let x2 = coalx + coalw / 2.0;
        let y1 = basetrainy(x1) - h - dy;
        let y2 = basetrainy(x2) - h - dy;
        routes.push(vec![(x1, y1), (x2, y2)]);
        dy += lines_dist;
      }

      // carriages

      let mut x1;
      let mut x2 = x2;
      for _i in 0..rng.gen_range(3, 10) {
        let w = 5.0 * trainh;
        let h = trainh;
        x1 = x2 - carriage_dist;
        x2 = x1 - w;

        let x2_is_on_bridge = a.0.min(b.0) < x2 && x2 < a.0.max(b.0);
        if !x2_is_on_bridge {
          break;
        }

        let y1 = basetrainy(x1);
        let y2 = basetrainy(x2);
        // base
        let mut dy = 0.0;
        loop {
          if dy > h {
            break;
          }
          routes.push(vec![(x1, y1 - dy), (x2, y2 - dy)]);
          dy += lines_dist;
        }
      }

      // smoke

      for _j in 0..8 {
        let mut ang = -PI / 2.0;
        let incr = 0.3;
        let mut angdelta = 0.3;
        let mut amp = 0.2;
        let mut route = Vec::new();
        let mut lastp = (chimneyx, chimneyytop);
        let count = rng.gen_range(20, 400);
        for i in 0..count {
          let v = rng.gen_range(-1.0, 1.0);
          let disp = (
            v * amp * (ang + PI / 2.0).cos(),
            v * amp * (ang + PI / 2.0).sin(),
          );
          let p = (
            lastp.0 + incr * ang.cos() + disp.0,
            lastp.1 + incr * ang.sin() + disp.1,
          );
          if p.0 < pad
            || p.0 > width - pad
            || p.1 < pad
            || p.1 > height - pad
            || p.1 > height_map[(p.0 / precision) as usize % height_map.len()]
          {
            break;
          }
          route.push(p);
          ang -= angdelta;
          angdelta /= 1.3;
          amp = (amp * 1.01 + 0.01).min(0.7);
          lastp = p;
          if route.len() > 1 && rng.gen_bool(0.5) {
            if rng.gen_bool(0.7 - 0.6 * i as f64 / (count as f64)) {
              // randomly drop
              routes.push(route);
            }
            route = Vec::new();
          }
        }
        routes.push(route);
      }

      break; // we found our bridge, stops
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
