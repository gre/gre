pub fn mix(a: f32, b: f32, x: f32) -> f32 {
  (1. - x) * a + x * b
}

pub fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
  let k = ((x - a) / (b - a)).max(0.0).min(1.0);
  return k * k * (3.0 - 2.0 * k);
}
