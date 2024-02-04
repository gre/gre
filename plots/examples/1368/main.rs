use clap::*;
use gre::*;
use isosurface::{marching_cubes::MarchingCubes, source::Source};
use kiss3d::nalgebra::{Perspective3, Point3, Rotation3, Vector2, Vector3};
use rand::prelude::*;
use std::f32::consts::PI;
use std::ops::{Add, Mul, Sub};
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  height: f64,
  #[clap(short, long, default_value = "210.0")]
  width: f64,
  #[clap(short, long, default_value = "10.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

trait BinaryOps<T> {
  fn intersect(&self, other: T) -> T;
  fn difference(&self, other: T) -> T;
  fn union(&self, other: T) -> T;
  fn smooth_intersect(&self, k: T, other: T) -> T;
  fn smooth_difference(&self, k: T, other: T) -> T;
  fn smooth_union(&self, k: T, other: T) -> T;
}

impl BinaryOps<f32> for f32 {
  fn intersect(&self, other: f32) -> f32 {
    self.max(other)
  }
  fn difference(&self, other: f32) -> f32 {
    self.max(-other)
  }
  fn union(&self, other: f32) -> f32 {
    self.min(other)
  }

  fn smooth_intersect(&self, k: f32, other: f32) -> f32 {
    let h = (0.5 - 0.5 * (self - other) / k).max(0.0).min(1.0);
    mix(*self, other, h) + k * h * (1.0 - h)
  }

  fn smooth_difference(&self, k: f32, other: f32) -> f32 {
    let h = (0.5 - 0.5 * (other + self) / k).max(0.0).min(1.0);
    mix(*self, -other, h) + k * h * (1.0 - h)
  }

  fn smooth_union(&self, k: f32, other: f32) -> f32 {
    let h = (0.5 + 0.5 * (self - other) / k).max(0.0).min(1.0);
    mix(*self, other, h) - k * h * (1.0 - h)
  }
}

fn mix(a: f32, b: f32, x: f32) -> f32 {
  (1. - x) * a + x * b
}

fn p_r(p: (f32, f32), a: f32) -> (f32, f32) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

// ported from GLSL https://www.shadertoy.com/view/4ltSW8
fn klein_bottle(p: Vector3<f32>, t: f32) -> f32 {
  let mut d = 1e10f32;
  let mut p = p;
  p.y -= 0.8;

  let q = p + Vector3::new(1.0 - ((1.0 - p.y) / 3.0 * PI).cos(), 0.0, 0.0);
  let y = (0.5 * (1.0 - p.y) / 3.0 * PI).sin().powi(2);

  let tube_hollow = ((q.xz().norm() - 0.5 + 0.25 * y).abs() - t)
    .max(q.y - 1.0)
    .max(-q.y - 2.0);
  let tube_solid = (q.xz().norm() - 0.5 + 0.25 * y)
    .max(q.y - 1.0)
    .max(-q.y - 2.0);

  let mut q = p - Vector3::new(0.0, 1.0, 0.0);
  d = d.min((Vector2::new(q.xz().norm() - 1.0, q.y).norm() - 0.5).abs() - t);

  q = p;
  d = d.min(
    (((q.xz().norm() - 1.5 + 1.25 * y).max(q.y - 1.0)).max(-q.y - 2.0) - t)
      .max(-tube_solid),
  );

  d = d.min(tube_hollow);

  q = p + Vector3::new(1.0, 2.0, 0.0);
  d = d.min(
    ((Vector2::new(q.xy().norm() - 1.0, q.z).norm() - 0.25).abs() - t).max(q.y),
  );

  d
}

struct Shape {
  rotation: f32,
  scale: f32,
  t: f32,
}
impl Source for Shape {
  fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
    let p = Vector3::new(x, y, z);
    let s;
    let mut p = p - Vector3::new(0.5, 0.5, 0.5);
    let pr = p_r((p.x, p.z), self.rotation);
    p.x = pr.0;
    p.z = pr.1;
    let p = p / self.scale;
    let p = p + Vector3::new(0.5, 0.5, 0.5);
    s = klein_bottle(p, self.t);
    s
  }
}

