use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "2")]
  pub divx: usize,
  #[clap(short, long, default_value = "3")]
  pub divy: usize,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "10.0")]
  pub padin: f64,
  #[clap(short, long, default_value = "4.0")]
  pub margout: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "15000")]
  pub iterations: usize,
}

#[derive(Clone)]
struct Value2D {
  width: f64,
  height: f64,
  ang1: f64,
  ang2: f64,
  ang3: f64,
  f1: f64,
  f2: f64,
  f3: f64,
  amp2: f64,
  linearf: f64,
  noisefactor: f64,
  noisebalance: f64,
  yfactor: f64,
  seed: f64,
  count: usize,
  symx: bool,
  symy: bool,
  rotation: f64,
}
impl Value2D {
  fn map(&self, x: f64, y: f64) -> f64 {
    let ratio = self.width / self.height;
    let ang1 = self.ang1;
    let ang2 = self.ang2;
    let ang3 = self.ang3;
    let f1 = self.f1;
    let f2 = self.f2;
    let f3 = self.f3;
    let amp2 = self.amp2;
    let noisefactor = self.noisefactor;
    let noisebalance = self.noisebalance;
    let yfactor = self.yfactor;
    let seed = self.seed;
    let linearf = self.linearf;
    let symx = self.symx;
    let symy = self.symy;
    let rot = self.rotation;

    let (x, y) = p_r((ratio * (x - 0.5), y - 0.5), rot);
    let x = x + 0.5;
    let y = y + 0.5;

    let x = if symx { (1. - x).min(x) } else { x };
    let y = if symy { (1. - y).min(y) } else { y };

    let perlin = Perlin::new();

    let mut q = p_r(((x - 0.5), y - 0.5), ang1);
    q.0 += 0.5;
    q.1 += 0.5;
    let mut p = p_r(((x - 0.5), y - 0.5), ang2);
    p.0 += 0.5;
    p.1 += 0.5;
    let mut r = p_r(((x - 0.5), y - 0.5), ang3);
    r.0 += 0.5;
    r.1 += 0.5;

    let n = 0.5
      + noisefactor
        * (noisebalance
          * ((1. - amp2)
            * perlin.get([
              f1 * p.0,
              f1 * p.1,
              seed + 3.0 * perlin.get([seed * 5.3, f2 * p.0, f2 * p.1]),
            ])
            + amp2 * perlin.get([f2 * q.0, f2 * q.1, seed / 0.0364]))
          + (1. - noisebalance)
            * perlin.get([f3 * r.0, f3 * r.1, seed * 3.873]));
    (n + linearf * (mix(x, y, yfactor) - 0.5)).max(0.0).min(1.0)
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let divx = opts.divx;
  let divy = opts.divy;

  let black = Ink("Black", "#111", "#222", 0.35);
  let moonstone = Ink("Moonstone", "#666", "#888", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let turquoise = Ink("Turquoise", "#00B4E6", "#005A8C", 0.35);
  //let poppy_red = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
  //let spring_green = Ink("Spring Green", "#7d9900", "#6c6b00", 0.35);

  let inks = vec![black, amber, pink, soft_mint, turquoise, moonstone, black];
  let mut rng = rng_from_seed(opts.seed);

  let precision = 0.2;
  let mut paint = PaintMask::new(precision, width, height);
  paint.paint_borders(pad);

  let mut frames = vec![];
  let m = opts.margout;
  let p = opts.padin;
  let dx: f64 = (width - 2.0 * pad) / (divx as f64);
  let dy: f64 = (height - 2.0 * pad) / (divy as f64);
  for x in 0..divx {
    for y in 0..divy {
      let xmin = x as f64 * dx + pad + m;
      let ymin = y as f64 * dy + pad + m;
      let xmax = (x + 1) as f64 * dx + pad - m;
      let ymax = (y + 1) as f64 * dy + pad - m;
      let (pattern, strokew): (Box<dyn BandPattern>, f64) =
        match (x + y * divx) % 6 {
          0 => (Box::new(MedievalBandLRectPattern::new()), 0.08 * p),
          1 => (
            Box::new(MedievalBandFeatherTrianglePattern::new()),
            0.06 * p,
          ),
          2 => (Box::new(MedievalBandForkPattern::new()), 0.06 * p),
          3 => (Box::new(MedievalBandComb::new()), 0.04 * p),
          4 => (Box::new(MedievalBandCurvePattern::new()), 0.04 * p),
          _ => (Box::new(MedievalBandConcentric::new(2)), 0.08 * p),
        };
      frames.extend(framing(
        &mut rng,
        &mut paint,
        0,
        (xmin, ymin, xmax, ymax),
        pattern.as_ref(),
        p,
        m,
        strokew,
        3.0,
        20000,
      ));
    }
  }

  let f1 = rng.gen_range(1.0, 8.0);
  let f2 = rng.gen_range(1.0, 8.0);
  let f3 = rng.gen_range(1.0, 8.0);

  let amp2 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

  let linearf = rng.gen_range(0.2, 0.8);
  let noisefactor = rng.gen_range(0.1, 0.5);
  let noisebalance = rng.gen_range(0.1, 0.5);
  let ang1 = if rng.gen_bool(0.5) {
    rng.gen_range(-PI, PI)
  } else {
    0.0
  };
  let ang2 = if rng.gen_bool(0.5) {
    rng.gen_range(-PI, PI)
  } else {
    ang1
  };
  let ang3 = if rng.gen_bool(0.5) {
    rng.gen_range(-PI, PI)
  } else {
    ang2
  };
  let rotation = rng.gen_range(-PI, PI) * rng.gen_range(-1.0f64, 1.0).max(0.0);

  let mut values = vec![];

  let count = rng.gen_range(2, 5);

  for i in 0..count {
    let yfactor = (2.0 * (i as f64) / (count as f64)) % 2.0;
    let count = rng.gen_range(10, 30);

    let valuef = Value2D {
      width,
      height,
      ang1,
      ang2,
      ang3,
      f1,
      f2,
      f3,
      amp2,
      linearf,
      noisefactor,
      noisebalance,
      yfactor,
      seed: opts.seed,
      count,
      symx: rng.gen_bool(0.5),
      symy: rng.gen_bool(0.5),
      rotation,
    };
    values.push(valuef.clone());
  }

  let density = 2.0;

  let mmod = 2 + (rng.gen_range(0., 10.) * rng.gen_range(0., 1.)) as usize;

  let coloring = |route: &Vec<(f64, f64)>| {
    let p = route[0];
    let xi = (p.0 / width * (divx as f64)).floor() as usize;
    let yi = (p.1 / height * (divy as f64)).floor() as usize;
    return (xi + yi * divx) % (inks.len() - 1) + 1;
  };

  let filling = WormsFilling::rand(&mut rng);
  let mut fills = vec![];
  fills.extend(filling.fill(
    &mut rng,
    &|x, y| {
      let mut i = 0;
      for f in values.iter() {
        let v = f.map(x / width, y / height);
        let i1 = (v * (f.count as f64 + 1.)) as usize;
        i += i1;
      }
      if i % mmod != 0 {
        density
      } else {
        0.0
      }
    },
    (0., 0., width, height),
    &coloring,
    opts.iterations,
  ));
  fills = regular_clip(&fills, &mut paint);

  let routes = vec![frames, fills].concat();

  inks
    .iter()
    .enumerate()
    .map(|(ci, &ink)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(ink.0)).as_str());
      l = l.add(base_path(ink.1, ink.3, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes_with_colors(
  input_routes: &Vec<(usize, Vec<(f64, f64)>)>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  // locate the intersection where inside and outside cross
  let search = |inside: (f64, f64), outside: (f64, f64), n| {
    let mut a = inside;
    let mut b = outside;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if is_outside(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for (clrp, input_route) in input_routes.iter() {
    let clr = *clrp;
    if input_route.len() < 2 {
      continue;
    }

    let mut prev = input_route[0];
    let mut prev_is_outside = is_outside(prev);
    let mut route = vec![];
    if !prev_is_outside {
      // prev is not to crop. we can start with it
      route.push(prev);
    }

    for &p in input_route.iter().skip(1) {
      // we iterate in small steps to detect any interruption
      let static_prev = prev;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let d = (dx * dx + dy * dy).sqrt();
      let vx = dx / d;
      let vy = dy / d;
      let iterations = (d / stepping).ceil() as usize;
      let mut v = 0.0;
      for _i in 0..iterations {
        v = (v + stepping).min(d);
        let q = (static_prev.0 + vx * v, static_prev.1 + vy * v);
        let q_is_outside = is_outside(q);
        if prev_is_outside != q_is_outside {
          // we have a crossing. we search it precisely
          let intersection = if prev_is_outside {
            search(q, prev, dichotomic_iterations)
          } else {
            search(prev, q, dichotomic_iterations)
          };

          if q_is_outside {
            // we close the path
            route.push(intersection);
            if route.len() > 1 {
              // we have a valid route to accumulate
              routes.push((clr, route));
            }
            route = vec![];
          } else {
            // we open the path
            route.push(intersection);
          }
          prev_is_outside = q_is_outside;
        }

        prev = q;
      }

      // prev should be == p
      if !prev_is_outside {
        // prev is not to crop. we can start with it
        route.push(p);
      }
    }

    if route.len() > 1 {
      // we have a valid route to accumulate
      routes.push((clr, route));
    }
  }

  routes
}

#[derive(Clone)]
pub struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
}

impl PaintMask {
  fn clone_empty(&self) -> Self {
    Self {
      mask: vec![false; self.mask.len()],
      precision: self.precision,
      width: self.width,
      height: self.height,
    }
  }

  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    }
  }

  fn is_painted(&self, point: (f64, f64)) -> bool {
    let precision = self.precision;
    let wi = (self.width / precision) as usize;
    let hi = (self.height / precision) as usize;
    let x = ((point.0.max(0.) / precision) as usize).min(wi - 1);
    let y = ((point.1.max(0.) / precision) as usize).min(hi - 1);
    self.mask[x + y * wi]
  }

  fn paint_rectangle(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        self.mask[x + y * wi] = true;
      }
    }
  }

  fn paint_borders(&mut self, pad: f64) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
  }

