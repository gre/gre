use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "320.0")]
  height: f64,
  #[clap(short, long, default_value = "240.0")]
  width: f64,
  #[clap(short, long, default_value = "20.0")]
  pad: f64,
  #[clap(short, long, default_value = "8433.0")]
  seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();

  let freqx = rng.gen_range(0.0, 0.03) * rng.gen_range(0.0, 1.0);
  let freqy = freqx;
  let freq2x = rng.gen_range(0.02, 0.06) * rng.gen_range(0.2, 1.0);
  let freq2y =
    freq2x * (1.0 + rng.gen_range(-1.0, 3.0) * rng.gen_range(0.0, 1.0));
  let angleamp = rng.gen_range(1.0, 2.0) * rng.gen_range(0.2, 1.0);

  let size = 20.0;

  let mut positions = vec![];
  let mut x = pad + size;
  while x < width - pad - size {
    let mut y = pad + size;
    while y < height - pad - size {
      positions.push((x, y));
      y += rng.gen_range(0.2, 0.5) * size;
    }
    x += rng.gen_range(0.2, 0.5) * size;
  }

  for _ in 0..300 {
    let x = rng.gen_range(pad + size, width - pad - size);
    let y = rng.gen_range(pad + size, height - pad - size);
    positions.push((x, y));
  }

  let mut routes = Vec::new();
  for (x, y) in positions {
    let mut local: Vec<_> = vec![];
    let s = rng.gen_range(0.3, 1.0) * size;
    let n2 = perlin.get([
      // angle follow a noise field
      freq2x * x,
      freq2y * y,
      100. + opts.seed / 0.3794,
    ]);
    let n = perlin.get([
      // angle follow a noise field
      freqx * x,
      freqy * y,
      100. + opts.seed / 0.03794,
    ]);
    let ang = angleamp * n;
    let clr =
      (((n2 + 0.5) * 2.0) as usize + (2.0 * y / height as f64) as usize) % 2;
    local.push(vec![(-1.0, 0.0), (1.0, 0.0)]);
    let acos = ang.cos();
    let asin = ang.sin();
    routes.extend(local.iter().map(|rt| {
      (
        clr,
        rt.iter()
          .map(|p| {
            // rotate
            let p = (p.0 * acos - p.1 * asin, p.0 * asin + p.1 * acos);
            // scale
            let p = (p.0 * s, p.1 * s);
            // translate
            let p = (p.0 + x, p.1 + y);
            p
          })
          .collect(),
      )
    }));
  }

  vec!["white", "black"]
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 4.0, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("grey", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