fn make_triangles_from_vertices_indices(
  vert: &Vec<f32>,
  idx: &Vec<u32>,
) -> Vec<Tri> {
  let mut triangles = vec![];
  for face in idx.chunks(3) {
    let i1 = face[0] as usize;
    let i2 = face[1] as usize;
    let i3 = face[2] as usize;
    let v1 = Point3::new(vert[i1 * 3], vert[i1 * 3 + 1], vert[i1 * 3 + 2]);
    let v2 = Point3::new(vert[i2 * 3], vert[i2 * 3 + 1], vert[i2 * 3 + 2]);
    let v3 = Point3::new(vert[i3 * 3], vert[i3 * 3 + 1], vert[i3 * 3 + 2]);
    triangles.push(Tri::new(v3, v2, v1));
  }
  triangles
}

#[derive(Debug, Clone)]
struct Tri {
  v1: Point3<f32>,
  v2: Point3<f32>,
  v3: Point3<f32>,
}

impl Sub<Vector3<f32>> for Tri {
  type Output = Tri;

  fn sub(self, v: Vector3<f32>) -> Self::Output {
    Tri {
      v1: self.v1 - v,
      v2: self.v2 - v,
      v3: self.v3 - v,
    }
  }
}

impl Add<Vector3<f32>> for Tri {
  type Output = Tri;

  fn add(self, v: Vector3<f32>) -> Self::Output {
    Tri {
      v1: self.v1 + v,
      v2: self.v2 + v,
      v3: self.v3 + v,
    }
  }
}

impl Mul<Tri> for f32 {
  type Output = Tri;

  fn mul(self, tri: Tri) -> Self::Output {
    Tri {
      v1: self * tri.v1,
      v2: self * tri.v2,
      v3: self * tri.v3,
    }
  }
}

impl Mul<Tri> for Rotation3<f32> {
  type Output = Tri;

  fn mul(self, tri: Tri) -> Self::Output {
    Tri {
      v1: self * tri.v1,
      v2: self * tri.v2,
      v3: self * tri.v3,
    }
  }
}

impl Tri {
  fn new(v1: Point3<f32>, v2: Point3<f32>, v3: Point3<f32>) -> Self {
    Tri { v1, v2, v3 }
  }
}

struct Camera {
  aspect: f32,
  fovy: f32,
  znear: f32,
  zfar: f32,
}

