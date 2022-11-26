use clap::*;
use gre::*;
use noise::*;
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
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn dist3d(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
  let dx = a.0 - b.0;
  let dy = a.1 - b.1;
  let dz = a.2 - b.2;
  (dx * dx + dy * dy + dz * dz).sqrt()
}

fn mix3d(a: (f64, f64, f64), b: (f64, f64, f64), v: f64) -> (f64, f64, f64) {
  (mix(a.0, b.0, v), mix(a.1, b.1, v), mix(a.2, b.2, v))
}

fn project_along_direction(
  origin: (f64, f64, f64),
  direction: (f64, f64, f64),
  amp: f64,
) -> (f64, f64, f64) {
  (
    origin.0 + amp * direction.0,
    origin.1 + amp * direction.1,
    origin.2 + amp * direction.2,
  )
}

fn cylinder_spiral_raymarch(
  axis: ((f64, f64, f64), (f64, f64, f64)),
  axis_incr: f64,
  angle_incr: f64,
  ray_dist_max: f64,
  map_f: &dyn Fn(&(f64, f64, f64)) -> f64,
  project: &dyn Fn(&(f64, f64, f64)) -> (f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = vec![];

  let mut curves = vec![];
  let mut curve = vec![];

  let l = dist3d(axis.0, axis.1);
  let mut axis_progress = 0.0;
  let mut angle = 0f64;
  let turns_f = 2.0 * PI / angle_incr;

  let epsilon = 0.0001;

  loop {
    if axis_progress > l {
      break;
    }

    let p = mix3d(axis.0, axis.1, axis_progress / l);

    // TODO orthogonal to axis
    let direction = (angle.cos(), 0.0, angle.sin());

    let mut found = false;
    let mut ray_dist = 0.0;
    loop {
      if ray_dist > ray_dist_max {
        break;
      }

      let q = project_along_direction(p, direction, ray_dist);
      let d = map_f(&q);
      if d >= -epsilon {
        if ray_dist < epsilon {
          break;
        }
        found = true;
        break;
      }
      ray_dist -= d * 0.8;
    }

    if found {
      let q = project_along_direction(p, direction, ray_dist);
      curve.push(q);
    } else {
      if curve.len() > 1 {
        curves.push(curve);
        curve = vec![];
      } else if curve.len() > 0 {
        curve = vec![];
      }
    }

    axis_progress += axis_incr / turns_f;
    angle += angle_incr;
  }

  if curve.len() > 1 {
    curves.push(curve);
  }

  for curve in curves {
    routes.push(curve.iter().map(project).collect());
  }

  routes
}

fn clamp(a: f64, from: f64, to: f64) -> f64 {
  (a).max(from).min(to)
}

fn op_smooth_union(d1: f64, d2: f64, k: f64) -> f64 {
  let h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0.0, 1.0);
  mix(d2, d1, h) - k * h * (1.0 - h)
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;

  let perlin = Perlin::new();

  let map_f = |&p: &(f64, f64, f64)| {
    let sphere1 = dist3d((0.0, 4.0, 0.0), p) - 2.0;
    let sphere2 = dist3d((0.0, 7.0, 0.0), p) - 1.5;

    let amp = 0.4;
    let f = 0.5;
    let n1 = amp * perlin.get([opts.seed + f * p.0, f * p.1, f * p.2]);

    let amp = 0.2;
    let f = 5.5;
    let n2 = amp * perlin.get([f * p.0, opts.seed + f * p.1, f * p.2]);

    let n = n1 + n2;

    op_smooth_union(sphere2, sphere1, 0.8) + n
  };

  let project = |&(x, y, _z): &(f64, f64, f64)| -> (f64, f64) {
    // TODO camera perspective (center, focus point)
    let center = (0.0, 5.0);
    let ratio = height / 10.0;
    (
      (x - center.0) * ratio + width / 2.0,
      height - (y - center.1) * ratio - height / 2.0,
    )
  };

  let routes = cylinder_spiral_raymarch(
    ((0.0, 0.0, 0.0), (0.0, 10.0, 0.0)),
    0.04,
    0.05,
    50.0,
    &map_f,
    &project,
  );

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
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
