use chull::ConvexHullWrapper;
use clap::*;
use gre::rng_from_seed;
use kiss3d::nalgebra::{Point3, Vector3};
use rand::Rng;
use std::{convert::TryInto, f32::consts::PI, fs::File, io::BufWriter};
use stl::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "result.stl")]
  file: String,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "22.0")]
  pub max_radius: f32,
  #[clap(short, long, default_value = "30.0")]
  pub max_dist: f32,
}

fn art(opts: &Opts) -> Vec<Triangle> {
  let mut rng = rng_from_seed(opts.seed);
  let mut triangles = vec![];

  let height = rng.gen_range(60.0, 80.0);

  // we try to make a tree made of low poly balls connected with a trunc.
  // trunc can "try" to rotate around the center
  // we build the shape from bottom to top to make it printable "as-is"
  let count = rng.gen_range(2, 8);
  for i in 0..count {
    let radius = rng.gen_range(8.0, opts.max_radius);
    let samples = radius as usize + rng.gen_range(0, 20);
    let z = radius + rng.gen_range(0.0, 10.0);
    let ang = rng.gen_range(-PI, PI);
    let dist = rng.gen_range(0.0, opts.max_dist);
    let x = dist * ang.cos();
    let y = dist * ang.sin();
    let center = Point3::new(x, y, z);
    for tri in low_poly_ball(&mut rng, samples, center, radius) {
      triangles.push(tri);
    }

    // how much vertex for the polygon
    let edges = 5;
    let width = 2.0;
    let start = (0..edges)
      .map(|i| {
        let ang = i as f32 * 2. * PI / (edges as f32);
        let dx = width * ang.cos();
        let dy = width * ang.sin();
        Point3::new(center.x + dx, center.y + dy, center.z)
      })
      .collect();

    let stepping = 5.0;
    let mut dz = 0.0;
    let maxdz = height - center.z;
    let mut path = vec![];
    let trunkrotfreq = 0.2;
    let trunkrotamp = 3.0;
    let centeringpow = 0.5;
    loop {
      let last = dz > maxdz;
      if last {
        dz = maxdz;
      }
      let centeringfactor = ((dz - radius) / (maxdz - radius))
        .max(0.0)
        .powf(centeringpow);

      let a = i as f32 + dz * trunkrotfreq;

      let dx = mix(0.0, -center.x, centeringfactor) + trunkrotamp * a.cos();
      let dy = mix(0.0, -center.y, centeringfactor) + trunkrotamp * a.sin();

      path.push(Vector3::new(dx, dy, dz));

      dz += stepping;
      if last {
        break;
      }
    }

    for t in extrude_along_path(&start, &path) {
      triangles.push(t);
    }
  }

  triangles
}

fn extrude_along_path(
  polygon: &Vec<Point3<f32>>,
  path: &Vec<Vector3<f32>>,
) -> Vec<Triangle> {
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
      triangles.push(stl_tri(a, b, c));
      triangles.push(stl_tri(d, c, b));
    }
  }

  // side faces
  let center = polygon
    .iter()
    .fold(Point3::origin(), |acc, p| acc + Vector3::new(p.x, p.y, p.z))
    / polygon.len() as f32;
  for step in vec![path[0], path[path.len() - 1]] {
    for j in 0..polygon.len() {
      let a = polygon[j] + step;
      let b = polygon[(j + 1) % polygon.len()] + step;
      triangles.push(stl_tri(a, b, center + step));
    }
  }
  triangles
}

fn low_poly_ball<R: Rng>(
  rng: &mut R,
  count: usize,
  center: Point3<f32>,
  radius: f32,
) -> Vec<Triangle> {
  let mut triangles = vec![];
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
  for tri in hull(&points) {
    triangles.push(tri);
  }
  triangles
}

fn hull(points: &Vec<Point3<f32>>) -> Vec<Triangle> {
  let mut triangles = vec![];
  let pts: Vec<Vec<f32>> = points.iter().map(|p| vec![p.x, p.y, p.z]).collect();
  let object = ConvexHullWrapper::try_new(&pts, None).unwrap();
  let (v, faces) = object.vertices_indices();
  for face in faces.chunks(3) {
    let a = &v[face[0]];
    let b = &v[face[1]];
    let c = &v[face[2]];
    triangles.push(stl_tri(
      Point3::new(a[0], a[1], a[2]),
      Point3::new(b[0], b[1], b[2]),
      Point3::new(c[0], c[1], c[2]),
    ));
  }
  triangles
}

fn stl_tri(v1: Point3<f32>, v2: Point3<f32>, v3: Point3<f32>) -> Triangle {
  let normal = (v2 - v1).cross(&(v3 - v1)).normalize();
  // Create the stl::Triangle struct using the normal vector and vertices
  Triangle {
    normal: [normal.x, normal.y, normal.z],
    v1: [v1.x, v1.y, v1.z],
    v2: [v2.x, v2.y, v2.z],
    v3: [v3.x, v3.y, v3.z],
    attr_byte_count: 0,
  }
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
    triangles,
  };
  write_stl(&mut bw, &stl).unwrap();
}

fn mix(a: f32, b: f32, x: f32) -> f32 {
  (1. - x) * a + x * b
}
