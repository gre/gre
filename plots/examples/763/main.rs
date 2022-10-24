use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "148.0")]
  pub height: f64,
  #[clap(short, long, default_value = "105.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

#[derive(Clone, Copy)]
enum Couleur {
  Coeur,
  Pique,
  Trefle,
  Carreau,
}

fn pillar(radius: f64) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let iterations = (radius * 1.2) as usize;
  if iterations < 2 {
    return routes;
  }
  let dx = -0.4 * radius;
  let dy = 0.3 * radius;
  for i in 0..iterations {
    let p = (i as f64) / (iterations - 1) as f64;
    let path = vec![
      (0.0, 0.0),
      lerp_point((dx / 2.0, dy / 2.0), (0.0, dy), 0.4 + 0.6 * p),
      (dx, dy),
    ];
    routes.push(path_subdivide_to_curve(path, 2, mix(0.5, 1.0, p)));
  }
  routes = vec![routes.clone(), flipx_routes(routes.clone())].concat();
  routes
}

fn heart(radius: f64) -> Vec<(f64, f64)> {
  let spins = radius / 0.35;
  let iterations = (spins * 100.0) as usize;
  let mut route = Vec::new();
  if iterations < 2 {
    return route;
  }
  for i in 0..iterations {
    let p = (i as f64) / (iterations - 1) as f64;
    let r = 0.8 * radius * (p * (1.0 + 1.5 / spins)).min(1.0);
    let t = spins * 2.0 * PI * p;
    let x = r * (t.sin().powf(3.0));
    let mut v = t.cos() - t.cos().powf(4.0) + 0.55;
    v = mix(-1.0, v, smoothstep(-3.0, 0.0, v));

    let y = -r * v;
    route.push((x, y));
  }
  route
}

fn card_color(radius: f64, color: Couleur) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  match color {
    Couleur::Coeur => {
      routes.push(heart(radius));
    }
    Couleur::Carreau => {
      let mut r = radius;
      loop {
        if r < 0.1 {
          break;
        }
        let w = 1.1 * r;
        let h = 1.6 * r;
        let c = (0.0, 0.0);
        let path = vec![
          (-w, 0.0),
          c,
          (0.0, h),
          c,
          (w, 0.0),
          c,
          (0.0, -h),
          c,
          (-w, 0.0),
        ];
        let path = path_subdivide_to_curve_it(path, 0.55);
        let path = path_subdivide_to_curve_it(path, 0.6);
        let path = path_subdivide_to_curve_it(path, 0.65);
        routes.push(path);
        r -= 0.6;
      }
    }
    Couleur::Pique => {
      routes.push(flipy(heart(radius)));
      routes =
        vec![routes, translate_routes(pillar(radius), 0.0, 0.75 * radius)]
          .concat();
    }
    Couleur::Trefle => {
      let r = 0.45 * radius;
      let dy = r;
      let dy2 = 0.45 * r;
      let dx = r;
      for (x, y) in vec![(0.0, -dy), (dx, dy2), (-dx, dy2)] {
        routes.push(spiral_optimized(x, y, r, 0.35, 0.1));
        routes.push(circle_route((x, y), r, 80));
      }
      routes =
        vec![routes, translate_routes(pillar(radius), 0.0, 0.6 * radius)]
          .concat();
    }
  }

  routes
}

fn card_a(radius: f64) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let top = (0.0, -radius);
  let ang = 2.2f64;
  let bottomleft = (radius * ang.cos(), radius * ang.sin());
  let bottomright = (-bottomleft.0, bottomleft.1);
  routes.push(vec![bottomleft, top, bottomright]);
  let dx = 0.35 * radius;
  let dy = 0.2 * radius;
  routes.push(vec![(-dx, dy), (dx, dy)]);
  routes
}

fn translate_routes(
  routes: Vec<Vec<(f64, f64)>>,
  dx: f64,
  dy: f64,
) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| route.iter().map(|&p| (p.0 + dx, p.1 + dy)).collect())
    .collect()
}

fn flipy(route: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
  route.iter().map(|&p| (p.0, -p.1)).collect()
}

fn flipy_routes(routes: Vec<Vec<(f64, f64)>>) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| route.iter().map(|&p| (p.0, -p.1)).collect())
    .collect()
}

fn flipx_routes(routes: Vec<Vec<(f64, f64)>>) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| route.iter().map(|&p| (-p.0, p.1)).collect())
    .collect()
}

