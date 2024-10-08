use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "8.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "38.0")]
  pub size: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

// TODO this function needs to return where the payload position is
fn trebuchet<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  height: f64,
  action_percent: f64,
  xflip: bool,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();

  let xmul = if xflip { -1.0 } else { 1.0 };

  let w = 0.6 * height;

  let line_width = 0.04 * height;
  let line_dist = 0.3;

  // make the base plank
  let mut route = Vec::new();
  let mut l = 0.0;
  let mut rev = false;
  while l < line_width {
    let a = (origin.0 - w / 2.0, origin.1 - l);
    let b = (origin.0 + w / 2.0, origin.1 - l);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    l += line_dist;
    rev = !rev;
  }
  routes.push(route);

  let frame_h = height * 0.5;
  let pivot = (origin.0, origin.1 - height * 0.45);

  // main stick
  let mut route = Vec::new();
  let mut l = 0.0;
  let mut rev = false;
  while l < line_width {
    let a = (origin.0 + l - line_width / 2.0, origin.1);
    let b = (origin.0 + l - line_width / 2.0, origin.1 - frame_h);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    l += line_dist;
    rev = !rev;
  }
  routes.push(route);

  let line_width = 0.03 * height;

  let possible_positions = vec![0.3, 0.5, 0.7, 1.0];
  let mut indexes = (0..possible_positions.len()).collect::<Vec<_>>();
  rng.shuffle(&mut indexes);
  let count = rng.gen_range(1, indexes.len());

  // structure frames
  let mut frames = vec![];
  for i in &indexes[..count] {
    let hf = possible_positions[*i];
    let wf = rng.gen_range(0.3, 0.5) - 0.2 * hf;
    frames.push((wf * w, hf * frame_h));
  }
  for (dx, dy) in frames {
    let mut route = Vec::new();
    let mut l = 0.0;
    let mut rev = false;
    while l < line_width {
      let a = (origin.0 - dx, origin.1 - l);
      let b = (origin.0, origin.1 - dy - l);
      let c = (origin.0 + dx, origin.1 - l);
      if rev {
        route.push(a);
        route.push(b);
        route.push(c);
      } else {
        route.push(c);
        route.push(b);
        route.push(a);
      }
      l += line_dist;
      rev = !rev;
    }
    routes.push(route);
  }

  // beam
  let size_factor = rng.gen_range(0.0, 1.0);
  let beam_main_length = mix(0.5, 0.8, size_factor) * height;
  let beam_second_length = 0.2 * height;
  let angle = mix(mix(2.5, 3.0, size_factor), 6.0, action_percent);
  let acos = angle.cos();
  let asin = angle.sin();

  let pivot1 = (
    pivot.0 + xmul * beam_main_length * acos,
    pivot.1 + beam_main_length * asin,
  );

  let pivot2 = (
    pivot.0 - xmul * beam_second_length * acos,
    pivot.1 - beam_second_length * asin,
  );

  let mut route = Vec::new();
  let mut l = 0.0;
  let mut rev = false;
  while l < line_width {
    let m = l - line_width / 2.0;
    let disp = (-asin * m, acos * m);
    let a = (pivot1.0 + disp.0, pivot1.1 + disp.1);
    let b = (pivot2.0 + disp.0, pivot2.1 + disp.1);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    l += line_dist;
    rev = !rev;
  }
  routes.push(route);

  // counterweight parts
  let f = rng.gen_range(0.0, 1.0);
  let cw_height = mix(0.15, 0.25, 1.0 - f) * height;
  let cw_width = rng.gen_range(0.1, 0.25) * height;
  let stickh = mix(0.01, 0.1, f) * height;

  // counterweight stick
  let mut route = Vec::new();
  let mut l = 0.0;
  let mut rev = false;
  while l < line_width {
    let a = (pivot2.0 + l - line_width / 2.0, pivot2.1);
    let b = (pivot2.0 + l - line_width / 2.0, pivot2.1 + stickh);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    l += line_dist;
    rev = !rev;
  }
  routes.push(route);

  // counterweight block
  let dy = rng.gen_range(0.0, 1.0) * stickh;
  let center = (pivot2.0, pivot2.1 + dy);
  let rad = dy + cw_height * rng.gen_range(0.95, 1.1);
  let anglestart = PI / 4.0;
  let angleeng = 3.0 * PI / 4.0;

  let square = (
    pivot2.0 - cw_width / 2.0,
    pivot2.1 + stickh,
    pivot2.0 + cw_width / 2.0,
    pivot2.1 + stickh + cw_height,
  );

  let line_dist = 0.4;
  let mut route = Vec::new();
  let mut x = square.0;
  let mut rev = false;
  while x < square.2 {
    let mut y = if rev { square.3 } else { square.1 };
    let mut horizontal_points_count = 0;
    loop {
      if rev {
        if y < square.1 {
          break;
        }
      } else {
        if y > square.3 {
          break;
        }
      }

      let dx = x - center.0;
      let dy = y - center.1;
      let d = (dx * dx + dy * dy).sqrt();
      let is_inside_circle = d < rad;
      let a = dy.atan2(dx);
      let is_inside_angle = a > anglestart && a < angleeng;
      let is_inside_counterweight = is_inside_circle && is_inside_angle;

      if is_inside_counterweight {
        if horizontal_points_count < 2 {
          route.push((x, y));
          horizontal_points_count += 1;
        } else {
          let l = route.len();
          route[l - 1] = (x, y);
        }
      } else {
        horizontal_points_count = 0;
        if route.len() > 1 {
          routes.push(route);
          route = Vec::new();
        } else if route.len() > 0 {
          route = Vec::new();
        }
      }

      y += if rev { -line_dist } else { line_dist };
    }
    x += line_dist;
    rev = !rev;
  }
  if route.len() > 1 {
    routes.push(route);
  }
  // TODO contouring of the counterweight

  if rng.gen_bool(0.5) {
    // triangle structure on the counterweight
    let mainsz = rng.gen_range(0.1, 0.16);

    // vertical
    let mut l = 0.0;
    let mut rev = false;
    while l < 0.04 * height {
      let mut route = Vec::new();
      let sz = mainsz * height;
      let a = (pivot2.0, pivot2.1 + stickh - l);
      let b = (pivot2.0 + xmul * sz, pivot2.1 + stickh - l);
      if rev {
        route.push(b);
        route.push(a);
      } else {
        route.push(a);
        route.push(b);
      }
      routes.push(route);
      l += line_dist;
      rev = !rev;
    }

    // triangle side
    let mut l = 0.0;
    let mut rev = false;
    while l < 0.03 * height {
      let mut route = Vec::new();
      let sz = 0.1 * height;
      let a = (pivot2.0, pivot2.1 + cw_height / 2.0 + stickh - l);
      let b = (pivot2.0 + xmul * sz, pivot2.1 + stickh - l);
      if rev {
        route.push(b);
        route.push(a);
      } else {
        route.push(a);
        route.push(b);
      }
      routes.push(route);
      l += 1.4 * line_dist;
      rev = !rev;
    }

    // tip
    let mut l = 0.0;
    let mut rev = false;
    while l < 0.02 * height {
      let mut route = Vec::new();
      let sz = mainsz * height;
      let h = 0.03 * height;
      let a = (
        pivot2.0 + xmul * (sz + l),
        pivot2.1 + stickh - 0.04 * height,
      );
      let b = (pivot2.0 + xmul * (sz + l), pivot2.1 + stickh + h);
      if rev {
        route.push(b);
        route.push(a);
      } else {
        route.push(a);
        route.push(b);
      }
      routes.push(route);
      l += line_dist;
      rev = !rev;
    }
  }

  // sling
  let length = rng.gen_range(0.3, 0.5) * height;
  let inity = pivot1.1 + length;
  let miny = origin.1 - 0.06 * height;
  let dx = (inity - miny).max(0.0);
  let center = (pivot1.0 + dx, inity.min(miny));
  let angle = 2.5 * PI * action_percent.powf(1.5) * xmul;
  // rotate center around pivot1 by angle
  let dx = center.0 - pivot1.0;
  let dy = center.1 - pivot1.1;
  let acos = angle.cos();
  let asin = angle.sin();
  let center = (
    pivot1.0 + xmul * dx * acos - dy * asin,
    pivot1.1 + xmul * dx * asin + dy * acos,
  );
  let dt = 0.04 * height;
  let center1 = (center.0 + dt * acos, center.1 + dt * asin);
  let center2 = (center.0 - dt * acos, center.1 - dt * asin);
  let p = (mix(center.0, pivot1.0, 0.5), mix(center.1, pivot1.1, 0.5));
  routes.push(vec![pivot1, p]);
  routes.push(vec![center2, p, center1]);

  let mut r = line_width;
  while r > line_dist / 2.0 {
    routes.push(circle_route(center, r, 16));
    r -= 0.8;
  }

  // rope to attach the beam on a wheel

  let wheel_radius = 0.04 * height;
  let wheel_center = (
    origin.0 - 0.2 * xmul * height,
    origin.1 - wheel_radius - 0.06 * height,
  );
  routes.push(vec![
    (wheel_center.0, origin.1),
    wheel_center,
    (wheel_center.0 - 0.1 * xmul * height, origin.1),
  ]);

  let mut r = 0.3;
  while r < wheel_radius {
    routes.push(circle_route(wheel_center, r, 10));
    r += 0.5;
  }

  let beam_anchor = (mix(pivot1.0, pivot.0, 0.5), mix(pivot1.1, pivot.1, 0.5));
  let beam_anchor_half = (
    mix(beam_anchor.0, wheel_center.0, 0.5),
    mix(beam_anchor.1, wheel_center.1, 0.5),
  );
  let beam_anchor1 = (mix(pivot1.0, pivot.0, 0.3), mix(pivot1.1, pivot.1, 0.3));
  let beam_anchor2 = (mix(pivot1.0, pivot.0, 0.7), mix(pivot1.1, pivot.1, 0.7));

  let mut ropes = vec![beam_anchor1, beam_anchor_half, beam_anchor2];

  if action_percent < 0.1 {
    let a = (wheel_center.0 + 0.5 * wheel_radius * xmul, wheel_center.1);
    let b = (wheel_center.0 - 0.5 * wheel_radius * xmul, wheel_center.1);
    routes.push(vec![a, beam_anchor_half, b]);
  } else {
    let left = ropes[0];
    ropes[1].1 -= rng.gen_range(0.1, 0.2) * height;
    let right = ropes[2];
    ropes = path_subdivide_to_curve_it(ropes, 0.8);
    ropes = shake(ropes, 0.1 * height, rng);
    ropes = path_subdivide_to_curve_it(ropes, 0.75);
    ropes = path_subdivide_to_curve_it(ropes, 0.7);

    ropes[0] = left;
    let l = ropes.len();
    ropes[l - 1] = right;
  }

  routes.push(ropes);

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;
  // let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();
  let mut y = pad;
  let mut xflip = false;
  while y < height - pad - size {
    let mut x = pad;
    while x < width - pad - size {
      let mut rng = rng_from_seed(opts.seed + y);
      let origin = (pad + x + size / 2.0, pad + y + size);
      let sz = size - 2.0 * pad;
      let action = (x - pad) / (width - pad - size);
      let belfry_routes = trebuchet(&mut rng, origin, sz, action, xflip);
      routes.extend(belfry_routes);
      x += size;
    }
    y += size;
    xflip = !xflip;
  }

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
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

fn circle_route(center: (f64, f64), r: f64, count: usize) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn path_subdivide_to_curve_it(
  path: Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path;
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

fn shake<R: Rng>(
  path: Vec<(f64, f64)>,
  scale: f64,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  path
    .iter()
    .map(|&(x, y)| {
      let dx = rng.gen_range(-scale, scale);
      let dy = rng.gen_range(-scale, scale);
      (x + dx, y + dy)
    })
    .collect()
}
