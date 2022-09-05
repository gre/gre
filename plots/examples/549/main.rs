use clap::*;
use gre::*;
use noise::*;
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
}

fn art(opts: &Opts) -> Vec<Group> {
  let colors = vec!["red", "yellow"];
  let pad = 20.0;
  let size = opts.width.min(opts.height) - 2.0 * pad;
  let incr = 0.3;
  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      let mut y = pad;
      loop {
        if y > pad + size {
          break;
        }
        let x = if i == 0 {
          mix(pad, pad + size, 0.1 + 0.8 * (y - pad) / size)
        } else {
          pad + size
        };
        data = render_route(data, vec![(pad, y), (x, y)]);
        y += incr;
      }
      let mut l = layer(color);
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
