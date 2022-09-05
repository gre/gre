use clap::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let height = 210f64;
  let width = 297f64;
  let perlin = Perlin::new();

  let size = 100.;
  let f2 = (8., 8.);
  let f1 = (14., 14.);
  let amp1 = 0.9;
  let amp2 = 0.1;
  let samples = 100000;
  let spins = 100.0;
  let splits = 3.0;
  let split_threshold = 0.94;
  let pow = 1.2;
  let noise_angle_freq = 3.0;
  let noise_angle_displacement_freq = 0.1;
  let noise_angle_displacement = 0.02;

  let parametric = |p: f64| {
    let p1 = (splits * p).floor();
    let p2 = splits * p - p1;
    let t = (p1 + split_threshold * p2) / splits;
    let t2 = (p1 + split_threshold * p2.powf(pow)) / splits;
    let scale = 1.0 - t2;
    let mut p = (
      scale
        * amp1
        * ((spins * 2. * PI * t).cos()
          + amp2
            * mix(
              (spins * f1.0 * PI * t).cos(),
              (spins * f2.0 * PI * t).cos(),
              t,
            )),
      scale
        * amp1
        * ((spins * 2. * PI * t).sin()
          + amp2
            * mix(
              (spins * f1.1 * PI * t).sin(),
              (spins * f2.1 * PI * t).sin(),
              t,
            )),
    );
    let noise_angle = 2.
      * PI
      * perlin.get([
        noise_angle_freq * p.0,
        noise_angle_freq * p.1,
        100.0 + opts.seed,
      ]);
    let noise_amp = noise_angle_displacement
      * perlin.get([
        noise_angle_displacement_freq * p.0,
        noise_angle_displacement_freq * p.1,
        opts.seed,
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
    if euclidian_dist(p, last) > 1.5 {
      routes.push(route);
      route = Vec::new();
    }
    route.push(p);
    last = p;
  }
  routes.push(route);

  let color = "white";
  let data = routes
    .iter()
    .fold(Data::new(), |data, route| render_route(data, route.clone()));
  let mut l = layer(color);
  l = l.add(base_path(color, 0.2, data));
  l = l.add(signature(1., (215.0, 185.0), color));
  vec![l]
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("black");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
