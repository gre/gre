use clap::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  width: f64,
  #[clap(short, long, default_value = "210.0")]
  height: f64,
  #[clap(short, long, default_value = "10.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);
  let mut routes = vec![];

  let w = ((width - 2. * pad) / 10.0).floor() * 10.0;
  let h = ((height - 2. * pad) / 10.0).floor() * 10.0;
  let xfrom = (width - w) / 2.0;
  let yfrom = (height - h) / 2.0;
  let xto = xfrom + w;
  let yto = yfrom + h;

  let mut curve = vec![];

  let f = rng.gen_range(0.02, 0.1);
  let amp = rng.gen_range(0.1, 0.4) * h;

  let mut x = 0.0;
  while x <= w {
    let line = vec![(xfrom + x, yfrom), (xfrom + x, yto)];
    if x % 10.0 < 1.0 {
      routes.push((1, line.clone()));
    }
    routes.push((1, line.clone()));
    x += 2.0;

    let v = amp * (x * f).cos();

    if 10.0 < x && x < w - 10.0 {
      curve.push((x + xfrom, height / 2.0 + v));
      if 20.0 < x && x < w - 20.0 {
        if rng.gen_bool(0.5) {
          let p = (
            x + xfrom + rng.gen_range(-0.5, 0.5),
            height / 2.0
              + v
              + rng.gen_range(-10.0, 10.0) * rng.gen_range(0.0, 1.0),
          );
          routes
            .push((0, vec![(p.0 - 1.0, p.1 - 1.0), (p.0 + 1.0, p.1 + 1.0)]));
          routes
            .push((0, vec![(p.0 + 1.0, p.1 - 1.0), (p.0 - 1.0, p.1 + 1.0)]));
        }
      }
    }
  }

  routes.push((2, curve.clone()));

  let mut y = 0.0;
  while y <= h {
    let line = vec![(xfrom, yfrom + y), (xto, yfrom + y)];
    routes.push((1, line.clone()));
    if y % 10.0 < 1.0 {
      routes.push((1, line));
    }
    y += 2.0;
  }

  vec!["#000", "#e50", "#f00"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
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
