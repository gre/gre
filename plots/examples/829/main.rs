use std::f64::consts::PI;

use clap::*;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

struct Slice {
  routes_above: Vec<(usize, Vec<(f64, f64)>)>,
  routes_below: Vec<(usize, Vec<(f64, f64)>)>,
  a: (f64, f64),
  b: (f64, f64),
}

fn slice_routes(
  routes: Vec<(usize, Vec<(f64, f64)>)>,
  cuta: (f64, f64),
  cutb: (f64, f64),
) -> Slice {
  let mut routes_above = Vec::new();
  let mut routes_below = Vec::new();

  let mut amin = lerp_point(cuta, cutb, 0.5);
  let mut bmin = amin;
  let mut dista = 99999.0;
  let mut distb = 0.0;

  for (clr, r) in routes.clone() {
    if r.len() < 2 {
      continue;
    }
    let mut prev = r[0];
    let mut route = vec![prev];
    for &p in r.iter().skip(1) {
      if let Some(c) = collides_segment(prev, p, cuta, cutb) {
        let la = euclidian_dist(c, cuta);
        if la > distb {
          distb = la;
          bmin = c;
        }
        if la < dista {
          dista = la;
          amin = c;
        }

        route.push(c);
        if route.len() > 1 {
          if !is_left(cuta, cutb, prev) {
            routes_above.push((clr, route));
          } else {
            routes_below.push((clr, route));
          }
        }
        route = vec![c, p];
      } else {
        route.push(p);
      }
      prev = p;
    }
    if route.len() > 1 {
      if !is_left(cuta, cutb, prev) {
        routes_above.push((clr, route));
      } else {
        routes_below.push((clr, route));
      }
    }
  }

  Slice {
    routes_above,
    routes_below,
    a: amin,
    b: bmin,
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();
  let mut p = rng.gen_range(10.0, 50.0);
  let min = rng.gen_range(-100f64, 30.0).max(0.1);
  let mut i = 0;
  let mut incr = 0.5;
  let border_count = rng.gen_range(10, 20);
  let splitincr = rng.gen_range(0.0, 8.0);
  loop {
    let x2 = width - p;
    let y2 = height - p;
    if x2 - min < p || y2 - min < p {
      break;
    }
    routes.push((0, vec![(p, p), (x2, p), (x2, y2), (p, y2), (p, p)]));
    i += 1;
    p += incr;
    if i % border_count == 0 {
      p += splitincr;
      incr = rng.gen_range(0.3, 0.9)
        + rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0);
    }
  }

  let count = rng.gen_range(8, 28);
  let split = rng.gen_range(-0.2, 0.2);
  let max_slide = rng.gen_range(0.0, 20.0);
  let pow = rng.gen_range(0.8, 2.5);
  for _i in 0..count {
    let x = width * rng.gen_range(0.4, 0.6);
    let y = height * rng.gen_range(0.3, 0.7);
    let a = rng.gen_range(-PI, PI);
    let dx = a.cos();
    let dy = a.sin();
    let amp = 200.0;
    let left = (x - amp * dx, y - amp * dy);
    let right = (x + amp * dx, y + amp * dy);
    let slice = slice_routes(routes.clone(), left, right);
    let slide =
      rng.gen_range(0.0, max_slide) * rng.gen_range(0f64, 1.0).powf(pow);
    let l = euclidian_dist(slice.a, slice.b);
    let v = ((slice.b.0 - slice.a.0) / l, (slice.b.1 - slice.a.1) / l);
    let n = (v.1, -v.0);
    routes = vec![
      translate_routes(
        slice.routes_above,
        (v.0 * slide + n.0 * split, v.1 * slide + n.1 * split),
      ),
      translate_routes(
        slice.routes_below,
        (-v.0 * slide - n.0 * split, -v.1 * slide - n.1 * split),
      ),
    ]
    .concat();
  }

  let routes_copy = routes.clone();
  let mut routes = vec![];
  for (c, r) in routes_copy {
    let l = r.len();
    if l > 0 {
      let mut route = vec![];
      let mut all = vec![];
      for p in r.clone() {
        if strictly_in_boundaries(p, bound) {
          route.push(p);
        } else {
          let l = route.len();
          if l > 1 {
            all.push(route);
            route = vec![];
          } else if l > 0 {
            route = vec![];
          }
        }
      }
      if route.len() > 1 {
        all.push(route);
      }
      for r in all {
        let route = rdp(&r.clone(), 0.1);
        routes.push((c, route));
      }
    }
  }

  vec!["#a96", "#a96"]
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn is_left(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
  ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)) > 0.0
}

fn translate_routes(
  routes: Vec<(usize, Vec<(f64, f64)>)>,
  (tx, ty): (f64, f64),
) -> Vec<(usize, Vec<(f64, f64)>)> {
  routes
    .iter()
    .map(|(i, route)| {
      (*i, route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    })
    .collect()
}
