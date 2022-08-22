use clap::*;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
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
  #[clap(short, long, default_value = "48.0")]
  pub seed: f64,
}

// I did this weekly challenge as a goal to master better my Rust code
// and especially wanted to learn how to separate the data from its rendering
// In rust, we use struct to describe complex data

// I defined shapes of Circle, Stroke, Ray which are my primitives for this creation:

pub struct CircleObject {
  x: f64,
  y: f64,
  r: f64,
  dr: f64,
  border: f64,
  fill: bool,
}

pub struct StrokeObject {
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
  strokewidth: f64,
}
pub struct RayObject {
  x: f64,
  y: f64,
  astart: f64,
  aend: f64,
  rays: usize,
}

// Then, we define a trait "Plottable" that means
// we can convert the object into strokes to execute on a plotter!
trait Plottable {
  // implements this to define all strokes of the "fill" part (inside of the shape)
  fn fill(&self, bounds: (f64, f64, f64, f64)) -> Vec<Vec<(f64, f64)>>;
  // implements this to define all strokes for the contour part (that will be black)
  fn stroke(&self, bounds: (f64, f64, f64, f64)) -> Vec<Vec<(f64, f64)>>;
}

// Now, we can implement in a decoupled way, the functions for each Object

impl Plottable for CircleObject {
  fn stroke(&self, _bounds: (f64, f64, f64, f64)) -> Vec<Vec<(f64, f64)>> {
    let delta = 0.3;
    let steps = (self.border / delta) as usize;
    if steps <= 0 {
      return Vec::new();
    }
    (0..steps)
      .map(|i| {
        let r = self.r - i as f64 * delta;
        circle_route((self.x, self.y), r, (r * 10.0) as usize)
      })
      .collect()
  }

  fn fill(&self, _bounds: (f64, f64, f64, f64)) -> Vec<Vec<(f64, f64)>> {
    if !self.fill {
      return Vec::new();
    }
    let mut routes = Vec::new();
    routes.push(spiral(self.x, self.y, self.r, self.dr));
    routes
  }
}

impl Plottable for StrokeObject {
  fn fill(&self, _bounds: (f64, f64, f64, f64)) -> Vec<Vec<(f64, f64)>> {
    Vec::new()
  }

  fn stroke(&self, _bounds: (f64, f64, f64, f64)) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();
    let a = PI / 2.0 + (self.y2 - self.y1).atan2(self.x2 - self.x1);
    let delta = 0.3;
    let count = (self.strokewidth / delta).max(1.0) as usize;
    for i in 0..count {
      let d = delta * (i as f64 - (count as f64 - 1.0) / 2.0);
      let dx = d * a.cos();
      let dy = d * a.sin();
      routes.push(vec![
        (self.x1 + dx, self.y1 + dy),
        (self.x2 + dx, self.y2 + dy),
      ]);
    }
    routes
  }
}

impl Plottable for RayObject {
  fn stroke(&self, _bounds: (f64, f64, f64, f64)) -> Vec<Vec<(f64, f64)>> {
    Vec::new()
  }

  fn fill(&self, bounds: (f64, f64, f64, f64)) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();

    let aincr = (self.aend - self.astart) / (self.rays as f64);
    let mut a = self.astart;
    for _i in 0..self.rays {
      let x2 = self.x + 999. * a.cos();
      let y2 = self.x + 999. * a.sin();
      let p1 = collides_segment(
        (self.x, self.y),
        (x2, y2),
        (bounds.0, bounds.1),
        (bounds.2, bounds.1),
      );
      let p2 = collides_segment(
        (self.x, self.y),
        (x2, y2),
        (bounds.0, bounds.3),
        (bounds.2, bounds.3),
      );
      if let Some(p1) = p1 {
        if let Some(p2) = p2 {
          routes.push(vec![p1, p2]);
        }
      }
      a += aincr;
    }
    let first = routes[0].clone();
    let last = routes[routes.len() - 1].clone();
    vec![vec![first], routes, vec![last]].concat()
  }
}

