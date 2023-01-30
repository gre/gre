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
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn spiral_square_parametric(
  phases: usize,
  t: f64,
  turns: usize,
  center: (f64, f64),
  radius: f64,
  square_size: f64,
) -> (f64, f64) {
  let twopi = 2.0 * PI;
  let radiusend = square_size
    * if phases % 2 == 0 {
      1.0
    } else {
      (2.0_f64).sqrt()
    };
  let ang = (turns as f64) * twopi * t;
  let r = mix(radius, radiusend, t);

  let acos = ang.cos();
  let asin = ang.sin();

  let tp = t * (phases as f64);
  let tremain = tp - tp.floor();

  let tsquare = if tp % 2.0 < 1.0 {
    tremain
  } else {
    1.0 - tremain
  };

  let x = mix(r * acos, r * acos.powi(2) * acos.signum(), tsquare);
  let y = mix(r * asin, r * asin.powi(2) * asin.signum(), tsquare);

  let (mut x, mut y) = p_r((x, y), PI / 4.0);

  x += center.0;
  y += center.1;

  (x, y)
}

fn spiral_square(
  phases: usize,
  resolution: usize,
  turns: usize,
  min_dist: f64,
  f: f64,
  amp: f64,
  center: (f64, f64),
  radius: f64,
  square_size: f64,
) -> Vec<(f64, f64)> {
  let mut points = vec![];
  let mut phase = 0.0;
  let mut last_p = (0.0, 0.0);

  for i in 0..resolution {
    let t = i as f64 / (resolution as f64);
    let p =
      spiral_square_parametric(phases, t, turns, center, radius, square_size);
    let t2 = t - 1.0 / (turns as f64);
    let disp = {
      let q = spiral_square_parametric(
        phases,
        t2,
        turns,
        center,
        radius,
        square_size,
      );
      let d = (euclidian_dist(p, q) - min_dist).max(0.0);
      let step = euclidian_dist(p, last_p);
      phase += step * f;
      let r = amp * d * phase.cos();
      (r * phase.cos(), r * phase.sin())
    };
    last_p = p;
    points.push((p.0 + disp.0, p.1 + disp.1));
  }
  points
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut rng = rng_from_seed(opts.seed);

  let m = rng.gen_range(0.0, 1.0);

  let phases = 1
    + (rng.gen_range(1., 10.)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;
  let turns = mix(10., 80., m) as usize;
  let iterations = 5000 * turns;
  let min_dist = rng.gen_range(-0.5f64, 2.0).max(0.0);
  let freq = rng.gen_range(1.0f64, 8.0) * (1. - 0.5 * m);
  let amp = rng.gen_range(0.0, 1.0);

  let routes = vec![spiral_square(
    phases,
    iterations,
    turns,
    min_dist,
    freq,
    amp,
    (width / 2.0, height / 2.0),
    rng.gen_range(-0.1f64, 0.05).max(0.0) * width,
    width / 2.0 - pad,
  )];

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
