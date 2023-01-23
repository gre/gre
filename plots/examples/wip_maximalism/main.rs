use clap::*;
use gre::rng_from_seed;
use isosurface::{marching_cubes::MarchingCubes, source::Source};
use kiss3d::nalgebra::{Point3, Vector3};
use rand::Rng;
use std::f32::consts::PI;
use std::{
  convert::TryInto,
  fs::File,
  io::BufWriter,
  ops::{Mul, Sub},
};
use stl::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  // The file name to be used for the output STL file
  #[clap(short, long, default_value = "result.stl")]
  file: String,
  // The millimeters bounding size for the generated shape
  #[clap(short, long, default_value = "60.0")]
  pub scale: f32,

  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

// We use sign distance function paradigm here:

// Create a rounded box shape
fn sd_box_rounded(p: Vector3<f32>, b: Vector3<f32>, r: f32) -> f32 {
  let q = p.abs() - b;
  Vector3::new(q.x.max(0.0), q.y.max(0.0), q.z.max(0.0)).norm()
    + q.x.max(q.y).max(q.z).min(0.0)
    - r
}

/*
fn sd_sphere(p: Vector3<f32>, s: f32) -> f32 {
  p.norm() - s
}
*/

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

fn v3(x: f32, y: f32, z: f32) -> Vector3<f32> {
  Vector3::new(x, y, z)
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

impl Source for Column {
  fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
    let p = v3(x, y, z) - v3(0.5, 0.5, 0.5);

    let mut s = 999.;

    /*
    sd_sphere(p, 0.52)
     .smooth_difference(0.05, sd_box_rounded(p, v3(0.5, 0.5, 0.3), 0.01));
     */
    // s = s.smooth_union(0.2, sd_sphere(p - v3(0.0, 0.0, 0.2), 0.1));

    for PathData {
      path,
      width,
      smoothing,
    } in self.paths.iter()
    {
      for (a, b) in path.iter().zip(path.iter().skip(1)) {
        s = s.smooth_union(*smoothing, sd_capsule(p, *a, *b, *width));
      }
    }

    /*
    for _i in 0..20 {
      s = s.smooth_union(
        0.05,
        sd_capsule(
          p - v3(0.0, 0.0, 0.2),
          v3(
            rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
            rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
            rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
          ),
          v3(
            rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
            rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
            rng.gen_range(-0.5, 0.5),
          ),
          rng.gen_range(0.03, 0.08),
        ),
      );
    }
    */

    if self.with_base {
      let w = 0.48;
      let b = sd_box_rounded(p - v3(0.0, 0.0, -0.5), v3(w, w, 0.08), 0.02);
      s = s.smooth_union(0.05, b);
      s = s.union(b);
    }

    // make sure things are cropped on the [0,1] domain
    let boundaries = sd_box_rounded(p, v3(0.48, 0.48, 0.48), 0.01);
    boundaries.intersect(s)
  }
}

struct PathData {
  path: Vec<Vector3<f32>>,
  smoothing: f32,
  width: f32,
}

struct Column {
  scale: f32,
  grid_size: usize,
  paths: Vec<PathData>,
  with_base: bool,
}
impl Column {
  fn new(
    seed: f64,
    scale: f32,
    path_count: usize,
    with_base: bool,
    grid_size: usize,
  ) -> Column {
    let mut rng = rng_from_seed(seed);

    let spikes = (rng.gen_range(0.0, 12.0) * rng.gen_range(0.0, 1.0)) as usize;
    let zigzagfactor = rng.gen_range(0.1, 0.9);
    let max_path_length = rng.gen_range(20.0, 80.0);
    let cdist = rng.gen_range(0.4, 0.6);

    let mut paths = vec![];

    let centers = (0..path_count)
      .map(|_| {
        let x = rng.gen_range(-cdist, cdist);
        let y = rng.gen_range(-cdist, cdist);
        (x, y)
      })
      .collect::<Vec<_>>();

    let sum = centers
      .iter()
      .fold((0.0, 0.0), |(a, b), (x, y)| (a + x, b + y));
    let center = (sum.0 / path_count as f32, sum.1 / path_count as f32);

    for c in centers {
      let smoothing = rng.gen_range(0.0, 0.1)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
      let width = 0.05 + rng.gen_range(0.0, 0.05) * rng.gen_range(0.0, 1.0);

      let count = (4.0
        + rng.gen_range(0.0, max_path_length) * rng.gen_range(0.0, 1.0))
        as usize;

      let max_height = 1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

      let basex = c.0 - center.0;
      let basey = c.1 - center.1;

      let path: Vec<Vector3<f32>> = (0..count)
        .map(|i| {
          let p = i as f32 / count as f32;
          let z: f32 = if i == 0 {
            -0.48
          } else {
            mix(
              rng.gen_range(-0.45, 0.45),
              p * max_height - 0.5,
              rng.gen_range(zigzagfactor, 1.0) - 0.1,
            )
          };
          let mul = 0.3 + 1.3 * z.max(0.0);
          let mixing = rng.gen_range(0.0, 1.0) * 0.5 + 0.5 * (0.5 - z);
          v3(
            mix(rng.gen_range(-0.5, 0.5) * mul, basex, mixing),
            mix(rng.gen_range(-0.5, 0.5) * mul, basey, mixing),
            z,
          )
        })
        .collect();

      paths.push(PathData {
        path: path.clone(),
        smoothing,
        width,
      });

      let smoothing = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);
      let width = (22.0 - (grid_size as f32)).max(0.0).min(12.0) / 100.0
        + 0.04
        + rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);
      for p in path.iter().skip(path.len() - spikes) {
        paths.push(PathData {
          path: vec![
            p.clone(),
            v3(
              rng.gen_range(-0.5, 0.5),
              rng.gen_range(-0.5, 0.5),
              0.5 - rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0),
            ),
          ],
          smoothing,
          width,
        });
      }
    }

    Column {
      scale,
      grid_size,
      paths,
      with_base,
    }
  }

  fn to_triangles(&self) -> Vec<Tri> {
    let mut all: Vec<Vec<Tri>> = vec![];

    {
      // Generate the triangles with marching cubes
      let mut vertices = vec![];
      let mut indices = vec![];
      let mut marching = MarchingCubes::new(self.grid_size);
      marching.extract(self, &mut vertices, &mut indices);
      let items = make_triangles_from_vertices_indices(&vertices, &indices);
      all.push(
        items
          .iter()
          .map(|tri| self.scale * (tri.clone() - Vector3::new(0.5, 0.5, 0.5)))
          .collect(),
      );
    }

    all.concat()
  }
}

