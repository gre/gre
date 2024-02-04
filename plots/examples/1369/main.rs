use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  height: f64,
  #[clap(short, long, default_value = "210.0")]
  width: f64,
  #[clap(short, long, default_value = "10.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn flower<R: Rng>(
  rng: &mut R,
  (ox, oy): (f64, f64),
  r: f64,
  count: usize,
  clr1: usize,
  clr2: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let s = r / 2.;
  routes.push((clr2, spiral_optimized(ox, oy, s, 0.4, 0.01)));
  routes.push((clr1, circle_route((ox, oy), s, 40)));
  let angdiff = rng.gen_range(0.0, PI);
  for i in 0..count {
    let p = i as f64 / (count as f64) + angdiff;
    let a = p * 2. * PI;
    let s = rng.gen_range(0.65, 0.75) * s;
    let x = ox + r * a.cos();
    let y = oy + r * a.sin();
    routes.push((clr1, spiral_optimized(x, y, s, 0.4, 0.01)));
    routes.push((clr1, circle_route((x, y), s, 40)));
  }
  routes
}

fn flower2<R: Rng>(
  rng: &mut R,
  (ox, oy): (f64, f64),
  r: f64,
  count: usize,
  clr1: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let s = r / 2.;

  routes.push((clr1, circle_route((ox, oy), s, 40)));

  let sz = 0.25 * r;
  routes.push((
    clr1,
    vec![
      (ox - sz * rng.gen_range(0.5, 1.0), oy - sz),
      (ox + sz, oy + sz),
    ],
  ));
  routes.push((
    clr1,
    vec![
      (ox + sz, oy - sz),
      (ox - sz, oy + sz * rng.gen_range(0.5, 1.0)),
    ],
  ));

  let angdiff = rng.gen_range(0.0, PI);
  for i in 0..count {
    let p = i as f64 / (count as f64) + angdiff;
    let a = p * 2. * PI;
    let s = 0.35 * s;
    let x = ox + 0.8 * r * a.cos();
    let y = oy + 0.8 * r * a.sin();
    routes.push((clr1, spiral_optimized(x, y, s, 0.5, 0.02)));
    routes.push((clr1, circle_route((x, y), s, 40)));
  }
  routes
}

fn polylines_smooth_union_filled(
  lines: &Vec<Vec<(f64, f64)>>,
  clr: usize,
  linew: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let mut circles = vec![];
  let r = 0.5 * linew;
  for line in lines {
    for p in step_polyline(line, (0.4 * linew).max(0.4)) {
      routes.extend(clip_routes_with_colors(
        &vec![(clr, circle_route(p, linew / 2.0, 8))],
        &|q| {
          circles
            .iter()
            .any(|c: &VCircle| euclidian_dist((c.x, c.y), q) < c.r)
        },
        0.5,
        4,
      ));
      circles.push(VCircle::new(p.0, p.1, r));
    }
  }
  routes
}

fn leaf<R: Rng>(
  rng: &mut R,
  (ox, oy): (f64, f64),
  amp: f64,
  rot: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut lines = vec![];
  lines.push(vec![(0.0, -0.1), (0.0, -1.0)]);
  lines.push(vec![(0.0, -0.2), (-0.2, -0.6)]);
  lines.push(vec![(0.0, -0.2), (0.2, -0.6)]);

  for i in 0..2 {
    let dx = (i as f64 - 0.5) * 2.0;
    lines.push(vec![(0.0, -0.05), (0.5 * dx, -0.15)]);
    lines.push(vec![(0.0, 0.0), (0.8 * dx, 0.0)]);
    lines.push(vec![(0.0, 0.05), (0.5 * dx, 0.15)]);
  }

  let mut all = vec![];
  for line in lines {
    let l = line
      .iter()
      .map(|&(x, y)| {
        let p = p_r((x, y), rot);
        (ox + p.0 * amp, oy + p.1 * amp)
      })
      .collect();
    all.push(l);
  }

  let linew = rng.gen_range(0.2, 0.3) * amp;
  polylines_smooth_union_filled(&all, clr, linew)
}

