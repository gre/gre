use std::ops::{Add, Mul, Sub};

use kiss3d::nalgebra::{Point3, Rotation3, Vector3};

#[derive(Clone, Copy)]
pub struct Tri {
  pub a: Point3<f32>,
  pub b: Point3<f32>,
  pub c: Point3<f32>,
}

impl Tri {
  pub fn new(a: Point3<f32>, b: Point3<f32>, c: Point3<f32>) -> Self {
    Self { a, b, c }
  }
}

impl Sub<Vector3<f32>> for Tri {
  type Output = Tri;

  fn sub(self, v: Vector3<f32>) -> Self::Output {
    Tri {
      a: self.a - v,
      b: self.b - v,
      c: self.c - v,
    }
  }
}

impl Add<Vector3<f32>> for Tri {
  type Output = Tri;

  fn add(self, v: Vector3<f32>) -> Self::Output {
    Tri {
      a: self.a + v,
      b: self.b + v,
      c: self.c + v,
    }
  }
}

impl Mul<Tri> for f32 {
  type Output = Tri;

  fn mul(self, tri: Tri) -> Self::Output {
    Tri {
      a: self * tri.a,
      b: self * tri.b,
      c: self * tri.c,
    }
  }
}

impl Mul<Tri> for Rotation3<f32> {
  type Output = Tri;

  fn mul(self, tri: Tri) -> Self::Output {
    Tri {
      a: self * tri.a,
      b: self * tri.b,
      c: self * tri.c,
    }
  }
}
