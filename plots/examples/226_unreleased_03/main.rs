use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let mut rng = rng_from_seed(opts.seed);
  let get_color = image_get_color("images/eye2.png").unwrap();

  let colors = vec!["white"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let dim = 4000;
      let samples = 20000;
      let f = |p| {
        let rgb = get_color(p);
        let c = grayscale(rgb);
        let dist = euclidian_dist(p, (0.5, 0.5));
        c.powf(2.0) * smoothstep(0.5, 0.0, dist)
      };
      let mut samples = sample_2d_candidates_f64(&f, dim, samples, &mut rng);
      samples = tsp(samples, time::Duration::seconds(60));
      let pad = 20.0;
      let width = 297.0;
      let height = 210.0;
      let dx = (width - height) / 2.0;
      let boundaries = (dx + pad, pad, dx + height - pad, height - pad);
      let stroke_dist = 1.0;

      let mut l = layer(color);
      let mut data = Data::new();
      for p in samples {
        let a = project_in_boundaries(p, boundaries);
        let b = follow_angle(a, (p.0 - 0.5).atan2(p.1 - 0.5), stroke_dist);
        data = data.move_to(a).line_to(b);
      }
      l = l.add(base_path(color, 0.35, data));
      if ci == colors.len() - 1 {
        l = l.add(signature(
          1.0,
          (dx + height - 44.0, height - pad + 2.0),
          color,
        ));
      }
      l
    })
    .collect()
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
