use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let (width, height, sign_pos) = (297., 210., (200.0, 180.));
  let precision = 1.0;
  let pad = 20.;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  let ratio = width / height;
  let bounds = (pad, pad, width - pad, height - pad);

  let f = |(x, y): (f64, f64)| {
    -0.5
      + 4. * euclidian_dist((ratio * (x - 0.5), y - 0.5), (0., 0.))
      + 0.3
        * perlin.get([
          1.2 * x,
          1.2 * y,
          opts.seed
            + perlin.get([
              4. + opts.seed,
              3.8 * x + 1.2 * perlin.get([6.1 * x, 5.3 * y, 8. + opts.seed]),
              3.2 * y + 1.2 * perlin.get([6.2 * x, 5.8 * y, 9. + opts.seed]),
            ]),
        ])
  };
  let samples = opts.samples;
  let thresholds: Vec<f64> =
    (0..samples).map(|i| i as f64 / (samples as f64)).collect();

  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, bounds);
  let mut data = Data::new();
  for route in routes {
    data = render_route(data, route);
  }
  let color = "white";
  let mut l = layer(color);
  l = l.add(base_path(color, 0.35, data));
  l = l.add(signature(1.0, sign_pos, color));
  vec![l]
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "18.0")]
  seed: f64,
  #[clap(short, long, default_value = "120")]
  samples: usize,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("firebrick");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