fn waving_leaf<R: Rng>(
  rng: &mut R,
  (ox, oy): (f64, f64),
  amp: f64,
  rot: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let mut lines = vec![];
  let count = rng.gen_range(10, 14);
  let rep = 3isize;
  let disp = rng.gen_range(0.03, 0.05);
  for i in -rep..rep {
    let dy = i as f64 * 0.03;
    let mut line = vec![];
    for j in 0..count + 1 {
      let x = (j as f64 - 0.5 * count as f64) / (count as f64);
      let y = dy + disp * (j as f64 * PI / 2.0).sin();
      line.push((x, y));
    }
    lines.push(line);
  }

  for line in lines {
    let route = line
      .iter()
      .map(|&(x, y)| {
        let p = p_r((x, y), rot + PI / 2.);
        (ox + p.0 * amp, oy + p.1 * amp)
      })
      .collect();
    routes.push((clr, route));
  }

  routes
}

fn plate<R: Rng>(
  paint: &mut PaintMask,
  rng: &mut R,
  (ox, oy): (f64, f64),
  r: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  let mut ringext = vec![];
  let mut ringint1 = vec![];
  let mut ringint2 = vec![];

  let divs = 9;
  let adiv = 2. * PI / (divs as f64);
  let adisp = rng.gen_range(0.0, PI);
  for d in 0..divs {
    let abase = adisp
      + (d as f64 + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 0.8)) * adiv;
    let amp = 0.25 * r;
    let size = 0.05 * r;
    let p = (ox + amp * abase.cos(), oy + amp * abase.sin());
    routes.extend(flower2(rng, p, size, 9, 1));
  }

  let divs = 13;
  let adiv = 2. * PI / (divs as f64);
  for d in 0..divs {
    let abase = d as f64 * adiv;
    let res = 20;
    for i in 0..res + 1 {
      let a = abase + (i as f64) * adiv / (res as f64);
      let p = (ox + r * a.cos(), oy + r * a.sin());
      ringext.push(p);
    }

    let res = 20;
    let rbase = 1.0 / 3.0;
    let oscm = 3.0;
    let osc = 0.01;
    for i in 0..res + 1 {
      let x = (i as f64) / (res as f64);
      let a = abase + x * adiv;
      let c = (2. * PI * oscm * x).cos();
      let amp = (rbase + osc * c) * r;
      let p = (ox + amp * a.cos(), oy + amp * a.sin());
      ringint1.push(p);
      let c = (2. * PI * oscm * x + PI).cos();
      let amp = (rbase + 0.8 * osc * c) * r;
      let p = (ox + amp * a.cos(), oy + amp * a.sin());
      ringint2.push(p);
    }

    // FLOWERS
    for (angp, distp) in vec![
      (-0.25, 0.42),
      (-0.3, 0.62),
      (-0.2, 0.83),
      (0.24, 0.52),
      (0.25, 0.7),
      (0.1, 0.88),
    ] {
      let ang = abase + angp * adiv;
      let dist = r * distp;
      let p = (ox + dist * ang.cos(), oy + dist * ang.sin());
      let fsize = 0.03 * r;
      routes.extend(flower(rng, p, fsize, 6, 0, 1));
    }

    // LEAFS
    for (angp, distp, rotadd) in vec![
      (-0.3, 0.52, 0.0),
      (-0.2, 0.72, -PI * 0.3),
      (0.2, 0.4, PI),
      (0.26, 0.61, PI),
      (0.25, 0.81, PI),
    ] {
      let ang = abase + angp * adiv;
      let dist = r * distp;
      let p = (ox + dist * ang.cos(), oy + dist * ang.sin());
      let size = rng.gen_range(0.05, 0.06) * r;
      let rot = -ang + rotadd;
      routes.extend(leaf(rng, p, size, rot, 0));
      let dir = -rot + PI / 2.0;
      let amp = 0.1 * r;
      let q = (p.0 + amp * dir.cos(), p.1 + amp * dir.sin());
      // reverse the ang of q with center
      let qang = (q.1 - oy).atan2(q.0 - ox);
      let b = (ox + 0.33 * r * qang.cos(), oy + 0.33 * r * qang.sin());
      let line = vec![p, q, lerp_point(q, b, 0.3), b];
      let line = path_subdivide_to_curve_it(line, 0.8);
      let line = step_polyline(&line, 5.0);
      let line = line
        .iter()
        .map(|&(x, y)| {
          let disp = rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);
          let ang = rng.gen_range(0.0, 2.0 * PI);
          (x + disp * ang.cos(), y + disp * ang.sin())
        })
        .collect();
      let line = path_subdivide_to_curve_it(line, 0.8);
      routes.push((0, line));
    }

    // WAVING LEAFS
    for (i, adiff) in vec![-0.3, 0.3].iter().enumerate() {
      let a = abase + adiff * adiv + rng.gen_range(-0.01, 0.01);
      let amp = rng.gen_range(0.95, 0.953) * r;
      let p = (ox + amp * a.cos(), oy + amp * a.sin());
      let size = rng.gen_range(0.14, 0.16) * r;
      routes.extend(waving_leaf(rng, p, size, -a, 0));

      let dir = a - PI / 2. + (if i == 0 { 0.0 } else { 1. }) * PI;
      let amp = 0.1 * r;
      let q = (p.0 + amp * dir.cos(), p.1 + amp * dir.sin());
      // reverse the ang of q with center
      let qang = (q.1 - oy).atan2(q.0 - ox);
      let b = (ox + 0.33 * r * qang.cos(), oy + 0.33 * r * qang.sin());
      let a = lerp_point(q, b, 0.1);
      let shuff = rng.gen_range(0.0, 0.1) * r;
      let ap = (
        a.0 + rng.gen_range(-shuff, shuff),
        a.1 + rng.gen_range(-shuff, shuff),
      );
      let ap2 = (
        a.0 + rng.gen_range(-shuff, shuff),
        a.1 + rng.gen_range(-shuff, shuff),
      );
      let line = vec![p, q, a, ap, ap2, b];

      let line = path_subdivide_to_curve_it(line, 0.75);
      let line = path_subdivide_to_curve_it(line, 0.75);
      let line = step_polyline(&line, 4.0);
      let line = line
        .iter()
        .map(|&(x, y)| {
          let disp = rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);
          let ang = rng.gen_range(0.0, 2.0 * PI);
          (x + disp * ang.cos(), y + disp * ang.sin())
        })
        .collect();
      let line = path_subdivide_to_curve_it(line, 0.8);

      routes.push((0, line));
    }
  }

  routes.push((0, ringext));
  routes.push((0, ringint1));
  routes.push((0, ringint2));

  let f = |xf: f64, yf: f64| {
    let gx = (4.0 * (xf.abs() - yf)) as isize;
    let gy = (4.0 * (100.0 - xf.abs() - yf)) as isize;
    let xlim = (1. - smoothstep(0.0, 1.0, yf)).powf(0.5)
      * smoothstep(-1.0, -0.9, yf).powf(0.5);
    let border = 0.06;
    let clr = if xf.abs() < xlim {
      if gx <= 0 || gx > 3 || xf.abs() > xlim - border || yf < -1.0 + border {
        0
      } else {
        if (gx % 2) + (gy % 2) == 1 {
          1
        } else {
          4
        }
      }
    } else {
      4
    };
    clr
  };

  let cy = oy + 0.01 * r;
  let w = 0.28 * r;
  let h = 0.36 * r;
  let y2 = cy + 0.5 * h;
  let x2 = ox + 0.5 * w;
  let mut y = cy - 0.5 * h;
  while y <= y2 {
    let yf = 2. * (y - cy) / h;
    let mut x = ox - 0.5 * w;
    let mut lastclr = f(-1.0, yf);
    let mut lastp = (x, y);
    while x <= x2 {
      let xf = 2. * (x - ox) / w;
      let clr = f(xf, yf);
      if lastclr != clr {
        let p = (x, y);
        routes.push((lastclr, vec![lastp, p]));
        lastclr = clr;
        lastp = p;
      }
      x += 0.1;
    }
    if lastp.0 != x {
      routes.push((lastclr, vec![lastp, (x, y)]));
    }
    y += 0.4;
  }

  for _ in 0..300 {
    let amp1 = rng.gen_range(0.35, 0.95);
    let amp2 = (amp1 + rng.gen_range(-0.05f64, 0.05) * rng.gen_range(0.0, 1.0))
      .max(0.33)
      .min(0.97);
    let a = rng.gen_range(-PI, PI);
    let angl = rng.gen_range(0.05, 0.2);
    let mut line = vec![];
    let cnt = 10;
    for i in 0..cnt + 1 {
      let percent = i as f64 / (cnt as f64);
      let amp = mix(amp1, amp2, percent);
      let ang = a - angl + percent * 2. * angl;
      let p = (ox + amp * r * ang.cos(), oy + amp * r * ang.sin());
      line.push(p);
    }
    if rng.gen_bool(0.7) {
      let line = step_polyline(&line, 4.0);
      let line = line
        .iter()
        .map(|&(x, y)| {
          let disp = rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);
          let ang = rng.gen_range(0.0, 2.0 * PI);
          (x + disp * ang.cos(), y + disp * ang.sin())
        })
        .collect();
      let line = path_subdivide_to_curve_it(line, 0.8);
      let line = path_subdivide_to_curve_it(line, 0.7);
      routes.push((1, line));
    } else {
      let line = step_polyline(&line, rng.gen_range(0.7, 1.0));
      let mut last = line[0];
      let to =
        (1. + (line.len() - 1) as f64 * rng.gen_range(0.0, 1.0)) as usize;
      for i in 1..to {
        let p = line[i];
        let dx = p.0 - last.0;
        let dy = p.1 - last.1;
        last = p;
        let d = mix(0.1, 1.4, i as f64 / (to as f64));
        let ang = dy.atan2(dx) + PI / 2.0;
        let p1 = (p.0 + d * ang.cos(), p.1 + d * ang.sin());
        let p2 = (p.0 - d * ang.cos(), p.1 - d * ang.sin());
        routes.push((1, vec![p1, p2]));
      }
    }
  }

  for (_, rt) in &routes {
    paint.paint_polyline(rt, 1.0);
  }

  // let inner = VCircle::new(ox, oy, 0.333 * r);

  let seed = rng.gen();
  let is_valid = |c: &VCircle| {
    // !inner.contains(c)
    //  &&
    !paint.is_painted((c.x, c.y))
      && circle_route((c.x, c.y), c.r, 7)
        .iter()
        .all(|p| !paint.is_painted(*p))
  };
  let circles = packing(
    seed,
    100000,
    1000,
    1,
    1.0,
    (ox - r, oy - r, ox + r, oy + r),
    &VCircle::new(ox, oy, 0.95 * r),
    &is_valid,
    1.5,
    3.0,
  );

  let perlin = Perlin::new();

  for c in circles {
    let rt = circle_route((c.x, c.y), c.r, 16);
    let seed = rng.gen_range(0.0, 1000.0);
    let baseamp = rng.gen_range(-0.5, 1.0);
    let mulamp = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
    let mut rt = rt
      .iter()
      .enumerate()
      .map(|(i, &p)| {
        let n = perlin.get([seed, 0.01 * p.0, 0.01 * p.1]);
        let amp = (baseamp + i as f64 * mulamp) * c.r;
        let ang = (c.y - p.1).atan2(c.x - p.0);
        let p = (p.0 + amp * n * ang.cos(), p.1 + amp * n * ang.sin());
        p
      })
      .collect::<Vec<_>>();
    rt.truncate(
      ((rt.len() as f64 * rng.gen_range(0.5, 2.0)) as usize).min(rt.len()),
    );
    routes.push((1, rt));
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let precision = 0.3;
  let mut rng = rng_from_seed(opts.seed);
  let mut paint = PaintMask::new(precision, width, height);
  paint.paint_borders(pad);
  let mut routes = vec![];

  routes.extend(plate(
    &mut paint,
    &mut rng,
    (width / 2., height / 2.),
    width.min(height) / 2.0 - pad,
  ));

  vec!["#226", "#e50", "#910"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
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

#[derive(Clone)]
struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
  wi: usize,
  hi: usize,
}

impl PaintMask {
  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
      wi,
      hi,
    }
  }

  fn is_painted(&self, (x, y): (f64, f64)) -> bool {
    let precision = self.precision;
    let wi = self.wi;
    let hi = self.hi;
    let xi = ((x / precision) as usize).min(wi - 1);
    let yi = ((y / precision) as usize).min(hi - 1);
    self.mask[xi + yi * wi]
  }

  fn paint_rectangle(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
    self.paint_rectangle_v(minx, miny, maxx, maxy, true);
  }

  fn paint_rectangle_v(
    &mut self,
    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
    v: bool,
  ) {
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
        self.mask[x + y * wi] = v;
      }
    }
  }

  fn paint_borders(&mut self, pad: f64) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
  }

  pub fn paint_polyline(&mut self, polyline: &Vec<(f64, f64)>, strokew: f64) {
    let len = polyline.len();
    if len < 1 {
      return;
    }
    let first = polyline[0];
    let mut minx = first.0;
    let mut miny = first.1;
    let mut maxx = first.0;
    let mut maxy = first.1;
    let mut i = 1;
    while i < len {
      let (x, y) = polyline[i];
      if x < minx {
        minx = x;
      }
      if x > maxx {
        maxx = x;
      }
      if y < miny {
        miny = y;
      }
      if y > maxy {
        maxy = y;
      }
      i += 1;
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
      let xf = x as f64 * precision;
      for y in miny..maxy {
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let yf = y as f64 * precision;
        let point = (xf, yf);
        let mut i = 1;
        let mut prev = polyline[0];
        while i < len {
          let next = polyline[i];
          if point_in_segment(point, prev, next, strokew) {
            self.mask[j] = true;
            break;
          }
          i += 1;
          prev = next;
        }
      }
    }
  }
}

