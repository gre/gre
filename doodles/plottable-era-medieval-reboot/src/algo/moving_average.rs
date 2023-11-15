/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn moving_average_2d(
  values: &Vec<(f32, f32)>,
  smooth: usize,
) -> Vec<(f32, f32)> {
  let sf = smooth as f32;
  let mut sum = (0.0, 0.0);
  let mut acc = Vec::new();
  let mut out = Vec::new();
  for (i, &h) in values.iter().enumerate() {
    if acc.len() == smooth {
      let avg = (sum.0 / sf, sum.1 / sf);
      let prev: (f32, f32) = acc.remove(0);
      sum = (sum.0 - prev.0, sum.1 - prev.1);
      out.push(avg);
    }
    acc.push(h);
    sum = (sum.0 + h.0, sum.1 + h.1);
  }
  out
}