  /*
  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision).floor() as usize;
    let miny = ((miny).max(0.).min(self.height) / precision).floor() as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision).ceil() as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision).ceil() as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  fn paint_circle(&mut self, cx: f64, cy: f64, cr: f64) {
    let (minx, miny, maxx, maxy) = (
      (cx - cr).max(0.),
      (cy - cr).max(0.),
      (cx + cr).min(self.width),
      (cy + cr).min(self.height),
    );
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if euclidian_dist(point, (cx, cy)) < cr {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  fn paint_segment(&mut self, a: (f64, f64), b: (f64, f64), strokew: f64) {
    let (minx, miny, maxx, maxy) = (
      (a.0.min(b.0) - strokew).max(0.),
      (a.1.min(b.1) - strokew).max(0.),
      (a.0.max(b.0) + strokew).min(self.width),
      (a.1.max(b.1) + strokew).min(self.height),
    );
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if sd_segment(point, a, b) < strokew {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
  */

  fn paint_polyline(&mut self, polyline: &Vec<(f64, f64)>, strokew: f64) {
    if polyline.len() < 1 {
      return;
    }
    let first = polyline[0];
    let mut minx = first.0;
    let mut miny = first.1;
    let mut maxx = first.0;
    let mut maxy = first.1;
    for p in polyline.iter().skip(1) {
      minx = minx.min(p.0);
      miny = miny.min(p.1);
      maxx = maxx.max(p.0);
      maxy = maxy.max(p.1);
    }
    minx = (minx - strokew).max(0.0);
    miny = (miny - strokew).max(0.0);
    maxx = (maxx + strokew).min(self.width);
    maxy = (maxy + strokew).min(self.height);

    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        for i in 0..polyline.len() - 1 {
          let a = polyline[i];
          let b = polyline[i + 1];
          if sd_segment(point, a, b) < strokew {
            self.mask[x + y * wi] = true;
            break;
          }
        }
      }
    }
  }
}

