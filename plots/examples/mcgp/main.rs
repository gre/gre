use clap::*;
use gre::*;
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
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut routes = Vec::new();

  routes.push(circuit(width));

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

fn circuit(width: f64) -> Vec<(f64, f64)> {
  let data = vec![
    (29, 61),
    (119, 54),
    (184, 56),
    (217, 61),
    (251, 52),
    (286, 68),
    (322, 47),
    (369, 141),
    (436, 113),
    (489, 145),
    (529, 131),
    (600, 153),
    (631, 110),
    (674, 76),
    (689, 32),
    (719, 15),
    (756, 10),
    (792, 64),
    (844, 80),
    (834, 45),
    (797, 2),
    (861, 39),
    (998, 170),
    (857, 389),
    (668, 269),
    (320, 297),
    (3, 135),
    (29, 61),
  ];

  let ratio = width / 1000.0;
  data
    .iter()
    .map(|&(x, y)| (ratio * (x as f64), ratio * (y as f64)))
    .collect()
}
