use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes(
  input_routes: &Vec<(usize, Vec<(f64, f64)>)>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  // locate the intersection where inside and outside cross
  let search = |inside: (f64, f64), outside: (f64, f64), n| {
    let mut a = inside;
    let mut b = outside;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if is_outside(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for (c, input_route) in input_routes.iter() {
    if input_route.len() < 2 {
      continue;
    }
    let clr = *c;

    let mut prev = input_route[0];
    let mut prev_is_outside = is_outside(prev);
    let mut route = vec![];
    if !prev_is_outside {
      // prev is not to crop. we can start with it
      route.push(prev);
    }

    for &p in input_route.iter().skip(1) {
      // we iterate in small steps to detect any interruption
      let static_prev = prev;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let d = (dx * dx + dy * dy).sqrt();
      let vx = dx / d;
      let vy = dy / d;
      let iterations = (d / stepping).ceil() as usize;
      let mut v = 0.0;
      for _i in 0..iterations {
        v = (v + stepping).min(d);
        let q = (static_prev.0 + vx * v, static_prev.1 + vy * v);
        let q_is_outside = is_outside(q);
        if prev_is_outside != q_is_outside {
          // we have a crossing. we search it precisely
          let intersection = if prev_is_outside {
            search(q, prev, dichotomic_iterations)
          } else {
            search(prev, q, dichotomic_iterations)
          };

          if q_is_outside {
            // we close the path
            route.push(intersection);
            if route.len() > 1 {
              // we have a valid route to accumulate
              routes.push((clr, route));
            }
            route = vec![];
          } else {
            // we open the path
            route.push(intersection);
          }
          prev_is_outside = q_is_outside;
        }

        prev = q;
      }

      // prev should be == p
      if !prev_is_outside {
        // prev is not to crop. we can start with it
        route.push(p);
      }
    }

    if route.len() > 1 {
      // we have a valid route to accumulate
      routes.push((clr, route));
    }
  }

  routes
}

struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
}

impl PaintMask {
  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    }
  }

  fn is_painted(&self, point: (f64, f64)) -> bool {
    // check out of bounds
    if point.0 <= 0.0
      || point.0 >= self.width
      || point.1 <= 0.0
      || point.1 >= self.height
    {
      return false;
    }
    let precision = self.precision;
    let width = self.width;
    let x = (point.0 / precision) as usize;
    let y = (point.1 / precision) as usize;
    let wi = (width / precision) as usize;
    self.mask[x + y * wi]
  }

  fn paint_circle(&mut self, center: (f64, f64), radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let minx = ((center.0 - radius) / precision) as usize;
    let miny = ((center.1 - radius) / precision) as usize;
    let maxx = ((center.0 + radius) / precision) as usize;
    let maxy = ((center.1 + radius) / precision) as usize;
    let wi = (width / precision) as usize;
    let r2 = radius * radius;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        let dx = point.0 - center.0;
        let dy = point.1 - center.1;
        if dx * dx + dy * dy < r2 {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
}

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "500.0")]
  pub width: f64,
  #[clap(short, long, default_value = "500.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "137.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound: (f64, f64, f64, f64) = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);

  let mut mask = PaintMask::new(0.1, width, height);

  let min = rng.gen_range(3.0, 4.0);
  let max = min + rng.gen_range(10.0, 30.0);
  let internal_pad = pad + max * 2.0;
  let diff = min * 0.7;

  // we use perlin noise for the randomness on context of this 'Noise' theme
  let perlin = Perlin::new();
  let f = rng.gen_range(1.0, 10.0);
  let f2 = rng.gen_range(1.0, 10.0);
  let f3 = rng.gen_range(1.0, 10.0);
  let amp2 = rng.gen_range(1.0, 20.0) * rng.gen_range(0.2, 1.0);
  let amp3 = rng.gen_range(1.0, 20.0) * rng.gen_range(0.2, 1.0);

  let heff: f64 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 10.0);

  let base = 0.8 + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0);
  let mul = 1.5 + rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0);

  let div = rng.gen_range(0.0, 0.5);
  let oscamp = 0.8;
  let oscfreq = 50.0;

  // centers of the squares
  let mut centers = vec![];
  let mut x = internal_pad;
  loop {
    if x > width - internal_pad {
      break;
    }
    let mut y = internal_pad;
    loop {
      if y > height - internal_pad {
        break;
      }
      let dx = x - width / 2.0;
      let dy = y - height / 2.0;
      let distc = (dx * dx + dy * dy).sqrt();
      let horizdist = (y - height / 2.0).abs() / (height / 2.0);

      let n = perlin.get([
        3.1 * opts.seed
          + 7.6
          + amp2
            * perlin.get([
              f2 * x / width,
              f2 * y / width,
              opts.seed / 7.7
                + amp3
                  * perlin.get([
                    f3 * x / width,
                    opts.seed / 3.3,
                    f3 * y / width,
                  ]),
            ]),
        f * x / width,
        f * y / width + horizdist * heff * distc / width,
      ]) - oscamp * (oscfreq * (distc / width)).cos().powf(4.0);
      let s = n + div * rng.gen_range(-1.0, 1.0);
      centers.push(((x, y), n, s));
      y += diff;
    }
    x += diff;
  }
  rng.shuffle(&mut centers);
  centers.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
  centers.reverse();

  let mut routes = Vec::new(); // all the paths to draw are stored here
  for (c, n, _) in centers {
    /*
    if c.1 > height / 2.0 {
      n = 1.0 - n;
    }
    */

    let v = base + mul * n;
    let v = v.max(0.0).min(1.0);

    let size = mix(max, min, v);
    let mut all = vec![];

    let route = circle_route(c, size, (10.0 + 2. * size) as usize);

    let clr = 0;

    // we draw the square border
    if v > 0.0 {
      all.push((clr, route.clone()));
    }

    // we sometimes fill it completely with zig-zag lines
    if v > 0.01 {
      let incr = mix(2.0, 0.4, v);
      let route = spiral_optimized(c.0, c.1, size, incr, 0.1);
      all.push((clr, route));
    }

    // used for collision detection
    let is_outside =
      |p: (f64, f64)| !strictly_in_boundaries(p, bound) || mask.is_painted(p);
    routes.extend(clip_routes(&all, &is_outside, 1., 4));

    mask.paint_circle(c, size);
  }

  vec!["black"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.3, data));
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