fn card_ace(
  origin: (f64, f64),
  radius: f64,
  angle: f64,
  color: Couleur,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();

  let a = 1.0f64;
  let dx = radius * a.cos();
  let dy = radius * a.sin();

  let path = vec![(dx, dy), (dx, -dy), (-dx, -dy), (-dx, dy), (dx, dy)];
  let path = path_subdivide_to_curve_it(path, 0.94);
  let path = path_subdivide_to_curve_it(path, 0.86);
  let path = path_subdivide_to_curve_it(path, 0.82);
  routes.push(path);

  let clr = match color {
    Couleur::Coeur | Couleur::Carreau => 0,
    _ => 1,
  };

  let r1 = 0.28 * radius;
  let d2x = 0.35 * radius;
  let d2y = 0.48 * radius;
  let r2 = 0.08 * radius;
  let d3x = d2x;
  let d3y = 0.65 * radius;
  let r3 = 0.08 * radius;

  routes = vec![
    routes,
    card_color(r1, color),
    translate_routes(card_color(r2, color), -d2x, -d2y),
    translate_routes(flipy_routes(card_color(r2, color)), d2x, d2y),
    translate_routes(card_a(r3), -d3x, -d3y),
    translate_routes(flipy_routes(card_a(r3)), d3x, d3y),
  ]
  .concat();

  //  rotate & translate
  let routes = routes
    .iter()
    .map(|route| {
      (
        clr,
        route
          .iter()
          .map(|&p| {
            let mut p = p_r(p, angle);
            p = round_point((p.0 + origin.0, p.1 + origin.1), 0.01);
            p
          })
          .collect(),
      )
    })
    .collect();

  routes
}

fn cell(opts: &Opts) -> Vec<(usize, Vec<(f64, f64)>)> {
  let seed = opts.seed;
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  // Prepare all the random values
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();

  let mut routes = Vec::new();

  let total_pad = pad + 2.0;
  let bound2 = (total_pad, total_pad, width - total_pad, height - total_pad);

  let does_overlap = |c: &VCircle| {
    strictly_in_boundaries((c.x, c.y), bound2)
      && circle_route((c.x, c.y), c.r, 8)
        .iter()
        .all(|&p| strictly_in_boundaries(p, bound2))
  };

  let ppad = rng.gen_range(0.3, 0.4);
  let min = ppad + rng.gen_range(1.0, 2.0);
  let max = ppad + rng.gen_range(8.0, 20.0);

  let mut cards = Vec::new();
  let f = rng.gen_range(0.0, 0.05) * rng.gen_range(0.0, 1.0);
  let f2 = rng.gen_range(0.0, 0.04) * rng.gen_range(0.0, 1.0);
  let f3 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let a3 = rng.gen_range(0.0, 10.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);
  let mountainvar = rng.gen_range(0.01, 0.05);
  for i in 0..1 {
    let circles = packing(
      seed + i as f64 * 111. / 3.,
      1000000,
      2000,
      rng.gen_range(1, 8),
      ppad,
      bound2,
      &does_overlap,
      min,
      max,
    );
    let amp = rng.gen_range(-1f64, 3.0).max(0.0);
    for c in circles {
      let angle = amp * perlin.get([opts.seed * 7.7 + 444., c.x * f, c.y * f]);
      let couleur =
        match if perlin.get([opts.seed / 0.3 + 44., c.x * f2, c.y * f2])
          + a3 * perlin.get([opts.seed / 0.3 + 44., c.x * f3, c.y * f3])
          + mountainvar * (c.y - height / 2.0)
          > 0.0
        {
          rng.gen_range(0, 2)
        } else {
          rng.gen_range(2, 4)
        } {
          0 => Couleur::Carreau,
          1 => Couleur::Coeur,
          2 => Couleur::Pique,
          _ => Couleur::Trefle,
        };
      cards.push(card_ace((c.x, c.y), c.r, angle, couleur));
    }
  }
  routes = vec![routes, cards.concat()].concat();

  // External frame to around the whole piece
  let mut d = 0.0;
  loop {
    if d > 2.0 {
      break;
    }
    routes.push((
      2,
      vec![
        (pad + d, pad + d),
        (pad + d, height - pad - d),
        (width - pad - d, height - pad - d),
        (width - pad - d, pad + d),
        (pad + d, pad + d),
      ],
    ));
    d += 0.3;
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let routes = cell(opts);

  // Make the SVG
  let colors = vec!["#f00", "#000", "#000"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (c, route) in routes.clone() {
        if c == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
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

fn circle_route(center: (f64, f64), r: f64, count: usize) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f64,
  y: f64,
  r: f64,
}
impl VCircle {
  fn new(x: f64, y: f64, r: f64) -> Self {
    VCircle { x, y, r }
  }
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

fn scaling_search<F: FnMut(f64) -> bool>(
  mut f: F,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let mut from = min_scale;
  let mut to = max_scale;
  loop {
    if !f(from) {
      return None;
    }
    if to - from < 0.1 {
      return Some(from);
    }
    let middle = (to + from) / 2.0;
    if !f(middle) {
      to = middle;
    } else {
      from = middle;
    }
  }
}

fn search_circle_radius(
  does_overlap: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) =
      search_circle_radius(&does_overlap, &circles, x, y, min_scale, max_scale)
    {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn path_subdivide_to_curve_it(
  path: Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path;
  }
  let mut route = Vec::new();
  let mut first = path[0];
  let mut last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if looped {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for i in 1..(l - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if looped {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if looped {
    route.push(first);
  }
  route
}

fn path_subdivide_to_curve(
  path: Vec<(f64, f64)>,
  n: usize,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}
