use clap::*;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn bounce(
  p: (f64, f64),
  angle: f64,
  force: f64,
  gravity: f64,
  friction: f64,
  bounce_factor: f64,
  clr: usize,
  cycles: usize,
  circler: f64,
  bounds: (f64, f64, f64, f64),
  mask: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  // we will simulate gravity and bounce a circle on the floor
  // with circle_route(center, r, count)
  let mut p = p;
  let floory = bounds.3 - circler;
  if p.1 > floory {
    p.1 = floory;
  }
  let mut vel = (angle.cos() * force, angle.sin() * force);
  let mut positions = vec![];
  for _i in 0..cycles {
    if p.1 > floory {
      vel.1 = -vel.1 * bounce_factor;
      p.1 = floory;
      // break if the force is too small
      let curforce = (vel.0 * vel.0 + vel.1 * vel.1).sqrt();
      if curforce < 0.1 {
        break;
      }
    }
    if p.0 < bounds.0 + circler {
      p.0 = bounds.0 + circler;
      vel.0 = -vel.0;
    }
    if p.0 > bounds.2 - circler {
      p.0 = bounds.2 - circler;
      vel.0 = -vel.0;
    }

    positions.push(p);

    p.0 += vel.0;
    p.1 += vel.1;

    // apply gravity
    vel.1 += gravity;

    // apply friction
    let curforce = (vel.0 * vel.0 + vel.1 * vel.1).sqrt();
    let newforce = (curforce - friction).max(0.0);
    if newforce <= 0.0 {
      break;
    }
    let ang = vel.1.atan2(vel.0);
    vel = (newforce * ang.cos(), newforce * ang.sin());
  }

  positions.reverse();
  for (i, &p) in positions.iter().enumerate() {
    let mut rts = vec![(clr, circle_route(p, circler, 50))];
    if i == 0 {
      rts.push((clr, spiral_optimized(p.0, p.1, circler, 2.0, 0.1)));
    }
    let is_outside =
      |p: (f64, f64)| mask.is_painted(p) || out_of_boundaries(p, bounds);
    rts = clip_routes_with_colors(&rts, &is_outside, 0.3, 10);
    routes.extend(rts);
    mask.paint_circle(p.0, p.1, circler);
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let bounds = (pad, pad, width - pad, height - pad);

  let mut routes = vec![];
  let mut mask = PaintMask::new(0.2, width, height);

  let mut rng = rng_from_seed(opts.seed);

  let count = (2. + rng.gen_range(0., 40.) * rng.gen_range(0., 1.)) as usize;
  let maxforce = rng.gen_range(0.0, 5.0);
  let m = rng.gen_range(0.0, 1.0);
  let gravity = mix(0.05, 0.15, m);
  let friction = mix(0.01, 0.04, 1. - m);

  for i in 0..count {
    let cycles = rng.gen_range(10, 500);
    let circler = 8.0
      + rng.gen_range(-5.0, 10.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
    let (origin, ang) = if rng.gen_bool(0.5) {
      (
        (
          pad + circler,
          height
            - pad
            - circler
            - height * rng.gen_range(0.0, 0.7) * rng.gen_range(0.0, 1.0),
        ),
        rng.gen_range(-PI / 4., PI / 4.) * rng.gen_range(0.0, 1.0)
          - rng.gen_range(0.0, 1.0),
      )
    } else if rng.gen_bool(0.5) {
      (
        (
          pad + circler + rng.gen_range(0.0, 50.0),
          height - pad - circler,
        ),
        -PI / 3. + rng.gen_range(-PI / 4., PI / 4.) * rng.gen_range(0.0, 1.0),
      )
    } else {
      (
        (
          width / 2.0
            + rng.gen_range(0.0, 1.0) * rng.gen_range(-0.1, 0.1) * width,
          height - pad - circler,
        ),
        -PI / 2.0 + PI * rng.gen_range(-0.4, 0.4),
      )
    };
    let clr = if rng.gen_bool(1.0 / (i as f64 * 2.0 + 1.0)) {
      1
    } else {
      0
    };
    let force = 3.0
      + rng.gen_range(0.0, maxforce)
        * (if clr == 1 {
          1.
        } else {
          rng.gen_range(0.0, 1.0)
        });
    let bounce_factor = 0.7;
    let bounce_routes = bounce(
      origin,
      ang,
      force,
      gravity,
      friction,
      bounce_factor,
      clr,
      cycles,
      circler,
      bounds,
      &mut mask,
    );
    routes.extend(bounce_routes);
  }

  vec!["silver", "gold"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.55, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "243.0")]
  seed: f64,
  #[clap(short, long, default_value = "297")]
  width: f64,
  #[clap(short, long, default_value = "210")]
  height: f64,
  #[clap(short, long, default_value = "10")]
  pad: f64,
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

  fn paint_circle(&mut self, cx: f64, cy: f64, cr: f64) {
    let (minx, miny, maxx, maxy) = (
      (cx - cr).max(0.),
      (cy - cr).max(0.),
      (cx + cr).min(self.width),
      (cy + cr).min(self.height),
    );
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if euclidian_dist(point, (cx, cy)) < cr {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes_with_colors(
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

  for (clrp, input_route) in input_routes.iter() {
    let clr = *clrp;
    if input_route.len() < 2 {
      continue;
    }

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
