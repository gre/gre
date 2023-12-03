pub fn mix(a: f32, b: f32, x: f32) -> f32 {
  (1. - x) * a + x * b
}

pub fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
  let k = ((x - a) / (b - a)).max(0.0).min(1.0);
  return k * k * (3.0 - 2.0 * k);
}

pub fn values_subdivide_to_curve_it(
  values: &Vec<f32>,
  interpolation: f32,
) -> Vec<f32> {
  let l = values.len();
  if l < 3 {
    return values.clone();
  }
  let mut route = Vec::new();
  let mut first = values[0];
  let mut last = values[l - 1];
  route.push(first);
  for i in 1..(l - 1) {
    let p = values[i];
    let p1 = mix(values[i - 1], p, interpolation);
    let p2 = mix(values[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  route.push(last);
  route
}

pub fn values_subdivide_to_curve(
  values: &Vec<f32>,
  n: usize,
  interpolation: f32,
) -> Vec<f32> {
  if n == 0 {
    return values.clone();
  }
  let mut route = values.clone();
  for _i in 0..n {
    route = values_subdivide_to_curve_it(&route, interpolation);
  }
  route
}
