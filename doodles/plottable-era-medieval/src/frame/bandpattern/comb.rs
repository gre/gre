use crate::algo::math1d::mix;

use super::BandPattern;

pub struct MedievalBandComb {
  cellw: f32,
  twistx: f32,
  pady: f32,
  ysplits: usize,
  comblength: f32,
}
impl MedievalBandComb {
  pub fn new() -> Self {
    Self {
      cellw: 2.0,
      twistx: 0.4,
      pady: 0.2,
      ysplits: 4,
      comblength: 0.5,
    }
  }
}
impl BandPattern for MedievalBandComb {
  fn pattern(
    &self,
    clr: usize,
    length: f32,
    bandw: f32,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;
    let twistx = self.twistx * cellw;
    let pady = self.pady * bandw;
    let ysplits = self.ysplits;
    let comblength = self.comblength * cellw;

    // round the cellw to make the exact length
    let n = (length / cellw).round() as usize;
    let cellw = length / (n as f32);

    let mut p = 0.0;
    for _i in 0..(n + 1) {
      let dy = bandw;
      let maxp = (p + twistx).min(length);
      routes.push((clr, vec![(p, -dy), (maxp, dy)]));
      for j in 0..ysplits {
        let y =
          ((j as f32 + 0.5) / (ysplits as f32) - 0.5) * (2.0 * (bandw - pady));
        let x = mix(p, p + twistx, (y + bandw) / (2.0 * bandw));
        routes.push((clr, vec![(x.min(length), y), (x - comblength, y)]));
      }
      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f32) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    let pady = self.pady * bandw;
    let ysplits = self.ysplits;

    for j in 0..ysplits {
      let y =
        ((j as f32 + 0.5) / (ysplits as f32) - 0.5) * (2.0 * (bandw - pady));
      routes.push((clr, vec![(-bandw, y), (bandw, y)]));
    }
    routes
  }
}
