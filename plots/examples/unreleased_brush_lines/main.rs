use clap::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let colors = vec!["cyan", "black"];
  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut l = layer(color);
      if i == colors.len() - 1 {
        l = l.add(signature(1.0, (250.0, 190.0), color));
      } else {
        let mut rng = rng_from_seed(opts.seed);
        let mut data = Data::new();
        let width = 297.;
        let height = 210.;
        let pad = 20.;
        for _i in 0..120 {
          let y = rng.gen_range(pad, height - pad);
          data = data.move_to((pad, y));
          data = data.line_to((width - pad, y));
        }
        l = l.add(base_path(color, 2., data));
      }
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "1.0")]
  seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
