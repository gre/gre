use clap::*;
use gre::*;
use isosurface::{marching_cubes::MarchingCubes, source::Source};
use kiss3d::nalgebra::{Perspective3, Point3, Rotation3, Vector3};
use rand::prelude::*;
use std::f32::consts::PI;
use std::ops::{Add, Mul, Sub};
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
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

// We use sign distance function paradigm here:

fn sd_capsule(
  p: Vector3<f32>,
  a: Vector3<f32>,
  b: Vector3<f32>,
  r: f32,
) -> f32 {
  let pa = p - a;
  let ba = b - a;
  let h = (pa.dot(&ba) / ba.dot(&ba)).max(0.0).min(1.0);
  (pa - ba * h).norm() - r
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

struct Shape {
  seed: f64,
}
impl Source for Shape {
  fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
    let p = Vector3::new(x, y, z);
    let mut s = 999.;
    let mut rng = rng_from_seed(self.seed);
    let count =
      5 + (rng.gen_range(0., 200.) * rng.gen_range(0.0, 1.0)) as usize;
    let max_size = rng.gen_range(0.05, 0.1);
    for _i in 0..count {
      let a = Vector3::new(
        rng.gen_range(0.1, 0.9),
        rng.gen_range(0.1, 0.9),
        rng.gen_range(0.1, 0.9),
      );
      let b = if rng.gen_bool(0.2) {
        a
      } else {
        Vector3::new(
          rng.gen_range(0.1, 0.9),
          rng.gen_range(0.1, 0.9),
          rng.gen_range(0.1, 0.9),
        )
      };
      s = s.smooth_union(
        rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0),
        sd_capsule(p, a, b, max_size * rng.gen_range(0.6, 1.0)),
      );
    }
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

  let mut rng = rng_from_seed(opts.seed);
  let grid_size = rng.gen_range(50, 70);
  let mut vertices = vec![];
  let mut indices = vec![];
  let source = Shape { seed: opts.seed };
  let mut marching = MarchingCubes::new(grid_size);
  marching.extract(&source, &mut vertices, &mut indices);
  let triangles = make_triangles_from_vertices_indices(&vertices, &indices);

  // project triangles to 2D with a camera
  let dist = 2.2;
  let cam = Camera::new((width / height) as f32, 1.2, 1.0, 6.0);
  let rot =
    Rotation3::from_axis_angle(&Vector3::z_axis(), rng.gen_range(-PI, PI))
      * Rotation3::from_axis_angle(&Vector3::y_axis(), rng.gen_range(-PI, PI))
      * Rotation3::from_axis_angle(&Vector3::x_axis(), rng.gen_range(-PI, PI));
  let mut projected = triangles
    .iter()
    .map(|tri| {
      let t = tri.clone() + Vector3::new(-0.5, -0.5, -0.5);
      let t = rot * t;
      let t = t + Vector3::new(0., 0., -dist);
      cam.project(&t)
    })
    .collect::<Vec<_>>();

  // sort by z-index
  let mut data = projected
    .iter()
    .map(|tri| {
      let z = tri.v1.z + tri.v2.z + tri.v3.z;
      (tri.clone(), z)
    })
    .collect::<Vec<(Tri, f32)>>();
  data.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
  projected = data.iter().map(|(tri, _)| tri.clone()).collect::<Vec<_>>();

  let mut passage = Passage::new(0.5, width, height);

  let mut routes: Vec<Vec<(f64, f64)>> = vec![];
  let mut shadow_routes: Vec<Vec<(f64, f64)>> = vec![];

  let mut polygons = vec![];

  let translate = 0.5;
  for tri in projected {
    let points: Vec<(f64, f64)> = vec![tri.v1, tri.v2, tri.v3]
      .iter()
      .map(|p| {
        (
          (p.x as f64 + translate) * width,
          (p.y as f64 + translate) * height,
        )
      })
      .collect();

    // quick hack. triangles are small enough to ignore cases where it partially overlaps
    let center = centroid(&points);
    let zavg = (tri.v1.z + tri.v2.z + tri.v3.z) / 3.0;
    let hidden = is_inside_polygons(center, &polygons);
    if hidden {
      continue;
    }
    let shadow_factor = smoothstep(0.0, 0.4, zavg);
    if shadow_factor > 0.99 || shadow_factor > 0.0 {
      if shadow_factor > 0.33 && rng.gen_bool(shadow_factor as f64 - 0.2) {
        shadow_routes.push(vec![points[0], center]);
      }
      if shadow_factor > 0.66 && rng.gen_bool(shadow_factor as f64 - 0.4) {
        shadow_routes.push(vec![points[1], center]);
      }
      if rng.gen_bool(shadow_factor as f64) {
        shadow_routes.push(vec![points[2], center]);
      }
    }
    for pts in points.windows(2) {
      let a = pts[0];
      let b = pts[1];
      if passage.get(a) + passage.get(b) > 10 {
        continue;
      }
      let path = vec![a, b];
      // check that this route was not already added
      let mut already_added = false;
      for route in routes.clone() {
        if same_point(a, route[0]) && same_point(b, route[1])
          || same_point(a, route[1]) && same_point(b, route[0])
        {
          already_added = true;
          break;
        }
      }
      if !already_added {
        passage.count(a);
        passage.count(b);
        routes.push(path);
      }
    }
    polygons.push(points);
  }

  vec![("darkblue", shadow_routes), ("black", routes)]
    .iter()
    .enumerate()
    .map(|(i, (color, routes))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
  let k = ((x - a) / (b - a)).max(0.0).min(1.0);
  return k * k * (3.0 - 2.0 * k);
}

fn is_inside_a_polygon(p: (f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
  let mut inside = false;
  let mut j = polygon.len() - 1;
  for i in 0..polygon.len() {
    let pi = polygon[i];
    let pj = polygon[j];
    if (pi.1 > p.1) != (pj.1 > p.1)
      && p.0 < (pj.0 - pi.0) * (p.1 - pi.1) / (pj.1 - pi.1) + pi.0
    {
      inside = !inside;
    }
    j = i;
  }
  inside
}

fn same_point(a: (f64, f64), b: (f64, f64)) -> bool {
  (a.0 - b.0).abs() < 0.0001 && (a.1 - b.1).abs() < 0.0001
}

fn is_inside_polygons(p: (f64, f64), polygons: &Vec<Vec<(f64, f64)>>) -> bool {
  for polygon in polygons {
    if is_inside_a_polygon(p, polygon) {
      return true;
    }
  }
  false
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

fn centroid(points: &Vec<(f64, f64)>) -> (f64, f64) {
  let mut x = 0.0;
  let mut y = 0.0;
  for (x_, y_) in points {
    x += x_;
    y += y_;
  }
  (x / points.len() as f64, y / points.len() as f64)
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

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }
}
