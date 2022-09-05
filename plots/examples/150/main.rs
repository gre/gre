use clap::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: &Opts) -> Vec<Group> {
  let progress = opts.index as f64 / opts.frames as f64;
  let width = 297f64;
  let height = 210f64;
  let frame = (progress * 34.) as usize;
  let get_color =
    image_gif_get_color("images/spinner_dancer.gif", frame).unwrap();
  let perlin = Perlin::new();
  let samples = 200000;
  let f1 = (opts.f1x, opts.f1y);
  let f2 = (opts.f2x, opts.f2y);
  let amp1 = opts.amp1;
  let amp2 = opts.amp2;
  let spins = opts.spins;
  let splits = opts.splits;
  let split_threshold = opts.split_threshold;
  let pow = opts.pow;

  let colors = vec!["black", "black"];
  let mut layers = Vec::new();
  for (i, color) in colors.iter().enumerate() {
    let size = opts.size - i as f64 * opts.size_diff;
    let parametric = |p: f64| {
      let p1 = (splits * p).floor();
      let p2 = splits * p - p1;
      let t = (p1 + split_threshold * p2) / splits;
      let mut t2 = (p1 + split_threshold * p2.powf(pow)) / splits;
      let initial = 1. / spins;
      t2 = (t2 - initial).max(0.) / (1. - initial);
      let scale = 1.0 - t2 * (1.0 - i as f64 * opts.size_diff / size);
      let s = spins;
      let mut p = (
        scale
          * amp1
          * ((s * 2. * PI * t).sin()
            + amp2
              * mix((s * f1.1 * PI * t).sin(), (s * f2.1 * PI * t).sin(), t)),
        0.07
          - scale
            * amp1
            * ((s * 2. * PI * t).cos()
              + amp2
                * mix((s * f1.0 * PI * t).cos(), (s * f2.0 * PI * t).cos(), t)),
      );
      let noise_angle = p.1.atan2(p.0);
      let noise_amp = 0.003
        * perlin.get([
          opts.a * (progress * PI).sin()
            + 4.8 * p.0
            + perlin.get([
              7.8 * p.0,
              4.2 * p.1 + opts.b * (progress * PI).sin(),
              40. + opts.seed,
            ]),
          4.8 * p.1
            + 0.8
              * perlin.get([
                4.5 * p.0 + opts.c * ((1. - progress) * PI).sin(),
                6.8 * p.1
                  + perlin.get([
                    20.5 * p.0 + opts.d * (2. * PI * progress).cos(),
                    20.8 * p.1,
                    200. + opts.seed,
                  ]),
                20. + opts.seed,
              ]),
          100. + opts.seed + i as f64 * opts.seed_diff,
        ])
        + 0.03
          * (1. - t)
          * perlin.get([
            0.7 * p.0
              + perlin.get([
                2.9 * p.0 + opts.e * (2. * PI * progress).cos(),
                1.7 * p.1,
                2000.0,
              ]),
            0.7 * p.1
              + perlin.get([
                3.1 * p.0,
                2.5 * p.1 + opts.e * (2. * PI * progress).sin(),
                2100.0,
              ]),
            1000.,
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
      let q = normalize_in_boundaries(o, (-0.9, -0.8, 0.9, 1.));
      let f = smoothstep(0.1, 0.0, grayscale(get_color(q)));
      if f > 0.5 || euclidian_dist(p, last) > 2.0 {
        if route.len() > 1 {
          routes.push(route);
        }
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
    l = l.add(base_path(color, 0.3, data));
    if i == 1 {
      l = l.add(signature(1.0, (208., 185.), color));
    }
    layers.push(l);
  }
  layers
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "2")]
  index: usize,
  #[clap(short, long, default_value = "8")]
  frames: usize,
  #[clap(short, long, default_value = "62.")]
  seed: f64,
  #[clap(short, long, default_value = "20.")]
  seed_diff: f64,
  #[clap(short, long, default_value = "100.")]
  size: f64,
  #[clap(short, long, default_value = "-0.4")]
  size_diff: f64,
  #[clap(short, long, default_value = "8.")]
  f1x: f64,
  #[clap(short, long, default_value = "8.")]
  f1y: f64,
  #[clap(short, long, default_value = "4.")]
  f2x: f64,
  #[clap(short, long, default_value = "6.")]
  f2y: f64,
  #[clap(short, long, default_value = "0.9")]
  amp1: f64,
  #[clap(short, long, default_value = "0.1")]
  amp2: f64,
  #[clap(short, long, default_value = "110.0")]
  spins: f64,
  #[clap(short, long, default_value = "1.0")]
  splits: f64,
  #[clap(short, long, default_value = "1.0")]
  split_threshold: f64,
  #[clap(short, long, default_value = "1.1")]
  pow: f64,
  #[clap(short, long, default_value = "1.0")]
  a: f64,
  #[clap(short, long, default_value = "0.4")]
  b: f64,
  #[clap(short, long, default_value = "1.0")]
  c: f64,
  #[clap(short, long, default_value = "4.0")]
  d: f64,
  #[clap(short, long, default_value = "0.2")]
  e: f64,
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
