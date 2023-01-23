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
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn sd_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
  manhattan: bool,
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

  if manhattan {
    return (pa_x - h_x).abs().max((pa_y - h_y).abs());
  } else {
    ((pa_x - h_x) * (pa_x - h_x) + (pa_y - h_y) * (pa_y - h_y)).sqrt()
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let precision = 0.4;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);

  let w = (width as f64 / precision) as u32;
  let h = (height as f64 / precision) as u32;

  let length = rng.gen_range(2.0, 3.0);
  let samples = (rng.gen_range(300.0, 400.0) / length) as usize;
  let pattern = (4.0, rng.gen_range(0.2f64, 4.0).max(0.0).round());

  let divisor = (samples as f64 * (pattern.0 + pattern.1) / pattern.0).floor();
  let thresholds: Vec<f64> = (0..samples)
    .map(|i| (i as f64 + pattern.1 * (i as f64 / pattern.0).floor()) / divisor)
    .collect();

  let balance = rng.gen_range(-0.5, 2.0);

  let offsetmax = rng.gen_range(0.0, 0.05);

  let segments: Vec<((f64, f64), (f64, f64), bool, f64)> =
    (0..(2.0 + rng.gen_range(0., 50.) * rng.gen_range(0.0, 1.0)) as usize)
      .map(|_| {
        (
          (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
          (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
          if balance >= 1.0 {
            true
          } else if balance <= 0.0 {
            false
          } else {
            rng.gen_bool(balance)
          },
          offsetmax * rng.gen_range(-2.0f64, 1.0).max(0.0),
        )
      })
      .collect();

  let distortion = rng.gen_range(-0.05f64, 0.02).max(0.0);
  let ratio = width / height;

  let xflip = rng.gen_bool(0.9);
  let yflip = rng.gen_bool(0.3);
  let yoffset = rng.gen_range(-10.0f64, 1.0).max(0.0);

  let f = |p: (f64, f64)| {
    let mut p = p;

    p = (
      p.0,
      p.1 + distortion * (p.0 - 0.5).abs() * (p.0 * 40.0 + p.1.sin()).cos(),
    );

    if xflip {
      p.0 = p.0.min(1.0 - p.0);
    }
    if yflip {
      p.1 = p.1.min(1.0 - p.1);
    }

    let mut s = 9999.0f64;

    for &(from, to, manhattan, offset) in segments.iter() {
      s = s.min(
        sd_segment(
          (p.0 * ratio, p.1),
          (from.0 * ratio, from.1),
          (to.0 * ratio, to.1),
          manhattan,
        ) - offset,
      );
    }
    s + yoffset * (p.1 - 0.5)
  };

  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, bound);

  let should_crop = |p| !strictly_in_boundaries(p, bound);
  let mut cutted_points = vec![];
  routes =
    crop_routes_with_predicate(&routes, &should_crop, &mut cutted_points);

  let offset = rng.gen_range(-5.0f64, 5.0).max(0.0);
  let mul = width / divisor;
  let mut frame = vec![];
  for i in 0..(pattern.0).round() as usize {
    let l = offset + i as f64 * mul;
    frame.push(vec![
      (pad - l, pad - l),
      (width - pad + l, pad - l),
      (width - pad + l, height - pad + l),
      (pad - l, height - pad + l),
      (pad - l, pad - l),
    ]);
  }

  vec!["gold"]
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for route in routes.clone() {
        let simplified = rdp(&route, 0.1);
        if route_length(&simplified) > 2.0 {
          data = render_route(data, simplified);
        }
      }
      for route in frame.clone() {
        data = render_route(data, route);
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

fn route_length(route: &Vec<(f64, f64)>) -> f64 {
  let mut length = 0.0;
  for i in 0..route.len() - 1 {
    length += (route[i].0 - route[i + 1].0).powi(2)
      + (route[i].1 - route[i + 1].1).powi(2);
  }
  length.sqrt()
}

fn crop_routes_with_predicate(
  input_routes: &Vec<Vec<(f64, f64)>>,
  should_crop: &dyn Fn((f64, f64)) -> bool,
  cutted_points: &mut Vec<(f64, f64)>,
) -> Vec<Vec<(f64, f64)>> {
  let search = |a_, b_, n| {
    let mut a = a_;
    let mut b = b_;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if should_crop(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes {
    if input_route.len() < 2 {
      continue;
    }
    let mut prev = input_route[0];
    let mut route = vec![];
    if !should_crop(prev) {
      // prev is not to crop. we can start with it
      route.push(prev);
    } else {
      if !should_crop(input_route[1]) {
        // prev is to crop, but [1] is outside. we start with the exact intersection
        let intersection = search(input_route[1], prev, 7);
        prev = intersection;
        cutted_points.push(intersection);
        route.push(intersection);
      } else {
        cutted_points.push(prev);
      }
    }
    // cut segments with crop logic
    for &p in input_route.iter().skip(1) {
      // TODO here, we must do step by step to detect crop inside the segment (prev, p)

      if should_crop(p) {
        if route.len() > 0 {
          // prev is outside, p is to crop
          let intersection = search(prev, p, 7);
          cutted_points.push(intersection);
          route.push(intersection);
          routes.push(route);
          route = vec![];
        } else {
          cutted_points.push(p);
        }
      } else {
        // nothing to crop
        route.push(p);
      }
      prev = p;
    }
    if route.len() >= 2 {
      routes.push(route);
    }
  }

  routes
}
