use clap::*;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let height = 297.;
  let width = 210.;
  let pad = 10.0;
  let pad2 = 5.0;
  let colors = vec![
    "rgb(51,51,51)",
    "rgb(249,76,76)",
    "rgb(0,183,249)",
    "rgb(170,107,191)",
    "rgb(114,140,178)",
    "rgb(255,102,191)",
    "rgb(255,178,51)",
    "rgb(255,107,12)",
    "rgb(0,153,153)",
  ];
  let mut rng = rng_from_seed(opts.seed);
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let height_space = height - colors.len() as f64 * pad2 - 2. * pad;
      let h = height_space / (colors.len() as f64);
      let w = width - 2. * pad;

      let mut route = Vec::new();

      let count = 800;
      for i in 0..count {
        let pi = i as f64 / (count as f64);
        let p = pi.powf(2.0);
        let sign = if i % 2 == 0 { -1. } else { 1. };
        let dy = sign
          * 0.5
          * rng.gen_range(0f64, 1.).powf(0.5)
          * h
          * (1.0 - pi.powf(6.))
          * (pi * 4.).min(1.).powf(0.5);
        route.push((w * p + pad, (ci as f64 + 0.5) * (h + pad2) + dy + pad));
      }

      let mut l = layer(color);
      let mut data = Data::new();
      data = render_route(data, route);
      l = l.add(base_path(color, 0.2, data));
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
