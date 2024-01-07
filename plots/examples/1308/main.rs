use clap::*;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "420.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

#[derive(Clone)]
struct SpiralBinary {
  origin: (f64, f64),
  dr: f64,
  step_dist: f64,
  step_l: f64,
  data_loops: bool,
  radius: f64,
  outer_radius: f64,
  data: String,
  clr: usize,
}

impl SpiralBinary {
  fn new(
    origin: (f64, f64),
    clr: usize,
    dr: f64,
    step_dist: f64,
    step_l: f64,
    data_loops: bool,
    radius: f64,
    outer_radius: f64,
    data: String,
  ) -> Self {
    SpiralBinary {
      origin,
      clr,
      dr,
      step_dist,
      step_l,
      data_loops,
      radius,
      outer_radius,
      data,
    }
  }

  fn render(&self, level: usize) -> Vec<(usize, Vec<(f64, f64)>)> {
    let origin = self.origin;
    let dr = self.dr;
    let step_dist = self.step_dist;
    let step_l = self.step_l;
    let data_loops = self.data_loops;
    let radius = self.radius;
    let outer_radius = self.outer_radius;
    let clr = self.clr;

    let mut data_encoded = vec![];
    for c in self.data.chars() {
      let b = c as u8;
      for i in 0..8 {
        data_encoded.push((b >> i) & 1 == 1);
      }
    }

    let mut points = spiral(origin.0, origin.1, outer_radius, dr, 0.2);

    points.reverse();

    let mut circle_routes = vec![];
    let mut routes = Vec::new();
    let mut routes_in_step = vec![];
    let mut route = vec![];
    let mut route_in_step = vec![];
    let mut dist = 0.;
    let mut has_reached_mid_step = false;
    let mut lastp = points[0];
    let mut i = 0;
    for p in points {
      let c = if euclidian_dist(p, origin) <= radius + step_l {
        clr
      } else {
        clr + 1
      };
      let d = euclidian_dist(p, lastp);
      lastp = p;
      dist += d;
      let data_is_true = i < data_encoded.len() && data_encoded[i];
      let is_in_step = dist > step_dist;
      if is_in_step {
        if !data_is_true {
          route_in_step.push(p);
        }
        if route.len() > 1 {
          routes.push((c, route));
        }
        route = vec![];
      } else {
        route.push(p);
        if route_in_step.len() > 1 {
          routes_in_step.push((c, route_in_step));
        }
        route_in_step = vec![];
      }

      if dist > step_dist + step_l / 2. && !has_reached_mid_step {
        has_reached_mid_step = true;
        if data_is_true {
          if level == 0 {
            circle_routes.push((c, circle_route(p, step_l / 2., 16)));
          } else {
            let m = step_l / (2.0 * radius);
            let sub = SpiralBinary::new(
              p,
              c,
              dr * m,
              step_dist * m,
              step_l * m,
              false,
              radius * m,
              radius * m,
              self.data.clone(),
            );
            circle_routes.extend(sub.render(level - 1));
          }
        }
      }

      if dist > step_dist + step_l {
        if data_loops {
          i = (i + 1) % data_encoded.len();
        } else {
          i += 1;
        }
        dist = 0.0;
        route = vec![];
        if route_in_step.len() > 1 {
          routes_in_step.push((c, route_in_step));
        }
        route_in_step = vec![];
        has_reached_mid_step = false;
      }
    }
    if route.len() > 1 {
      routes.push((clr, route));
    }
    if route_in_step.len() > 1 {
      routes_in_step.push((clr, route_in_step));
    }

    let mut all = vec![];
    all.extend(circle_routes);
    if level == 0 {
      all.extend(routes);
      all.extend(routes_in_step);
    } else {
      all.extend(routes_in_step);
      for (clr, rt) in routes {
        let s = 0.5 * step_l * step_l / radius;
        for p in step_polyline(&rt, s) {
          all.push((clr, circle_route(p, s / 2.0, 16)));
        }
      }
    }

    all
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  let mut routes = vec![];

  let dr = 1.6;
  let step_dist = 1.2;
  let step_l = 1.5;

  let word = "recursion";
  let l = (8 * word.len()) as f64 * (step_dist + step_l);
  let radius: f64 = (1.25 * l / PI).sqrt();

  let m = 2.0 * radius / step_l;
  let s = SpiralBinary::new(
    (width / 2.0, height / 2.0),
    0,
    dr * m,
    step_dist * m,
    step_l * m,
    false,
    radius * m,
    width.min(height) * 0.7,
    String::from(word),
  );
  let mainc =
    VCircle::new(width / 2.0, height / 2.0, (radius + 0.5 * step_l) * m);

  let mut circles = vec![];
  let path = vec![
    (0.1 * width, 0.0),
    (0.55 * width, 0.5 * height),
    (0.7 * width, height),
  ];
  let path = path_subdivide_to_curve_it(&path, 0.7);
  let path = path_subdivide_to_curve_it(&path, 0.7);
  let path = path_subdivide_to_curve_it(&path, 0.7);
  let path = step_polyline(&path, m * step_l);
  for p in path {
    let circle = VCircle::new(p.0, p.1, radius);
    if mainc.collides(&circle) {
      continue;
    }
    circles.push(VCircle::new(p.0, p.1, 2.0 + radius));
    let inner = SpiralBinary::new(
      p,
      1,
      dr,
      step_dist,
      step_l,
      false,
      radius,
      radius,
      String::from(word),
    );
    routes.extend(inner.render(0));
  }

  routes.extend(clip_routes_with_colors(
    &s.render(1),
    &|p| circles.iter().any(|c| c.includes(p)),
    0.3,
    4,
  ));

  /*
  let mut rng = rng_from_seed(opts.seed);
  let mut circles = vec![];
  for i in 0..1000000 {
    let x = rng.gen_range(bound.0, bound.2);
    let y = rng.gen_range(bound.1, bound.3);

    let circle = VCircle::new(x, y, radius);

    if circles.iter().any(|c| circle.collides(c)) {
      continue;
    }

    let s = SpiralBinary::new(
      (x, y),
      0,
      dr,
      step_dist,
      step_l,
      false,
      radius,
      String::from(word),
    );
    routes.extend(s.render());

    circles.push(circle);
  }
  */

  routes =
    clip_routes_with_colors(&routes, &|p| out_of_boundaries(p, bound), 0.3, 4);

  /*
  routes.push((
    0,
    vec![
      (pad, pad),
      (width - pad, pad),
      (width - pad, height - pad),
      (pad, height - pad),
      (pad, pad),
    ],
  ));
  */

  vec!["white", "gold"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn spiral(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = round_point((x + r * a.cos(), y + r * a.sin()), 0.01);
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = approx / (r + 8.0);
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
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

pub fn clip_routes_with_colors(
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

#[derive(Clone, Copy, Debug)]
pub struct VCircle {
  pub x: f64,
  pub y: f64,
  pub r: f64,
}
impl VCircle {
  pub fn new(x: f64, y: f64, r: f64) -> Self {
    VCircle { x, y, r }
  }
  pub fn pos(self: &Self) -> (f64, f64) {
    (self.x, self.y)
  }
  pub fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist(p, (self.x, self.y)) < self.r
  }
  pub fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  pub fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

fn step_polyline(path: &Vec<(f64, f64)>, step: f64) -> Vec<(f64, f64)> {
  let plen = path.len();
  let mut route = vec![];
  if plen < 1 {
    return route;
  }
  let mut lastp = path[0];
  route.push(lastp);
  let mut i = 0;
  while i < plen - 1 {
    let b = path[i + 1];
    let dist = euclidian_dist(lastp, b);
    if dist < step {
      i += 1;
    } else if dist >= step {
      let p = lerp_point(lastp, b, step / dist);
      route.push(p);
      lastp = p;
    }
  }
  route
}

fn path_subdivide_to_curve_it(
  path: &Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path.clone();
  }
  let mut route = Vec::new();
  let mut first = path[0];
  let mut last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if looped {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for i in 1..(l - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if looped {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if looped {
    route.push(first);
  }
  route
}
