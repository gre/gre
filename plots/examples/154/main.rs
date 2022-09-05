use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let (width, height, sign_pos) = (297., 210., (208.0, 176.));
  let precision = 0.2;
  let pad = 20.;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  let bounds = (pad, pad, width - pad, height - pad);

  let sdf_box2 = |(x, y): (f64, f64), (w, h): (f64, f64)| {
    let dx = x.abs() - w;
    let dy = y.abs() - h;
    euclidian_dist((0., 0.), (dx.max(0.), dy.max(0.)))
      + dx.min(0.).max(dy.min(0.))
  };

  let f = |(x, y): (f64, f64)| {
    let c = ((x - 0.5) * width / height, y - 0.5);
    smoothstep(-0.2, 0.2, sdf_box2(c, (0.2, 0.2)))
      + 0.01
        * perlin.get([
          2. * x,
          2. * y,
          opts.seed
            + 0.5
              * perlin.get([
                4. + opts.seed,
                6. * x + 0.5 * perlin.get([20. * x, 20. * y, 20. + opts.seed]),
                6. * y + 0.5 * perlin.get([20. * x, 20. * y, 30. + opts.seed]),
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
  let color = "black";
  let mut l = layer(color);
  l = l.add(base_path(color, 0.25, data));
  l = l.add(signature(1.0, sign_pos, color));
  vec![l]
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "100")]
  samples: usize,
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
