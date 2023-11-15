use crate::algo::math2d::lerp_point;

use super::BandPattern;

pub struct MedievalBandFeatherTrianglePattern {
  cellw: f32,
  feather_ratio: f32,
  count1: usize,
  count2: usize,
}
impl MedievalBandFeatherTrianglePattern {
  pub fn new() -> Self {
    Self {
      cellw: 6.0,
      count1: 3,
      count2: 3,
      feather_ratio: 0.66,
    }
  }

  pub fn feather(
    &self,
    clr: usize,
    a: (f32, f32),
    b: (f32, f32),
    c: (f32, f32),
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    // TODO add a pad
    let mut routes = vec![]; // array of (clr, path)
    let count1 = self.count1;
    let count2 = self.count2;
    for i in 0..count1 {
      let t = ((i + 1) as f32 / (count1 + 1) as f32) * self.feather_ratio;
      let p = lerp_point(a, b, t);
      let q = lerp_point(a, c, t);
      routes.push((clr, vec![p, q]));
    }
    for i in 0..count2 {
      let t = (i as f32 + 1.0) / (count2 + 1) as f32;
      let end_bc = lerp_point(b, c, t);
      routes
        .push((clr, vec![lerp_point(a, end_bc, self.feather_ratio), end_bc]));
    }
    routes
  }
}
impl BandPattern for MedievalBandFeatherTrianglePattern {
  fn pattern(
    &self,
    clr: usize,
    length: f32,
    bandw: f32,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;

    // round the cellw to make the exact length
    let n = (length / cellw).round() as usize;
    let cellw = length / (n as f32);

    let mut p = 0.0;
    for _i in 0..n {
      let dy = bandw;
      routes
        .push((clr, vec![(p, dy), (p + cellw / 2.0, -dy), (p + cellw, dy)]));

      routes.extend(self.feather(
        clr,
        (p, dy),
        (p + cellw / 2.0, -dy),
        (p + cellw, dy),
      ));

      routes.extend(self.feather(
        clr,
        (p - cellw / 2.0, -dy),
        (p + cellw / 2.0, -dy),
        (p, dy),
      ));

      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f32) -> Vec<(usize, Vec<(f32, f32)>)> {
    let cellw = self.cellw * bandw;
    let mut routes = self.feather(
      clr,
      (-bandw, cellw - bandw),
      (-bandw, -bandw),
      (bandw, bandw),
    );
    routes.push((clr, vec![(-bandw, -bandw), (bandw, bandw)]));
    routes
  }
}
