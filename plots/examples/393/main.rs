use clap::*;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let mut rng = rng_from_seed(opts.seed);
  let colors = vec!["aquamarine", "crimson", "dodgerblue"];
  let pad = 10.;
  let width = 297.;
  let height = 210.;
  let mut circles = Vec::new();
  for _i in 0..rng.gen_range(8, 12) {
    let x = rng.gen_range(0., width);
    let y = rng.gen_range(0., height);
    let r = rng.gen_range(30.0, 140.0) * rng.gen_range(0.2, 1.0);
    circles.push((x, y, r));
  }

  let color = |p| {
    let mut count = 0;
    for &(x, y, r) in circles.iter() {
      if euclidian_dist((x, y), p) < r {
        count += 1;
      }
    }
    count % colors.len()
  };

  let project =
    |x, y| (pad + (width - 2. * pad) * x, pad + (height - 2. * pad) * y);

  let mut lines = Vec::new();

  for _i in 0..12000 {
    let mut p1: f64 = rng.gen_range(-0.3, 1.3);
    let delta = rng.gen_range(0.08, 0.16);
    let mut p2: f64 = if p1 < 0.5 { p1 + delta } else { p1 - delta };
    p1 = p1.max(0.0).min(1.0);
    p2 = p2.max(0.0).min(1.0);
    if (p1 - p2).abs() < 0.01 {
      continue;
    }
    let other = rng.gen_range(0.0, 1.0);
    if rng.gen_bool(0.4) {
      let clr = color(project((p1 + p2) / 2.0, other));
      lines.push((project(p1, other), project(p2, other), clr));
    } else {
      let clr = color(project(other, (p1 + p2) / 2.0));
      lines.push((project(other, p1), project(other, p2), clr));
    }
  }

  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();

      for &(a, b, clr) in lines.iter() {
        if clr == ci {
          data = data.move_to(a);
          data = data.line_to(b);
        }
      }

      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));

      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "153.0")]
  seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
