use super::tri::Tri;
use kiss3d::nalgebra::{
  Matrix3, Point3, Rotation3, Unit, UnitVector3, Vector3,
};
use std::ops::Add;

pub fn triangularize_polygon(poly: &Vec<Point3<f32>>) -> Vec<Tri> {
  let mut triangles = Vec::new();
  let n = poly.len();
  if n < 3 {
    return triangles;
  }
  for i in 1..n - 1 {
    triangles.push(Tri::new(poly[0], poly[i], poly[i + 1]));
  }
  triangles
}

/**
 * Build a 3D object, resulting of extrusion of a polygon built along a path.
 */

pub fn build_polygonal_path(
  // number of vertices of the polygon
  polysize: usize,
  // (vertex_index, step_index) -> (angle, radius)
  polar_f: &mut dyn FnMut(usize, usize) -> (f32, f32),
  // 3D path to follow and project onto
  // TODO we need to give a way to specify the orientation of the path
  path: &Vec<Point3<f32>>,
) -> Vec<Tri> {
  let mut triangles = Vec::new();

  if path.len() < 2 {
    return triangles;
  }

  let pathlen = path.len();

  let mut polys = vec![];

  let mut prev = path[0];
  let mut prev_dir = Unit::new_normalize(path[1] - path[0]);

  for i in 0..pathlen {
    let point = path[i];

    // Calculate direction vector based on previous and next points
    let next_dir = if i < pathlen - 1 {
      Unit::new_normalize(path[i + 1] - point)
    } else {
      Unit::new_normalize(point - prev)
    };

    // Smoothly interpolate between previous and next direction vectors
    let direction = Unit::new_normalize(Vector3::new(
      prev_dir.x + next_dir.x,
      prev_dir.y + next_dir.y,
      prev_dir.z + next_dir.z,
    ));

    // Calculate a perpendicular vector to the given direction
    let up = Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0));
    let perpendicular = direction.cross(&up);

    let mut poly = Vec::new();
    for j in 0..polysize {
      let (angle, radius) = polar_f(j, i);
      let rotation = Rotation3::from_axis_angle(&direction, angle);
      let point_on_circle =
        Point3::from(point + radius * (rotation * perpendicular));
      poly.push(point_on_circle);
    }
    polys.push(poly);

    prev = point;
    prev_dir = next_dir;
  }

  // extrusions
  for i in 1..pathlen {
    let p1 = &polys[i - 1];
    let p2 = &polys[i];
    for j in 0..polysize {
      let a = p1[j];
      let b = p1[(j + 1) % polysize];
      let c = p2[j];
      let d = p2[(j + 1) % polysize];
      triangles.push(Tri::new(a, b, c));
      triangles.push(Tri::new(d, c, b));
    }
  }

  // side faces
  let mut poly = polys[0].clone();
  poly.reverse();
  triangles.extend(triangularize_polygon(&poly));

  triangles.extend(triangularize_polygon(&polys[pathlen - 1]));

  triangles
}

pub fn extrude_along_path(
  polygon: &Vec<Point3<f32>>,
  path: &Vec<Vector3<f32>>,
) -> Vec<Tri> {
  let mut triangles = Vec::new();

  let pathlen = path.len();

  let mut rotated_polys = vec![];

  for i in 1..pathlen {
    let step1 = path[i - 1];
    let step2 = path[i];

    let diff = (step2 - step1).normalize();

    let mut rotated_poly = vec![];

    let m = Matrix3::identity(); //Vector3::z_axis().angle(&diff);

    let axis = Vector3::new(0.0, 0.0, -1.0);
    //let m = Rotation3::rotation_between(&axis, &diff).unwrap();
    for p in polygon {
      rotated_poly.push(m * p);
    }
    rotated_polys.push(rotated_poly);

    /*
    if diff.x == 0.0 && diff.y == 0.0 {
      rotated_polys.push(polygon.clone());
    } else {
      let m = Rotation3::look_at_lh(&diff, &Vector3::z_axis());
      for p in polygon {
        rotated_poly.push(m * p);
      }
      rotated_polys.push(rotated_poly);
    }
    */

    // let n = m * Point3::new(1.0, 0.0, 0.0);
    // Vector3::z_axis()- diff
  }

  // extrusions
  for i in 1..pathlen {
    let step1 = path[i - 1];
    let step2 = path[i];
    let poly1 = rotated_polys[i - 1].clone();
    let poly2 = rotated_polys[(i).min(rotated_polys.len() - 1)].clone();

    for j in 0..poly1.len() {
      let a = poly1[j] + step1;
      let b = poly1[(j + 1) % poly1.len()] + step1;
      let c = poly2[j] + step2;
      let d = poly2[(j + 1) % poly2.len()] + step2;
      triangles.push(Tri::new(a, b, c));
      triangles.push(Tri::new(d, c, b));
    }
  }

  /*
  // side faces
  let center = polygon
    .iter()
    .fold(Point3::origin(), |acc, p| acc + Vector3::new(p.x, p.y, p.z))
    / polygon.len() as f32;
  for step in vec![path[0], path[path.len() - 1]] {
    for j in 0..polygon.len() {
      let a = polygon[j] + step;
      let b = polygon[(j + 1) % polygon.len()] + step;
      triangles.push(Tri::new(a, b, center + step));
    }
  }
  */
  triangles
}

pub fn extrude_along_path_no_rotation(
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
  for step in vec![path[0], path[path.len() - 1]] {
    for j in 0..polygon.len() {
      let a = polygon[j] + step;
      let b = polygon[(j + 1) % polygon.len()] + step;
      triangles.push(Tri::new(a, b, center + step));
    }
  }
  triangles
}
