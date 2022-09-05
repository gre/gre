use clap::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "2")]
  index: usize,
  #[clap(short, long, default_value = "8")]
  frames: usize,
  #[clap(short, long, default_value = "43.")]
  seed: f64,
}

pub fn fill<F: FnMut((f64, f64)) -> f64>(
  data: Data,
  bounds: (f64, f64, f64, f64),
  increment: (f64, f64),
  mut f: F,
  threshold: f64,
  threshold_dir: bool,
) -> Data {
  let mut d = data;
  let mut p = (0.0, 0.0);
  loop {
    let mut q = p;
    let incr2 = (0.05 * increment.1, 0.05 * increment.0); // heuristic as it won't work with all kind of angles
    let mut r = Vec::new();
    loop {
      let next = (q.0 + incr2.0, q.1 + incr2.1);
      let v = f(normalize_in_boundaries(q, bounds));
      let write =
        threshold_dir && v < threshold || !threshold_dir && v > threshold;
      if write && r.len() == 0 || !write && r.len() == 1 {
        r.push(q);
      }
      if r.len() == 2 {
        d = render_route(d, r);
        r = Vec::new();
      }
      q = next;
      if out_of_boundaries(q, bounds) {
        if r.len() == 1 {
          r.push(q);
        }
        break;
      }
    }

    if r.len() == 2 {
      d = render_route(d, r);
    }

    p = (p.0 + increment.0, p.1 + increment.1);
    if out_of_boundaries(p, bounds) {
      break;
    }
  }
  d
}

fn shape(progress: f64, seed: f64, inside: bool) -> Path {
  let (w, h) = (120, 120);
  let bounds = (0.0, 0.0, 120.0, 120.0);
  let precision = 0.5;
  let width = (w as f64 / precision) as u32;
  let height = (h as f64 / precision) as u32;
  let perlin = Perlin::new();
  let f = |origin: (f64, f64)| {
    // this work is inspired from https://greweb.me/shaderday/68
    let t = 2. * PI * progress;
    let p = (origin.0 - 0.5, origin.1 - 0.5);
    let p1 = p_r(p, -2. * t);
    let p2 = p_r(p, t);
    let n = perlin.get([2.5 * p1.0, 2.5 * p1.1, 0.6 * seed + 7.643]);
    let n2 = perlin.get([
      3. * p2.0,
      3. * p2.1,
      1.7 * seed + 1.5 * perlin.get([7. * p2.0, 7. * p2.1, 1.7 * seed]),
    ]);
    let r = 0.15 + 0.08 * t.sin();
    f_op_union_round(
      length((p.0 - 0.1 * t.cos(), p.1)) - r - 0.15 * n,
      length(p) - r - 0.1 * n2,
      0.2,
    )
    .abs()
      - 0.1
  };
  let res = contour(width, height, f, &vec![-0.001, 0.0, 0.001]);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, bounds);
  let mut data = Data::new();
  data = fill(data, bounds, (0.0, 1.0), f, 0.001, inside);
  data = fill(data, bounds, (1.0, 0.0), f, 0.001, inside);
  if !inside {
    for _i in 0..3 {
      data = render_route(data, boundaries_route(bounds));
    }
  }
  for route in routes {
    data = render_route(data, route);
  }
  base_path("black", 0.2, data)
}

fn art(opts: &Opts) -> Vec<Group> {
  let p1 = opts.index as f64 / opts.frames as f64;
  let p2 = (0.5 + p1) % 1.0;
  let seed = opts.seed;
  vec![
    Group::new()
      .set("transform", "translate(20,60)")
      .add(shape(p1, seed, false)),
    Group::new()
      .set("transform", "translate(160,60)")
      .add(shape(p2, seed, true)),
    layer("black").add(signature(0.8, (120.0, 180.0), "black")),
  ]
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_24x30_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
