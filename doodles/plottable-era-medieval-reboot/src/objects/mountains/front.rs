use crate::{
  algo::{clipping::clip_routes_with_colors, paintmask::PaintMask},
  objects::palmtree::PalmTree,
};
use noise::*;
use rand::prelude::*;

pub struct FrontMountains {
  pub clr: usize,
  pub ybase: f32,
  pub ystart: f32,
  pub width: f32,
}

impl FrontMountains {
  pub fn render<R: Rng>(
    &mut self,
    rng: &mut R,
    paint: &mut PaintMask,
  ) -> Vec<(usize, Vec<(f32, f32)>)> {
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
      let divmin = count as f32 * 0.3;
      let divmax = count as f32 * 0.6;
      let yamp = ((i as f32 + 1.0) * h / rng.gen_range(divmin..divmax)).min(h);

      let f1 = rng.gen_range(0.01..0.03) * rng.gen_range(0.0..1.0);
      let amp2 = rng.gen_range(0.0..2.0) * rng.gen_range(0.0..1.0);
      let f2 = rng.gen_range(0.0..0.05) * rng.gen_range(0.0..1.0);
      let amp3 = rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
      let f3 = rng.gen_range(0.0..0.1) * rng.gen_range(0.0..1.0);
      let seed1 = rng.gen_range(0.0..100.0);
      let seed2 = rng.gen_range(0.0..100.0);
      let seed3 = rng.gen_range(0.0..100.0);

      let valuef = |x32, y32| {
        let x = x32 as f64;
        let y = y32 as f64;
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
            ]) as f32;
        n
      };

      let (rts, curve) =
        stroke_mountains(paint, 0.0, width, xincr, y, yamp, &valuef, clr);

      routes.extend(rts);
      curves.push(curve);
    }

    let mut palm_tree_candidates = vec![];
    let palm_mod = 10;
    let palm_pad = width * 0.05;
    let palm_ythreshold = paint.height * 0.9;

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
      if max < palm_ythreshold
        && i % palm_mod == 0
        && palm_pad < p.0
        && p.0 < width - palm_pad
        && rng.gen_bool(0.2)
      {
        palm_tree_candidates.push((p.0, max));
      }
    }

    for o in palm_tree_candidates {
      let tree = PalmTree {
        origin: o,
        size: rng.gen_range(0.02..0.04) * paint.height,
      };
      routes.extend(tree.render(rng, paint));
    }

    routes
  }
}

fn stroke_mountains(
  paint: &mut PaintMask,
  xfrom: f32,
  xto: f32,
  xincr: f32,
  ybase: f32,
  yamp: f32,
  valuef: &dyn Fn(f32, f32) -> f32,
  clr: usize,
) -> (Vec<(usize, Vec<(f32, f32)>)>, Vec<(f32, f32)>) {
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

  // TODO rework the implementation

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
