use super::BandPattern;

pub struct MedievalBandForkPattern {
  cellw: f32,
  cutx: f32,
  spacex: f32,
  pady: f32,
  simplecorner: bool,
}
impl MedievalBandForkPattern {
  pub fn new() -> Self {
    Self {
      cellw: 2.0,
      cutx: 0.6,
      spacex: 0.3,
      pady: 0.1,
      simplecorner: false,
    }
  }
}
impl BandPattern for MedievalBandForkPattern {
  fn pattern(
    &self,
    clr: usize,
    length: f32,
    bandw: f32,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;
    let cutx = cellw * self.cutx;
    let spacex = self.spacex * cellw;
    let pady = self.pady * bandw;
    let dy = bandw / 2.0 - pady;

    // round the cellw to make the exact length
    // we eat an extra space for the last fork
    let l = length + (cellw - cutx);
    let n = (l / cellw).round() as usize;
    let cellw = l / (n as f32);

    let mut p = 0.0;
    for _i in 0..n {
      routes.push((clr, vec![(p, 0.0), (p + cutx - spacex, 0.0)]));
      routes.push((clr, vec![(p + cutx, 0.0), (p + cellw, 0.0)]));

      routes.push((
        clr,
        vec![(p, -dy), (p + cutx, -dy), (p + cutx, dy), (p, dy)],
      ));

      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f32) -> Vec<(usize, Vec<(f32, f32)>)> {
    if self.simplecorner {
      return vec![(clr, vec![(bandw, 0.0), (0.0, 0.0), (0.0, bandw)])];
    }
    let sz = bandw * (0.5 - 2.0 * self.pady);
    let rect = vec![(-sz, -sz), (sz, -sz), (sz, sz), (-sz, sz), (-sz, -sz)];
    vec![
      (clr, vec![(bandw, 0.0), (sz, 0.0)]),
      (clr, vec![(0.0, sz), (0.0, bandw)]),
      (clr, rect),
    ]
  }
}
