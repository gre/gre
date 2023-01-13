use chull::ConvexHullWrapper;
use clap::*;
use gre::rng_from_seed;
use kiss3d::nalgebra::{Point3, Rotation3, Vector3};
use rand::Rng;
use std::{
  convert::TryInto,
  f32::consts::PI,
  fs::File,
  io::BufWriter,
  ops::{Add, Mul},
};
use stl::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "result.stl")]
  file: String,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Tri> {
  let mut rng = rng_from_seed(opts.seed);
  let mut triangles = vec![];

  let mut spots: Vec<Vector3<f32>> = vec![Vector3::zeros()];

  let count = rng.gen_range(2, 4);
  let mut prev = Rotation3::identity();
  let rotations = (0..count)
    .map(|i| {
      if i == 0 {
        return Rotation3::identity();
      }
      let axis = if rng.gen_bool(0.5) {
        Vector3::y_axis()
      } else {
        Vector3::x_axis()
      };
      let angle = PI / 2.0 + rng.gen_range(0.0, 1.0) * rng.gen_range(-PI, PI);
      let rotation = Rotation3::from_axis_angle(&axis, angle);
      let axis = Vector3::z_axis();
      let angle = PI / 2.0
        + rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(-PI, PI);
      let rotation2 = Rotation3::from_axis_angle(&axis, angle);
      let r = prev * rotation * rotation2;
      prev = r;
      r
    })
    .collect::<Vec<Rotation3<f32>>>();

  let count = rng.gen_range(8, 22);
  let disp = rng.gen_range(3.0, 12.0);
  let incr = rng.gen_range(8.0, 20.0);
  let sphere_spots = rng.gen_range(0, 2);

  for _i in 0..count {
    let l = spots.len();
    if l == 0 {
      break;
    }
    let i = (l as f32
      * (1.0 - rng.gen_range(0.5, 1.0) * rng.gen_range(0.0, 1.0)))
      as usize;
    let translation: Vector3<f32> = spots[i];
    spots.remove(i);
    let rotation = *rng.choose(&rotations).unwrap();

    let is_rectangle = rng.gen_bool(0.6);

    let mut new_spots = vec![];

    let tris = if is_rectangle {
      let size = 5.0
        + rng.gen_range(0.0, 10.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0);
      let height = size + rng.gen_range(0.0, 80.0) * rng.gen_range(0.0, 1.0);
      let edges = if rng.gen_bool(0.4) {
        4
      } else if rng.gen_bool(0.8) {
        rng.gen_range(3, 7)
      } else {
        64
      };
      let polygon = regular_polygon_on_z_axis(edges, size);
      let extrusion = vec![
        Vector3::new(0.0, 0.0, -height),
        Vector3::new(0.0, 0.0, height),
      ];

      let mut z = -height;
      loop {
        if z > height {
          break;
        }
        let radius = rng.gen_range(0.0, disp);
        new_spots.push(Vector3::new(0.0, 0.0, z) + rand_disp(&mut rng, radius));
        z += incr;
      }

      extrude_along_path(&polygon, &extrusion)
    } else {
      let size = rng.gen_range(5.0, 15.0);
      let count = if rng.gen_bool(0.5) {
        rng.gen_range(4, 20)
      } else {
        rng.gen_range(100, 1000)
      };
      let demiball = rng.gen_bool(0.3);
      let center = Point3::origin();
      let mut triangles = vec![];
      let mut points = vec![];
      for _i in 0..count {
        let v = rand_disp(&mut rng, size);
        let mut x = v.x;
        if demiball {
          x = x.abs();
        }
        let p = center + Vector3::new(x, v.y, v.z);
        points.push(p);
      }
      if let Ok(tri) = hull(&points) {
        triangles.extend(tri);
        for p in points.iter().take(sphere_spots) {
          let radius = rng.gen_range(0.0, disp);
          new_spots.push((p - Point3::origin()) + rand_disp(&mut rng, radius));
        }
      }
      triangles
    };

    for t in tris {
      triangles.push(rotation * t + translation);
    }
    if new_spots.len() > 0 {
      if rng.gen_bool(0.1) {
        spots = vec![];
      }
      for s in new_spots {
        spots.push(rotation * s + translation);
      }
    }
  }

  // recenter everything, make a ground that is big enough for all shapes

  triangles
}