// And finally =>

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;
  let bound = (pad, pad, width - pad, height - pad);
  let fill_colors = vec!["yellow", "red", "turquoise", "blue"];

  // all objects we put there will need to implement "Plottable"
  let mut objects: Vec<(usize, Box<dyn Plottable>)> = Vec::new();

  let mut rng = rng_from_seed(opts.seed);

  // ADD our main circle
  let mainobjradius = 80.0;
  objects.push((
    0,
    Box::new(CircleObject {
      border: 8.0,
      r: mainobjradius,
      dr: rng.gen_range(0.3, 0.4),
      x: width / 2.0,
      y: height / 2.0,
      fill: false,
    }),
  ));

  // ADD 2 rays
  for i in 0..2 {
    let w2 = 0.6 * width;
    let x = rng.gen_range((i as f64 - 0.5) * w2, (i as f64 + 0.5) * w2);
    let y = rng.gen_range(-100.0, -50.0);
    let a = (height / 2.0 - y
      + mainobjradius * rng.gen_range(-0.3, 0.3) * rng.gen_range(0.0, 1.0))
    .atan2(width / 2.0 - x);
    let opening = rng.gen_range(0.2, 0.25);
    let ray = RayObject {
      x,
      y,
      astart: a - 0.5 * opening,
      aend: a + 0.5 * opening,
      rays: 70,
    };
    let clr = i * 2;
    objects.push((clr, Box::new(ray)));
  }

  // ADD many circles
  let max_circles = rng.gen_range(16, 28);
  for i in 0..max_circles {
    let f = i as f64 / (max_circles as f64);
    let mut x = width / 2.0;
    let mut y = height / 2.0;
    let rfactor = 0.99 * (1.0 - f);
    let r = 2.0
      + rng.gen_range(0.0, 42.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(rfactor, 1.0);
    let border =
      rng.gen_range(0.0, (20f64 / r).min(0.5 * r)) * rng.gen_range(0.0, 1.0);
    let projr = f.powf(0.66) * (mainobjradius - r - 10.0);
    let proja = i as f64;
    x += projr * proja.cos();
    y += projr * proja.sin();
    let clr = rng.gen_range(0, fill_colors.len());
    objects.push((
      clr,
      Box::new(CircleObject {
        x,
        y,
        r,
        dr: rng.gen_range(0.5, 0.8),
        border,
        fill: true,
      }),
    ));
  }

  // ADD many strokes
  let max_strokes = rng.gen_range(6, 12);
  for _i in 0..max_strokes {
    let strokewidth = rng.gen_range(0.0, 1.2);
    let r = rng.gen_range(40.0, 70.0);
    let a1 = rng.gen_range(0.0, 2.0 * PI);
    let a2 = a1 + PI + rng.gen_range(-1.4, 1.4);
    let x1 = width / 2.0 + r * a1.cos();
    let y1 = height / 2.0 + r * a1.sin();
    let x2 = width / 2.0 + r * a2.cos();
    let y2 = height / 2.0 + r * a2.sin();
    objects.push((
      0,
      Box::new(StrokeObject {
        x1,
        y1,
        x2,
        y2,
        strokewidth,
      }),
    ));
    if rng.gen_bool(0.5) {
      continue;
    }
    // we also will do 2 ticks along the line
    let angle = PI / 2.0
      + (y2 - y1).atan2(x2 - x1)
      + rng.gen_range(-0.8, 0.8) * rng.gen_range(0.0, 1.0);
    let c = rng.gen_range(0.1, 0.9);
    let centers = vec![c, c + rng.gen_range(-0.1, 0.1)];
    for c in centers {
      let x = mix(x1, x2, c);
      let y = mix(y1, y2, c);
      let a = angle + rng.gen_range(-0.06, 0.06);
      let l1 = rng.gen_range(3.0, 5.0);
      let l2 = l1 + rng.gen_range(-0.1, 0.1);
      let x1 = x - l1 * a.cos();
      let y1 = y - l1 * a.sin();
      let x2 = x + l2 * a.cos();
      let y2 = y + l2 * a.sin();
      objects.push((
        0,
        Box::new(StrokeObject {
          x1,
          y1,
          x2,
          y2,
          strokewidth,
        }),
      ));
    }
  }

  // Finally, we can render it all!
  let contour_color = fill_colors.len();
  let colors = vec![fill_colors, vec!["black"]].concat();

  // for each color =>
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();

      // this is black, we do the contours
      if contour_color == ci {
        for (_c, o) in objects.iter() {
          for route in o.stroke(bound) {
            data = render_route(data, route);
          }
        }
      }

      // for all objects that are defined on the color, we will FILL the shapes
      for (c, o) in objects.iter() {
        if *c == ci {
          for route in o.fill(bound) {
            data = render_route(data, route);
          }
        }
      }

      let mut l = layer(color); // SVG layer to be easy to plot
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
