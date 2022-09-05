use std::f64::consts::PI;

use clap::*;
use geo::intersects::Intersects;
use geo::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::{Circle, Group};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "3")]
  index: usize,
  #[clap(short, long, default_value = "11")]
  frames: usize,
  #[clap(short, long, default_value = "15.0")]
  seed: f64,
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
  fn contains(self: &Self, c: &VCircle) -> bool {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - self.r + c.r < 0.0
  }
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist((self.x, self.y), p) < self.r
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
  container: &VCircle,
  circles: &Vec<VCircle>,
  height_map: Vec<f64>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let f = |size| {
    let c = VCircle::new(x, y, size);
    let factor = 2. * container.r / (height_map.len() as f64);
    let collides_height_map = height_map.iter().enumerate().any(|(i, &y)| {
      let x = container.x - container.r + i as f64 * factor;
      (c.x - x).abs() < c.r && y < c.y || c.includes((x, y))
    });
    !collides_height_map
      && container.contains(&c)
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(f, min_scale, max_scale)
}

fn packing_circles(
  extra_circles: Vec<VCircle>,
  seed: f64,
  iterations: usize,
  desired_count: usize,
  pad: f64,
  container: &VCircle,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut all = extra_circles.clone();
  let mut circles = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = container.x - container.r;
  let y1 = container.y - container.r;
  let x2 = container.x + container.r;
  let y2 = container.y + container.r;
  let max_scale = max_scale.min(container.r);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    if let Some(size) = search_circle_radius(
      &container,
      &all,
      Vec::new(),
      x,
      y,
      min_scale,
      max_scale,
    ) {
      let circle = VCircle::new(x, y, size - pad);
      circles.push(circle.clone());
      all.push(circle.clone());
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
}

fn packing_waves(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  pad: f64,
  container: &VCircle,
  height_map: &Vec<f64>,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = container.x - container.r;
  let y1 = container.y - container.r;
  let x2 = container.x + container.r;
  let y2 = container.y + container.r;
  let max_scale = max_scale.min(container.r);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    if let Some(size) = search_circle_radius(
      &container,
      &circles,
      height_map.clone(),
      x,
      y,
      min_scale,
      max_scale,
    ) {
      let circle = VCircle::new(x, y, size - pad);
      circles.push(circle.clone());
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
}

fn waves_in_circle(
  seed: f64,
  circle: &VCircle,
  sy: f64,
  dy: f64,
) -> (Vec<Vec<(f64, f64)>>, Vec<f64>) {
  let offset_y = 6.0 + 0.4 * circle.r;
  let mut routes = Vec::new();
  let mut base_y = circle.y + circle.r + offset_y;
  let perlin = Perlin::new();
  let mut passage = Passage2DCounter::new(0.5, circle.r * 2.0, circle.r * 2.0);
  let passage_limit = 8;
  let mut height_map: Vec<f64> = Vec::new();
  loop {
    if base_y < circle.y + sy * circle.r {
      break;
    }
    let precision = 0.2;
    let mut route = Vec::new();
    let mut x = circle.x - circle.r;
    let mut was_outside = true;
    let mut i = 0;
    loop {
      if x > circle.x + circle.r {
        break;
      }
      let y = base_y
        + offset_y
          * perlin.get([
            0.02 * x,
            0.02 * base_y,
            seed
              + 1.6
                * perlin.get([
                  0.03 * x
                    + 0.1
                      * perlin.get([0.2 * x, 0.2 * base_y, 100. + seed / 3.0]),
                  0.03 * base_y,
                  10. + seed / 7.0,
                ]),
          ]);
      let mut collides = false;
      if i >= height_map.len() {
        height_map.push(y);
      } else {
        if y > height_map[i] {
          collides = true;
        } else {
          height_map[i] = y;
        }
      }
      let inside = !collides
        && circle.includes((x, y))
        && passage.count((x - circle.x + circle.r, y - circle.y + circle.r))
          < passage_limit;
      if inside {
        if was_outside {
          if route.len() > 2 {
            routes.push(route);
          }
          route = Vec::new();
        }
        was_outside = false;
        route.push((x, y));
      } else {
        was_outside = true;
      }
      x += precision;
      i += 1;
    }
    routes.push(route);

    base_y -= dy;
  }
  (routes, height_map)
}

type CirclesAndRoutes = (Vec<VCircle>, Vec<Vec<(f64, f64)>>);

fn waveball(n: usize, seed: f64, c: &VCircle) -> CirclesAndRoutes {
  if n > 3 {
    return (Vec::new(), Vec::new());
  }
  let (waves, height_map) = waves_in_circle(seed, c, 0.1, 0.6);

  let res = packing_waves(seed, 50000, 500, 3.0, c, &height_map, 2.0, 100.0)
    .par_iter()
    .filter(|circle| circle.r > 1.0)
    .map(|circle| {
      waveball(n + 1, seed + circle.x * 9. + circle.y / 29., circle)
    })
    .collect::<Vec<_>>();

  let mut circles_acc = Vec::new();
  let mut routes_acc = Vec::new();
  circles_acc.push(vec![c.clone()]);
  routes_acc.push(waves);
  for (circles, routes) in res {
    circles_acc.push(circles);
    routes_acc.push(routes);
  }
  let circles = circles_acc.concat();
  let routes = routes_acc.concat();
  (circles, routes)
}

fn contours(seed: f64, circle: &VCircle) -> CirclesAndRoutes {
  let precision = 0.5;
  let w = (2. * circle.r as f64 / precision) as u32;
  let h = (2. * circle.r as f64 / precision) as u32;
  let perlin = Perlin::new();

  let f = |(x, y): (f64, f64)| {
    let d = euclidian_dist((x, y), (0.5, 0.5));
    0.2
      + 2.0 * d
      + mix(
        0.7
          * perlin.get([
            2. * x + 0.01 * circle.x,
            2. * y + 0.01 * circle.y,
            0.4 * seed
              + 0.6
                * perlin.get([
                  3. * x + 1. * perlin.get([6. * x, 6. * y, 5. + seed]),
                  3. * y + 1. * perlin.get([5. * x, 5. * y, 4. + seed]),
                  10. + seed,
                ]),
          ]),
        0.0,
        smoothstep(0.3, 0.499, d),
      )
  };
  let count = (1. * circle.r) as usize;
  let thresholds = (0..count).map(|i| i as f64 / (count as f64)).collect();
  let res = contour(w, h, f, &thresholds);
  let routes = features_to_routes(res, precision)
    .iter()
    .filter_map(|route| {
      let mut mapped = Vec::new();
      for p in route {
        let newp = (circle.x - circle.r + p.0, circle.y - circle.r + p.1);
        if !circle.includes(newp) {
          return None;
        }
        mapped.push(newp);
      }
      Some(mapped)
    })
    .collect();
  let circles = vec![circle.clone()];
  (circles, routes)
}

fn scaling_search_in_container<F: FnMut(f64, f64, f64, f64) -> Polygon<f64>>(
  mut make_shape: F,
  container: &VCircle,
  polys: &Vec<Polygon<f64>>,
  x: f64,
  y: f64,
  a: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let poly = make_shape(x, y, size, a);
    poly
      .exterior()
      .points_iter()
      .all(|p| container.includes(p.x_y()))
      && !polys.iter().any(|p| poly.intersects(p))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn poly_accumulate<F: FnMut(f64, f64, f64, f64) -> Polygon<f64>>(
  p: f64,
  mut make_shape: F,
  seed: f64,
  iterations: usize,
  desired_count: usize,
  pad: f64,
  container: &VCircle,
  min_scale: f64,
) -> Vec<Polygon<f64>> {
  let mut polys = Vec::new();
  let mut shapes = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = container.x - container.r;
  let y1 = container.y - container.r;
  let x2 = container.x + container.r;
  let y2 = container.y + container.r;
  let max_scale = 0.6 * container.r;
  for i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    let a: f64 = 0.5 * PI * p + (i as f64) * PI / 5.0;
    if let Some(size) = scaling_search_in_container(
      &mut make_shape,
      container,
      &polys,
      x,
      y,
      a,
      min_scale,
      max_scale,
    ) {
      let poly = make_shape(x, y, size - pad, a);
      polys.push(poly);
      for i in 0..8 {
        let l = size - pad - (i as f64 * 0.4).powf(2.0);
        if l < 0.1 {
          break;
        }
        shapes.push(make_shape(x, y, l, a));
      }
    }
    if polys.len() > desired_count {
      break;
    }
  }
  shapes
}

fn add(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
  (a.0 + b.0, a.1 + b.1)
}

fn rotated_square_as_polygon(
  x: f64,
  y: f64,
  size: f64,
  angle: f64,
) -> Polygon<f64> {
  polygon![
    add(p_r((-size, -size), angle), (x, y)).into(),
    add(p_r((size, -size), angle), (x, y)).into(),
    add(p_r((size, size), angle), (x, y)).into(),
    add(p_r((-size, size), angle), (x, y)).into(),
  ]
}

fn squares(p: f64, seed: f64, circle: &VCircle) -> CirclesAndRoutes {
  let polygons = poly_accumulate(
    p,
    &rotated_square_as_polygon,
    seed,
    200000,
    5000,
    0.4,
    circle,
    0.6,
  );
  let routes = polygons
    .iter()
    .map(|poly| poly.exterior().points_iter().map(|p| p.x_y()).collect())
    .collect();
  (vec![circle.clone()], routes)
}

fn art(opts: Opts) -> Vec<Group> {
  let width = 300.0;
  let height = 240.0;
  let stroke_width = 0.3;

  let p = (opts.index as f64) / (opts.frames as f64);
  let rot3 = 2. * PI / 3.;
  let a1 = 2. * PI * p;
  let a2 = a1 + rot3;
  let a3 = a2 + rot3;
  let rfrom = 20.0;
  let rto = 64.0;
  let rfull = 115.0;
  let off = 62.0;
  let r1 = mix(rfrom, rto, 0.5 + 0.5 * a1.cos());
  let r2 = mix(rfrom, rto, 0.5 + 0.5 * a2.cos());
  let r3 = mix(rfrom, rto, 0.5 + 0.5 * a3.cos());

  let c = (width / 2.0 - (rto - rfrom) / 2., height / 2.0);
  let c1 = VCircle::new(c.0 + off * a1.cos(), c.1 + off * a1.sin(), r1);
  let c2 = VCircle::new(c.0 + off * a2.cos(), c.1 + off * a2.sin(), r2);
  let c3 = VCircle::new(c.0 + off * a3.cos(), c.1 + off * a3.sin(), r3);
  let (circles1, routes1) = waveball(0, opts.seed, &c1);
  let (circles2, routes2) = squares(p, opts.seed, &c2);
  let (circles3, routes3) = contours(opts.seed, &c3);

  let mainc = VCircle::new(width / 2., height / 2., rfull);

  let primaries = packing_circles(
    vec![c1, c2, c3],
    opts.seed,
    100000,
    1000,
    1.0,
    &mainc,
    1.0,
    50.0,
  );

  let secondaries = primaries
    .par_iter()
    .filter(|p| p.r > 3.0)
    .map(|c| {
      packing_circles(
        vec![],
        opts.seed + c.x / 3. + c.y * 143.6,
        100000,
        400,
        0.8,
        &c,
        0.8,
        2.0,
      )
    })
    .collect::<Vec<_>>()
    .concat();

  let third = secondaries
    .par_iter()
    .filter(|p| p.r > 2.0)
    .map(|c| {
      packing_circles(
        vec![],
        opts.seed + c.x / 7. + c.y * 43.6,
        100000,
        400,
        0.8,
        &c,
        0.8,
        2.0,
      )
    })
    .collect::<Vec<_>>()
    .concat();

  let circles = vec![vec![mainc], primaries, secondaries, third].concat();

  let mut layers = Vec::new();
  let color = "grey";
  let mut l = layer(color);
  for c in circles {
    l = l.add(
      Circle::new()
        .set("r", c.r)
        .set("cx", c.x)
        .set("cy", c.y)
        .set("stroke", color)
        .set("stroke-width", stroke_width)
        .set("fill", "none")
        .set("style", "mix-blend-mode: multiply;"),
    );
  }
  l = l.add(signature(0.8, (195.0, 225.0), color));
  layers.push(l);

  let circles = vec![circles1, circles2, circles3].concat();
  let routes = vec![routes1, routes2, routes3].concat();

  let color = "black";
  let mut l = layer(color);
  for c in circles {
    l = l.add(
      Circle::new()
        .set("r", c.r)
        .set("cx", c.x)
        .set("cy", c.y)
        .set("stroke", color)
        .set("stroke-width", stroke_width)
        .set("fill", "none")
        .set("style", "mix-blend-mode: multiply;"),
    );
  }
  let mut data = Data::new();
  for r in routes {
    data = render_route(data, r);
  }
  l = l.add(base_path(color, stroke_width, data));
  layers.push(l);

  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_24x30_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
