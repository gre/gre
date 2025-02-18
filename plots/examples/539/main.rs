use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "../public/logo.jpg")]
  animation: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let colors = vec!["black"];
  let perlin = Perlin::new();

  let get_color = image_get_color(opts.animation.as_str()).unwrap();

  let f = |(x, y)| {
    let p = (mix(-0.0, 1.0, x), mix(-0.15, 1.15, y));
    /*
    let freq = 100.0;
    let amp = 0.002;
    p.0 += amp
        * perlin.get([
            36.7 + 1.7 * opts.seed,
            freq * x,
            freq * y,
        ]);
    p.1 += amp
        * perlin.get([
            -7.3 * opts.seed,
            freq * x,
            freq * y,
        ]);
        */
    if out_of_boundaries(p, (0.0, 0.0, 1.0, 1.0)) {
      return 0.0;
    }
    1.0 - get_color(p).0
  };

  let mut routes = Vec::new();
  /*
  let pad = 10.0;
  let bounds = (pad, pad, width - pad, height - pad);
  let thresholds = vec![0.5];
  let precision = 0.5;
  let w = (width / precision) as u32;
  let h = (height / precision) as u32;
  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, bounds);
  */

  let aincr = 0.001;
  let rincr = aincr / 12.0;
  let mut r = 0.1;
  let mut a = 0f64;
  let center = (width / 2.0, height / 2.0);
  let mut route = Vec::new();
  let min_stroke = 0.1;
  loop {
    if r > 200.0 {
      break;
    }
    let p = (center.0 + r * a.cos(), center.1 + r * a.sin());
    let n = (p.0 / width, p.1 / height);
    let mut g = p;
    let freq = 8.0;
    let amp = 0.5;
    g.0 += amp * perlin.get([freq * n.0, freq * n.1, 7.7 * opts.seed]);
    g.1 += amp * perlin.get([freq * n.0, freq * n.1, -3.3 * opts.seed]);

    let should_draw =
      strictly_in_boundaries(n, (0.0, 0.0, 1.0, 1.0)) && f(n) > 0.5;

    if !should_draw {
      if route.len() > 1 {
        routes.push(route);
      }
      route = Vec::new();
    } else {
      let l = route.len();
      if l == 0 {
        route.push(g);
      } else if euclidian_dist(route[l - 1], g) > min_stroke {
        route.push(g);
      }
    }

    r += rincr;
    a += aincr;
  }
  if route.len() > 1 {
    routes.push(route);
  }

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      let mut l = layer(color);
      for route in routes.clone() {
        data = render_route(data, route);
      }
      l = l.add(base_path(color, 0.35, data));
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