fn point_in_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
  strokew: f64,
) -> bool {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;
  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let mut h = dot_pa_ba / dot_ba_ba;
  if h < 0.0 {
    h = 0.0;
  } else if h > 1.0 {
    h = 1.0;
  }
  let h_x = ba_x * h;
  let h_y = ba_y * h;
  let dx = pa_x - h_x;
  let dy = pa_y - h_y;
  dx * dx + dy * dy < strokew * strokew
}

fn path_subdivide_to_curve_it(
  path: Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path;
  }
  let mut route = Vec::new();
  let mut first = path[0];
  let mut last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if looped {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for i in 1..(l - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if looped {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if looped {
    route.push(first);
  }
  route
}

fn step_polyline(path: &Vec<(f64, f64)>, step: f64) -> Vec<(f64, f64)> {
  let plen = path.len();
  let mut route = vec![];
  if plen < 1 {
    return route;
  }
  let mut lastp = path[0];
  route.push(lastp);
  let mut i = 0;
  while i < plen - 1 {
    let b = path[i + 1];
    let dist = euclidian_dist(lastp, b);
    if dist < step {
      i += 1;
    } else if dist >= step {
      let p = lerp_point(lastp, b, step / dist);
      route.push(p);
      lastp = p;
    }
  }
  route
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f64,
  y: f64,
  r: f64,
}
impl VCircle {
  fn new(x: f64, y: f64, r: f64) -> Self {
    VCircle { x, y, r }
  }
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
  fn contains(self: &Self, c: &VCircle) -> bool {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - self.r + c.r < 0.0
  }
  fn inside_bounds(
    self: &Self,
    (x1, y1, x2, y2): (f64, f64, f64, f64),
  ) -> bool {
    x1 <= self.x - self.r
      && self.x + self.r <= x2
      && y1 <= self.y - self.r
      && self.y + self.r <= y2
  }
}

fn scaling_search<F: FnMut(f64) -> bool>(
  mut f: F,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let mut from = min_scale;
  let mut to = max_scale;
  loop {
    if !f(from) {
      return None;
    }
    if to - from < 0.1 {
      return Some(from);
    }
    let middle = (to + from) / 2.0;
    if !f(middle) {
      to = middle;
    } else {
      from = middle;
    }
  }
}

fn search_circle_radius(
  container_boundaries: (f64, f64, f64, f64),
  container_circle: &VCircle,
  is_valid: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    c.inside_bounds(container_boundaries)
      && container_circle.contains(&c)
      && is_valid(&c)
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  container_boundaries: (f64, f64, f64, f64),
  container: &VCircle,
  is_valid: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = container.x - container.r;
  let y1 = container.y - container.r;
  let x2 = container.x + container.r;
  let y2 = container.y + container.r;
  let max_scale = max_scale.min(container.r);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    if let Some(size) = search_circle_radius(
      container_boundaries,
      &container,
      is_valid,
      &circles,
      x,
      y,
      min_scale,
      max_scale,
    ) {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
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
