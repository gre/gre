use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
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
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn web_cordons(
  angles: &Vec<f64>,
  center: (f64, f64),
  radius: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = vec![];
  for a in angles.clone() {
    let main = vec![
      center,
      (center.0 + radius * a.cos(), center.1 + radius * a.sin()),
    ];
    routes.extend(cordon(main, 3.0, 0.5, 0.0, 3, false, 1.0, 0.0));
  }
  routes
}

fn web<R: Rng>(
  rng: &mut R,
  center: (f64, f64),
  radius: f64,
  angles: &Vec<f64>,
  densitymin: f64,
  densitymax: f64,
  mul: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = vec![];
  let splits = angles.len();

  let mut highest = (0..splits).map(|_i| 0.0).collect::<Vec<f64>>();
  let mut finished = (0..splits).map(|_i| false).collect::<Vec<bool>>();
  let mut i = 0;
  let mut route = vec![];
  loop {
    let h = highest[i];
    highest[i] +=
      rng.gen_range(densitymin, densitymax) * (1.0 + mul * h / radius);
    if highest[i] > radius {
      finished[i] = true;
    }
    let a = angles[i];
    let p = (center.0 + h * a.cos(), center.1 + h * a.sin());
    route.push(p);
    i = (i + 1) % splits;

    if finished.iter().all(|&f| f) {
      break;
    }
  }
  routes.push(route);

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();

  let splits = rng.gen_range(6, 20);

  let mut angles = vec![];

  for ai in 0..splits {
    let d = rng.gen_range(0.0, 0.5) * rng.gen_range(-1.0, 1.0);
    let a = (ai as f64 + d) * 2. * PI / (splits as f64);
    angles.push(a);
  }

  let mut webs = vec![];

  webs.extend(web_cordons(
    &angles,
    (width / 2.0, height / 2.0),
    height.max(width),
  ));

  if rng.gen_bool(0.5) {
    let r = rng.gen_range(0.0, 0.5);

    webs.extend(web(
      &mut rng,
      (width / 2.0, height / 2.0),
      r * height.min(width),
      &angles,
      0.4,
      2.0,
      1.0,
    ));
  }

  if rng.gen_bool(0.5) {
    let r = rng.gen_range(0.0, 0.7);

    webs.extend(web(
      &mut rng,
      (width / 2.0, height / 2.0),
      r * height.min(width),
      &angles,
      1.0,
      3.0,
      2.0,
    ));
  }

  let r = rng.gen_range(0.5, 1.0);

  let densitymax = rng.gen_range(2.0, 7.0);
  let mul = rng.gen_range(0.0, 3.0);

  webs.extend(web(
    &mut rng,
    (width / 2.0, height / 2.0),
    r * height.max(width),
    &angles,
    0.3,
    densitymax,
    mul,
  ));

  let is_outside =
    |(x, y)| x < pad || x > width - pad || y < pad || y > height - pad;

  routes.extend(clip_routes(&webs, &is_outside, 0.5, 5));

  vec![(routes, "white")]
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
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes(
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

fn cordon(
  path: Vec<(f64, f64)>,
  width: f64,
  noiseamp: f64,
  corner_pad: f64,
  tracks_count: usize,
  reconnect: bool,
  freq_mul: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let precision = 0.5;
  let r = precision;
  let mut pindex = 0;
  let mut p = path[pindex];
  let perlin = Perlin::new();
  let mut tracks = Vec::new();
  for _xi in 0..tracks_count {
    tracks.push(Vec::new());
  }
  for &next in path.iter().skip(1) {
    let dx = next.0 - p.0;
    let dy = next.1 - p.1;
    let a = dy.atan2(dx);
    let mut i = 0.0;
    let acos = a.cos();
    let asin = a.sin();
    let mut dist = (dx * dx + dy * dy).sqrt();
    if pindex != 0 {
      dist -= corner_pad;
      p.0 += corner_pad * acos;
      p.1 += corner_pad * asin;
    }
    if pindex == path.len() - 1 {
      dist -= corner_pad;
    }
    loop {
      if i >= dist {
        p = next;
        break;
      }
      p.0 += r * acos;
      p.1 += r * asin;
      for xi in 0..tracks_count {
        let variation = ((xi as f64 + (tracks_count as f64 * phase))
          % (tracks_count as f64)
          - ((tracks_count - 1) as f64 / 2.0))
          / (tracks_count as f64);
        let mut delta = variation * width;
        let noisefreq = freq_mul * (0.1 + 0.2 * (0.5 - variation.abs()));
        delta += noiseamp
          * perlin.get([
            //
            noisefreq * p.0,
            noisefreq * p.1,
            10.0 * xi as f64,
          ]);
        let a2 = a + PI / 2.0;
        let q = (p.0 + delta * a2.cos(), p.1 + delta * a2.sin());
        tracks[xi].push(q);
      }
      i += r;
    }
    pindex += 1;
  }
  for track in tracks {
    let mut track_copy = track.clone();
    if reconnect {
      track_copy.push(track[0]);
    }
    routes.push(track_copy);
  }
  routes
}
