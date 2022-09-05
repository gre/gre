use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "117.0")]
  seed: f64,
  #[clap(short, long, default_value = "0")]
  index: usize,
  #[clap(short, long, default_value = "4")]
  frames: usize,
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
  seed: f64,
  circle: &VCircle,
  dy: f64,
  p: f64,
) -> (Vec<Vec<(f64, f64)>>, Vec<f64>) {
  let mut routes = Vec::new();
  let mut base_y = circle.y + 2. * circle.r;
  let perlin = Perlin::new();
  let mut passage = Passage2DCounter::new(0.35, circle.r * 2.0, circle.r * 2.0);
  let passage_limit = 10;
  let precision = 0.2;
  let mut height_map: Vec<f64> = Vec::new();
  loop {
    if base_y < circle.y - 0.5 * circle.r {
      break;
    }
    let a = p * 2. * PI;
    let v = perlin.get([12. * base_y, seed]);
    if v < 0.3 {
      let mut route = Vec::new();
      let mut x = circle.x - circle.r;
      let mut was_outside = true;
      let mut i = 0;
      loop {
        if x > circle.x + circle.r {
          break;
        }
        let y = base_y
          + (circle.r
            - 0.6
              * euclidian_dist(
                (circle.x, circle.y + 0.9 * circle.r),
                (x, base_y),
              ))
          .max(0.)
            * (0.6
              * perlin.get([
                0.01 * x,
                0.01 * base_y,
                seed
                  + 2.0
                    * perlin.get([
                      0.05 * base_y
                        + 0.003
                          * perlin.get([base_y, 0.3 * x, 10. + 5.3 * seed])
                        + 0.05 * a.cos(),
                      0.02 * x + 0.02 * a.sin(),
                      1. + 0.7 * seed,
                    ]),
              ])
              - 5.0
                * perlin
                  .get([
                    0.006 * x + 0.05 * a.cos(),
                    0.005 * base_y,
                    -7.
                      + 9. * seed
                      + 0.02
                        * perlin.get([0.02 * base_y, 0.02 * x, seed / 7. - 9.])
                      + 0.05 * a.sin(),
                  ])
                  .powf(2.0));
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
    }

    base_y -= dy;
  }
  (routes, height_map)
}

type WaveballRes = (Vec<VCircle>, Vec<Vec<(f64, f64)>>);

fn waveball(n: usize, seed: f64, c: &VCircle, p: f64) -> WaveballRes {
  if n > 3 {
    return (Vec::new(), Vec::new());
  }
  let (waves, _height_map) = waves_in_circle(seed, c, 0.2, p);
  let mut circles_acc = Vec::new();
  let mut routes_acc = Vec::new();
  circles_acc.push(vec![c.clone()]);
  routes_acc.push(waves);
  let circles = circles_acc.concat();
  let routes = routes_acc.concat();
  (circles, routes)
}

fn art(opts: Opts) -> Vec<Group> {
  let width = 300.0;
  let height = 240.0;
  let pad = 10.0;
  let stroke_width = 0.35;

  let p = opts.index as f64 / opts.frames as f64;

  let circle = VCircle::new(width / 2.0, height / 2.0, height / 2.0 - pad);
  let (circles, routes) = waveball(0, opts.seed, &circle, p);

  let mut layers = Vec::new();
  let color = "black";
  let mut l = layer(color);
  for c in circles.clone() {
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
  for r in routes.clone() {
    data = render_route(data, r);
  }
  l = l.add(base_path(color, stroke_width, data));
  l = l.add(signature(0.8, (185.0, 224.0), color));
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
