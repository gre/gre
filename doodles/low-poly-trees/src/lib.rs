use algo::{
  extrusion::{build_polygonal_path, extrude_along_path},
  math1d::mix,
};
use nalgebra::{Matrix3, Point3, Rotation3, Vector3};
use noise::*;
use primitives::lowpolyball::low_poly_ball;
use rand::prelude::*;
use std::{f32::consts::PI, io::BufWriter};
use wasm_bindgen::prelude::*;
mod fxhash;
use fxhash::*;
mod performance;
use performance::*;
mod global;
use global::*;
mod algo;
mod primitives;
mod stlexport;

use stlexport::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – TEMPLATE
 */

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
pub fn render(hash: String, debug: bool) -> ArtResult {
  let mut perf = PerfRecords::start(debug);
  let mut rng = rng_from_hash(&hash);

  let mut objects = vec![];

  perf.span("all", &vec![]);

  let global = GlobalCtx::rand(&mut rng);

  /*
  tris.push((
    0,
    lowpolyball::low_poly_ball(&mut rng, 20, Point3::origin(), 10.0),
  ));
  */

  let edges = 6;
  let max_radius = 20.0;
  let trunkrotfreq = 0.2;
  let centeringpow = 0.5;
  let max_dist = 40.0;
  let trunkrotamp = 2.0;
  let stepping = 5.0;
  let trunk_thickness = 3.0;

  let height = rng.gen_range(60.0..80.0);

  // branching system
  // let mut branches;
  // let mut spawns = vec![];

  let stepping = 3.0;

  let mut build_branch = |rng: &mut StdRng,
                          origin: Point3<f32>,
                          dir: Vector3<f32>,
                          length: f32,
                          thickness: f32| {
    let perlin = Perlin::new(rng.gen());
    let seed = rng.gen_range(0.0..100.0);

    let subdivisions = (length / stepping) as usize;
    if subdivisions < 1 {
      return (vec![], vec![]);
    }
    let rotang = 10.0 / subdivisions as f32;
    let f = 0.5;

    let mut path = vec![];
    let mut p = origin;
    let mut direction = dir;
    let mut orientation = Matrix3::identity();

    let mut orientations = vec![];

    for _ in 0..subdivisions + 1 {
      let l = length / subdivisions as f32;
      path.push(p);
      p += direction * l;

      let n1 = perlin.get([
        f * p.x as f64 + seed / 0.6,
        f * p.y as f64 + seed * 3.3358534342,
        f * p.z as f64 + seed,
      ]) as f32;

      let n2 = perlin.get([
        f * p.x as f64 + seed * 0.3,
        f * p.y as f64 + seed * 7.3358534342,
        f * p.z as f64 + seed,
      ]) as f32;

      let n3 = perlin.get([
        f * p.x as f64 + seed / 0.016,
        f * p.y as f64 + seed / 0.472,
        f * p.z as f64,
      ]) as f32;

      let roll = n1 * rotang;
      let pitch = n2 * rotang;
      let yaw = n3 * rotang;
      let rotation = Rotation3::from_euler_angles(roll, pitch, yaw);
      direction = rotation * direction;

      orientation = rotation * orientation;
      orientations.push(orientation);
    }

    let edges = 6;
    let branch = build_polygonal_path(
      edges,
      &mut |i, j| {
        let ang = i as f32 * 2. * PI / (edges as f32);
        let rad = thickness;
        let rad = rad * mix(1.0, 0.2, (j as f32) / subdivisions as f32);
        (ang, rad)
      },
      &path,
    );
    objects.push((1, branch));

    (path, orientations)
  };

  let p = Point3::origin();
  let direction = Vector3::new(0.0, 0.0, 1.0);
  let branch_length = 50.0;
  let thickness = 4.0;
  let (path, orientations) =
    build_branch(&mut rng, p, direction, branch_length, thickness);

  // spawn on the middle

  let divs = 20;
  for _ in 0..divs {
    let i = rng.gen_range(1..path.len());
    let f = i as f32 / path.len() as f32;

    let p = path[i];
    let or = orientations[i];
    let a = rng.gen_range(-PI..PI);
    let acos = a.cos();
    let asin = a.sin();
    let dir = or * Vector3::new(acos, asin, 0.0);
    let l = branch_length * f;
    build_branch(&mut rng, p, dir, l, thickness * (0.3 + 0.5 * f));
  }

  /*
  // we try to make a tree made of low poly balls connected with a trunc.
  // trunc can "try" to rotate around the center
  // we build the shape from bottom to top to make it printable "as-is"
  let count = rng.gen_range(5..10);
  for i in 0..count {
    let radius = rng.gen_range(8.0..max_radius);
    let samples = radius as usize + rng.gen_range(0..20);
    let z = radius + rng.gen_range(0.0..10.0);
    let ang = rng.gen_range(-PI..PI);
    let dist = rng.gen_range(0.0..max_dist);
    let x = dist * ang.cos();
    let y = dist * ang.sin();
    let center = Point3::new(x, y, z);


    let mut bush_tris = vec![];
    for tri in low_poly_ball(&mut rng, 20, center, radius) {
      bush_tris.push(tri);
    }
    objects.push((0, bush_tris));


    let mut dz = 0.0;
    let maxdz = height - center.z;
    let mut path = vec![];
    let angstart = rng.gen_range(0.0..2.0 * PI);

    loop {
      let last = dz > maxdz;
      if last {
        dz = maxdz;
      }
      let centeringfactor = ((dz - radius) / (maxdz - radius))
        .max(0.0)
        .powf(centeringpow);

      let a = angstart + dz * trunkrotfreq;

      let dx = mix(0.0, center.x, centeringfactor) + trunkrotamp * a.cos();
      let dy = mix(0.0, center.y, centeringfactor) + trunkrotamp * a.sin();

      path.push(Point3::new(dx, dy, center.z + maxdz - dz));

      dz += stepping;
      if last {
        break;
      }
    }

    let trunk_tris = build_polygonal_path(
      edges,
      &mut |i, j| {
        let ang = i as f32 * 2. * PI / (edges as f32);
        let rad = trunk_thickness;
        let rad = rad * mix(1.0, 0.3, (j as f32+0.5) / path.len() as f32);
        (ang, rad)
      },
      &path,
    );
    objects.push((1, trunk_tris));
  }
  */

  // export to stl
  let mut bw = BufWriter::new(Vec::new());
  stl_export(&mut bw, &objects, &global.palette);

  // export features
  let feature = global.to_feature();
  let feature_json = feature.to_json();

  perf.span_end("all", &vec![]);

  return ArtResult::new(bw.into_inner().unwrap(), feature_json);
}
