use std::f64::consts::PI;

use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "300.0")]
  pub width: f64,
  #[clap(short, long, default_value = "400.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  let dr = 2.0;
  let step_dist = 10.0;
  let step_l = 3.0;
  let data_loops = true;
  let data = "palette";
  let mut data_encoded = vec![];
  for c in data.chars() {
    let b = c as u8;
    for i in 0..8 {
      data_encoded.push((b >> i) & 1 == 1);
    }
  }

  let mut points = spiral(width / 2., height / 2., height * 0.66, dr, 0.05);
  points.reverse();

  let mut routes = Vec::new();
  let mut route = vec![];
  let mut dist = 0.;
  let mut has_reached_mid_step = false;
  let mut lastp = points[0];
  let mut i = 0;
  for p in points {
    let d = euclidian_dist(p, lastp);
    lastp = p;
    dist += d;
    let is_in_step =
      dist > step_dist && (i >= data_encoded.len() || data_encoded[i]);
    if is_in_step {
      if route.len() > 1 {
        routes.push(route);
      }
      route = vec![];
    } else {
      route.push(p);
    }

    if dist > step_dist + step_l / 2. && !has_reached_mid_step {
      has_reached_mid_step = true;

      if data_loops {
        i = (i + 1) % data_encoded.len();
      } else {
        i += 1;
      }
    }

    if dist > step_dist + step_l {
      dist = 0.0;
      route = vec![];
      has_reached_mid_step = false;
    }
  }
  if route.len() > 1 {
    routes.push(route);
  }

  routes =
    clip_routes_with_colors(&routes, &|p| out_of_boundaries(p, bound), 0.3, 4);

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 1.2, data));
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
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

pub fn clip_routes_with_colors(
  input_routes: &Vec<Vec<(f64, f64)>>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<Vec<(f64, f64)>> {
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

  for input_route in input_routes.iter() {
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
              routes.push(route);
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
      routes.push(route);
    }
  }

  routes
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}
