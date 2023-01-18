use clap::*;
use gre::*;
use std::f64::consts::PI;
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
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let precision = 0.1;

  let mut routes = Vec::new();

  let mut route = vec![];
  let size = 2.0;
  let mut da = 1.0;
  let dx = 2.6 * size;

  let gridh = 2 * ((0.25 * (height - 2.0 * pad) / size).floor() as usize);
  let gridw = 2 * ((0.5 * (width - 2.0 * pad - dx) / dx).floor() as usize) + 1;

  let mut p = (pad + dx / 2.0, pad);
  let mut xi = 0;
  let mut yi = 0;
  let mut a = 0.0f64;
  loop {
    if yi >= gridh {
      xi += 1;
      yi = 0;
      da = -da;
      p.0 += dx;
    }
    if xi >= gridw {
      break;
    }
    route.push(p);
    p = (p.0 + precision * a.cos(), p.1 + precision * a.sin());
    a += da * precision / size;
    let offseta = ((xi % 2) as f64) * PI;
    if a >= PI - offseta {
      a = PI - offseta;
      da = -da;
      yi += 1;
      if xi < gridw - 1 {
        routes.push(vec![
          (p.0 + dx * 0.2, p.1 + dx * 0.4),
          (p.0 + dx * 0.8, p.1 + dx * 0.4),
        ]);
      }
    } else if a <= -offseta {
      a = -offseta;
      da = -da;
      yi += 1;
    }
  }

  let mut rev = route.clone();
  rev.reverse();
  route.append(&mut rev);
  routes.push(route);

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
