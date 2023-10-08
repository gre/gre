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
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "9841.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = Vec::new();

  // (from, to) in index pos
  let mut snakes = vec![];
  let mut ladders = vec![];

  let ppad = 0.2;

  let boardw = 10;
  let boardh = 10;
  let pladders = 0.1;
  let psnakes = 0.1;

  let w = (width - 2. * pad) / (boardw as f64);
  let h = (height - 2. * pad) / (boardh as f64);

  routes.push((
    0,
    vec![
      (pad - ppad, pad - ppad),
      (pad - ppad, height - pad + ppad),
      (width - pad + ppad, height - pad + ppad),
      (width - pad + ppad, pad - ppad),
      (pad - ppad, pad - ppad),
    ],
  ));

  let mut i = 0;
  loop {
    if i >= boardh * boardw {
      break;
    }
    let yi = i / boardw;
    let ltr = yi % 2 == 0;
    let xi = if ltr {
      i % boardw
    } else {
      boardw - 1 - i % boardw
    };
    let x = pad + w * (xi as f64);
    let y = pad + h * (yi as f64);

    let mut clr = 0;

    // ladders
    if yi > 0 && rng.gen_bool(pladders) {
      let mut candidates = vec![];
      for pxi in 0..boardw {
        for pyi in 0..boardh {
          let dx = if pxi > xi { pxi - xi } else { xi - pxi };
          if pyi + dx < yi {
            if pxi == 0 && pyi == 0 {
              continue;
            }
            candidates.push((pxi, pyi));
          }
        }
      }

      if candidates.len() > 0 {
        let ci = rng.gen_range(0, candidates.len());
        let ci = (ci as f64 * rng.gen_range(0.5, 1.0)) as usize;
        let (destxi, destyi) = candidates[ci];

        if !ladders
          .iter()
          .any(|&((x, y), _)| (x, y) == (destxi, destyi))
          && !snakes
            .iter()
            .any(|&((x, y), _, _)| (x, y) == (destxi, destyi))
        {
          clr = 1;
          ladders.push(((xi, yi), (destxi, destyi)));
        }
      }
    }
    // snakes
    if yi < boardh - 1 && !(xi == 0 && yi == 0) && rng.gen_bool(psnakes) {
      let mut candidates = vec![];
      for pxi in 0..boardw {
        for pyi in 0..boardh {
          let dx = if pxi > xi { pxi - xi } else { xi - pxi };
          if pyi > yi + dx {
            if pxi == 0 && pyi == 0 {
              continue;
            }
            clr = 1;
            candidates.push((pxi, pyi));
          }
        }
      }

      if candidates.len() > 0 {
        let ci = rng.gen_range(0, candidates.len());
        let ci = (ci as f64 * rng.gen_range(0.5, 1.0)) as usize;
        let (destxi, destyi) = candidates[ci];

        if !ladders
          .iter()
          .any(|&((x, y), _)| (x, y) == (destxi, destyi))
          && !snakes
            .iter()
            .any(|&((x, y), _, _)| (x, y) == (destxi, destyi))
        {
          clr = rng.gen_range(2, 5);
          snakes.push(((xi, yi), (destxi, destyi), clr));
        }
      }
    }

    let t = 0.2 * w;

    let mut left = 1.;
    let mut right = 1.;
    let mut top = 0.;
    let mut bottom = 0.;

    if xi == boardw - 1 {
      right = 0.;
    }
    if xi == 0 {
      left = 0.;
    }

    if xi == 0 {
      top = if ltr { -1. } else { 0. };
      bottom = if !ltr { -1. } else { 0. };
    }
    if xi == boardw - 1 {
      top = if ltr { 0. } else { -1. };
      bottom = if !ltr { 0. } else { -1. };
    }

    if yi == boardh - 1 {
      bottom = 0.;
    }
    if yi == 0 {
      top = 0.;
    }

    if ltr {
      left = -left;
      right = -right;
    }

    let cell = vec![
      (x + ppad, y + ppad),
      (x + 0.5 * w, y + ppad + top * t),
      (x + w - ppad, y + ppad),
      (x + w - ppad + right * t, y + 0.5 * h),
      (x + w - ppad, y + h - ppad),
      (x + 0.5 * w, y + h - ppad + bottom * t),
      (x + ppad, y + h - ppad),
      (x + ppad + left * t, y + 0.5 * h),
      (x + ppad, y + ppad),
    ];

    routes.push((clr, cell));

    if i == boardh * boardw - 1 {
      let mut d = 0.1;
      loop {
        if d > w * 0.3 {
          break;
        }
        let r = vec![
          // triangle
          (x + w * 0.4, y + h * 0.5 - d),
          (x + w * 0.4 + d, y + h * 0.5),
          (x + w * 0.4, y + h * 0.5 + d),
        ];
        routes.push((1, r));
        d += 0.8;
      }
    }
    if i == 0 {
      let mut d = 0.1;
      loop {
        if d > w * 0.2 {
          break;
        }
        let r = vec![
          // square
          (x + w * 0.4 - d, y + h * 0.5 - d),
          (x + w * 0.4 + d, y + h * 0.5 - d),
          (x + w * 0.4 + d, y + h * 0.5 + d),
          (x + w * 0.4 - d, y + h * 0.5 + d),
          (x + w * 0.4 - d, y + h * 0.5 - d),
        ];
        routes.push((1, r));
        d += 0.5;
      }
    }

    i += 1;
  }

  for (from, to) in ladders {
    let clr = 1;
    let x1 = pad + w * (from.0 as f64 + rng.gen_range(0.4, 0.6));
    let y1 = pad + h * (from.1 as f64 + rng.gen_range(0.4, 0.6));
    let x2 = pad + w * (to.0 as f64 + rng.gen_range(0.4, 0.6));
    let y2 = pad + h * (to.1 as f64 + rng.gen_range(0.4, 0.6));

    let s = w * 0.18;
    let dbl = 0.6;
    let s2 = s + dbl;

    routes.push((clr, vec![(x1 - s, y1), (x2 - s, y2)]));
    routes.push((clr, vec![(x1 - s2, y1), (x2 - s2, y2)]));
    routes.push((clr, vec![(x1 + s2, y1), (x2 + s2, y2)]));
    routes.push((clr, vec![(x1 + s, y1), (x2 + s, y2)]));

    let step = 3. * s;
    let mut v = step;

    loop {
      if v > y1 - y2 - step * 0.5 {
        break;
      }

      let x = mix(x2, x1, v / (y1 - y2));
      routes.push((clr, vec![(x - s, y2 + v), (x + s, y2 + v)]));
      routes.push((clr, vec![(x - s, y2 + v + dbl), (x + s, y2 + v + dbl)]));

      v += step;
    }

    // TODO
  }

  for (from, to, clr) in snakes {
    let x1 = pad + w * (from.0 as f64 + 0.5);
    let y1 = pad + h * (from.1 as f64 + 0.5);
    let x2 = pad + w * (to.0 as f64 + 0.5);
    let y2 = pad + h * (to.1 as f64 + 0.5);

    let l = euclidian_dist((x1, y1), (x2, y2));
    let incr = 0.1 * w;
    let m = rng.gen_range(0.0, 1.0);
    let freq = mix(0.5, 2.0, m);
    let amp = mix(3.0, 0.5, m) * (l / height);
    let mut v = l - incr;
    loop {
      if v < incr {
        break;
      }
      let xp = v / l;

      let nearcenter = 2. * (0.5 - (xp - 0.5).abs());

      let r = mix(0.3, 0.01, xp.powf(1.5)) * w;

      let y = mix(y1, y2, v / l);
      let xd = amp * w * (freq * y / h).cos() * nearcenter;

      let p = (xd + mix(x1, x2, xp), y);
      let circle = circle_route(p, r, 32);

      let is_outside = |(x, y)| polygon_includes_point(&circle, (x, y));

      routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);

      routes.push((clr, circle));
      v -= mix(incr, 0.5 * incr, xp);
    }

    // routes.push((clr, route));
    // TODO
  }

  vec![
    "white", // white
    "#f90",  // gold
    "#f0f",  // pink
    "#09f",  // blue
    "#3f3",  // green
  ]
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

fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  point: (f64, f64),
) -> bool {
  let mut c = false;
  for i in 0..polygon.len() {
    let j = (i + 1) % polygon.len();
    if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
      && (point.0
        < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
          / (polygon[j].1 - polygon[i].1)
          + polygon[i].0)
    {
      c = !c;
    }
  }
  c
}
