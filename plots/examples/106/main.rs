use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let height = 297f64;
  let width = 210f64;
  let w = 180.;
  let h = 260.;
  let granularity = 2f64;
  let counts = [500, 1500];
  let colors = vec!["seagreen", "#713"];
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();

  let sx = (width - w) / 2.;
  let sy = (height - h) / 2.;
  colors
    .iter()
    .enumerate()
    .map(|(g, color)| {
      let count = counts[g];
      let data = (0..count)
        .map(|_i| {
          let mut route = Vec::new();
          /*
          let x = sx
              + rng.gen_range(
                  w * ((i as f64)
                      / (count as f64)
                      - 0.2),
                  w * ((i as f64)
                      / (count as f64)
                      + 0.2),
              );
              */
          let x = sx + rng.gen_range(0., w);
          let y = sy
            + rng.gen_range(-0.3 * (1 - g) as f64 * h, h - 0.3 * g as f64 * h);
          let len = rng.gen_range(0.1 * h, 0.8 * h);
          let y_from = y.max(sy);
          let y_to = (y + len).min(sy + h);
          let mut p = (x, y_from);
          loop {
            if p.1 > y_to {
              break;
            }
            route.push(p);
            p.0 += 0.1 * perlin.get([0.1 * p.0, 0.1 * p.1, opts.seed]);
            p.1 += granularity;
          }
          route
        })
        .filter(|r| r.len() >= 2)
        .fold(Data::new(), |data, route| render_route(data, route));

      let mut l = layer(color);
      l = l.add(base_path(color, 0.3, data));
      if g == colors.len() - 1 {
        l = l.add(signature(2.0, (150.0, 276.0), color));
      }
      l
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
  let mut document = base_a4_portrait("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
