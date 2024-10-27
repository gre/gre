use super::tri::Tri;
use chull::ConvexHullWrapper;
use kiss3d::nalgebra::Point3;

pub fn hull(points: &Vec<Point3<f32>>) -> Vec<Tri> {
  let mut triangles = vec![];
  let pts: Vec<Vec<f32>> = points.iter().map(|p| vec![p.x, p.y, p.z]).collect();
  let object = ConvexHullWrapper::try_new(&pts, None).unwrap();
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
  triangles
}
