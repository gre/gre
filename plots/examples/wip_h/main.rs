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
  #[clap(short, long, default_value = "420.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
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

fn sd_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
) -> f64 {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;

  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let h = (dot_pa_ba / dot_ba_ba).max(0.0).min(1.0);

  let h_x = ba_x * h;
  let h_y = ba_y * h;

  //  ((pa_x - h_x) * (pa_x - h_x) + (pa_y - h_y) * (pa_y - h_y)).sqrt()
  // manhattan distance version
  (pa_x - h_x).abs().max((pa_y - h_y).abs())
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let precision = 0.5;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);

  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;

  let offset = rng.gen_range(0.0, 0.1);
  let length = rng.gen_range(4.0, 10.0);
  let samples = (rng.gen_range(100.0, 300.0) / length) as usize;
  let pattern = (
    rng.gen_range(3.0f64, 8.0).round(),
    rng.gen_range(-5.0f64, 10.0).round().max(0.0),
  );

  let thresholds: Vec<f64> = (0..samples)
    .map(|i| {
      (i as f64 + pattern.1 * (i as f64 / pattern.0).floor())
        / (samples as f64 * (pattern.0 + pattern.1) / pattern.0).floor()
    })
    .collect();

  let xpos = 0.3;
  let hpos = 0.25;
  let f = |p: (f64, f64)| {
    //let x = x.min(1.0 - x);
    //6.0 * (y - 1.0 / 2.0).abs().min(2.0 * (x - xpos).abs())

    // sd_segment((x, y), (0.2, 0.2), (0.8, 0.8))
    // make a H shape with sd_segment union
    length
      * (sd_segment(p, (xpos, hpos), (xpos, 1.0 - hpos))
        .min(sd_segment(p, (xpos, 0.5), (1.0 - xpos, 0.5)))
        .min(sd_segment(p, (1.0 - xpos, hpos), (1.0 - xpos, 1.0 - hpos)))
        - offset)
  };

  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, bound);

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> =
    routes.iter().map(|r| (0, r.clone())).collect();

  /*
  let mut p = rng.gen_range(30.0, 60.0);
  let min = rng.gen_range(-100f64, 30.0).max(0.1);
  let mut i = 0;
  let border_count = rng.gen_range(5, 30);
  let splitincr = rng.gen_range(1.0, 20.0);
  loop {
    let x2 = width - p;
    let y2 = height - p;
    if x2 - min < p || y2 - min < p {
      break;
    }
    routes.push((0, vec![(p, p), (x2, p), (x2, y2), (p, y2), (p, p)]));
    p += 0.5;
    i += 1;
    if i % border_count == 0 {
      p += splitincr;
    }
  }
  */

  let count = rng.gen_range(4, 32);
  let split = rng.gen_range(-0.5, 0.5);
  let max_slide = rng.gen_range(1.0, 20.0);
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
    let slide = rng.gen_range(0.0, max_slide) * rng.gen_range(0.0, 1.0);
    let l = euclidian_dist(slice.a, slice.b);
    let v = ((slice.b.0 - slice.a.0) / l, (slice.b.1 - slice.a.1) / l);
    let n = (v.1, -v.0);
    //routes_above.push((clr, vec![amin, bmin]));
    //routes_below.push((clr, vec![amin, bmin]));
    routes = vec![
      translate_routes(
        slice.routes_above,
        (v.0 * slide + n.0 * split, v.1 * slide + n.1 * split),
      )
      .iter()
      .map(|(ci, route)| ((ci + 1) % 2, route.clone()))
      .collect(),
      translate_routes(
        slice.routes_below,
        (-v.0 * slide - n.0 * split, -v.1 * slide - n.1 * split),
      ),
    ]
    .concat();
  }

  let should_draw_line =
    |a, b| strictly_in_boundaries(a, bound) && strictly_in_boundaries(b, bound);

  vec!["white", "gold"]
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if ci == i {
          data = render_route_when(data, route, &should_draw_line);
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
  let mut document = base_document("black", opts.width, opts.height);
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