struct Art {
  triangles: Vec<Tri>,
}

impl Art {
  fn new(opts: &Opts) -> Art {
    let mut rng = rng_from_seed(opts.seed);
    let mut triangles = vec![];

    let count =
      (1. + rng.gen_range(0.0, 5.0) * rng.gen_range(0.2, 1.0)) as usize;

    let data: Vec<(f64, f32, usize, usize)> = (0..count)
      .map(|i| {
        let seed = opts.seed + i as f64 / 0.07;
        let scale = opts.scale;
        let count =
          (1. + rng.gen_range(0.0, 8.0) * rng.gen_range(0.5, 1.0)) as usize;
        let grid_size =
          rng.gen_range(10, 20) + (2usize).pow(rng.gen_range(0, 6));
        (seed, scale, count, grid_size)
      })
      .collect();

    let max_index = data
      .iter()
      .map(|o| o.3)
      .enumerate()
      .max_by_key(|o| o.1)
      .unwrap()
      .0;

    /*
    let min_index = data
      .iter()
      .map(|o| o.3)
      .enumerate()
      .min_by_key(|o| o.1)
      .unwrap()
      .0;
      */

    for (i, &(seed, scale, count, grid_size)) in data.iter().enumerate() {
      let column = Column::new(seed, scale, count, i == max_index, grid_size);
      let tri = column.to_triangles();

      /*
      // we mess with the mesh by glitching one vertex
      if min_index == i && grid_size < 20 {
        let points = tri
          .iter()
          .flat_map(|t| vec![t.v1, t.v2, t.v3])
          .collect::<Vec<_>>();

        let highest_point = points.iter().max_by_key(|p| p.z as i32).unwrap();

        // we need to map tri to raise the highest point by a random amount
        let z = rng.gen_range(5.0, 10.0) * rng.gen_range(0.0, 1.0);
        let transversal = z;
        let translation = Vector3::new(
          rng.gen_range(-0.5, 0.5) * transversal,
          rng.gen_range(-0.5, 0.5) * transversal,
          z,
        );
        tri = tri
          .iter()
          .map(|t| {
            let mut t = t.clone();
            if t.v1 == *highest_point {
              t.v1 += translation;
            }
            if t.v2 == *highest_point {
              t.v2 += translation;
            }
            if t.v3 == *highest_point {
              t.v3 += translation;
            }
            t
          })
          .collect();
      }
      */

      triangles.extend(tri);
    }

    // crystal
    let r = opts.scale * rng.gen_range(0.2, 0.3);
    let count = rng.gen_range(8, 20);
    let mut points = points_around_sphere(
      &mut rng,
      count,
      Point3::new(0., 0., 0.45 * opts.scale),
      r,
    );
    points.push(Point3::new(0., 0., 0.45 * opts.scale - r));

    if let Ok(all) = hull(&points) {
      triangles.extend(all);
    }

    Art { triangles }
  }

  fn to_triangles(&self) -> Vec<Tri> {
    self.triangles.clone()
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

fn points_around_sphere<R: Rng>(
  rng: &mut R,
  count: usize,
  center: Point3<f32>,
  radius: f32,
) -> Vec<Point3<f32>> {
  let mut points = vec![];
  for _i in 0..count {
    let theta = rng.gen_range(0.0, PI);
    let phi = rng.gen_range(0.0, 2.0 * PI);
    let x = radius * theta.sin() * phi.cos();
    let y = radius * theta.sin() * phi.sin();
    let z = radius * theta.cos();
    let p = center + Vector3::new(x, y, z);
    points.push(p);
  }
  points
}

fn hull(points: &Vec<Point3<f32>>) -> Result<Vec<Tri>, chull::ErrorKind> {
  let mut triangles = vec![];
  let pts: Vec<Vec<f32>> = points.iter().map(|p| vec![p.x, p.y, p.z]).collect();
  let object = chull::ConvexHullWrapper::try_new(&pts, None)?;
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

impl Tri {
  fn new(v1: Point3<f32>, v2: Point3<f32>, v3: Point3<f32>) -> Self {
    Tri { v1, v2, v3 }
  }

  fn to_stl(&self) -> Triangle {
    let v1 = self.v1;
    let v2 = self.v2;
    let v3 = self.v3;
    let normal = (v2 - v1).cross(&(v3 - v1)).normalize();
    Triangle {
      normal: [normal.x, normal.y, normal.z],
      v1: stl_point3(v1),
      v2: stl_point3(v2),
      v3: stl_point3(v3),
      attr_byte_count: 0,
    }
  }
}

fn stl_point3(p: Point3<f32>) -> [f32; 3] {
  [p.x, p.y, p.z]
}

fn main() {
  let opts: Opts = Opts::parse();
  let triangles = Art::new(&opts).to_triangles();
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

fn mix(a: f32, b: f32, x: f32) -> f32 {
  (1. - x) * a + x * b
}
