use clap::*;
use geo::polygon;
use geo::prelude::Area;
use geo::prelude::BoundingRect;
use geo::prelude::Centroid;
use geo::translate::Translate;
use geo::Contains;
use geo::Polygon;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

// a simple implementation of cutting a convex polygon in 2 with a line
fn cut_polygon(
  poly: &Polygon<f64>,
  a: (f64, f64),
  b: (f64, f64),
) -> Vec<Polygon<f64>> {
  let mut prev: Option<(f64, f64)> = None;
  let mut first = Vec::new();
  let mut second = Vec::new();
  let mut on_first = true;
  for p in poly.exterior() {
    let to = p.x_y();
    if let Some(from) = prev {
      let collision = collides_segment(from, to, a, b);
      if let Some(c) = collision {
        first.push(c);
        second.push(c);
        on_first = !on_first;
      }
    }
    if on_first {
      first.push(to);
    } else {
      second.push(to);
    }
    prev = Some(to);
  }
  if second.len() < 2 {
    return vec![poly.clone()];
  }
  return vec![
    Polygon::new(first.into(), vec![]),
    Polygon::new(second.into(), vec![]),
  ];
}

fn rec<R: Rng>(
  rng: &mut R,
  polygon: &Polygon<f64>,
  depth: usize,
) -> Vec<Polygon<f64>> {
  let mut center = polygon.centroid().unwrap();
  let bounds = polygon.bounding_rect().unwrap();
  let w = bounds.width();
  let h = bounds.height();
  let max = 1.001 - 1. / ((depth as f64) + 1.);
  /*
  center = center.translate(
    rng.gen_range(0.0, max) * rng.gen_range(-0.5, 0.5) * w,
    rng.gen_range(0.0, max) * rng.gen_range(-0.5, 0.5) * h,
  );
  */
  let ang = rng.gen_range(0.0, 2. * PI);
  let dx = ang.cos();
  let dy = ang.sin();
  let amp = 1000.0;
  let a = center.translate(amp * dx, amp * dy).x_y();
  let b = center.translate(-amp * dx, -amp * dy).x_y();
  let mut cut = cut_polygon(polygon, a, b);
  if cut.len() == 1 {
    return vec![polygon.clone()];
  }

  // move the pieces
  cut = cut
    .iter()
    .map(|p| {
      let newcenter = p.centroid().unwrap();
      let dx = newcenter.x() - center.x();
      let dy = newcenter.y() - center.y();
      let dist = (dx * dx + dy * dy).sqrt();
      let amp = depth as f64 + 1.0;
      let poly = p.translate(amp * dx / dist, amp * dy / dist);
      poly
    })
    .collect();

  if depth <= 0 || rng.gen_range(0.0, 1.0) < 1.3 / (depth as f64 + 1.0) {
    return cut;
  }
  let mut all = Vec::new();
  for poly in cut {
    let inside = rec(rng, &poly, depth - 1);
    for p in inside {
      if p.signed_area() > 0.5 {
        all.push(p);
      }
    }
  }
  return all;
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut rng = rng_from_seed(opts.seed);

  let size1 = width * rng.gen_range(0.7, 0.8);
  let size2 = height * rng.gen_range(0.7, 0.8);

  let mut polygons = vec![];

  let depth = rng.gen_range(5, 10);

  let w = width / 4.0;
  let h = 0.4 * w;
  let spacew = 0.6 * h;
  let spaceh = 0.4 * h;
  let diff = -w * 0.5 - spacew * 0.5;

  let mut y = 0.0;
  let mut yi = 0;

  let mut centers = vec![];

  while y < height {
    let mut xi = 0;
    let mut x = if yi % 2 == 0 {
      diff
    } else {
      -0.5 * w - 0.5 * spacew + diff
    };

    while x < width {
      let x1 = x.max(pad);
      let x2 = (x + w).min(width - pad);
      let y1 = y.max(pad);
      let y2 = (y + h).min(height - pad);
      if x2 - x1 < 0.0 || y2 - y1 < 0.0 {
        x += w + spacew;
        continue;
      }
      let poly = polygon![
        (x1, y1).into(),
        (x2, y1).into(),
        (x2, y2).into(),
        (x1, y2).into(),
      ];

      if xi == 1 + (if yi % 2 == 0 { 1 } else { 0 }) {
        let polys = rec(&mut rng, &poly, depth);
        let center = poly.centroid().unwrap();
        if polys.len() > 1
          && center.y() > 0.1 * height
          && center.y() < 0.9 * height
        {
          centers.push(center);
        }
        polygons.extend(polys);
      } else {
        polygons.push(poly);
      }

      x += w + spacew;
      xi += 1;
    }
    y += h + spaceh;
    yi += 1;
  }

  /*
  rec(&mut rng, &poly1, depth);
  */

  let rot = PI / rng.gen_range(1.0, 3.0);
  let step = 0.6;
  let straight = 0.1;
  let count = rng.gen_range(10000, 20000);
  let min_l = 5;
  let max_l = 40;
  let decrease_value = 1.0;
  let search_max = 500;
  let min_weight = 1.0;
  let mut bail_out = 0;

  let precision = 0.5;

  let mut map = WeightMap::new(width, height, precision);

  let density = 4.0;

  map.fill_fn(&mut rng, &mut |p: (f64, f64), _rng| {
    for poly in &polygons {
      if poly.contains(&geo::Point::new(p.0, p.1)) {
        return density;
      }
    }
    0.0
  });

  let mut routes = vec![];

  for _i in 0..count {
    let top = map.search_weight_top(&mut rng, search_max, min_weight);
    if top.is_none() {
      bail_out += 1;
      if bail_out > 10 {
        break;
      }
    }
    if let Some(o) = top {
      let angle = rng.gen_range(-PI, PI);

      if let Some(a) = map.best_direction(o, step, angle, PI, PI / 4.0, 0.0) {
        let route = map.dig_random_route(
          o,
          a,
          step,
          rot,
          straight,
          max_l,
          decrease_value,
        );
        if route.len() >= min_l {
          let rt = rdp(&route, 0.05);
          routes.push(rt);
        }
      }
    }
  }

  /*
  let mut gold_routes = vec![];

  let mut path = vec![];
  for c in centers.clone() {
    path.push(c.x_y());
  }
  gold_routes.push(path);
  */

  vec![("white", routes)]
    .iter()
    .enumerate()
    .map(|(ci, (color, rts))| {
      let mut data = Data::new();
      if ci == 0 {
        for poly in polygons.iter() {
          data = render_polygon_stroke(data, poly.clone());
        }
      }
      for route in rts.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.55, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "322.0")]
  seed: f64,
  #[clap(short, long, default_value = "297")]
  width: f64,
  #[clap(short, long, default_value = "420")]
  height: f64,
  #[clap(short, long, default_value = "20")]
  pad: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

struct WeightMap {
  weights: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl WeightMap {
  fn new(width: f64, height: f64, precision: f64) -> WeightMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let weights = vec![0.0; w * h];
    WeightMap {
      weights,
      w,
      h,
      width,
      height,
      precision,
    }
  }
  fn fill_fn<R: Rng>(
    &mut self,
    rng: &mut R,
    f: &mut impl Fn((f64, f64), &mut R) -> f64,
  ) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p, rng);
        self.weights[y * self.w + x] = v;
      }
    }
  }

  // do a simple bilinear interpolation
  fn get_weight(&self, p: (f64, f64)) -> f64 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let w00 = self.weights[y0 * self.w + x0];
    let w01 = self.weights[y0 * self.w + x1];
    let w10 = self.weights[y1 * self.w + x0];
    let w11 = self.weights[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the weights around the point p with a given radius
  fn decrease_weight_gaussian(
    &mut self,
    p: (f64, f64),
    radius: f64,
    value: f64,
  ) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = (x - radius).floor() as usize;
    let y0 = (y - radius).floor() as usize;
    let x1 = (x + radius).ceil() as usize;
    let y1 = (y + radius).ceil() as usize;
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let d = (p.0 - p.0).hypot(p.1 - p.1);
        if d < radius {
          let w = self.weights[y * self.w + x];
          let v = value * (1.0 - d / radius);
          self.weights[y * self.w + x] = w - v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  fn best_direction(
    &self,
    p: (f64, f64),
    step: f64,
    angle: f64,
    max_ang_rotation: f64,
    angle_precision: f64,
    straight_factor: f64,
  ) -> Option<f64> {
    let mut best_ang = None;
    let mut best_weight = 0.0;
    let mut a = -max_ang_rotation;
    while a < max_ang_rotation {
      let ang = a + angle;
      let dx = step * ang.cos();
      let dy = step * ang.sin();
      let np = (p.0 + dx, p.1 + dy);
      if np.0 < 0.0 || np.0 > self.width || np.1 < 0.0 || np.1 > self.height {
        a += angle_precision;
        continue;
      }
      // more important when a is near 0.0 depending on straight factor
      let wmul = (1.0 - straight_factor)
        + (1.0 - a.abs() / max_ang_rotation) * straight_factor;
      let weight = self.get_weight(np) * wmul;
      if weight > best_weight {
        best_weight = weight;
        best_ang = Some(ang);
      }
      a += angle_precision;
    }
    return best_ang;
  }

  fn search_weight_top<R: Rng>(
    &mut self,
    rng: &mut R,
    search_max: usize,
    min_weight: f64,
  ) -> Option<(f64, f64)> {
    let mut best_w = min_weight;
    let mut best_p = None;
    for _i in 0..search_max {
      let x = rng.gen_range(0.0, self.width);
      let y = rng.gen_range(0.0, self.height);
      let p = (x, y);
      let w = self.get_weight(p);
      if w > best_w {
        best_w = w;
        best_p = Some(p);
      }
    }
    return best_p;
  }

  fn dig_random_route(
    &mut self,
    origin: (f64, f64),
    initial_angle: f64,
    step: f64,
    max_ang_rotation: f64,
    straight_factor: f64,
    max_length: usize,
    decrease_value: f64,
  ) -> Vec<(f64, f64)> {
    let mut route = Vec::new();
    let mut p = origin;
    let mut angle = initial_angle;
    for _i in 0..max_length {
      if let Some(ang) = self.best_direction(
        p,
        step,
        angle,
        max_ang_rotation,
        0.2 * max_ang_rotation,
        straight_factor,
      ) {
        angle = ang;
        let prev = p;
        p = (p.0 + step * angle.cos(), p.1 + step * angle.sin());
        route.push(p);
        self.decrease_weight_gaussian(prev, step, decrease_value);
      } else {
        break;
      }
    }

    route
  }
}
