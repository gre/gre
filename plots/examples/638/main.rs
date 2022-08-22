use clap::*;
use geo::map_coords::MapCoordsInplace;
use geo::prelude::{BoundingRect, Centroid, Contains};
use geo::Point;
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
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let colors = vec!["red", "blue"];
  let scales = vec![0.1, 0.3, 0.6];

  let pad = 10.0;
  let perlin = Perlin::new();

  let mut rng = rng_from_seed(opts.seed);

  let max_perturbation =
    (rng.gen_range(-50.0, 200.0) * rng.gen_range(0.0, 1f64)).max(0.0);

  let voronoi_size =
    8 + (rng.gen_range(60f64, 800.0) * rng.gen_range(0.0, 1.0)) as usize;

  let sample_offset = rng.gen_range(-0.2, 0.6) * rng.gen_range(0.0, 1.0);
  let sample_pow = rng.gen_range(1.0, 4.0);

  let noiseamp = rng.gen_range(0.0, 0.8);
  let noisefreq = rng.gen_range(1.0, 8.0);

  let candidates = sample_2d_candidates_f64(
    &|p| sample_offset + (2. * (0.5 - (p.1 - 0.5).abs())).powf(sample_pow),
    800,
    voronoi_size,
    &mut rng,
  );

  let polys = sample_square_voronoi_polys(candidates, 0.0);

  let routes: Vec<(usize, Vec<Vec<(f64, f64)>>)> = polys
    .iter()
    .filter_map(|local_poly| {
      let out_of_bounds = local_poly
        .exterior()
        .points()
        .any(|p| p.x() < 0.0 || p.y() < 0.0 || p.x() > 1.0 || p.y() > 1.0);
      if out_of_bounds {
        return None;
      }

      let perturbation = rng.gen_range(-0.00001, max_perturbation);

      let center = local_poly.centroid().unwrap();
      let threshold = 0.5
        + noiseamp
          * perlin.get([
            //
            opts.seed + noisefreq * perturbation,
            noisefreq * center.x(),
            noisefreq * center.y(),
          ]);

      let color = if center.y() > threshold { 0 } else { 1 };

      let mut poly = local_poly.clone();
      poly.map_coords_inplace(|&(x, y)| {
        (
          x * (opts.height - 2. * pad) + pad + (opts.width - opts.height) / 2.0,
          y * (opts.height - 2. * pad) + pad,
        )
      });

      let mut routes = Vec::new();
      let bound = poly.bounding_rect().unwrap();
      let topleft = bound.min();
      let bottomright = bound.max();

      let yinterval = rng.choose(&scales).unwrap_or(&1.0);
      let precision = 0.1;
      let mut alternate = false;

      if rng.gen_bool(0.5) {
        let mut y = topleft.y;
        loop {
          if y > bottomright.y {
            break;
          }
          let mut x = if alternate { bottomright.x } else { topleft.x }; // TODO alternate direction
          let xstop = if alternate {
            topleft.x - 2.0 * precision
          } else {
            bottomright.x + 2.0 * precision
          };
          let mut route = Vec::new();
          loop {
            if alternate && x < xstop || !alternate && x > xstop {
              break;
            }

            let p = (x, y);
            if poly.contains(&Point::new(x, y)) {
              // inside polygon => do line
              if route.len() == 0 {
                route.push(p);
              }
            } else {
              // outside => stop line
              if route.len() > 0 {
                route.push(p);
                routes.push(route);
                route = Vec::new();
              }
            }

            if alternate {
              x -= precision;
            } else {
              x += precision;
            }
          }

          y += yinterval;
          alternate = !alternate;
        }
      } else {
        let mut x = topleft.x;
        loop {
          if x > bottomright.x {
            break;
          }
          let mut y = if alternate { bottomright.y } else { topleft.y }; // TODO alternate direction
          let stop = if alternate {
            topleft.y - 2.0 * precision
          } else {
            bottomright.y + 2.0 * precision
          };
          let mut route = Vec::new();
          loop {
            if alternate && y < stop || !alternate && y > stop {
              break;
            }

            let p = (x, y);
            if poly.contains(&Point::new(x, y)) {
              // inside polygon => do line
              if route.len() == 0 {
                route.push(p);
              }
            } else {
              // outside => stop line
              if route.len() > 0 {
                route.push(p);
                routes.push(route);
                route = Vec::new();
              }
            }

            if alternate {
              y -= precision;
            } else {
              y += precision;
            }
          }

          x += yinterval;
          alternate = !alternate;
        }
      }

      let route = routes.concat();

      Some((color, vec![route]))
    })
    .collect();

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      let mut l = layer(color);
      for (clri, routes) in routes.clone() {
        if clri == i {
          for route in routes.iter() {
            data = render_route(data, route.clone());
          }
        }
      }
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
