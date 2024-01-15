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
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut rng = rng_from_seed(opts.seed);
  let mut routes = vec![];

  let max = rng.gen_range(0.0, 1000.0);
  let a = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let c = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let d = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let e = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let g = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let h = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let j = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let k = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let m = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);
  let n = rng.gen_range(0.0f64, max) * rng.gen_range(0.0, 1.0);

  let res = 0.3
    + rng.gen_range(0.0, 10.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
  let xsplits = (width / res) as usize;
  let ysplits = rng.gen_range(20, 100);
  let amp = rng.gen_range(1.0, 10.0);
  let ampmin = rng.gen_range(-1.0f64, 2.0).min(1.0);

  for y in 0..ysplits + 1 {
    let mut route = vec![];
    let yf = y as f64 / (ysplits as f64);
    let ycd = 2. * (0.5 - (yf - 0.5).abs());
    for x in 0..xsplits + 1 {
      let xf = x as f64 / (xsplits as f64);
      let b = xf;
      let f = yf;
      let i = xf;
      let l = yf;
      // the formula of genuary =)
      let v = (a * b + c + d * (e * f + g).sin()).sin()
        + (h * i + j + k * (l * m + n).sin()).sin();
      let x = mix(pad, width - pad, xf);
      let y = mix(pad + 2. * amp, height - pad - 2. * amp, yf);
      let y = y + v * mix(ampmin, amp, ycd);
      route.push((x, y));
    }
    routes.push((0, route));
  }

  vec!["white"]
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
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
