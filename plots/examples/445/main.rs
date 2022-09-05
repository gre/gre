use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::{path::Data, Group};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
  height: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let seed = opts.seed;
  let pad = 10.0;
  let boundaries = (pad, pad, width - pad, height - pad);
  let stroke_width = 0.35;
  let precision = 0.5;
  let color = "black";
  let perlin = Perlin::new();

  let mut rng = rng_from_seed(opts.seed);
  let lines = rng.gen_range(24, 48);
  let rows = lines;
  let length = rng.gen_range(300.0, 500.0) / (lines as f64);
  let a1 = rng.gen_range(2.0, 6.0);
  let a2 = rng.gen_range(0.3, 1.0);
  let a3 = rng.gen_range(0.3, 2.0);
  let f1 = rng.gen_range(1.0, 8.0) * rng.gen_range(0.2, 1.0);
  let f2 = rng.gen_range(3.0, 9.0);
  let f3 = rng.gen_range(8.0, 40.0);

  let field = |(x, y): (f64, f64)| {
    a1 * perlin.get([
      f1 * x,
      f1 * y,
      seed
        + a2
          * perlin.get([
            -seed + a3 * perlin.get([f3 * x, f3 * y, 1.0 + seed]),
            f2 * x,
            f2 * y,
          ]),
    ])
  };

  let iterations = (length / precision) as usize;
  let mut routes = Vec::new();
  for l in 0..lines {
    for r in 0..rows {
      let mut p = (
        boundaries.0
          + (boundaries.2 - boundaries.0) * (l as f64) / (lines as f64),
        boundaries.1
          + (boundaries.2 - boundaries.0) * (r as f64) / (rows as f64),
      );
      let mut route = Vec::new();
      for _i in 0..iterations {
        let normalized = (
          (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
          (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
        );
        let angle = field(normalized);
        let (px, py) = p;
        p = (p.0 + precision * angle.cos(), p.1 + precision * angle.sin());
        if p.0 < boundaries.0
          || p.0 > boundaries.2
          || p.1 < boundaries.1
          || p.1 > boundaries.3
        {
          break;
        }
        let x = px;
        let y = py;
        route.push((x, y));
      }
      routes.push(route);
    }
  }

  let mut layers = Vec::new();
  let mut l = layer(color);
  let mut data = Data::new();
  for r in routes.clone() {
    data = render_route(data, r);
  }
  l = l.add(base_path(color, stroke_width, data));
  layers.push(l);
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
