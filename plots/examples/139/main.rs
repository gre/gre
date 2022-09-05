use clap::*;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn spiral(
  data: Data,
  rot: f64,
  origin: (f64, f64),
  initial_offset: f64,
  length: f64,
  d_length: f64,
) -> Data {
  let mut d = data;
  let mut a: f64 = 0.0;
  let mut p = origin;
  let mut l = length;
  d = d.move_to((p.0 + initial_offset, p.1));
  loop {
    if l < 0.0 {
      break;
    }
    p = (p.0 + l * a.cos(), p.1 + l * a.sin());
    d = d.line_to(p);
    a += rot;
    l -= d_length;
  }
  d
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.5")]
  angle: f64,
  #[clap(short, long, default_value = "170.")]
  length: f64,
  #[clap(short, long, default_value = "0.8")]
  step: f64,
}

fn art(opts: Opts) -> Vec<Group> {
  vec![
    layer("brush").add(base_path(
      "black",
      0.5,
      spiral(
        Data::new(),
        opts.angle * PI,
        (60., 20.),
        1.0,
        opts.length,
        opts.step,
      ),
    )),
    layer("signature").add(signature(2.0, (120.0, 185.0), "black")),
  ]
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
