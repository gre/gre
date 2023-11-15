use crate::algo::{clipping::clip_routes_with_colors, paintmask::PaintMask};
use noise::*;
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Mountains {
  pub clr: usize,
  pub ybase: f64,
  pub ystart: f64,
  pub width: f64,
  pub ridge: Option<Vec<(f64, f64)>>,
}

impl Mountains {
  pub fn init(clr: usize, ybase: f64, ystart: f64, width: f64) -> Self {
    Self {
      clr,
      ybase,
      ystart,
      width,
      ridge: None,
    }
  }

  pub fn ridge(&self) -> Vec<(f64, f64)> {
    self.ridge.clone().unwrap()
  }

  pub fn render<R: Rng>(
    &mut self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let clr = self.clr;
    let ybase = self.ybase;
    let ystart = self.ystart;
    let width = self.width;

    // TODO rework the implementation
    // TODO we may split the idea of front vs back mountains

    let mut routes = vec![];
    let perlin = Perlin::new(rng.gen());
    // mini mountains
    let count = rng.gen_range(2..12);
    let h = ybase - ystart;
    let xincr = 1.0;

    let mut curves = vec![];

    for i in 0..count {
      let y = ybase;
      let divmin = count as f64 * 0.3;
      let divmax = count as f64 * 0.6;
      let yamp = (i as f64 + 1.0) * h / rng.gen_range(divmin..divmax);

      let f1 = rng.gen_range(0.01..0.03) * rng.gen_range(0.0..1.0);
      let amp2 = rng.gen_range(0.0..2.0) * rng.gen_range(0.0..1.0);
      let f2 = rng.gen_range(0.0..0.05) * rng.gen_range(0.0..1.0);
      let amp3 = rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
      let f3 = rng.gen_range(0.0..0.1) * rng.gen_range(0.0..1.0);
      let seed1 = rng.gen_range(0.0..100.0);
      let seed2 = rng.gen_range(0.0..100.0);
      let seed3 = rng.gen_range(0.0..100.0);

      let valuef = |x, y| {
        let n = 0.5
          + 0.5
            * perlin.get([
              f1 * x,
              f1 * y,
              amp2
                * perlin.get([
                  f2 * x,
                  seed2 + amp3 * perlin.get([seed3, f3 * x, f3 * y]),
                  f2 * y,
                ])
                + seed1
                + i as f64 * 55.5,
            ]);
        n
      };

      let (rts, curve) =
        stroke_mountains(paint, 0.0, width, xincr, y, yamp, &valuef, clr);

      routes.extend(rts);
      curves.push(curve);
    }

    let mut ridge = vec![];
    let first = curves[0].clone();
    let len = first.len();
    for i in 0..len {
      let p = first[i];
      let mut max = p.1;
      for curve in curves.iter().skip(1) {
        let y = curve[i].1;
        if y < max {
          max = y;
        }
      }
      ridge.push((p.0, max));
    }
    self.ridge = Some(ridge);

    routes
  }
}

fn stroke_mountains(
  paint: &mut PaintMask,
  xfrom: f64,
  xto: f64,
  xincr: f64,
  ybase: f64,
  yamp: f64,
  valuef: &dyn Fn(f64, f64) -> f64,
  clr: usize,
) -> (Vec<(usize, Vec<(f64, f64)>)>, Vec<(f64, f64)>) {
  let mut routes = vec![];

  // sample the curve with f
  let mut curve = vec![];
  let mut x = xfrom;
  while x < xto {
    let y = ybase - yamp * valuef(x, ybase);
    curve.push((x, y));
    x += xincr;
  }
  if x > xto {
    let y = ybase - yamp * valuef(xto, ybase);
    curve.push((xto, y));
  }

  if curve.len() < 2 {
    return (routes, curve);
  }

  // make the polygons
  let mut polys = vec![];
  let len = curve.len();
  for j in 1..len {
    let i = j - 1;
    let mut poly = vec![];
    let a = curve[i];
    let b = curve[j];
    poly.push(a);
    poly.push(b);
    poly.push((b.0, ybase));
    poly.push((a.0, ybase));
    polys.push(poly);
  }

  routes.push((clr, curve.clone()));

  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }
  (routes, curve.clone())
}
