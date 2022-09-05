use clap::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let colors = vec!["black"];
  let width = 297.0;
  let height = 210.0;
  let radius = 100.0;
  let bounds = (
    width / 2. - radius,
    height / 2. - radius,
    width / 2. + radius,
    height / 2. + radius,
  );
  let stroke_width = 0.3;
  let desired_count = 2000;
  let upper_limit = 100000;
  let pad = 0.5;
  let threshold_radius = 0.6;

  let mut rng = rng_from_seed(opts.seed);
  let mut circles = Vec::new();
  for _i in 0..upper_limit {
    let x: f64 = rng.gen_range(bounds.0, bounds.2);
    let y: f64 = rng.gen_range(bounds.1, bounds.3);
    let mut r = (x - bounds.0)
      .min(y - bounds.1)
      .min(bounds.2 - x)
      .min(bounds.3 - y);
    for &(x2, y2, r2) in circles.iter() {
      r = r.min(euclidian_dist((x, y), (x2, y2)) - r2);
    }
    r -= pad;
    if r > threshold_radius {
      circles.push((x, y, r));
    }
    if circles.len() > desired_count {
      break;
    }
  }

  colors
    .iter()
    .enumerate()
    .map(|(i, &color)| {
      let mut l = layer(color);
      let mut data = Data::new();
      data = render_route(data, boundaries_route(bounds));
      l = l.add(base_path(color, stroke_width, data));
      for c in circles.iter() {
        l = l.add(
          Circle::new()
            .set("r", c.2)
            .set("cx", c.0)
            .set("cy", c.1)
            .set("stroke", color)
            .set("stroke-width", stroke_width)
            .set("fill", "none")
            .set("style", "mix-blend-mode: multiply;"),
        );
      }

      if i == colors.len() - 1 {
        l = l.add(signature(1.0, (250.0, 196.0), color));
      }
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "40.0")]
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
