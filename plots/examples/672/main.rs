use clap::*;
use geo::prelude::*;
use geo::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::{path::Data, Group};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "2772.0")]
  seed: f64,
  #[clap(short, long, default_value = "700.0")]
  pub width: f64,
  #[clap(short, long, default_value = "500.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "400000")]
  pub packing_iterations: usize,
}
fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;
  let scale = width / 700.0;
  let stroke_width = 0.35;

  let mut passage = Passage::new(0.5, width, height);

  let mut routes_amber = Vec::new();
  let mut routes_turquoise = Vec::new();
  let mut routes_soft_mint = Vec::new();
  let mut routes_purple = Vec::new();
  let mut routes_pink = Vec::new();
  let mut routes_dark_blue = Vec::new();

  let mut rng = rng_from_seed(opts.seed);

  // (1) Mountains
  let perlin = Perlin::new();
  let mountain_base_y = height * 0.6;

  let min_route = 2;
  let yincr = 0.5;
  let precision = 0.2;

  let mut base_y = height * 5.0;
  let stopy = rng.gen_range(0.4, 0.6) * height;
  let mut height_map: Vec<f64> = Vec::new();
  loop {
    if base_y < stopy {
      break;
    }

    let mut route = Vec::new();
    let mut x = pad;
    let mut was_outside = true;
    loop {
      if x > width - pad {
        break;
      }
      let xv = (4.0 - base_y / height) * (x - width / 2.) / scale;

      let amp = height * 0.05;
      let shape = -perlin
        .get([
          //
          88.3 + xv * 0.005,
          88.1 + base_y * 0.02 / scale,
          opts.seed / 7.3
            + perlin.get([
              //
              -opts.seed * 7.3,
              8.3 + xv * 0.02,
              8.1 + base_y * 0.1 / scale,
            ]),
        ])
        .abs()
        + 5.0
          * perlin.get([
            //
            88.3 + xv * 0.002,
            88.1 + base_y * 0.0001 / scale,
            opts.seed * 97.3,
          ])
        + 3.0
          * perlin.get([
            //
            opts.seed * 9.3,
            88.3 + xv * 0.001,
            88.1 + base_y * 0.0005 / scale,
          ])
        + 0.1
          * perlin.get([
            //
            88.3 + xv * 0.1,
            88.1 + base_y * 0.5 / scale,
            -opts.seed * 9.3,
          ]);

      let y = base_y + amp * shape;
      let mut collides = false;
      let xi = (x / precision) as usize;
      if xi >= height_map.len() {
        height_map.push(y.min(mountain_base_y));
      } else {
        if y > height_map[xi] {
          collides = true;
        } else {
          height_map[xi] = y;
        }
      }
      let inside =
        !collides && pad < x && x < width - pad && pad < y && y < height - pad;
      if inside {
        if was_outside {
          if route.len() > min_route {
            routes_purple.push(route);
          }
          route = Vec::new();
        }
        was_outside = false;
        passage.count((x, y));
        route.push((x, y));
      } else {
        was_outside = true;
      }

      x += precision;
    }

    base_y -= yincr;
  }

  // (2) Make neons
  let mut circles_count = 0;
  let cpad = rng.gen_range(1.5, 2.0) * (0.2 + 0.8 * scale);
  let min_scale = cpad + 0.8 * (0.7 + 0.3 * scale);
  let max_scale = min_scale + 1.0 * (0.7 + 0.3 * scale);
  let spiral_dr = 0.6;
  let neon_layers_count = 3;
  let anti_centering = rng.gen_range(0.0, 0.99);

  let neons = rng.gen_range(10, 32);
  let neon_width_base = (8.0 + rng.gen_range(00.0, 40.0) * rng.gen_range(0.0, 1.0)) * scale;
  let find_on_mountain_retries = rng.gen_range(3, 12);

  for i in 0..neons {
    let mut exterior = vec![];
    for _j in 0..find_on_mountain_retries {
      let w = neon_width_base + rng.gen_range(0.0, neon_width_base) * rng.gen_range(0.1, 1.0);
      let wf = rng.gen_range(0.0, 1.0);
      let w2 = wf * w;
      let dx: f64 = width * rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0) + (w - w2) / 2.0;
      let xw = width - 2.0 - pad - w2 ;
      let x2 =
        xw / 2.0 + rng.gen_range(-xw / 2.0, xw / 2.0) * rng.gen_range(anti_centering, 1.0);
      let x1 = (x2 + dx).min(width - w - pad).max(pad);
      let y1 = pad;
      let xi = ((x2 + w / 2.0) / precision) as usize;
      let h = height_map[xi];
      let y2 = h + 30.0;
      exterior = vec![(x1, y1), (w + x1, y1), (w2 + x2, y2), (x2, y2)];
      if h < mountain_base_y - 5.0 {
        break;
      }
    }

    if exterior.len() == 0 {
      continue;
    }

    let p = Polygon::new(
      exterior.into(),
      vec![],
    );

    let does_overlap = |(x, y): (f64, f64)| {
    let center: Point<f64> = (x, y).into();
    let inside = p.contains(&center);
      let xi = (x / precision) as usize;
      inside && y < height_map[xi]
    };
    let mut routes = Vec::new();
    for j in 0..neon_layers_count {
      let boundaries = p.bounding_rect().unwrap();
      let topleft = boundaries.min();
      let bottomright = boundaries.max();
      let bound = (topleft.x, topleft.y, bottomright.x, bottomright.y);
      let circles = packing(
        opts.seed * 7.7 + j as f64 / 3.0 + i as f64 * 11.1,
        opts.packing_iterations,
        5000,
        1,
        cpad,
        bound,
        &does_overlap,
        min_scale,
        max_scale,
      );
      for c in circles {
        let res = (c.r * 2.0 + 8.0) as usize;
        routes.push(circle_route((c.x, c.y), c.r, res));
        routes.push(spiral_optimized(c.x, c.y, c.r, spiral_dr, 0.3));
        circles_count += 1;
      }
    }

    for route in routes.clone() {
      for p in route {
        passage.count(p);
      }
    }

    let color_index = i % 3;
    match color_index {
      0 => {
        for r in routes {
          routes_pink.push(r);
        }
      }
      1 => {
        for r in routes {
          routes_soft_mint.push(r);
        }
      }
      _ => {
        for r in routes {
          routes_turquoise.push(r);
        }
      }
    }
  }

  println!("{} circles", circles_count);

  // (3) Sun

  let max_r: f64 = rng.gen_range(70.0, 90.0) * width / 700.0;
  let two_pi = 2.0 * PI;
  let xc = width / 2.0;
  let xi = (xc / precision) as usize;
  let yc =
    height_map[xi] + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0) * max_r;
  let dr = 0.3;
  let mut current_r: f64 = max_r + 2.0;
  let mut a: f64 = 0.0;
  let mut route = Vec::new();
  loop {
    let r = current_r.min(max_r);
    let p = round_point((xc + r * a.cos(), yc + r * a.sin()), 0.01);
    passage.count(p);
    let xi = (p.0 / precision) as usize;
    let ymountain = height_map[xi];
    let yp = (p.1 - yc) / max_r;
    let collides = p.1 > ymountain || 
    // slash
    ((yp + 1.0) * 16.0) % 2.0 > 2.8 - 1.5 * (1.0 + yp);
    if collides {
      if route.len() > 1 {
        routes_amber.push(route);
      }
      route = Vec::new();
    } else {
      route.push(p);
    }

    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    current_r -= dr * da / two_pi;
    if current_r < 0.1 {
      break;
    }
  }
  if route.len() > 1 {
    routes_amber.push(route);
  }

  // (4) wireframe ground
  let mut y = mountain_base_y;
  let mut dy = 2.0;
  loop {
    if y > height - pad {
      break;
    }
    routes_purple.push(vec![(pad, y), (width - pad, y)]);
    y += dy;
    dy *= 1.0;
  }
  let bound = (pad, pad, width - pad, height - pad);
  let aincr = PI / 32.0;
  let mut a = aincr / 2.0;
  loop {
    if a > PI {
      break;
    }
    let mut from = (width / 2.0, mountain_base_y);
    let disp = rng.gen_range(0.0, 8.0) * (0.5 + 0.5 * scale);
    from.0 += disp * a.cos();
    from.1 += disp * a.sin();
    let to = (from.0 + 999. * a.cos(), from.1 + 999. * a.sin());
    if let Some(p) = collide_segment_boundaries(from, to, bound) {
      routes_purple.push(vec![from, p]);
    }
    a += aincr;
  }

  // (5) apply reflections

  for (ci, routes) in vec![
    routes_amber.clone(),
    routes_pink.clone(),
    routes_purple.clone(),
    routes_soft_mint.clone(),
    routes_turquoise.clone()
  ].iter().enumerate() {
    let proba =
      if ci == 0 { 0.84 }
      else if ci == 2 { 0.95 }
      else { 0.9 };
    for route in routes.clone() {
      for (x, y) in route.clone() {
        if y > mountain_base_y {
          continue;
        }
        if rng.gen_bool(proba) {
          continue;
        }
        let p = (x, mountain_base_y * 2.0 - y);
        if p.1 > height - pad {
          continue;
        }
        if passage.count(p) < 8 {
          let w = rng.gen_range(1.0, 2.0);
          let line = vec![
            (p.0 - w, p.1),
            (p.0 + w, p.1),
          ];
          match ci {
             0 =>  {routes_amber.push(line);}
            1 => {routes_pink.push(line)}
            2 => {routes_purple.push(line)}
            3 => {routes_soft_mint.push(line)}
            _ => {routes_turquoise.push(line)}          
        }
      }}
    }
  }


  // (6) fill sky with concentric circles on empty spaces

  let r = 5.0 * (0.5 + 0.5 * scale);
  passage.grow_passage(r);
  
  let bound = (pad, pad,width-pad,  mountain_base_y-r);
  let cpad = 2.0 * (0.5 + 0.5 * scale);
  let does_overlap = |p: (f64, f64)| {
    strictly_in_boundaries(p, bound) &&
    passage.get(p) < 1
  };
  let circles = packing(
    opts.seed*3.3+999.,
    opts.packing_iterations,
    10000,
    1,
    cpad,
    bound,
    &does_overlap,
    min_scale,
    max_scale,
  );
  for c in circles {
    let res = (c.r * 2.0 + 8.0) as usize;
    routes_dark_blue.push(circle_route((c.x, c.y), c.r, res));
  }

  let data: Vec<(Vec<Vec<(f64, f64)>>, &str, usize)> = vec![
    (routes_amber, "#FC0", 0),
    (routes_pink, "#F39", 2),
    (routes_soft_mint, "#0D8", 3),
    (routes_turquoise, "#08E", 4),
    (routes_purple, "#505", 5),
    (routes_dark_blue, "#008", 1),
  ];

  data
    .iter()
    .map(|(routes, color, ci)| {
      let mut data = Data::new();
      for route in routes {
        data = render_route(data, route.clone());
      }
      layer(format!("{} {}", ci, color).as_str()).add(base_path(color, stroke_width, data))
    })
    .collect::<Vec<_>>()
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y))
      && !circles.iter().any(|other| c.collides(other))
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) = search_circle_radius(
      &does_overlap,
      &circles,
      x,
      y,
      min_scale,
      max_scale,
    ) {
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


#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }

  pub fn count_once(self: &mut Self, p: (f64, f64)) {
    let i = self.index(p);
    let v = self.counters[i];
    if v == 0 {
      self.counters[i] = 1;
    }
  }

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }

  pub fn grow_passage(self: &mut Self, radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters: Vec<usize> = self.counters.iter().cloned().collect();
    let mut mask = Vec::new();
    // TODO, in future for even better perf, I will rewrite this
    // working directly with index integers instead of having to use index() / count_once()
    let mut x = -radius;
    loop {
      if x >= radius {
        break;
      }
      let mut y = -radius;
      loop {
        if y >= radius {
          break;
        }
        if x * x + y * y < radius * radius {
          mask.push((x, y));
        }
        y += precision;
      }
      x += precision;
    }

    let mut x = 0.0;
    loop {
      if x >= width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= height {
          break;
        }
        let index = self.index((x, y));
        if counters[index] > 0 {
          for &(dx, dy) in mask.iter() {
            self.count_once((x + dx, y + dy));
          }
        }
        y += precision;
      }
      x += precision;
    }
  }

}