// TODO we can optim something as we just need a "point_in_segment"

fn sd_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
) -> f64 {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;

  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let h = (dot_pa_ba / dot_ba_ba).max(0.0).min(1.0);

  let h_x = ba_x * h;
  let h_y = ba_y * h;

  ((pa_x - h_x) * (pa_x - h_x) + (pa_y - h_y) * (pa_y - h_y)).sqrt()
}

fn regular_clip(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let is_outside = |p| paint.is_painted(p);
  clip_routes_with_colors(&routes, &is_outside, 0.5, 3)
}

/*
fn polygon_bounds(polygon: &Vec<(f64, f64)>) -> (f64, f64, f64, f64) {
  let mut minx = f64::MAX;
  let mut miny = f64::MAX;
  let mut maxx = f64::MIN;
  let mut maxy = f64::MIN;
  for &(x, y) in polygon {
    minx = minx.min(x);
    miny = miny.min(y);
    maxx = maxx.max(x);
    maxy = maxy.max(y);
  }
  (minx, miny, maxx, maxy)
}

fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  point: (f64, f64),
) -> bool {
  let mut c = false;
  for i in 0..polygon.len() {
    let j = (i + 1) % polygon.len();
    if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
      && (point.0
        < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
          / (polygon[j].1 - polygon[i].1)
          + polygon[i].0)
    {
      c = !c;
    }
  }
  c
}

fn regular_clip_polys(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
  polys: &Vec<Vec<(f64, f64)>>,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let rts = regular_clip(routes, paint);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }
  rts
}
*/

