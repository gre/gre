use clap::*;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let density = opts.density;
  let scale = opts.scale;

  let shape = jin;

  let mut data = Data::new();
  let d = density / scale;
  let samples = (d * width * height) as usize;
  let radius_from = opts.center_initial_distance * scale;
  let radius_to = 0.5 * height;
  let golden_angle = PI * (3.0 - (5.0 as f64).sqrt());
  for i in 0..samples {
    let a = golden_angle * (i as f64);
    let f = (i as f64) / (samples as f64);
    let amp = mix(radius_from, radius_to, f.powf(opts.spiral_pow));
    let p = (width / 2. + a.cos() * amp, height / 2. + a.sin() * amp);
    let s = scale * mix(1.0 + opts.scale_incr, 1.0 - opts.scale_drop, f);
    if p.0 > pad && p.0 < width - pad && p.1 > pad && p.1 < height - pad {
      data = shape(data, p, s, -a);
    }
  }

  let mut l = layer("black");
  l = l.add(base_path("black", 0.6, data));
  vec![l]
}

fn jin(data: Data, origin: (f64, f64), scale: f64, ang: f64) -> Data {
  let mut data = data;
  let refs = 70.;
  let proj = |x, y| {
    let x = x - 95.;
    let y = y - 205.0;
    let (x, y) = p_r((x, y), ang);
    let x = x * scale / refs;
    let y = y * scale / refs;
    let x = x + origin.0;
    let y = y + origin.1;
    (x, y)
  };

  data = data.move_to(proj(96.8, 97.6)).cubic_curve_to((
    proj(82.0, 129.4),
    proj(60.5, 160.1),
    proj(23.3, 180.2),
  ));

  data = data.move_to(proj(99.8, 107.3)).cubic_curve_to((
    proj(120.9, 127.1),
    proj(138.5, 151.3),
    proj(164.3, 165.5),
  ));
  data = data.move_to(proj(68.8, 161.5)).line_to(proj(118.3, 154.1));
  data = data.move_to(proj(54.2, 189.0)).line_to(proj(132.8, 181.0));
  data = data.move_to(proj(93.7, 161.7)).line_to(proj(96.1, 228.5));
  data = data.move_to(proj(58.2, 204.4)).line_to(proj(71.9, 218.6));
  data = data.move_to(proj(130.1, 201.6)).line_to(proj(109.7, 222.0));
  data = data.move_to(proj(36.6, 239.7)).line_to(proj(152.5, 228.7));

  data
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297")]
  width: f64,
  #[clap(short, long, default_value = "420")]
  height: f64,
  #[clap(short, long, default_value = "10")]
  pad: f64,
  #[clap(short, long, default_value = "0.05")]
  density: f64,
  #[clap(short, long, default_value = "5.0")]
  scale: f64,
  #[clap(short, long, default_value = "0.2")]
  scale_drop: f64,
  #[clap(short, long, default_value = "0.3")]
  scale_incr: f64,
  #[clap(short, long, default_value = "0.42")]
  spiral_pow: f64,
  #[clap(short, long, default_value = "0.0")]
  center_initial_distance: f64,
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
