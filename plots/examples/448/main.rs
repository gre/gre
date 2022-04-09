use std::f64::consts::PI;

use clap::Clap;
use gre::*;
use kiss3d::camera::*;
use kiss3d::nalgebra::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::Group;

#[derive(Clap)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "26.0")]
  seed: f64,
  #[clap(short, long, default_value = "0")]
  index: usize,
  #[clap(short, long, default_value = "8")]
  frames: usize,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
  height: f64,
  #[clap(short, long, default_value = "0.3")]
  amp: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let color = "#000";
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;
  let stroke_width = 0.35;
  let seed = opts.seed;
  let mut points = vec![];
  let w = 20;
  let h = 20;
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();
  let amp = opts.amp;
  let a1 = rng.gen_range(4., 6.);
  let a2 = rng.gen_range(1., 2.);
  let f1 = rng.gen_range(0.04, 0.14);
  let f2 = f1 * rng.gen_range(0.4, 1.4);
  let progress = opts.index as f64 / (opts.frames as f64);
  let y = |x: f32, z: f32| {
    a1 * (perlin.get([
      (x) as f64 * f1 + 2. * amp * (2. * PI * progress).cos(),
      seed + a2 * perlin.get([x as f64 * f2, z as f64 * f2]),
      (z) as f64 * f1 + amp * (2. * PI * progress).sin(),
    ]) as f32)
  };

  for z in 0..h {
    let ltr = z % 2 == 0;
    let zf = z as f32;
    for xi in 0..(w + 1) {
      let x = if ltr { xi } else { w - xi };
      let xf = x as f32;
      points.push(Point3::new(xf, y(xf, zf), zf));
      points.push(Point3::new(xf, y(xf, zf + 1.0), zf + 1.0));
      if x < w {
        points.push(Point3::new(xf + 1.0, y(xf + 1.0, zf + 1.0), zf + 1.0));
      }
      points.push(Point3::new(xf, y(xf, zf), zf));
    }
  }

  let camera = FirstPerson::new(Point3::new(-6.0, 10.0, -6.0), Point3::new(10.0, -4.0, 10.0));
  let dim = Vector2::new((width - 2. * pad) as f32, (height - 2. * pad) as f32);
  let offset = Vector2::new(pad as f32, pad as f32);
  let mut route = Vec::new();
  for p in points {
    let pr = camera.project(&p, &dim);
    let pos = ((offset.x + pr.x) as f64, (offset.y + dim.y - pr.y) as f64);
    route.push(pos);
  }

  let mut layers = Vec::new();
  let mut l = layer(color);
  let mut data = Data::new();
  data = render_route(data, route);
  l = l.add(base_path(color, stroke_width, data));
  layers.push(l);
  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
