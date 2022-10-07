use clap::*;
use geo::prelude::Intersects;
use geo::Polygon;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "148.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed1: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed2: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed3: f64,
}

fn scallop<R: Rng>(
  center: (f64, f64),
  size: f64,
  ang: f64,
  rng: &mut R,
) -> (Polygon, Vec<Vec<(f64, f64)>>) {
  let origin = (center.0, center.1 + size);
  let cuts = 3 + (size * 0.9) as usize;
  let mut points = Vec::new();
  points.push(origin);
  let opening = rng.gen_range(0.5, 0.7);
  for i in 0..cuts {
    let perc = i as f64 / (cuts as f64) - 0.5;
    let a = PI / 2.0 + perc * opening * PI;
    let amp = size * (1.0 - 0.1 * (2.0 * perc).abs().powf(2.0));
    let p = (origin.0 + amp * a.cos(), origin.1 + amp * a.sin());
    points.push(p);
  }

  points = points
    .iter()
    .map(|&p| {
      let p = (p.0, p.1 - size * 0.5);
      let p = p_r((p.0 - center.0, p.1 - center.1), ang);
      let p = (p.0 + center.0, p.1 + center.1);
      p
    })
    .collect();

  let mut routes = Vec::new();
  let origin = points[0];
  // routes.push(vec![points.clone(), vec![origin]].concat());
  let a = p_r((0.0, 0.08 * size), ang);
  let origin = (origin.0 + a.0, origin.1 + a.1);
  for p in points.iter().skip(2).take(cuts - 2) {
    let k = 0.15;
    let l = 0.85;
    let a = (mix(origin.0, p.0, k), mix(origin.1, p.1, k));
    let b = (mix(origin.0, p.0, l), mix(origin.1, p.1, l));
    routes.push(vec![a, b]);
  }

  let w = 0.25 * size;
  let h = 1.1;
  let rect: Vec<(f64, f64)> = vec![
    (center.0 - w, center.1 + h * size),
    (center.0 - w, center.1 + 1.0 * size),
    (center.0 + w, center.1 + 1.0 * size),
    (center.0 + w, center.1 + h * size),
  ]
  .iter()
  .map(|&p| {
    let p = (p.0, p.1 - size * 0.5);
    let p = p_r((p.0 - center.0, p.1 - center.1), ang);
    let p = (p.0 + center.0, p.1 + center.1);
    p
  })
  .collect();

  points = vec![rect, points.iter().skip(1).map(|&p| p).collect()].concat();
  points.push(points[0]);
  routes.push(points.clone());

  let poly = Polygon::new(points.into(), vec![]);
  (poly, routes)
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();
  let len = 2000;

  let min_route = 2;
  let peakfactor = rng.gen_range(-0.001, 0.002)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);
  let stopy = rng.gen_range(0.1, 0.4) * height;
  let ampfactor = rng.gen_range(0.03, 0.1);
  let ynoisefactor = rng.gen_range(0.02, 0.2);
  let yincr = rng.gen_range(0.3, 1.0);
  let amp2 = rng.gen_range(1.0, 12.0);
  let precision = 0.1;
  let offsetstrategy = rng.gen_range(0, 5);

  let mut points = Vec::new();

  let mut routes = Vec::new();

  let mut base_y = height * 5.0;
  let mut miny = height;
  let mut height_map: Vec<f64> = Vec::new();
  loop {
    if miny < stopy {
      break;
    }

    let mut route = Vec::new();
    let mut x = pad;
    let mut was_outside = true;
    loop {
      if x > width - pad {
        break;
      }
      let xv = (4.0 - base_y / height) * (x - width / 2.);

      let amp = height * ampfactor;
      let mut y = base_y;

      if offsetstrategy == 0 {
        y += amp * peakfactor * xv * xv;
      }

      y += -amp
        * perlin
          .get([
            //
            xv * 0.004 + 9.9,
            y * 0.02 - 3.1,
            77.
              + opts.seed / 7.3
              + perlin.get([
                //
                -opts.seed * 7.3,
                8.3 + xv * 0.015,
                y * 0.1,
              ]),
          ])
          .abs();

      if offsetstrategy == 1 {
        y += amp * peakfactor * xv * xv;
      }

      y += amp2
        * amp
        * perlin.get([
          //
          8.3 + xv * 0.008,
          88.1 + y * ynoisefactor,
          opts.seed * 97.3,
        ]);

      if offsetstrategy == 2 {
        y += amp * peakfactor * xv * xv;
      }

      y += amp
        * perlin.get([
          //
          opts.seed * 9.3 + 77.77,
          xv * 0.08 + 9.33,
          y * 0.5,
        ])
        * perlin
          .get([
            //
            xv * 0.015 - 88.33,
            88.1 + y * 0.2,
            -opts.seed / 7.7 - 6.66,
          ])
          .min(0.0);

      if offsetstrategy == 3 {
        y += amp * peakfactor * xv * xv;
      }

      y += 0.1
        * amp
        * (1.0 - miny / height)
        * perlin.get([
          //
          6666.6 + opts.seed * 1.3,
          8.3 + xv * 0.5,
          88.1 + y * 0.5,
        ]);

      if offsetstrategy == 4 {
        y += amp * peakfactor * xv * xv;
      }

      if y < miny {
        miny = y;
      }
      let mut collides = false;
      let xi = (x / precision) as usize;
      if xi >= height_map.len() {
        height_map.push(y);
      } else {
        if y > height_map[xi] {
          collides = true;
        } else {
          height_map[xi] = y;
        }
      }
      let inside =
        !collides && pad < x && x < width - pad && pad < y && y < height - pad;
      if inside {
        if was_outside {
          if route.len() > min_route {
            routes.push(route);
          }
          route = Vec::new();
        }
        was_outside = false;
        points.push((x, y));
      } else {
        was_outside = true;
      }

      x += precision;
    }

    if route.len() > min_route {
      routes.push(route);
    }

    base_y -= yincr;
    if rng.gen_bool(0.02) {
      base_y -= rng.gen_range(0.0, 10.0);
    }
  }

  rng.shuffle(&mut points);

  let perlin = Perlin::new();

  let noise_freq = rng.gen_range(0.0, 0.1)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);

  let min_size = rng.gen_range(1.5, 2.5);
  let max_size = rng.gen_range(min_size, 20.0);

  let mut polygons = Vec::new();
  let mut scallops = Vec::new();
  for p in points {
    if polygons.len() >= len {
      break;
    }
    let size = rng.gen_range(min_size, max_size);
    let ang = 2.0
      * PI
      * perlin.get([
        7.7 * opts.seed + 666.666,
        p.0 * noise_freq,
        p.1 * noise_freq,
      ]);
    let (poly, r) = scallop(p, size, ang, &mut rng);
    if !poly.exterior().points().all(|p| {
      strictly_in_boundaries(p.x_y(), (pad, pad, width - pad, height - pad))
    }) {
      continue;
    }
    if polygons.iter().all(|p| !poly.intersects(p)) {
      polygons.push(poly);
      scallops.push(r);
    }
  }

  routes = vec![routes, scallops.concat()].concat();

  for i in 0..10 {
    let d = i as f64 * 0.25;
    routes.push(vec![
      (pad + d, pad + d),
      (pad + d, height - pad - d),
      (width - pad - d, height - pad - d),
      (width - pad - d, pad + d),
      (pad + d, pad + d),
    ]);
  }

  let color = "black";
  let mut data = Data::new();
  for route in routes.clone() {
    data = render_route(data, route);
  }
  let mut l = layer(color);
  l = l.add(base_path(color, 0.35, data));
  vec![l]
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
