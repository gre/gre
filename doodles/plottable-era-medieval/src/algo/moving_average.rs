/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub fn moving_average_2d(
  values: &Vec<(f32, f32)>,
  smooth: usize,
) -> Vec<(f32, f32)> {
  if values.len() == 0 || smooth == 0 {
    return values.clone();
  }
  let sf = smooth as f32;
  let mut sum = (0.0, 0.0);
  let mut acc = Vec::new();
  let mut out = Vec::new();
  for &h in values.iter() {
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

pub fn center_vec_2d(values: &Vec<(f32, f32)>, len: usize) -> Vec<(f32, f32)> {
  if values.len() >= len {
    return values.clone();
  }
  // make a Vec<(f32, f32)> of len elements, containing values in the middle and padded with the extremes
  let mut out = vec![values[values.len() - 1]; len];
  let first = values[0];
  let offset = (len - values.len()) / 2;
  for i in 0..offset {
    out[i] = first;
  }
  for i in 0..values.len() {
    out[i + offset] = values[i];
  }
  out
}
