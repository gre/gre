use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
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
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist((self.x, self.y), p) < self.r
  }
}

fn waves_in_circle(
  opts: Opts,
  circle: &VCircle,
) -> (Vec<Vec<(f64, f64)>>, Vec<f64>) {
  let seed = opts.seed;
  let mut routes = Vec::new();
  let mut base_y = circle.y + 2. * circle.r;
  let perlin = Perlin::new();
  let get_color = image_get_color("images/world-map-2.png").unwrap();
  let f = |(x, y): (f64, f64)| {
    let mut p = (x, y);
    p.0 = p.0 / 0.8 - 0.1;
    p.1 = p.1 / 0.8 - 0.1;
    let c = get_color(p);
    if p.0 < 0.001 || p.0 > 0.999 || p.1 < 0.001 || p.1 > 0.999 {
      return 0.0;
    }
    smoothstep(0.0, 1.0, grayscale(c))
  };
  let mut passage = Passage2DCounter::new(0.4, circle.r * 2.0, circle.r * 2.0);
  let passage_limit = 10;
  let mut height_map: Vec<f64> = Vec::new();
  let mut line = 0;
  loop {
    if base_y < circle.y - circle.r - 10.0 {
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
      let l = f((
        (x - circle.x + circle.r) / (2. * circle.r),
        ((base_y - circle.r) * 1.4 + circle.r - circle.y + circle.r)
          / (2. * circle.r),
      ));
      let mut y = base_y;

      let m = 0.008 * circle.r;

      y -= m * 4.0 * l
        + m
          * 1.0
          * (0.2 + l)
          * perlin.get([
            0.2 * x,
            0.2 * y,
            seed
              + 4.0
                * perlin.get([
                  0.8 * y,
                  0.17 * x + perlin.get([0.2 * y, 0.06 * x, 100. + 7.3 * seed]),
                  10. + 0.3 * seed,
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
      if l < 0.0001 {
        collides = true;
      }
      let inside = !collides
        && passage.count((x - circle.x + circle.r, y - circle.y + circle.r))
          < passage_limit;
      if inside {
        if was_outside {
          if route.len() > 2 {
            if line % 2 == 0 {
              route.reverse();
            }
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
    if line % 2 == 0 {
      route.reverse();
    }
    routes.push(route);
    base_y -= 1.0;
    line += 1;
  }
  (routes, height_map)
}

fn wave(opts: Opts, c: &VCircle) -> Vec<Vec<(f64, f64)>> {
  let (waves, _height_map) = waves_in_circle(opts, c);
  waves
}

fn trip(a: (f64, f64), b: (f64, f64)) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let mut route = Vec::new();
  let radius = 1.0;
  let dr = 0.4;
  let approx = 0.1;
  let aspiral = spiral_optimized(a.0, a.1, radius, dr, approx);
  let mut bspiral = spiral_optimized(b.0, b.1, radius, dr, approx);
  bspiral.reverse();
  route.extend(aspiral.clone());
  route.extend(bspiral.clone());
  routes.push(route);
  routes
}

fn art(opts: Opts, width: f64, height: f64) -> Vec<Group> {
  let stroke_width = 0.35;

  let circle = VCircle::new(width / 2.0, height / 2.0, height * 0.7);
  let routes = wave(opts, &circle);
  let mut layers = Vec::new();

  let color = "white";
  let mut l = layer(color);
  let mut data = Data::new();
  for r in routes.iter() {
    let route = r.clone();
    data = render_route(data, route);
  }
  l = l.add(base_path(color, stroke_width, data));
  layers.push(l);

  let routes =
    trip((0.47 * width, 0.45 * height), (0.72 * width, 0.53 * height));

  let color = "gold";
  let mut l = layer(color);
  let mut data = Data::new();
  for r in routes.iter() {
    let route = r.clone();
    data = render_route(data, route);
  }
  l = l.add(base_path(color, stroke_width, data));
  layers.push(l);

  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let width = 297.0 / 2.0;
  let height = 210.0 / 2.0;
  let groups = art(opts, width, height);
  let mut document = base_document("black", width, height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
