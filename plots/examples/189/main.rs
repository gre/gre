use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use rayon::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "10.0")]
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
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    container.contains(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  pad: f64,
  container: &VCircle,
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
    if let Some(size) =
      search_circle_radius(&container, &circles, x, y, min_scale, max_scale)
    {
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
) -> Vec<Vec<(f64, f64)>> {
  let offset_y = 8.0 + 0.2 * circle.r;
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
              + 2.8
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
  routes
}

fn art(opts: Opts) -> Vec<Group> {
  let width = 300.0;
  let height = 240.0;
  let pad = 10.0;
  let stroke_width = 0.3;
  let waves_sy = -0.2;
  let waves_dy = 1.0;

  let bounds_container =
    VCircle::new(width / 2.0, height / 2.0, height / 2.0 - pad);

  let primaries =
    packing(opts.seed, 100000, 1000, 2.0, &bounds_container, 2.0, 100.0);

  let routes = primaries
    .par_iter()
    .filter(|circle| circle.r > 2.0)
    .map(|circle| {
      waves_in_circle(
        opts.seed + circle.x * 9. + circle.y / 29.,
        circle,
        waves_sy,
        waves_dy,
      )
    })
    .collect::<Vec<Vec<Vec<(f64, f64)>>>>()
    .concat();

  let mut layers = Vec::new();

  let color = "black";
  let mut l = layer(color);
  for c in vec![primaries].concat() {
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
  l = l.add(signature(0.8, (180.0, 220.0), color));
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
