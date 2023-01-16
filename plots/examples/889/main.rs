use clap::*;
use isosurface::{marching_cubes::MarchingCubes, source::Source};
use kiss3d::nalgebra::{Point3, Vector2, Vector3};
use num_traits::Pow;
use std::{
  convert::TryInto,
  f32::consts::PI,
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
  #[clap(short, long, default_value = "80.0")]
  pub scale: f32,
  // The resolution of the grid used for the shape generation
  // low value creates a low poly style
  #[clap(short, long, default_value = "16")]
  pub resolution: usize,
}

// We use sign distance function paradigm here:

// Create a rounded box shape
fn sd_round_box(p: Vector3<f32>, b: Vector3<f32>, r: f32) -> f32 {
  let q = p.abs() - b;
  Vector3::new(q.x.max(0.0), q.y.max(0.0), q.z.max(0.0)).norm()
    + q.x.max(q.y).max(q.z).min(0.0)
    - r
}

// Create an intersection of two shapes
fn op_intersection(d1: f32, d2: f32) -> f32 {
  d1.max(d2)
}

struct SineWavesPyramid {}
impl Source for SineWavesPyramid {
  fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
    let p = Vector3::new(x, y, z);

    // distance from the center of the shape
    let dist_xy_center = Vector2::new(x - 0.5, y - 0.5).norm();

    // frequency of the sine waves
    let f = 6.0 * PI;
    let fz = f * 0.8;

    // create the pyramid effect
    let zoff = 0.9 - 0.5 * dist_xy_center;
    let zamp = 1.8;
    let zpow = 1.0;

    // amplitude of the sine waves displacement
    let amp = 0.5 * (0.5 - dist_xy_center).max(0.0).powf(0.5);

    // rounded box configuration
    let border_radius = 0.05;
    let r = 0.45 - border_radius;
    let size = Vector3::new(r, r, r);
    let center = Vector3::new(0.5, 0.5, 1.0);

    // intersect a rounded box with a sine wave pyramid
    op_intersection(
      sd_round_box(p - center, size, border_radius),
      // pyramid
      zamp * (z.abs().pow(zpow) - zoff)
        + amp
          * (x * f + 0.5 * PI).sin()
          * (y * f + 0.5 * PI).sin()
          * (z * fz).sin(),
    )
  }
}

fn art(opts: &Opts) -> Vec<Tri> {
  let grid_size = opts.resolution;
  let mut vertices = vec![];
  let mut indices = vec![];
  let source = SineWavesPyramid {};
  let mut marching = MarchingCubes::new(grid_size);
  marching.extract(&source, &mut vertices, &mut indices);
  let triangles = make_triangles_from_vertices_indices(&vertices, &indices);
  triangles
    .iter()
    .map(|tri| opts.scale * (tri.clone() - Vector3::new(0.5, 0.5, 0.5)))
    .collect()
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
