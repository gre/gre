use crate::algo::math2d::euclidian_dist;

pub mod comb;
pub mod concentric;
pub mod curve;
pub mod feather;
pub mod fork;
pub mod lrect;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub trait BandPattern {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)>;

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)>;

  fn render_corner(
    &self,
    clr: usize,
    position: (f64, f64),
    angle: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let untranslated = self.corner(clr, bandw);
    let acos = angle.cos();
    let asin = angle.sin();
    let mut routes = vec![];
    for (clr, route) in untranslated {
      let mut r = vec![];
      for &p in route.iter() {
        let p = (
          p.0 * acos + p.1 * asin + position.0,
          p.1 * acos - p.0 * asin + position.1,
        );
        r.push(p);
      }
      routes.push((clr, r));
    }
    routes
  }

  fn render_band(
    &self,
    clr: usize,
    from: (f64, f64),
    to: (f64, f64),
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let l = euclidian_dist(from, to);
    let untranslated = self.pattern(clr, l, bandw);
    // rotate & translate
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let a = -dy.atan2(dx);
    let acos = a.cos();
    let asin = a.sin();
    let mut routes = vec![];
    for (clr, route) in untranslated {
      let mut r = vec![];
      for &p in route.iter() {
        let p = (
          p.0 * acos + p.1 * asin + from.0,
          p.1 * acos - p.0 * asin + from.1,
        );
        r.push(p);
      }
      routes.push((clr, r));
    }
    routes
  }
}
