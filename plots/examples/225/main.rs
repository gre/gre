use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "10.0")]
  seed: f64,
  #[clap(short, long, default_value = "3.0")]
  k: f64,
}

fn art(opts: Opts) -> Vec<Group> {
  let mut rng = rng_from_seed(opts.seed);
  let k = opts.k;

  let colors = vec!["black", "grey"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let dim = 2000;
      let samples = 20000;
      let noise = OpenSimplex::new();
      let f = |p: (f64, f64)| {
        let a1 = noise.get([10. + 0.3 * opts.seed, k * p.0, k * p.1]);
        let a2 = noise.get([p.0, p.1, 70.433 * opts.seed]);
        let b1 = noise
          .get([p.0 + k * 3. * a1 + 4.8 + opts.seed, p.1 + k * 2. * a2 - 3.7]);
        let b2 = noise
          .get([p.0 + k * 2. * a1 + 7.8 - opts.seed, p.1 + k * 3. * a2 - 1.7]);
        let v = smoothstep(
          -0.2,
          0.5,
          noise.get([
            -77.777 * opts.seed,
            k * p.0 + a2 + 2. * b1,
            k * p.1 + a1 + 2. * b2,
          ]),
        );
        if ci == 0 {
          v
        } else {
          1. - v
        }
      };
      let mut samples = sample_2d_candidates_f64(&f, dim, samples, &mut rng);
      samples = tsp(samples, time::Duration::seconds(60));
      let pad = 20.0;
      let width = 297.0;
      let height = 210.0;
      let boundaries = (pad, pad, width - pad, height - pad);
      let stroke_dist = 2.0;

      let mut l = layer(color);
      let mut data = Data::new();
      for p in samples {
        let a = project_in_boundaries(p, boundaries);
        let b = follow_angle(a, (p.0 - 0.5).atan2(p.1 - 0.5), stroke_dist);
        data = data.move_to(a).line_to(b);
      }
      l = l.add(base_path(color, 0.35, data));
      if ci == 0 {
        l = l.add(signature(0.8, (255.0, 190.0), color));
      }
      l
    })
    .collect()
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
