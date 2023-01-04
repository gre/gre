use std::f64::consts::PI;

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
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let bound = (pad, pad, width - pad, height - pad);

  let get_color =
    image_get_color("images/bladerunnerfinalcutposter.jpg").unwrap();

  // get_color()

  let mut rng = rng_from_seed(opts.seed);

  let mut data = vec![];

  let amp = 0.4;
  let dim = 900;
  let probability = 0.1;

  for (color, rgb, pow, samples) in vec![
    ("#0af", (0.1, 0.6, 1.), 2.8, 7000),
    ("#F39", (1., 0.2, 0.), 1.1, 16000),
    ("#Fc0", (1., 0.7, 0.), 1.8, 8000),
    ("white", (0.9, 0.9, 0.9), 2.0, 40000),
  ] {
    let mut passage = Passage::new(1.0, opts.width, opts.height);
    let passage_max = 2;

    let mut routes = Vec::new();
    let f = |p| {
      let c = get_color(p);
      let dr = rgb.0 - c.0;
      let dg = rgb.1 - c.1;
      let db = rgb.2 - c.2;
      let d = (dr * dr + dg * dg + db * db).sqrt();
      probability * (1.0 - d.powf(pow))
    };

    let points = sample_2d_candidates_f64(&f, dim, samples, &mut rng);

    for p in points {
      let (x, y) = project_in_boundaries(p, bound);
      if passage.count((x, y)) > passage_max {
        continue;
      }
      let ang = rng.gen_range(-PI, PI);
      let dx = ang.cos() * amp;
      let dy = ang.sin() * amp;
      routes.push(vec![(x - dx, y - dy), (x + dx, y + dy)]);
    }

    data.push((routes, color));
  }

  data
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.4, data));
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

#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }
}
