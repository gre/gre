use clap::*;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
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
  let bounds = (pad, pad, width - pad, height - pad);
  let mut black_routes = vec![];
  let mut red_routes = vec![];
  let mut blue_routes = vec![];
  let mut yellow_routes = vec![];
  let mut amber_routes = vec![];
  let mut grey_routes = vec![];

  let precision = 0.1;

  let radius = width / 4.0;
  let (cx, cy) = (width / 2.0, height / 2.0);

  let triangle_size = 2.2;
  let equilateral_triangle = vec![
    (cx, cy - triangle_size),
    (cx + triangle_size, cy + triangle_size * 0.5),
    (cx - triangle_size, cy + triangle_size * 0.5),
  ];

  let step = 1.0 / 3.0;
  let mut i = 0;
  let mut r = radius;
  loop {
    if r <= 0.0 {
      break;
    }
    let imod = i % 4;
    let ri = (3.0 * (r - step / 2.0) / radius).floor() as usize;

    let left_arc = arc(cx, cy, r, PI / 2.0, 3.0 * PI / 2.0, precision);
    let right_arc = arc(cx, cy, r, -PI / 2.0, PI / 2.0, precision);
    let full_circle = arc(cx, cy, r, 0.0, 2.0 * PI, precision);

    if ri == 0 {
      let mut cutted = vec![];
      let routes = crop_routes_with_predicate(
        &vec![left_arc],
        &|p| point_is_in_polygon(p, &equilateral_triangle),
        &mut cutted,
      );
      black_routes.extend(routes.clone());
      if imod == 0 {
        red_routes.push(cutted.clone());
      } else if imod == 2 {
        amber_routes.push(cutted.clone());
      }

      let routes = mirrorx(cx, &routes);
      let cutted = mirrorx_route(cx, &cutted);
      black_routes.push(cutted.clone());

      if imod == 0 {
        red_routes.extend(routes.clone());
      } else if imod == 2 {
        amber_routes.extend(routes.clone());
      }
    } else if ri == 1 {
      black_routes.push(left_arc.clone());
      if imod == 0 {
        yellow_routes.push(right_arc.clone());
      } else if imod == 2 {
        amber_routes.push(right_arc.clone());
      }
    } else {
      if imod == 0 {
        grey_routes.push(full_circle.clone());
      } else if imod != 1 {
        blue_routes.push(right_arc.clone());
      }
    }

    r -= step;
    i += 1;
  }

  let extern_spiral =
    open_spiral_optimized(cx, cy, width * 1.5, 0.5, 0.1, radius);

  let clip = |p| !strictly_in_boundaries(p, bounds);

  let mut cutted = vec![];
  red_routes.extend(crop_routes_with_predicate(
    &vec![extern_spiral],
    &clip,
    &mut cutted,
  ));

  for i in 0..3 {
    let p = pad + i as f64 * 0.3;
    red_routes.push(vec![
      (p, p),
      (width - p, p),
      (width - p, height - p),
      (p, height - p),
      (p, p),
    ]);
  }

  vec![
    (red_routes.clone(), "#d20"),
    (black_routes.clone(), "#000"),
    (blue_routes.clone(), "#39f"),
    (yellow_routes.clone(), "#ff3"),
    (amber_routes.clone(), "#d60"),
    (grey_routes.clone(), "#888"),
  ]
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

fn lerp_point(a: (f64, f64), b: (f64, f64), t: f64) -> (f64, f64) {
  return (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
}

fn mirrorx_route(cx: f64, route: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
  route
    .iter()
    .map(|p| (cx - (p.0 - cx), p.1))
    .collect::<Vec<(f64, f64)>>()
}

fn mirrorx(cx: f64, routes: &Vec<Vec<(f64, f64)>>) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| mirrorx_route(cx, &route))
    .collect::<Vec<Vec<(f64, f64)>>>()
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

fn arc(
  x: f64,
  y: f64,
  radius: f64,
  start_angle: f64,
  end_angle: f64,
  precision: f64,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  let mut a = start_angle;
  let da = precision * (end_angle - start_angle).signum() / (2.0 * PI * radius);
  loop {
    let p = (x + radius * a.cos(), y + radius * a.sin());
    route.push(p);
    a += da;
    if a >= end_angle {
      break;
    }
  }
  route
}

fn point_is_in_polygon(p: (f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
  let mut c = false;
  for i in 0..polygon.len() {
    let j = (i + 1) % polygon.len();
    if ((polygon[i].1 > p.1) != (polygon[j].1 > p.1))
      && (p.0
        < (polygon[j].0 - polygon[i].0) * (p.1 - polygon[i].1)
          / (polygon[j].1 - polygon[i].1)
          + polygon[i].0)
    {
      c = !c;
    }
  }
  return c;
}

fn open_spiral_optimized(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  approx: f64,
  open: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = PI / 2.0;
  loop {
    let p = round_point((x + r * a.cos(), y + r * a.sin()), 0.01);
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < open {
      break;
    }
  }
  route
}
