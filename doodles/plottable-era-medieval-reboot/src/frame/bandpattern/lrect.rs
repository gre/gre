use super::BandPattern;

pub struct MedievalBandLRectPattern {
  cellw: f32,
  padx: f32,
  pady: f32,
  offx: f32,
}
impl MedievalBandLRectPattern {
  pub fn new() -> Self {
    Self {
      cellw: 2.0,
      padx: 0.15,
      pady: 0.05,
      offx: 0.25,
    }
  }
}
impl BandPattern for MedievalBandLRectPattern {
  fn pattern(
    &self,
    clr: usize,
    length: f32,
    bandw: f32,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;
    let padx = self.padx * cellw;
    let pady = self.pady * cellw;
    let offx = self.offx * cellw;

    let l = length + 2.0 * padx;

    // round the cellw to make the exact length
    let n = (l / cellw).round() as usize;
    let cellw = l / (n as f32);

    let mut p = -padx;
    for _i in 0..n {
      routes.push((
        clr,
        vec![
          (p + padx + offx, -bandw / 2.0 + pady),
          (p + cellw - padx, -bandw / 2.0 + pady),
          (p + cellw - padx, bandw / 2.0 - pady),
        ],
      ));
      routes.push((
        clr,
        vec![
          (p + padx, -bandw / 2.0 + pady),
          (p + padx, bandw / 2.0 - pady),
          (p + cellw - padx - offx, bandw / 2.0 - pady),
        ],
      ));
      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f32) -> Vec<(usize, Vec<(f32, f32)>)> {
    let cellw = self.cellw * bandw;
    let pady = self.pady * cellw;
    let d = bandw / 2.0 - pady;
    vec![(clr, vec![(-d, -d), (d, -d), (d, d), (-d, d), (-d, -d)])]
  }
}
