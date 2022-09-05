use clap::*;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let h = 297.0;
  let w = 210.0;

  vec!["black"]
    .iter()
    .enumerate()
    .map(|(_i, color)| {
      let pad = opts.pad;
      let dy = opts.dy;
      let width = w - 2.0 * pad;
      let x1 = pad;
      let x2 = x1 + width * 0.25;
      let xm = x1 + width * 0.5;
      let x3 = x1 + width * 0.75;
      let x4 = w - pad;

      let mut data = Data::new();

      let mut y = pad + dy / 2.0;
      data = data.move_to((x1, y));

      loop {
        if y > h - 2.0 * pad {
          break;
        }
        data = data.quadratic_curve_to((x2, y - dy, xm, y));
        data = data.quadratic_curve_to((x3, y + dy, x4, y));

        y += opts.jump;

        data = data.quadratic_curve_to((x3, y - dy, xm, y));
        data = data.quadratic_curve_to((x2, y + dy, x1, y));

        y += opts.jump;
      }

      let mut l = layer(color);
      l = l.add(base_path(color, 0.5, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "20.")]
  pad: f64,
  #[clap(short, long, default_value = "30.")]
  dy: f64,
  #[clap(short, long, default_value = "1.")]
  jump: f64,
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