fn extrude_along_path(
  polygon: &Vec<Point3<f32>>,
  path: &Vec<Vector3<f32>>,
) -> Vec<Tri> {
  let mut triangles = Vec::new();

  let pathlen = path.len();

  // extrusions
  for i in 1..pathlen {
    let step1 = path[i - 1];
    let step2 = path[i];
    for j in 0..polygon.len() {
      let a = polygon[j] + step1;
      let b = polygon[(j + 1) % polygon.len()] + step1;
      let c = polygon[j] + step2;
      let d = polygon[(j + 1) % polygon.len()] + step2;
      triangles.push(Tri::new(a, b, c));
      triangles.push(Tri::new(d, c, b));
    }
  }

  // side faces
  let center = polygon
    .iter()
    .fold(Point3::origin(), |acc, p| acc + Vector3::new(p.x, p.y, p.z))
    / polygon.len() as f32;

  let step = path[0];
  for j in 0..polygon.len() {
    let b = polygon[j] + step;
    let a = polygon[(j + 1) % polygon.len()] + step;
    triangles.push(Tri::new(a, b, center + step));
  }

  let step = path[path.len() - 1];
  for j in 0..polygon.len() {
    let a = polygon[j] + step;
    let b = polygon[(j + 1) % polygon.len()] + step;
    triangles.push(Tri::new(a, b, center + step));
  }

  triangles
}

fn regular_polygon_on_z_axis(edges: usize, width: f32) -> Vec<Point3<f32>> {
  (0..edges)
    .map(|i| {
      let ang = i as f32 * 2. * PI / (edges as f32);
      let dx = width * ang.cos();
      let dy = width * ang.sin();
      Point3::new(dx, dy, 0.0)
    })
    .collect()
}

fn hull(points: &Vec<Point3<f32>>) -> Result<Vec<Tri>, chull::ErrorKind> {
  let mut triangles = vec![];
  let pts: Vec<Vec<f32>> = points.iter().map(|p| vec![p.x, p.y, p.z]).collect();
  let object = ConvexHullWrapper::try_new(&pts, None)?;
  let (v, faces) = object.vertices_indices();
  for face in faces.chunks(3) {
    let a = &v[face[0]];
    let b = &v[face[1]];
    let c = &v[face[2]];
    triangles.push(Tri::new(
      Point3::new(a[0], a[1], a[2]),
      Point3::new(b[0], b[1], b[2]),
      Point3::new(c[0], c[1], c[2]),
    ));
  }
  Ok(triangles)
}

struct Tri {
  v1: Point3<f32>,
  v2: Point3<f32>,
  v3: Point3<f32>,
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

  fn to_stl(&self) -> Triangle {
    let v1 = self.v1;
    let v2 = self.v2;
    let v3 = self.v3;
    let normal = (v2 - v1).cross(&(v3 - v1)).normalize();
    // Create the stl::Triangle struct using the normal vector and vertices
    Triangle {
      normal: [normal.x, normal.y, normal.z],
      v1: stl_point3(v1),
      v2: stl_point3(v2),
      v3: stl_point3(v3),
      attr_byte_count: 0,
    }
  }
}

fn rand_disp<R: Rng>(rng: &mut R, radius: f32) -> Vector3<f32> {
  let theta = rng.gen_range(0.0, PI);
  let phi = rng.gen_range(0.0, 2.0 * PI);
  let x = radius * theta.sin() * phi.cos();
  let y = radius * theta.sin() * phi.sin();
  let z = radius * theta.cos();
  Vector3::new(x, y, z)
}

fn stl_point3(p: Point3<f32>) -> [f32; 3] {
  [p.x, p.y, p.z]
}

/*
fn mix_point3(a: Point3<f32>, b: Point3<f32>, x: f32) -> Point3<f32> {
  Point3::new(mix(a.x, b.x, x), mix(a.y, b.y, x), mix(a.z, b.z, x))
}
fn mix_vec3(a: Vector3<f32>, b: Vector3<f32>, x: f32) -> Vector3<f32> {
  Vector3::new(mix(a.x, b.x, x), mix(a.y, b.y, x), mix(a.z, b.z, x))
}
fn mix(a: f32, b: f32, x: f32) -> f32 {
  (1. - x) * a + x * b
}
*/

fn main() {
  let opts: Opts = Opts::parse();
  let triangles = art(&opts);
  let f = File::create(opts.file).unwrap();
  let mut bw = BufWriter::new(f);
  let header: [u8; 80] = vec![0u8; 80].as_slice().try_into().unwrap();
  let stl = BinaryStlFile {
    header: BinaryStlHeader {
      header,
      num_triangles: triangles.len() as u32,
    },
    triangles: triangles.iter().map(|t| t.to_stl()).collect(),
  };
  write_stl(&mut bw, &stl).unwrap();
}
