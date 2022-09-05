use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "8.0")]
  seed: f64,
  #[clap(short, long, default_value = "0.5")]
  dy: f64,
  #[clap(short, long, default_value = "297.0")]
  width: f64,
  #[clap(short, long, default_value = "210.0")]
  height: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let colors = vec!["#000", "#000"];
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;
  let stroke_width = 0.25;
  let seed = opts.seed;
  let mut layers = Vec::new();
  let mut rng = rng_from_seed(seed);
  let dy = opts.dy;
  let min_route = 12;
  let res1 = (rng.gen_range(0.01, 0.8), rng.gen_range(0.01, 0.4));
  let res2 = (rng.gen_range(0.01, 0.8), rng.gen_range(0.01, 0.4));
  let r1 = rng.gen_range(0.0, 1.0);
  let r2 = rng.gen_range(0.0, 1.0);
  let r3 = rng.gen_range(0.0, 1.0);
  let r4 = rng.gen_range(0.0, 1.0);
  let xyratiof = rng.gen_range(1.0, 8.0);

  let perlin = Perlin::new();

  let low_poly_perlin = |(xr, yr): (f64, f64), x: f64, y: f64, s: f64| {
    // quadradic interpolation between 4 noise points
    let xi = x / xr;
    let yi = y / yr;
    let x1 = xr * xi.floor();
    let y1 = yr * yi.floor();
    let x2 = xr * xi.ceil();
    let y2 = yr * yi.ceil();
    let xp = xi - xi.floor();
    let yp = yi - yi.floor();
    let p1 = perlin.get([x1, y1, s]);
    let p2 = perlin.get([x2, y1, s]);
    let p3 = perlin.get([x2, y2, s]);
    let p4 = perlin.get([x1, y2, s]);
    mix(
      mix(p1 as f64, p2 as f64, xp),
      mix(p4 as f64, p3 as f64, xp),
      yp,
    )
  };

  for (ci, &color) in colors.iter().enumerate() {
    let from = height * 3.;
    let to = -height;
    let mut routes = Vec::new();
    let mut base_y = from;

    let mut height_map: Vec<f64> = Vec::new();
    loop {
      let precision = rng.gen_range(0.15, 0.25);
      if base_y < to {
        break;
      }
      let is_color = (base_y < height * 0.5) != (ci == 0);
      let mut route = Vec::new();
      let mut x = pad;
      let mut was_outside = true;
      loop {
        if x > width - pad {
          break;
        }
        let xv = (0.5 + base_y / height) * (x - width / 2.);
        let amp = mix(0.6, 1.2, r1)
          * height
          * (1.2 - 0.6 * ((x - width / 2.) / (width / 2.0)).abs())
          * (base_y / height);
        let shape = -low_poly_perlin(
          res1,
          mix(0.3, 0.8, r3) * 0.01 * xv,
          mix(0.3, 0.8, r3) * 0.01 * xyratiof * base_y,
          7.7 * seed
            + mix(0.05, 0.1, r2)
              * low_poly_perlin(
                res2,
                mix(0.2, 1.0, r4) * 0.1 * xyratiof * base_y,
                mix(0.2, 1.0, r4) * 0.1 * xv,
                seed / 3.,
              ),
        )
        .abs();
        let displacement =
          mix(
            0.0008,
            0.01,
            smoothstep(-0.2, -0.5, shape).powf(2.0)
              * (base_y / height).max(0.0).min(1.0),
          ) * perlin.get([seed * 9.3, 0.5 * xyratiof * base_y, 0.5 * x]);
        let y = base_y + amp * (shape + displacement);
        let mut collides = false;
        let xi = (x * 10.0) as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] - dy * 0.1 {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let inside = !collides
          && pad < x
          && x < width - pad
          && pad < y
          && y < height - pad;
        if inside {
          if was_outside {
            if route.len() > min_route && is_color {
              routes.push(route);
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push((x, y));
        } else {
          was_outside = true;
        }
        x += precision;
      }
      if route.len() > min_route && is_color {
        routes.push(route);
      }

      base_y -= dy;
    }

    let mut l = layer(color);
    let mut data = Data::new();
    for r in routes.clone() {
      data = render_route(data, r);
    }
    l = l.add(base_path(color, stroke_width, data));
    layers.push(l);
  }
  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
