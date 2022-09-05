use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

// IDEAS
// symmetry / repeat rotate
// variating width over position
// variating freq/amp over position
// add back 1/2/3 random heads & collides
// "flip" if we rotate > 90° (param), reverse tracks, to bounce
// change color on flip
// stop on the cordon with line / bridge
// dot dot dot rendering
// TODO: test gaussian distrib
// amplitude with function

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "24.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = 20.0;
  let width = opts.width;
  let height = opts.height;
  let colors = vec!["red", "cyan"];
  let mut rng = rng_from_seed(opts.seed);
  let samples = 20;
  let subdivisions = 3;
  let track_count = 28;
  let cordon_w = 20.0;
  let noise_freq = 0.02;
  let noiseamp = 4.0;
  let round = 4.0;

  colors
    .iter()
    .enumerate()
    .map(|(_i, color)| {
      let mut data = Data::new();
      let mut points = (0..samples)
        .map(|_i| {
          let y = height / 2.0
            + rng.gen_range(0.0, height / 2.0 - pad) * rng.gen_range(-1.0, 1.0);
          (rng.gen_range(pad, width - pad), y)
        })
        .collect::<Vec<(f64, f64)>>();

      // points = route_spiral(points);

      points.push(points[0]);

      // TRIANGLE
      /*
      let r = height / 2.0 - pad;
      let points = (0..4)
          .map(|i| {
              let a = 2. * PI * (i as f64) / 3.0;
              (width / 2.0 + r * a.cos(), height / 2.0 + r * a.sin())
          })
          .collect();
          */

      let path = path_subdivide_to_curve(points, subdivisions, 0.8);

      let routes =
        cordon(path, track_count, cordon_w, noise_freq, noiseamp, round);

      /*
      let paths = cordon(
        //
        path, 3, 10.0, 0.01, 5.0, 4.0,
      );
      let routes = paths
        .iter()
        .flat_map(|path| cordon(path.clone(), 6, 4.0, 0.05, 4.0, 4.0))
        .collect::<Vec<Vec<(f64, f64)>>>();
        */

      // let routes = vec![path];

      for route in routes {
        data = render_route(data, route);
      }

      let mut l = layer(color);
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

fn cordon(
  path: Vec<(f64, f64)>,
  tracks_count: usize,
  width: f64,
  noisefreq: f64,
  noiseamp: f64,
  round: f64,
) -> Vec<Vec<(f64, f64)>> {
  let l = path.len();
  let first = path[0];
  let last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
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
      dist -= round;
      p.0 += round * acos;
      p.1 += round * asin;
    }
    if pindex == path.len() - 1 {
      dist -= round;
    }
    loop {
      if i >= dist {
        p = next;
        break;
      }
      p.0 += r * acos;
      p.1 += r * asin;
      for xi in 0..tracks_count {
        let variation = ((xi as f64) - ((tracks_count - 1) as f64 / 2.0))
          / (tracks_count as f64);
        let mut delta = variation * width;
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
    if looped {
      track_copy.push(track[0]);
    }
    routes.push(track_copy);
  }
  routes
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

fn path_subdivide_to_curve(
  path: Vec<(f64, f64)>,
  n: usize,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}
