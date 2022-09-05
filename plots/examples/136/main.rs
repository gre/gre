use clap::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: &Opts) -> Vec<Group> {
  let progress = opts.index as f64 / opts.frames as f64;
  let (w, h, sign_pos) = (297, 210, (240.0, 180.));
  let precision = 0.5;
  let seed = opts.seed;
  let width = (w as f64 / precision) as u32;
  let height = (h as f64 / precision) as u32;
  let perlin = Perlin::new();
  let ratio = width as f64 / height as f64;
  let f = |(x, y): (f64, f64)| {
    let freq1 = opts.freq1;
    let amp2 = opts.amp2 * (0.5 + 0.5 * ((progress + x) * 2. * PI).cos());
    let freq2 = opts.freq2;
    let amp3 = opts.amp3 * (0.5 + 0.5 * ((progress + x) * 2. * PI).sin());
    let freq3 = opts.freq3;
    let amp4 = opts.amp4;
    let freq4 = opts.freq4;

    -0.6
      + 5.5 * euclidian_dist((x, y), (0.5, 0.5))
      + perlin.get([
        ratio * freq1 * x
          + amp2
            * perlin.get([
              ratio * freq2 * x,
              freq2 * y,
              seed
                + 1.
                + amp3 * perlin.get([ratio * freq3 * x, freq3 * y, seed + 11.]),
            ]),
        freq1 * y
          + amp2 * perlin.get([ratio * freq2 * x, freq2 * y, seed + 21.])
          + amp3 * perlin.get([ratio * freq3 * x, freq3 * y, seed + 31.]),
        amp4 * perlin.get([ratio * freq4 * x, freq4 * y, seed + 41.]),
      ])
  };
  let mut thresholds: Vec<f64> =
    (0..opts.layers).map(|i| i as f64 * opts.mult).collect();
  thresholds.push(thresholds[thresholds.len() - 1] + 0.001);
  thresholds.push(thresholds[0] - 0.001);

  let res = contour(width, height, f, &thresholds);
  let mut data = Data::new();
  let routes = crop_routes(
    &features_to_routes(res, precision),
    (10.0, 10.0, (width - 10) as f64, (height - 10) as f64),
  );
  for route in routes {
    data = render_route(data, route);
  }
  let color = "black";
  let mut l = layer(color);
  l = l.add(base_path(color, 0.2, data));
  l = l.add(signature(1.0, sign_pos, color));
  vec![l]
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "2")]
  index: usize,
  #[clap(short, long, default_value = "8")]
  frames: usize,
  #[clap(short, long, default_value = "50")]
  layers: usize,
  #[clap(short, long, default_value = "0.03")]
  mult: f64,
  #[clap(short, long, default_value = "158.")]
  seed: f64,
  #[clap(short, long, default_value = "1.")]
  freq1: f64,
  #[clap(short, long, default_value = "1.6")]
  amp2: f64,
  #[clap(short, long, default_value = "3.")]
  freq2: f64,
  #[clap(short, long, default_value = "2.")]
  amp3: f64,
  #[clap(short, long, default_value = "2.6")]
  freq3: f64,
  #[clap(short, long, default_value = "0.2")]
  amp4: f64,
  #[clap(short, long, default_value = "20.")]
  freq4: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
