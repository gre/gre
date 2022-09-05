use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "1176.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let cx = opts.width / 2.0;
  let cy = opts.height / 2.0;

  let mut rng = rng_from_seed(opts.seed);

  let amp1pow = rng.gen_range(0.8, 1.6);
  let amp_factor = rng.gen_range(0.0, 1.0);
  let freq1 = rng.gen_range(0.03, 0.06) * (1. - amp_factor);
  let amp1 = 0.1 + 0.4 * amp_factor;
  let freq2 = rng.gen_range(0.02, 0.06);
  let amp2 = rng.gen_range(2.0, 4.0);
  let freq3 = rng.gen_range(0.4, 0.6);
  let amp3 = 0.08;

  let displacement_amp = rng.gen_range(8.0, 40.0);

  let max_r = 70.0;

  let mut passage = Passage2DCounter::new(0.3, opts.width, opts.height);

  // logic
  let perlin = Perlin::new();
  let mut routes = Vec::new();
  let mut highest_by_angle = vec![0f64; 8000];
  let mut shape_bound = (opts.width, opts.height, 0.0, 0.0);

  let safe_h = -2.0;
  let r_increment = 0.5;
  let mut base_r = 0.2;
  loop {
    if base_r > max_r {
      break;
    }
    let mut route = Vec::new();
    let rotations = 800.0;
    let angle_delta =
      rng.gen_range(0, rotations as usize) as f64 / rotations * 2.0 * PI;
    let mut a = angle_delta;
    // TODO activate to create "snow" + prevent small lines < 0.3mm
    let angle_precision = 2. * PI / rotations;
    loop {
      if a - angle_delta > 2. * PI + 0.0001 {
        break;
      }

      let mut r = base_r;
      let x = cx + r * a.cos();
      let y = cy + r * a.sin();
      r += amp1
        * base_r
        * (base_r / max_r).powf(amp1pow)
        * perlin.get([
          -opts.seed
            + amp2
              * perlin.get([
                freq2 * x,
                opts.seed * 7.7 - 4.,
                freq2 * y
                  + amp3
                    * perlin.get([freq3 * x, opts.seed * 2.7 + 11., freq3 * y]),
              ]),
          freq1 * x,
          freq1 * y,
        ]);

      let hba_index = (highest_by_angle.len() as f64 * ((a) / 2. * PI))
        as usize
        % highest_by_angle.len();

      let should_draw = r > highest_by_angle[hba_index] + safe_h;

      let mut x = cx + r * a.cos();
      let mut y = cy + r * a.sin();

      let displacement_angle = 2.
        * PI
        * perlin.get([
          7.3 * opts.seed
            + 2.0 * perlin.get([0.005 * x, 0.005 * y, opts.seed * 3.7]),
          0.003 * x,
          0.003 * y,
        ]);
      let amp = displacement_amp
        * (base_r / max_r).powf(amp1pow)
        * perlin.get([
          opts.seed / 3.0 + perlin.get([0.005 * x, 0.005 * y, -opts.seed]),
          0.02 * x,
          0.02 * y,
        ]);
      x += amp * displacement_angle.cos();
      y += amp * displacement_angle.sin();

      let p = (x, y);

      if x < shape_bound.0 {
        shape_bound.0 = x;
      }
      if y < shape_bound.1 {
        shape_bound.1 = y;
      }
      if x > shape_bound.2 {
        shape_bound.2 = x;
      }
      if y > shape_bound.3 {
        shape_bound.3 = y;
      }

      if should_draw {
        highest_by_angle[hba_index] = r;
        route.push(p);
      } else {
        if route.len() > 1 {
          let mut simplified = Vec::new();
          simplified.push(route[0]);
          let mut dist = 0.0;
          let l = route.len();
          for i in 1..l {
            dist += euclidian_dist(route[i - 1], route[i]);
            if dist > 0.5 {
              simplified.push(route[i]);
              dist = 0.0;
            }
          }
          if dist > 0.0 {
            simplified.push(route[l - 1]);
          }
          if route.len() > 2 {
            routes.push(simplified);
          }
        }
        route = Vec::new();
      }
      a += angle_precision;
    }

    if route.len() > 0 {
      let mut simplified = Vec::new();
      simplified.push(route[0]);
      let mut dist = 0.0;
      let l = route.len();
      for i in 1..l {
        dist += euclidian_dist(route[i - 1], route[i]);
        if dist > 0.5 {
          simplified.push(route[i]);
          dist = 0.0;
        }
      }
      if dist > 0.0 {
        simplified.push(route[l - 1]);
      }
      if route.len() > 2 {
        routes.push(simplified);
      }
    }

    base_r += r_increment;
  }

  // center our shape
  let shape_cx = (shape_bound.0 + shape_bound.2) / 2.;
  let shape_cy = (shape_bound.1 + shape_bound.3) / 2.;
  let dx = cx - shape_cx;
  let dy = cy - shape_cy;
  routes = routes
    .iter()
    .map(|route| route.iter().map(|p| (p.0 + dx, p.1 + dy)).collect())
    .collect();

  println!("{}", routes.len());

  // render
  let colors = vec!["black"];
  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      let mut should_draw_line = |a, _b| passage.count(a) < 8;
      for route in routes.clone() {
        data = render_route_when(data, route, &mut should_draw_line);
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
