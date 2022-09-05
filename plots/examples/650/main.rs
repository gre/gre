use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

//~~~ inputs of the generator:
#[derive(Parser)]
#[clap()]
pub struct Opts {
  // takes a GIF
  #[clap(short, long, default_value = "./examples/650/s.gif")]
  animation: String,
  // saves a SVG
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  // for a given resolution:
  #[clap(short, long, default_value = "420.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
}

//~~~ our main artistic function starts here!
fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let animation = opts.animation.as_str();
  let pad = opts.pad;

  let mut routes = Vec::new(); // all the lines

  // for each x,y frames:
  let xframes = 6;
  let yframes = 4;
  let size = (width - pad * 2.0) / (xframes as f64);
  for rev in 0..yframes {
    let dy = (height - size * (yframes as f64)) / 2.0 + rev as f64 * size;
    for i in 0..xframes {
      //~~~ Make "f", a pixel color value function
      let get_color =
        image_gif_get_color(animation, i + rev * xframes).unwrap();
      let dx = pad + i as f64 * size;
      let bounds = (1.0, 1.0, size - 1.0, size - 1.0);
      let pixel_pad = 0.1;
      let f = |(x, y)| {
        let p = (
          -pixel_pad + x * (1.0 + 2.0 * pixel_pad),
          -pixel_pad + y * (1.0 + 2.0 * pixel_pad),
        );
        if out_of_boundaries(p, (0.0, 0.0, 1.0, 1.0)) {
          0.0
        } else {
          1. - get_color(p).0
        }
      };

      //~~~ use marching squares to do contouring
      let thresholds = vec![0.5, 0.1];
      let precision = 0.5;
      let w = (size as f64 / precision) as u32;
      let h = (size as f64 / precision) as u32;
      let res = contour(w, h, f, &thresholds);
      let mut all = features_to_routes(res, precision);
      all = crop_routes(&all, bounds);

      //~~~ fill the black area with a centered spiral
      let aincr = 0.005;
      let rincr = aincr / 12.0;
      let mut r = 0.1;
      let mut a = 0f64;
      let center = (size * 0.5, size * 0.5);
      let mut route = Vec::new();
      let min_stroke = 0.1;
      loop {
        if r > size {
          break; // we're far enough to stop
        }
        let p = (center.0 + r * a.cos(), center.1 + r * a.sin());
        let n = (p.0 / size, p.1 / size);
        let should_draw =
          strictly_in_boundaries(n, (0.0, 0.0, 1.0, 1.0)) && f(n) > 0.5;
        if !should_draw {
          if route.len() > 1 {
            all.push(route);
          }
          route = Vec::new();
        } else {
          let l = route.len();
          if l == 0 {
            route.push(p);
          } else if euclidian_dist(route[l - 1], p) > min_stroke {
            route.push(p);
          }
        }
        r += rincr;
        a += aincr;
      }
      if route.len() > 1 {
        all.push(route);
      }

      all = translate_routes(all, (dx, dy));
      routes = vec![routes, all].concat();

      //~~~ build the surounding frames...
      routes.push(vec![(dx, dy), (dx, dy + size)]);
    }
    let lastx = pad + size * (xframes as f64);
    routes.push(vec![(lastx, dy), (lastx, dy + size)]);
    routes.push(vec![(pad, dy), (lastx, dy)]);
  }
  let dy = (height - size * (yframes as f64)) / 2.0 + yframes as f64 * size;
  routes.push(vec![(pad, dy), (width - pad, dy)]);

  //~~~ and put it all in the SVG:
  let color = "black";
  let mut data = Data::new();
  let mut l = layer(color);
  for route in routes.clone() {
    data = render_route(data, route);
  }
  l = l.add(base_path(color, 0.35, data));
  vec![l]
}

//~~~ we can build the document and save!
fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

// just a dumb helper
fn translate_routes(
  routes: Vec<Vec<(f64, f64)>>,
  (tx, ty): (f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    .collect()
}
