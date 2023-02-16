use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "105.0")]
  pub width: f64,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let boundaries = (pad, pad, width-pad,height-pad);

  let mut black_routes = Vec::new();
  let mut color_routes = Vec::new();

let mut rng = rng_from_seed(opts.seed);

let dim = 3000;
let dr = 0.6;
let po = 2.;


let samples = 600;
for p in sample_2d_candidates_f64(&|p| (p.1/height).powf(po), dim, samples, &mut rng) {
  let (x,y) = project_in_boundaries(p, boundaries);
  let radius = mix(0.6, 2.2, y/height);
  black_routes.push(spiral_optimized(x,y, radius, dr, 0.05));
  black_routes.push(circle_route((x,y), radius, (3.0 * radius + 8.0) as usize));
}

let samples = 1400;
for p in sample_2d_candidates_f64(&|p| p.1/height, dim, samples, &mut rng) {
  let (x,y) = project_in_boundaries(p, boundaries);
  let radius = mix(0.6, 2.2, y/height);
  color_routes.push(spiral_optimized(x,y, radius, dr, 0.05));
  color_routes.push(circle_route((x,y), radius, (3.0 * radius + 8.0) as usize));
}
  

  vec![(black_routes, "#666"), (color_routes, "#09F")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.36, data));
      l
    })
    .collect()
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
