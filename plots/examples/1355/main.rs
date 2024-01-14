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

  let line_d = 0.9;
  let stripediv = 3;
  let gridw = 12;
  let gridh = 9;

  let cellh = (height - 2. * pad) / (gridh as f64);
  let cellw = (width - 2. * pad) / (gridw as f64);

  let mut rng = rng_from_seed(opts.seed);

  let mut stripes = vec![false; gridw * gridh];
  let mut grid1 = vec![0; gridw * gridh];
  let mut grid2 = vec![0; gridw * gridh];

  let k1 = (1.0 + rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0)) as usize;
  let k2 = (1.0 + rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0)) as usize;
  let k3 = (1.0 + rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0)) as usize;
  let k4 = (1.0 + rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0)) as usize;

  let panomalystripe = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);
  let panomalygrid1 = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);
  let panomalygrid2 = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);

  let stripemod = if rng.gen_bool(0.8) {
    2
  } else {
    rng.gen_range(2, 10)
  };

  let off1 = rng.gen_range(0, 3);
  let off2 = rng.gen_range(0, 3);

  for x in 0..gridw {
    for y in 0..gridh {
      let i = x + y * gridw;
      grid1[i] = (x / k1 + y / k2 + off1) % 3;
      if grid1[i] == 0 {
        grid1[i] = 2;
      }
      grid2[i] = (x / k3 + y / k4 + off2) % 3;
      if grid2[i] == 2 {
        grid2[i] = 0;
      }
      stripes[i] = (x / 2 + y) % stripemod == 0;
      if rng.gen_bool(panomalystripe) {
        stripes[i] = !stripes[i];
      }

      if rng.gen_bool(panomalygrid1) {
        grid1[i] = rng.gen_range(0, 4);
      }
      if rng.gen_bool(panomalygrid2) {
        grid2[i] = rng.gen_range(0, 4);
      }
    }
  }

  let mut x = pad;
  while x < width - pad {
    let xi = (gridw as f64 * (x - pad) / (width - 2. * pad)) as usize;
    for yi in 1..(gridh + 1) {
      let y1 = pad + (yi - 1) as f64 * cellh;
      let y2 = pad + yi as f64 * cellh;
      let clr = grid1[xi + (yi - 1) * gridw];
      routes.push((clr, vec![(x, y1), (x, y2)]));
    }
    x += line_d;
  }

  let mut y = pad;
  let mut linei = 0;
  while y < height - pad {
    let yi = (gridh as f64 * (y - pad) / (height - 2. * pad)) as usize;
    for xi in 1..(gridw + 1) {
      let x1 = pad + (xi - 1) as f64 * cellw;
      let x2 = pad + xi as f64 * cellw;
      let stripe = stripes[xi - 1 + yi * gridw];
      let clr = grid2[xi - 1 + yi * gridw];
      let alt = (linei / stripediv) % 2 == 0;
      let clr = if stripe && alt { (clr + 1) % 3 } else { clr };
      routes.push((clr, vec![(x1, y), (x2, y)]));
    }
    y += line_d;
    linei += 1;
  }

  let mut colors = vec![
    "#0f9", // soft mint
    "#bbb", // moonstone
    "#f60", // pumpkin
    "#fc2", // amber
    "#224", // indigo
    "#f33", // poppy red
  ];
  rng.shuffle(&mut colors);
  colors.truncate(3);

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
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
