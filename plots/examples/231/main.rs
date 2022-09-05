use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "1.0")]
  seed: f64,
  #[clap(short, long, default_value = "100")]
  samples: usize,
}

fn art(opts: Opts) -> Vec<Group> {
  let (width, height) = (297., 210.);
  let precision = 0.2;
  let pad = 20.;
  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;
  let perlin = Perlin::new();
  let bounds = (pad, pad, width - pad, height - pad);

  let colors = vec!["darkturquoise", "firebrick"];

  let f = |(x, y): (f64, f64)| {
    let c = ((x - 0.5) * width / height, y - 0.5);
    let f1 = 0.8;
    let f2 = 1.0;
    let f3 = 1.5;
    let f4 = 90.;
    let a1 = 0.4 * (2. * (0.49 - length(c)).max(0.));
    let a2 = 2.;
    let a3 = 2.;
    let a4 = 0.01;
    let n1 = a1
      * perlin.get([
        f1 * c.0,
        f1 * c.1,
        opts.seed
          + a2
            * perlin.get([
              6.1 + f2 * c.0,
              3.6 + f2 * c.1,
              opts.seed
                + 10.
                + a3
                  * perlin.get([
                    opts.seed,
                    f3 * c.0 + a4 * (f4 * c.1).cos(),
                    f3 * c.1 + a4 * (f4 * c.0).cos(),
                  ]),
            ]),
      ]);
    let sq = (c.0 + 0.5).min(0.5 - c.0).min((c.1 + 0.5).min(0.5 - c.1));
    -0.1 + 3. * sq + n1
  };
  let samples = opts.samples;

  colors
    .iter()
    .enumerate()
    .map(|(ci, &color)| {
      let pattern = (2., 3.);
      let thresholds: Vec<f64> = (0..samples)
        .map(|i| {
          (((i + ci) as f64 + pattern.1 * (i as f64 / pattern.0).floor())
            / (samples as f64 * (pattern.0 + pattern.1) / pattern.0).floor())
          .powf(1.2)
        })
        .collect();

      let res = contour(w, h, f, &thresholds);
      let mut routes = features_to_routes(res, precision);
      routes = crop_routes(&routes, bounds);
      let mut data = Data::new();
      for route in routes {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l = l.add(signature(1.0, (260., 195.), color));
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
