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

fn shape(
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
) -> Data {
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
  for Cycle {
    count,
    vradius,
    vx,
    vy,
  } in cycles
  {
    for _i in 0..count {
      let a_off = rng.gen_range(0.0, 1.0);
      let mut route = (0..resolution)
        .map(|j| {
          let a = (a_off + j as f64 / (resolution as f64)) % 1.;
          let ang = a * 2. * PI;
          let amp = amp_multiplier(a) * radius;
          disp(
            (center.0 + amp * ang.cos(), center.1 + amp * ang.sin()),
            amp,
          )
        })
        .collect::<Vec<(f64, f64)>>();
      route.push(route[0]);
      routes.push(route);

      radius += vradius;
      center.0 += vx;
      center.1 += vy;
    }
  }

  routes
    .iter()
    .fold(Data::new(), |data, route| render_route(data, route.clone()))
}

fn art(opts: Opts) -> Vec<Group> {
  let mut rng = rng_from_seed(opts.seed);

  let disp_harmonies = vec![
    (rng.gen_range(0.0, 0.4), rng.gen_range(0.0, 0.01)),
    (rng.gen_range(0.0, 1.0), rng.gen_range(0.005, 0.02)),
  ];
  let resolution = 600;
  let initial_radius = 28.;

  let iw = 80.;
  let ih = 38.;
  let wsize = 66.;
  let hsize = 66.;
  let w = 3;
  let h = 3;

  let mut configs = Vec::new();
  for x in 0..w {
    for y in 0..h {
      configs.push((
        "white",
        ShapeConfig {
          resolution,
          initial_radius,
          initial_center: (iw + wsize * x as f64, ih + hsize * y as f64),
          cycles: vec![
            Cycle {
              count: rng.gen_range(10, 16),
              vradius: -0.6,
              vx: (x - 1) as f64 / 2. + rng.gen_range(-0.2, 0.2),
              vy: ((y - 1) as f64 / 2.) * rng.gen_range(0.0, 1.),
            },
            Cycle {
              count: rng.gen_range(3, 6),
              vradius: rng.gen_range(-2., -0.8),
              vx: -(x - 1) as f64 / 2. + rng.gen_range(-0.2, 0.2),
              vy: (-(y - 1) as f64 / 2.) * rng.gen_range(0.0, 1.),
            },
          ],
          harmonies: vec![
            (rng.gen_range(0.0, 0.1), rng.gen_range(3.0, 6.0f64).floor()),
            (rng.gen_range(0.0, 0.15), rng.gen_range(1.0, 3.0f64).floor()),
          ],
          harmonies_mul: 2.,
          displacement: 0.02,
          disp_harmonies: disp_harmonies.clone(),
          seed: opts.seed,
        },
      ));
    }
  }

  configs
    .iter()
    .enumerate()
    .map(|(i, (color, config))| {
      let data = shape(config.clone());
      let mut g = layer(color);
      g = g.add(base_path(color, 0.4, data));
      if i == configs.len() - 1 {
        g = g.add(signature(1.0, (240.0, 190.0), color));
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
  let mut document = base_a4_landscape("black");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
