use clap::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "7.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "2.0")]
  pub incr: f64,
  #[clap(short, long, default_value = "0.5")]
  pub step: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let pad = opts.pad;

  let mut routes = Vec::new();

  let bound = (pad, pad, width - pad, width - pad);

  let mut rng = rng_from_seed(opts.seed);
  let speed = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let iangdiff = rng.gen_range(-1.0, 1.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);
  let iamp = rng.gen_range(0.0, 2.0);

  let incr = rng.gen_range(0.8, 2.0) * opts.incr;
  let count = rng.gen_range(3, 5);

  let step = opts.step;
  let dx = step / 2.0;
  let dy = -step / 2.0;

  for i in 0..count {
    let mut yfrom = bound.1;
    loop {
      let mut route = vec![];

      let mut p = (bound.0, yfrom);
      loop {
        if p.0 > bound.2 {
          break;
        }
        let centerdx = p.0 - (bound.2 + bound.0) / 2.0;
        let centerdy = p.1 - (bound.3 + bound.1) / 2.0;
        let length = (centerdx * centerdx + centerdy * centerdy).sqrt();
        let angle = -centerdy.atan2(centerdx);
        let l = speed * length + i as f64 * iangdiff;
        let amp = (i as f64 - 1.0)
          * iamp
          * smoothstep(width * 0.3, width * 0.1, (length - width * 0.15).abs())
          * smoothstep(0.5, 3.0, length);
        let disp = (amp * (angle + l).cos(), amp * (angle + l).sin());
        if strictly_in_boundaries(p, bound) {
          route.push((p.0 + disp.0, p.1 + disp.1));
        }
        p = (p.0 + dx, p.1 + dy);
      }

      if route.len() == 0 && yfrom > bound.3 {
        break;
      }
      yfrom += incr;
      routes.push(rdp(&route, 0.1));
    }
  }

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
