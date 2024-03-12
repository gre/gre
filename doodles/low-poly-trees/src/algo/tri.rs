use nalgebra::Point3;

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
