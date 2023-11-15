use super::BandPattern;

pub struct MedievalBandConcentric {
  count: usize,
}
impl MedievalBandConcentric {
  pub fn new(count: usize) -> Self {
    Self { count }
  }
}
impl BandPattern for MedievalBandConcentric {
  fn pattern(
    &self,
    clr: usize,
    length: f32,
    bandw: f32,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    for i in 0..self.count {
      let y =
        (i as f32 + 1.0) / (self.count as f32 + 1.0) * (2.0 * bandw) - bandw;
      routes.push((clr, vec![(0.0, y), (length, y)]));
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f32) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    for i in 0..self.count {
      let y =
        (i as f32 + 1.0) / (self.count as f32 + 1.0) * (2.0 * bandw) - bandw;
      routes.push((clr, vec![(y, bandw), (y, y), (bandw, y)]));
    }
    routes
  }
}
