use crate::algo::hull::hull;
use crate::algo::tri::Tri;
use nalgebra::{Point3, Vector3};
use rand::prelude::*;
use std::f32::consts::PI;

pub fn low_poly_ball(
  rng: &mut StdRng,
  count: usize,
  center: Point3<f32>,
  radius: f32,
) -> Vec<Tri> {
  let mut triangles = vec![];
  let mut points = vec![];
  for _i in 0..count {
    let theta = rng.gen_range(0.0..PI);
    let phi = rng.gen_range(0.0..2.0 * PI);
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
