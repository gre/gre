use clap::*;
use gre::*;
use noise::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "10")]
  tsp_it_seconds_limit: i64,
  #[clap(short, long, default_value = "5.0")]
  seed: f64,
  #[clap(short, long, default_value = "3.0")]
  k: f64,
  #[clap(short, long, default_value = "4.0")]
  m: f64,
}

fn art(opts: Opts) -> Vec<Group> {
  let mut rng = rng_from_seed(opts.seed);
  let k = opts.k;

  let colors = vec!["black"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let pad = 20.0;
      let width = 297.0;
      let height = 210.0;
      let boundaries = (pad, pad, width - pad, height - pad);
      let ratio = (boundaries.2 - boundaries.0) / (boundaries.3 - boundaries.1);

      let dim = 2000;
      let samples = 20000;
      let noise = OpenSimplex::new();
      let f = |point: (f64, f64)| {
        let p = (point.0 * opts.m * ratio, point.1 * opts.m);
        let a1 = noise.get([10. + 0.3 * opts.seed, p.0, p.1]);
        let a2 = noise.get([p.0, p.1, 70.433 * opts.seed]);
        let b1 =
          noise.get([p.0 + 4. * k * a1 + 4.8 + opts.seed, p.1 + k * a2 - 3.7]);
        let b2 =
          noise.get([p.0 + k * a1 + 7.8 - opts.seed, p.1 + 2. * k * a2 - 1.7]);
        smoothstep(
          -0.2,
          0.5,
          noise.get([
            -opts.seed,
            p.0 + 0.2 * k * a1 + 0.4 * k * b1,
            p.1 + 0.2 * k * a2 + 0.4 * k * b2,
          ]),
        )
      };
      let mut samples = sample_2d_candidates_f64(&f, dim, samples, &mut rng);
      // pre-tsp
      samples =
        tsp(samples, time::Duration::seconds(opts.tsp_it_seconds_limit));
      // split samples into chunks
      let chunk_size = 200;
      let mut chunks = Vec::new();
      let mut chunk = Vec::new();
      for p in samples {
        if chunk.len() >= chunk_size {
          chunks.push(chunk);
          chunk = Vec::new();
        }
        chunk.push(p);
      }
      chunks.push(chunk);

      // run tsp in parallel on each chunk
      chunks = chunks
        .par_iter()
        .map(|chunk| {
          tsp(
            chunk.clone(),
            time::Duration::seconds(opts.tsp_it_seconds_limit),
          )
        })
        .collect();

      let stroke_dist = 3.0;

      let mut l = layer(color);
      for chunk in chunks {
        let mut data = Data::new();
        for p in chunk {
          let a = project_in_boundaries(p, boundaries);
          let b = follow_angle(a, (p.0 - 0.5).atan2(p.1 - 0.5), stroke_dist);
          data = data.move_to(a).line_to(b);
        }
        l = l.add(base_path(color, 0.35, data));
      }
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
