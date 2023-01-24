/**
 * Organic Crystal – 2023 – CC BY-SA 4.0 https://creativecommons.org/licenses/by-sa/4.0/
 * Author: @greweb
 */
mod utils;

use isosurface::{marching_cubes::MarchingCubes, source::Source};
use nalgebra::{Point3, Vector3};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::{
  convert::TryInto,
  io::BufWriter,
  ops::{Mul, Sub},
};
use stl::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, Serialize)]
pub struct Features {
  pub resolution_style: String,
  pub complexity: String,
  pub crystal: String,
  pub style: String,
}

#[wasm_bindgen]
pub struct ArtResult {
  stl: Vec<u8>,
  features: String,
}
#[wasm_bindgen]
impl ArtResult {
  pub fn new(stl: Vec<u8>, features: String) -> ArtResult {
    ArtResult { stl, features }
  }
  pub fn stl(&self) -> Vec<u8> {
    self.stl.clone()
  }
  pub fn features(&self) -> String {
    self.features.clone()
  }
}

#[wasm_bindgen]
pub fn render(val: JsValue) -> ArtResult {
  let opts = serde_wasm_bindgen::from_value(val).unwrap();
  let mut bw = BufWriter::new(Vec::new());
  let features = art(&opts, &mut bw);
  let json = serde_json::to_string(&features).unwrap();
  return ArtResult::new(bw.into_inner().unwrap(), json);
}

pub fn art<T: std::io::Write>(opts: &Opts, bw: &mut BufWriter<T>) -> Features {
  let art = Art::new(&opts);
  let triangles = art.to_triangles();
  let features = art.get_features();

  let header: [u8; 80] = vec![0u8; 80].as_slice().try_into().unwrap();
  let stl = BinaryStlFile {
    header: BinaryStlHeader {
      header,
      num_triangles: triangles.len() as u32,
    },
    triangles: triangles.iter().map(|t| t.to_stl()).collect(),
  };
  write_stl(bw, &stl).unwrap();
  features
}

#[derive(Deserialize)]
pub struct Opts {
  // the seed of the art
  pub hash: String,
  // The millimeters bounding size for the generated shape
  pub scale: f32,
}

// Create a rounded box shape
fn sd_box_rounded(p: Vector3<f32>, b: Vector3<f32>, r: f32) -> f32 {
  let q = p.abs() - b;
  Vector3::new(q.x.max(0.0), q.y.max(0.0), q.z.max(0.0)).norm()
    + q.x.max(q.y).max(q.z).min(0.0)
    - r
}

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

    if self.with_base {
      let w = 0.48;
      let b = sd_box_rounded(p - v3(0.0, 0.0, -0.5), v3(w, w, 0.1), 0.02);
      s = s.smooth_union(0.05, b);
      s = s.union(b);
    }

    // make sure things are cropped on the [0,1] domain
    let boundaries = sd_box_rounded(p, v3(0.49, 0.49, 0.49), 0.005);
    s = boundaries.intersect(s);
    // s = s.difference(signature(p.xy() * 2.0).intersect(p.z + 0.47));
    s
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
  fn new<R: Rng>(
    rng: &mut R,
    scale: f32,
    path_count: usize,
    with_base: bool,
    grid_size: usize,
    spikes: usize,
    cdist: f32,
  ) -> Column {
    let zigzagfactor = 1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.2, 1.0);
    let max_path_length = rng.gen_range(20.0, 80.0);

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
      let width = (24.0 - (grid_size as f32)).max(0.0).min(12.0) / 90.0
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
  features: Features,
}

