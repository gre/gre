use clap::*;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = 210.;
  let height = 297.;
  let pad = 10.0;
  let dy = 40.0;
  let a = 3.0;
  let b = pad + dy;

  let mut rng = rng_from_seed(opts.seed);

  let mut curve = vec![];
  curve.push((width / 2.0, 0.0));
  for _i in 0..16 {
    let d = rng.gen_range(0.6, 1.2);
    curve.push((d * width, 0. + rng.gen_range(-dy, dy)));
    curve.push(((1. - d) * width, 0. + rng.gen_range(-dy, dy)));
  }
  curve.push((width / 2.0, 0.0));

  let count = ((height - b - 30.0) / a) as usize;
  let routes: Vec<(f64, f64)> = (0..count)
    .flat_map(|i| {
      let route: Vec<(f64, f64)> = curve
        .iter()
        .map(|&(x, y)| {
          let point = (x, y + i as f64 * a + b);
          point
        })
        .collect();
      route
    })
    .collect();

  let color = "#fb0";
  let mut data = Data::new();
  data = render_route_curve(data, routes);
  let mut l = layer(color);
  l = l.add(base_path(color, 0.25, data));
  vec![l]
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_a4_portrait("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
