use super::BandPattern;
use std::f32::consts::PI;

pub struct MedievalBandCurvePattern {
  xrep: f32,
  amp: f32,
  inner: f32,
  alt: bool,
}
impl MedievalBandCurvePattern {
  pub fn new() -> Self {
    Self {
      xrep: 4.0,
      amp: 0.5,
      inner: 0.05,
      alt: false,
    }
  }
}
impl BandPattern for MedievalBandCurvePattern {
  fn pattern(
    &self,
    clr: usize,
    length: f32,
    bandw: f32,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
    let mut routes = vec![];
    let xrep = self.xrep * bandw;
    // round the cellw to make the exact length
    let n = (length / xrep).round() as usize;
    let xrep = length / (n as f32);

    let amp = self.amp * bandw;

    let precision = 2.0;

    let mut curve1 = vec![];
    let mut curve2 = vec![];
    let mut p = 0.0;
    while p < length + precision {
      let x = p.min(length);
      let phase = 2.0 * PI * x / xrep;
      if self.alt {
        curve1.push((x, amp * phase.sin()));
        curve2.push((x, amp * (phase + PI).sin()));
      } else {
        curve1.push((x, amp * phase.cos()));
        curve2.push((x, amp * (phase + PI).cos()));
      }
      p += precision;
    }
    routes.push((clr, curve1));
    routes.push((clr, curve2));

    let mut p = 0.0;
    let off = if self.alt { 0.25 } else { 0.5 };
    for _i in 0..(2 * n) {
      routes.push((
        clr,
        vec![
          (p + xrep * (off - self.inner), 0.0),
          (p + xrep * (off + self.inner), 0.0),
        ],
      ));
      p += xrep / 2.0;
    }

    routes
  }

  fn corner(&self, clr: usize, bandw: f32) -> Vec<(usize, Vec<(f32, f32)>)> {
    let d = self.amp * bandw;
    let mut routes = vec![(
      clr,
      vec![
        (0.0, bandw),
        (0.0, 0.0),
        (bandw + self.xrep * bandw * self.inner, 0.0),
      ],
    )];
    if !self.alt {
      routes.push((clr, vec![(-d, bandw), (-d, -d), (bandw, -d)]));
      routes.push((clr, vec![(d, bandw), (d, d), (bandw, d)]));
    }
    routes
  }
}