trait BandPattern {
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
struct MedievalBandLRectPattern {
  cellw: f64,
  padx: f64,
  pady: f64,
  offx: f64,
}
impl MedievalBandLRectPattern {
  fn new() -> Self {
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
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;
    let padx = self.padx * cellw;
    let pady = self.pady * cellw;
    let offx = self.offx * cellw;

    let l = length + 2.0 * padx;

    // round the cellw to make the exact length
    let n = (l / cellw).round() as usize;
    let cellw = l / (n as f64);

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

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    let cellw = self.cellw * bandw;
    let pady = self.pady * cellw;
    let d = bandw / 2.0 - pady;
    vec![(clr, vec![(-d, -d), (d, -d), (d, d), (-d, d), (-d, -d)])]
  }
}

struct MedievalBandFeatherTrianglePattern {
  cellw: f64,
  feather_ratio: f64,
  count1: usize,
  count2: usize,
}
impl MedievalBandFeatherTrianglePattern {
  fn new() -> Self {
    Self {
      cellw: 6.0,
      count1: 3,
      count2: 3,
      feather_ratio: 0.66,
    }
  }

  fn feather(
    &self,
    clr: usize,
    a: (f64, f64),
    b: (f64, f64),
    c: (f64, f64),
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    // TODO add a pad
    let mut routes = vec![]; // array of (clr, path)
    let count1 = self.count1;
    let count2 = self.count2;
    for i in 0..count1 {
      let t = ((i + 1) as f64 / (count1 + 1) as f64) * self.feather_ratio;
      let p = lerp_point(a, b, t);
      let q = lerp_point(a, c, t);
      routes.push((clr, vec![p, q]));
    }
    for i in 0..count2 {
      let t = (i as f64 + 1.0) / (count2 + 1) as f64;
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
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;

    // round the cellw to make the exact length
    let n = (length / cellw).round() as usize;
    let cellw = length / (n as f64);

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

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
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

struct MedievalBandForkPattern {
  cellw: f64,
  cutx: f64,
  spacex: f64,
  pady: f64,
  simplecorner: bool,
}
impl MedievalBandForkPattern {
  fn new() -> Self {
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
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
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
    let cellw = l / (n as f64);

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

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
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

struct MedievalBandConcentric {
  count: usize,
}
impl MedievalBandConcentric {
  fn new(count: usize) -> Self {
    Self { count }
  }
}
impl BandPattern for MedievalBandConcentric {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    for i in 0..self.count {
      let y =
        (i as f64 + 1.0) / (self.count as f64 + 1.0) * (2.0 * bandw) - bandw;
      routes.push((clr, vec![(0.0, y), (length, y)]));
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    for i in 0..self.count {
      let y =
        (i as f64 + 1.0) / (self.count as f64 + 1.0) * (2.0 * bandw) - bandw;
      routes.push((clr, vec![(y, bandw), (y, y), (bandw, y)]));
    }
    routes
  }
}

struct MedievalBandCurvePattern {
  xrep: f64,
  amp: f64,
  inner: f64,
  alt: bool,
}
impl MedievalBandCurvePattern {
  fn new() -> Self {
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
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let xrep = self.xrep * bandw;
    // round the cellw to make the exact length
    let n = (length / xrep).round() as usize;
    let xrep = length / (n as f64);

    let amp = self.amp * bandw;

    let precision = 0.2;

    let mut curve1 = vec![];
    let mut curve2 = vec![];
    let mut p = 0.0;
    while p < length {
      let phase = 2.0 * PI * p / xrep;
      if self.alt {
        curve1.push((p, amp * phase.sin()));
        curve2.push((p, amp * (phase + PI).sin()));
      } else {
        curve1.push((p, amp * phase.cos()));
        curve2.push((p, amp * (phase + PI).cos()));
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

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
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

struct MedievalBandComb {
  cellw: f64,
  twistx: f64,
  pady: f64,
  ysplits: usize,
  comblength: f64,
}
impl MedievalBandComb {
  fn new() -> Self {
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
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;
    let twistx = self.twistx * cellw;
    let pady = self.pady * bandw;
    let ysplits = self.ysplits;
    let comblength = self.comblength * cellw;

    // round the cellw to make the exact length
    let n = (length / cellw).round() as usize;
    let cellw = length / (n as f64);

    let mut p = 0.0;
    for _i in 0..(n + 1) {
      let dy = bandw;
      let maxp = (p + twistx).min(length);
      routes.push((clr, vec![(p, -dy), (maxp, dy)]));
      for j in 0..ysplits {
        let y =
          ((j as f64 + 0.5) / (ysplits as f64) - 0.5) * (2.0 * (bandw - pady));
        let x = mix(p, p + twistx, (y + bandw) / (2.0 * bandw));
        routes.push((clr, vec![(x.min(length), y), (x - comblength, y)]));
      }
      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let pady = self.pady * bandw;
    let ysplits = self.ysplits;

    for j in 0..ysplits {
      let y =
        ((j as f64 + 0.5) / (ysplits as f64) - 0.5) * (2.0 * (bandw - pady));
      routes.push((clr, vec![(-bandw, y), (bandw, y)]));
    }
    routes
  }
}

fn framing<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  clr: usize,
  bound: (f64, f64, f64, f64),
  // pattern that will be colored for the framing
  pattern: &dyn BandPattern,
  // padding inside the frame
  padding: f64,
  // marging to exclude external
  margin: f64,
  // stroke width for the pattern
  strokew: f64,
  // density of the coloring
  density: f64,
  // nb of iteration of coloring logic
  iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  // outer
  routes.push((
    clr,
    vec![
      (bound.0 + strokew, bound.1 + strokew),
      (bound.2 - strokew, bound.1 + strokew),
      (bound.2 - strokew, bound.3 - strokew),
      (bound.0 + strokew, bound.3 - strokew),
      (bound.0 + strokew, bound.1 + strokew),
    ],
  ));
  // inner
  routes.push((
    clr,
    vec![
      (bound.0 + padding - strokew, bound.1 + padding - strokew),
      (bound.2 - padding + strokew, bound.1 + padding - strokew),
      (bound.2 - padding + strokew, bound.3 - padding + strokew),
      (bound.0 + padding - strokew, bound.3 - padding + strokew),
      (bound.0 + padding - strokew, bound.1 + padding - strokew),
    ],
  ));

  let hp = padding / 2.;
  let bandw = hp - strokew;

  // top
  routes.extend(pattern.render_band(
    clr,
    (bound.0 + padding, bound.1 + hp),
    (bound.2 - padding, bound.1 + hp),
    bandw,
  ));
  // topleft
  routes.extend(pattern.render_corner(
    clr,
    (bound.0 + hp, bound.1 + hp),
    0.0,
    bandw,
  ));

  // right
  routes.extend(pattern.render_band(
    clr,
    (bound.2 - hp, bound.1 + padding),
    (bound.2 - hp, bound.3 - padding),
    bandw,
  ));
  // topright
  routes.extend(pattern.render_corner(
    clr,
    (bound.2 - hp, bound.1 + hp),
    -0.5 * PI,
    bandw,
  ));

  // bottom
  routes.extend(pattern.render_band(
    clr,
    (bound.2 - padding, bound.3 - hp),
    (bound.0 + padding, bound.3 - hp),
    bandw,
  ));
  // bottomright
  routes.extend(pattern.render_corner(
    clr,
    (bound.2 - hp, bound.3 - hp),
    -PI,
    bandw,
  ));

  // left
  routes.extend(pattern.render_band(
    clr,
    (bound.0 + hp, bound.3 - padding),
    (bound.0 + hp, bound.1 + padding),
    bandw,
  ));
  // bottomleft
  routes.extend(pattern.render_corner(
    clr,
    (bound.0 + hp, bound.3 - hp),
    -1.5 * PI,
    bandw,
  ));

  // strokes -> fill -> strokes. will create nice textures!
  let mut drawings = paint.clone_empty();
  for (_clr, route) in routes.iter() {
    drawings.paint_polyline(route, strokew);
  }
  let filling = WormsFilling::rand(rng);
  let routes =
    filling.fill_in_paint(rng, &drawings, clr, density, bound, iterations);

  // we paint the mask for the paint to include our frame.

  // left
  paint.paint_rectangle(
    bound.0 - margin,
    bound.1 - margin,
    bound.0 + padding,
    bound.3 + margin,
  );
  // right
  paint.paint_rectangle(
    bound.2 - padding,
    bound.1 - margin,
    bound.2 + margin,
    bound.3 + margin,
  );
  // top
  paint.paint_rectangle(
    bound.0 - margin,
    bound.1 - margin,
    bound.2 + margin,
    bound.1 + padding,
  );
  // bottom
  paint.paint_rectangle(
    bound.0 - margin,
    bound.3 - padding,
    bound.2 + margin,
    bound.3 + margin,
  );

  routes
}

// homemade implementation of a filling technique that will spawn random worms that eat the space to colorize it!
struct WormsFilling {
  rot: f64,
  step: f64,
  straight: f64,
  min_l: usize,
  max_l: usize,
  decrease_value: f64,
  search_max: usize,
  min_weight: f64,
  freq: f64,
  seed: f64,
}
impl WormsFilling {
  // new
  fn rand<R: Rng>(rng: &mut R) -> Self {
    let seed = rng.gen_range(-999., 999.);
    let rot = PI / rng.gen_range(1.0, 2.0);
    let step = 0.4;
    let straight = rng.gen_range(0.0, 0.1);
    let min_l = 5;
    let max_l = 20;
    let decrease_value = 1.;
    let search_max = 500;
    let min_weight = 1.;
    let freq = 0.05;
    Self {
      rot,
      step,
      straight,
      min_l,
      max_l,
      decrease_value,
      search_max,
      min_weight,
      freq,
      seed,
    }
  }

  fn fill_in_paint<R: Rng>(
    &self,
    rng: &mut R,
    drawings: &PaintMask,
    clr: usize,
    density: f64,
    bound: (f64, f64, f64, f64),
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let f = |x, y| {
      if drawings.is_painted((x, y)) {
        density
      } else {
        0.0
      }
    };
    let coloring = |_: &Vec<(f64, f64)>| clr;
    self.fill(rng, &f, bound, &coloring, iterations)
  }

  fn fill<R: Rng>(
    &self,
    rng: &mut R,
    f: &dyn Fn(f64, f64) -> f64,
    bound: (f64, f64, f64, f64),
    clr: &dyn Fn(&Vec<(f64, f64)>) -> usize,
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let perlin = Perlin::new();
    let w = bound.2 - bound.0;
    let h = bound.3 - bound.1;
    let precision = 0.4;
    if w <= 2. * precision || h <= 2. * precision {
      return routes;
    }
    let mut map = WeightMap::new(w, h, 0.4);

    map.fill_fn(&|p| f(p.0 + bound.0, p.1 + bound.1));

    let seed = self.seed;
    let rot = self.rot;
    let step = self.step;
    let straight = self.straight;
    let min_l = self.min_l;
    let max_l = self.max_l;
    let decrease_value = self.decrease_value;
    let search_max = self.search_max;
    let min_weight = self.min_weight;
    let freq = self.freq;

    let mut bail_out = 0;

    for _i in 0..iterations {
      let top = map.search_weight_top(rng, search_max, min_weight);
      if top.is_none() {
        bail_out += 1;
        if bail_out > 10 {
          break;
        }
      }
      if let Some(o) = top {
        let angle = perlin.get([seed, freq * o.0, freq * o.1]);

        if let Some(a) = map.best_direction(o, step, angle, PI, PI / 4.0, 0.0) {
          let route = map.dig_random_route(
            o,
            a,
            step,
            rot,
            straight,
            max_l,
            decrease_value,
          );
          if route.len() >= min_l {
            let points: Vec<(f64, f64)> = rdp(&route, 0.05);
            // remap
            let rt = points
              .iter()
              .map(|&p| (p.0 + bound.0, p.1 + bound.1))
              .collect::<Vec<_>>();
            let c = clr(&rt);
            routes.push((c, rt));
          }
        }
      }
    }

    routes
  }
}

// data model that stores values information in 2D
struct WeightMap {
  weights: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl WeightMap {
  fn new(width: f64, height: f64, precision: f64) -> WeightMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let weights = vec![0.0; w * h];
    WeightMap {
      weights,
      w,
      h,
      width,
      height,
      precision,
    }
  }
  fn fill_fn(&mut self, f: &impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.weights[y * self.w + x] = v;
      }
    }
  }

  // do a simple bilinear interpolation
  fn get_weight(&self, p: (f64, f64)) -> f64 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let w00 = self.weights[y0 * self.w + x0];
    let w01 = self.weights[y0 * self.w + x1];
    let w10 = self.weights[y1 * self.w + x0];
    let w11 = self.weights[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the weights around the point p with a given radius
  fn decrease_weight_gaussian(
    &mut self,
    p: (f64, f64),
    radius: f64,
    value: f64,
  ) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = ((x - radius).floor().max(0.) as usize).min(self.w);
    let y0 = ((y - radius).floor().max(0.) as usize).min(self.h);
    let x1 = ((x + radius).ceil().max(0.) as usize).min(self.w);
    let y1 = ((y + radius).ceil().max(0.) as usize).min(self.h);
    if x0 >= self.w || y0 >= self.h {
      return;
    }
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let d = (p.0 - p.0).hypot(p.1 - p.1);
        if d < radius {
          let w = self.weights[y * self.w + x];
          let v = value * (1.0 - d / radius);
          self.weights[y * self.w + x] = w - v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  fn best_direction(
    &self,
    p: (f64, f64),
    step: f64,
    angle: f64,
    max_ang_rotation: f64,
    angle_precision: f64,
    straight_factor: f64,
  ) -> Option<f64> {
    let mut best_ang = None;
    let mut best_weight = 0.0;
    let mut a = -max_ang_rotation;
    while a < max_ang_rotation {
      let ang = a + angle;
      let dx = step * ang.cos();
      let dy = step * ang.sin();
      let np = (p.0 + dx, p.1 + dy);
      if np.0 < 0.0 || np.0 > self.width || np.1 < 0.0 || np.1 > self.height {
        a += angle_precision;
        continue;
      }
      // more important when a is near 0.0 depending on straight factor
      let wmul = (1.0 - straight_factor)
        + (1.0 - a.abs() / max_ang_rotation) * straight_factor;
      let weight = self.get_weight(np) * wmul;
      if weight > best_weight {
        best_weight = weight;
        best_ang = Some(ang);
      }
      a += angle_precision;
    }
    return best_ang;
  }

  // FIXME we could optim this by keeping track of tops and not searching too random
  fn search_weight_top<R: Rng>(
    &mut self,
    rng: &mut R,
    search_max: usize,
    min_weight: f64,
  ) -> Option<(f64, f64)> {
    let mut best_w = min_weight;
    let mut best_p = None;
    for _i in 0..search_max {
      let x = rng.gen_range(0.0, self.width);
      let y = rng.gen_range(0.0, self.height);
      let p = (x, y);
      let w = self.get_weight(p);
      if w > best_w {
        best_w = w;
        best_p = Some(p);
      }
    }
    return best_p;
  }

  fn dig_random_route(
    &mut self,
    origin: (f64, f64),
    initial_angle: f64,
    step: f64,
    max_ang_rotation: f64,
    straight_factor: f64,
    max_length: usize,
    decrease_value: f64,
  ) -> Vec<(f64, f64)> {
    let mut route = Vec::new();
    let mut p = origin;
    let mut angle = initial_angle;
    for _i in 0..max_length {
      if let Some(ang) = self.best_direction(
        p,
        step,
        angle,
        max_ang_rotation,
        0.2 * max_ang_rotation,
        straight_factor,
      ) {
        angle = ang;
        let prev = p;
        p = (p.0 + step * angle.cos(), p.1 + step * angle.sin());
        route.push(p);
        self.decrease_weight_gaussian(prev, step, decrease_value);
      } else {
        break;
      }
    }

    route
  }
}

#[derive(Clone, Copy)]
pub struct Ink(&'static str, &'static str, &'static str, f64);
#[derive(Clone, Copy)]
pub struct Paper(&'static str, &'static str, bool);
