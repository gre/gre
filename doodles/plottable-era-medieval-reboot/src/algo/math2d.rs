pub fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

pub fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

pub fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

pub fn collides_segment(
  from_1: (f64, f64),
  to_1: (f64, f64),
  from_2: (f64, f64),
  to_2: (f64, f64),
) -> Option<(f64, f64)> {
  // see https://stackoverflow.com/a/565282
  let p = from_1;
  let q = from_2;
  let r = (to_1.0 - p.0, to_1.1 - p.1);
  let s = (to_2.0 - q.0, to_2.1 - q.1);

  let r_cross_s = cross(r, s);
  let q_minus_p = (q.0 - p.0, q.1 - p.1);
  let q_minus_p_cross_r = cross(q_minus_p, r);

  // are the lines are parallel?
  if r_cross_s == 0.0 {
    // are the lines collinear?
    if q_minus_p_cross_r == 0.0 {
      // the lines are collinear
      None
    } else {
      // the lines are parallel but not collinear
      None
    }
  } else {
    // the lines are not parallel
    let t = cross(q_minus_p, div(s, r_cross_s));
    let u = cross(q_minus_p, div(r, r_cross_s));

    // are the intersection coordinates both in range?
    let t_in_range = 0.0 <= t && t <= 1.0;
    let u_in_range = 0.0 <= u && u <= 1.0;

    if t_in_range && u_in_range {
      // there is an intersection
      Some((p.0 + t * r.0, p.1 + t * r.1))
    } else {
      // there is no intersection
      None
    }
  }
}

fn cross(a: (f64, f64), b: (f64, f64)) -> f64 {
  a.0 * b.1 - a.1 * b.0
}

fn div(a: (f64, f64), b: f64) -> (f64, f64) {
  (a.0 / b, a.1 / b)
}

pub fn same_point(a: (f64, f64), b: (f64, f64)) -> bool {
  (a.0 - b.0).abs() < 0.0001 && (a.1 - b.1).abs() < 0.0001
}
