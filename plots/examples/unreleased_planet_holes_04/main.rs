use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Copy, Clone)]
struct Cycle {
  count: usize,
  vradius: f64,
  vx: f64,
  vy: f64,
}

#[derive(Clone)]
struct ShapeConfig {
  resolution: usize,
  initial_radius: f64,
  initial_center: (f64, f64),
  cycles: Vec<Cycle>,
  harmonies: Vec<(f64, f64)>,
  harmonies_mul: f64,
  displacement: f64,
  disp_harmonies: Vec<(f64, f64)>,
  seed: f64,
}

fn shapes(
  splits: usize,
  starts_div: f64,
  stops: f64,
  ShapeConfig {
    resolution,
    initial_radius,
    initial_center,
    cycles,
    harmonies,
    harmonies_mul,
    displacement,
    disp_harmonies,
    seed,
  }: ShapeConfig,
) -> Vec<Data> {
  let perlin = Perlin::new();

  let amp_multiplier = |a: f64| -> f64 {
    1. + harmonies_mul
      * harmonies
        .iter()
        .enumerate()
        .map(|(h, &(amp, f))| amp * perlin.get([seed + h as f64, f * a]))
        .sum::<f64>()
  };
  let disp = |p: (f64, f64), r: f64| -> (f64, f64) {
    let a = 2.
      * PI
      * disp_harmonies
        .iter()
        .enumerate()
        .map(|(h, &(amp, f))| {
          amp * perlin.get([100. + seed + h as f64, f * p.0, f * p.1])
        })
        .sum::<f64>();
    (
      p.0 + r * displacement * a.cos(),
      p.1 + r * displacement * a.cos(),
    )
  };

  let mut rng = rng_from_seed(seed);
  let mut routes = Vec::new();
  let mut radius = initial_radius;
  let mut center = initial_center;
  let mut a_off = 0.;
  for Cycle {
    count,
    vradius,
    vx,
    vy,
  } in cycles
  {
    for _i in 0..count {
      let mut route = (0..((resolution as f64 * stops) as usize))
        .map(|j| {
          let p = j as f64 / (resolution as f64);
          let a = a_off + p;
          let ang = a * 2. * PI;
          let amp = amp_multiplier(a % 1.) * radius;
          disp(
            (center.0 + amp * ang.cos(), center.1 + amp * ang.sin()),
            amp,
          )
        })
        .collect::<Vec<(f64, f64)>>();
      routes.push(route);

      radius += vradius;
      center.0 += vx;
      center.1 += vy;
      a_off += starts_div;
    }
  }

  (0..splits)
    .map(|l| {
      routes
        .iter()
        .enumerate()
        .fold(Data::new(), |data, (i, route)| {
          if i % splits == l {
            render_route(data, route.clone())
          } else {
            data
          }
        })
    })
    .collect()
}

fn art(opts: Opts) -> Vec<Group> {
  let width = 297.;
  let height = 210.;

  let mut rng = rng_from_seed(opts.seed);

  let cycles = vec![
    Cycle {
      count: rng.gen_range(40, 80),
      vradius: -0.5,
      vx: -0.2,
      vy: 0.0,
    },
    Cycle {
      count: rng.gen_range(30, 60),
      vradius: -0.8,
      vx: 0.4,
      vy: 0.3,
    },
  ];
  let harmonies = vec![
    (
      rng.gen_range(0.0, 0.05),
      rng.gen_range(2.0, 20.0f64).floor(),
    ),
    (rng.gen_range(0.0, 0.1), rng.gen_range(0.5, 6.0f64).floor()),
  ];
  let disp_harmonies = vec![
    (rng.gen_range(0.0, 0.4), rng.gen_range(0.0, 0.01)),
    (rng.gen_range(0.0, 1.0), rng.gen_range(0.005, 0.02)),
  ];
  let resolution = 1000;
  let initial_radius = 80.;

  let conf = ShapeConfig {
    resolution,
    initial_radius,
    initial_center: (width / 2., height / 2.),
    cycles: cycles.clone(),
    harmonies: harmonies.clone(),
    harmonies_mul: rng.gen_range(0., 2.),
    displacement: rng.gen_range(0., 0.2),
    disp_harmonies: disp_harmonies.clone(),
    seed: opts.seed,
  };

  let colors = vec!["darkviolet", "slateblue"];
  let data_splitted = shapes(colors.len(), 0.501, 0.6, conf);

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let data = data_splitted.get(i).unwrap().clone();
      let mut g = layer(color);
      g = g.add(base_path(color, 0.2, data));
      if i == colors.len() - 1 {
        g = g.add(signature(1.0, (220.0, 170.0), color));
      }
      g
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