impl Camera {
  fn new(aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Self {
    Camera {
      aspect,
      fovy,
      znear,
      zfar,
    }
  }
  fn project(&self, tri: &Tri) -> Tri {
    let proj = Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);
    Tri {
      v1: proj.project_point(&tri.v1),
      v2: proj.project_point(&tri.v2),
      v3: proj.project_point(&tri.v3),
    }
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let precision = 0.3;

  let mut rng = rng_from_seed(opts.seed);
  let mut paint = PaintMask::new(precision, width, height);
  paint.paint_borders(pad);

  // project triangles to 2D with a camera
  let dist: f32 = 1.0;
  let cam = Camera::new((width / height) as f32, 2.0, 1.0, 6.0);

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = vec![];

  let grid_size = rng.gen_range(50, 80);

  let scale = 0.16;
  let rotation = rng.gen_range(-PI, PI);
  let t = rng.gen_range(0.05, 0.12);

  let source = Shape { rotation, scale, t };

  let mut vertices = vec![];
  let mut indices = vec![];
  let mut marching = MarchingCubes::new(grid_size);

  marching.extract(&source, &mut vertices, &mut indices);

  // project triangles to 2D with a camera
  let triangles = make_triangles_from_vertices_indices(&vertices, &indices)
    .iter()
    .flat_map(|tri| {
      let t = tri.clone() + Vector3::new(-0.5, -0.5, -0.5);
      let t = t + Vector3::new(0., 0., -dist);
      let tri = cam.project(&t);
      let z = tri.v1.z + tri.v2.z + tri.v3.z;
      if !z.is_finite() {
        return None;
      }
      Some((tri.clone(), z, 0))
    })
    .collect::<Vec<(Tri, f32, usize)>>();

  let mut minx = width;
  let mut miny = height;
  let mut maxx = 0.;
  let mut maxy = 0.;

  for (tri, _z, clr) in triangles {
    let points: Vec<(f64, f64)> = vec![tri.v1, tri.v2, tri.v3, tri.v1]
      .iter()
      .map(|p| {
        let (x, y) = ((p.x as f64 + 0.5) * width, (p.y as f64 + 0.5) * height);
        if x < minx {
          minx = x;
        }
        if y < miny {
          miny = y;
        }
        if x > maxx {
          maxx = x;
        }
        if y > maxy {
          maxy = y;
        }
        (x, y)
      })
      .collect();

    let rts = vec![(clr, points.clone())];

    routes.extend(rts);
  }

  // translate to center
  let dx = (width - (maxx - minx)) / 2. - minx;
  let dy = (height - (maxy - miny)) / 2. - miny;
  for (_, route) in routes.iter_mut() {
    for p in route.iter_mut() {
      *p = (p.0 + dx, p.1 + dy);
    }
    paint.paint_polygon(&route);
  }

  let mut rts = vec![];
  let mut y = pad;
  let dy = rng.gen_range(6.0, 8.0);
  while y < height - pad {
    rts.push((1, vec![(pad, y), (width - pad, y)]));
    y += dy;
  }
  let rts = clip_routes_with_colors(&rts, &|p| paint.is_painted(p), 1.0, 4);
  routes.extend(rts);

  vec!["white", "white"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes_with_colors(
  input_routes: &Vec<(usize, Vec<(f64, f64)>)>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  // locate the intersection where inside and outside cross
  let search = |inside: (f64, f64), outside: (f64, f64), n| {
    let mut a = inside;
    let mut b = outside;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if is_outside(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for (clrp, input_route) in input_routes.iter() {
    let clr = *clrp;
    if input_route.len() < 2 {
      continue;
    }

    let mut prev = input_route[0];
    let mut prev_is_outside = is_outside(prev);
    let mut route = vec![];
    if !prev_is_outside {
      // prev is not to crop. we can start with it
      route.push(prev);
    }

    for &p in input_route.iter().skip(1) {
      // we iterate in small steps to detect any interruption
      let static_prev = prev;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let d = (dx * dx + dy * dy).sqrt();
      let vx = dx / d;
      let vy = dy / d;
      let iterations = (d / stepping).ceil() as usize;
      let mut v = 0.0;
      for _i in 0..iterations {
        v = (v + stepping).min(d);
        let q = (static_prev.0 + vx * v, static_prev.1 + vy * v);
        let q_is_outside = is_outside(q);
        if prev_is_outside != q_is_outside {
          // we have a crossing. we search it precisely
          let intersection = if prev_is_outside {
            search(q, prev, dichotomic_iterations)
          } else {
            search(prev, q, dichotomic_iterations)
          };

          if q_is_outside {
            // we close the path
            route.push(intersection);
            if route.len() > 1 {
              // we have a valid route to accumulate
              routes.push((clr, route));
            }
            route = vec![];
          } else {
            // we open the path
            route.push(intersection);
          }
          prev_is_outside = q_is_outside;
        }

        prev = q;
      }

      // prev should be == p
      if !prev_is_outside {
        // prev is not to crop. we can start with it
        route.push(p);
      }
    }

    if route.len() > 1 {
      // we have a valid route to accumulate
      routes.push((clr, route));
    }
  }

  routes
}

#[derive(Clone)]
struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
  wi: usize,
  hi: usize,
}

impl PaintMask {
  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
      wi,
      hi,
    }
  }

  fn is_painted(&self, (x, y): (f64, f64)) -> bool {
    let precision = self.precision;
    let wi = self.wi;
    let hi = self.hi;
    let xi = ((x / precision) as usize).min(wi - 1);
    let yi = ((y / precision) as usize).min(hi - 1);
    self.mask[xi + yi * wi]
  }

  fn paint_rectangle(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
    self.paint_rectangle_v(minx, miny, maxx, maxy, true);
  }

  fn paint_rectangle_v(
    &mut self,
    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
    v: bool,
  ) {
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        self.mask[x + y * wi] = v;
      }
    }
  }

  fn paint_borders(&mut self, pad: f64) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
  }

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let point = (x as f64 * precision, y as f64 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[j] = true;
        }
      }
    }
  }
}

fn polygon_bounds(polygon: &Vec<(f64, f64)>) -> (f64, f64, f64, f64) {
  let mut minx = f64::MAX;
  let mut miny = f64::MAX;
  let mut maxx = f64::MIN;
  let mut maxy = f64::MIN;
  for &(x, y) in polygon {
    minx = minx.min(x);
    miny = miny.min(y);
    maxx = maxx.max(x);
    maxy = maxy.max(y);
  }
  (minx, miny, maxx, maxy)
}

fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  point: (f64, f64),
) -> bool {
  let mut c = false;
  for i in 0..polygon.len() {
    let j = (i + 1) % polygon.len();
    if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
      && (point.0
        < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
          / (polygon[j].1 - polygon[i].1)
          + polygon[i].0)
    {
      c = !c;
    }
  }
  c
}