impl Art {
  fn new(opts: &Opts) -> Art {
    let mut rng = rng_from_fxhash(&opts.hash);
    let mut triangles = vec![];

    let total_segments =
      (rng.gen_range(0.0, 16.0) * rng.gen_range(0.0, 1.0)) as usize;
    let column_variety = rng.gen_range(0.0, 1.0);
    let columns_count =
      ((1. + total_segments as f64 * column_variety) as usize).min(6);
    let column_each_around_count = 1 + total_segments / columns_count;

    let data: Vec<(f32, usize, usize)> = (0..columns_count)
      .map(|_i| {
        let scale = opts.scale;
        let count =
          (column_each_around_count as f64 + rng.gen_range(0.5, 2.0)) as usize;
        let grid_size = rng.gen_range(12, 24)
          + (2usize)
            .pow((rng.gen_range(0., 7.) * rng.gen_range(0.3, 1.0)) as u32);
        (scale, count, grid_size)
      })
      .collect();

    let max_data_by_grid_size = data
      .iter()
      // .map(|o| o.2)
      .enumerate()
      .max_by_key(|o| o.1 .2)
      .unwrap();

    let min_data_by_grid_size = data
      .iter()
      // .map(|o| o.2)
      .enumerate()
      .min_by_key(|o| o.1 .2)
      .unwrap();

    let max_index = max_data_by_grid_size.0;

    let mut complexity_factor = 0.;

    for (i, &(scale, count, grid_size)) in data.iter().enumerate() {
      let spikes = (rng.gen_range(0.0, 6.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.6, 1.0)) as usize;
      let cdist = rng.gen_range(0.35, 0.75);
      let column = Column::new(
        &mut rng,
        scale,
        count,
        i == max_index,
        grid_size,
        spikes,
        cdist,
      );
      let tri = column.to_triangles();

      complexity_factor += tri.len() as f64 / (grid_size as f64).powf(3.0);

      triangles.extend(tri);
    }

    // crystal part

    let r = opts.scale * rng.gen_range(0.2, 0.33);

    // find the highest triangle in columns under the crystal
    let mut highest = 0.0;
    for tri in triangles.iter() {
      for p in tri.points() {
        let dist2 = p.x * p.x + p.y * p.y;
        if dist2 < r * r * 0.2 && p.z > highest {
          highest = p.z;
        }
      }
    }

    let count = rng.gen_range(5, 20);
    let min_touch = rng.gen_range(10.0, 20.0);
    let z =
      (opts.scale * rng.gen_range(0.45, 0.55)).min(highest + r - min_touch);
    let center = Point3::new(0., 0., z);
    let mut points = points_around_sphere(&mut rng, count, center, r);
    points.push(Point3::new(0., 0., z - r));

    let mut crystal_volume = 0.0;
    if let Ok((all, volume)) = hull(&points) {
      triangles.extend(all);
      crystal_volume = volume;
    }

    let low_poly_threshold = 34;
    let min_grid_size_is_low_poly =
      min_data_by_grid_size.1 .2 < low_poly_threshold;
    let max_grid_size_is_low_poly =
      max_data_by_grid_size.1 .2 < low_poly_threshold;

    let resolution_style =
      (if min_grid_size_is_low_poly && max_grid_size_is_low_poly {
        "LowPoly"
      } else if min_grid_size_is_low_poly || max_grid_size_is_low_poly {
        "MixedPoly"
      } else {
        "HighPoly"
      })
      .to_string();

    let crystal = (if crystal_volume < 0.1 {
      "None"
    } else if crystal_volume < 1500. {
      "Thin"
    } else if crystal_volume < 4200. {
      "Light"
    } else if crystal_volume < 12000. {
      "Medium"
    } else {
      "Heavy"
    })
    .to_string();

    let complexity = (if complexity_factor < 0.3 {
      "Minimal"
    } else if complexity_factor < 0.6 {
      "Low"
    } else if complexity_factor < 1.2 {
      "Medium"
    } else if complexity_factor < 1.8 {
      "High"
    } else {
      "Very High"
    })
    .to_string();

    let style = (if rng.gen_bool(0.02) {
      "Gold"
    } else if rng.gen_bool(0.05) {
      "SilkBlueGreen"
    } else {
      "BlackRedYellow"
    })
    .to_string();

    let features = Features {
      resolution_style,
      complexity,
      crystal,
      style,
    };

    Art {
      triangles,
      features,
    }
  }

  fn get_features(&self) -> Features {
    self.features.clone()
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

fn hull(
  points: &Vec<Point3<f32>>,
) -> Result<(Vec<Tri>, f32), chull::ErrorKind> {
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
  Ok((triangles, object.volume()))
}

impl Tri {
  fn new(v1: Point3<f32>, v2: Point3<f32>, v3: Point3<f32>) -> Self {
    Tri { v1, v2, v3 }
  }

  fn points(&self) -> Vec<Point3<f32>> {
    vec![self.v1, self.v2, self.v3]
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

fn mix(a: f32, b: f32, x: f32) -> f32 {
  (1. - x) * a + x * b
}

fn rng_from_fxhash(hash: &String) -> impl Rng {
  let mut bs = [0; 32];
  bs58::decode(hash.chars().skip(2).take(43).collect::<String>())
    .into(&mut bs)
    .unwrap();
  let rng = StdRng::from_seed(bs);
  return rng;
}

/*
fn v2(x: f32, y: f32) -> Vector2<f32> {
  Vector2::new(x, y)
}

fn sd_box2_rounded(p: Vector2<f32>, b: Vector2<f32>, r: f32) -> f32 {
  let q = p.abs() - b;
  Vector2::new(q.x.max(0.0), q.y.max(0.0)).norm() + q.x.max(q.y).min(0.0) - r
}

fn signature(p: Vector2<f32>) -> f32 {
  let mut s = sd_box2_rounded(p - v2(0.0, -0.2), v2(0.2, 0.3), 0.0)
    .smooth_union(0.3, sd_box2_rounded(p - v2(0.0, 0.1), v2(0.5, 0.1), 0.0));

  s = s.union(sd_box2_rounded(p - v2(0.3, 0.4), v2(0.1, 0.08), 0.0));
  s = s.union(sd_box2_rounded(p - v2(-0.3, 0.4), v2(0.1, 0.08), 0.0));

  s
*/
