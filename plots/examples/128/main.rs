use clap::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let width = 210f64;
  let height = 297f64;
  let perlin = Perlin::new();

  let samples = 100000;
  let f1 = (opts.f1x, opts.f1y);
  let f2 = (opts.f2x, opts.f2y);
  let amp1 = opts.amp1;
  let amp2 = opts.amp2;
  let spins = opts.spins;
  let splits = opts.splits;
  let split_threshold = opts.split_threshold;
  let pow = opts.pow;
  let noise_angle_freq = opts.noise_angle_freq;
  let noise_angle_displacement_freq = opts.noise_angle_displacement_freq;
  let noise_angle_displacement = opts.noise_angle_displacement;

  let colors = vec!["darkblue", "firebrick"];
  let mut layers = Vec::new();
  for (i, color) in colors.iter().enumerate() {
    let size = opts.size - i as f64 * opts.size_diff;
    let parametric = |p: f64| {
      let p1 = (splits * p).floor();
      let p2 = splits * p - p1;
      let t = (p1 + split_threshold * p2) / splits;
      let t2 = (p1 + split_threshold * p2.powf(pow)) / splits;
      let scale = 1.0 - t2;
      let s = spins;
      let mut p = (
        scale
          * amp1
          * ((s * 2. * PI * t).sin()
            + amp2
              * mix((s * f1.1 * PI * t).sin(), (s * f2.1 * PI * t).sin(), t)),
        scale
          * amp1
          * ((s * 2. * PI * t).cos()
            + amp2
              * mix((s * f1.0 * PI * t).cos(), (s * f2.0 * PI * t).cos(), t)),
      );
      let noise_angle = 2.
        * PI
        * perlin.get([
          noise_angle_freq * p.0,
          noise_angle_freq * p.1,
          100.0 + 0.1 * opts.seed + i as f64 * opts.seed_diff,
        ]);
      let noise_amp = noise_angle_displacement
        * perlin.get([
          noise_angle_displacement_freq * p.0,
          noise_angle_displacement_freq * p.1,
          0.1 * opts.seed + i as f64 * opts.seed_diff,
        ]);
      p.0 += noise_amp * noise_angle.cos();
      p.1 += noise_amp * noise_angle.sin();
      p
    };

    let mut routes = Vec::new();
    let mut route = Vec::new();
    let mut last = (-1000.0, -1000.0);
    for i in 0..(samples + 1) {
      let sp = i as f64 / (samples as f64);
      let o = parametric(sp);
      let p = (width * 0.5 + size * o.0, height * 0.5 + size * o.1);
      if euclidian_dist(p, last) > 2.0 {
        routes.push(route);
        route = Vec::new();
      }
      route.push(p);
      last = p;
    }
    routes.push(route);

    let data = routes
      .iter()
      .fold(Data::new(), |data, route| render_route(data, route.clone()));
    let mut l = layer(color);
    if i == 0 {
      l = l.add(signature(1.5, (150.0, 220.0), color));
    }
    l = l.add(base_path(color, 0.5, data));
    layers.push(l);
  }
  layers
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "10.0")]
  seed: f64,
  #[clap(short, long, default_value = "0.08")]
  seed_diff: f64,
  #[clap(short, long, default_value = "90.")]
  size: f64,
  #[clap(short, long, default_value = "1.")]
  size_diff: f64,
  #[clap(short, long, default_value = "12.")]
  f1x: f64,
  #[clap(short, long, default_value = "12.")]
  f1y: f64,
  #[clap(short, long, default_value = "2.")]
  f2x: f64,
  #[clap(short, long, default_value = "2.")]
  f2y: f64,
  #[clap(short, long, default_value = "0.9")]
  amp1: f64,
  #[clap(short, long, default_value = "0.2")]
  amp2: f64,
  #[clap(short, long, default_value = "80.0")]
  spins: f64,
  #[clap(short, long, default_value = "1.0")]
  splits: f64,
  #[clap(short, long, default_value = "1.0")]
  split_threshold: f64,
  #[clap(short, long, default_value = "1.1")]
  pow: f64,
  #[clap(short, long, default_value = "1.3")]
  noise_angle_freq: f64,
  #[clap(short, long, default_value = "2.9")]
  noise_angle_displacement_freq: f64,
  #[clap(short, long, default_value = "0.06")]
  noise_angle_displacement: f64,
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
