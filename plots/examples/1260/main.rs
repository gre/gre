use clap::*;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn make_drop<R: Rng>(
  rng: &mut R,
  (x, y): (f64, f64),
  sz: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = sz;
  let mut a = -PI / 2.0;
  let dr = 0.5;
  loop {
    let mx = rng.gen_range(0.3, 0.5)
      + 0.4 * smoothstep(rng.gen_range(-1., -0.5), 1.0, a.sin());
    let my = 1.0 + 0.3 * smoothstep(-0.6, -1.0, a.sin());
    let p = round_point((x + mx * r * a.cos(), y + my * r * a.sin()), 0.01);
    route.push(p);
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.1 {
      break;
    }
  }

  route
}

fn make_pipes<R: Rng>(
  rng: &mut R,
  route: Vec<(f64, f64)>,
  pipe_width: f64,
  coloring: usize,
  clr: usize,
  (rand_displacementx, rand_displacementy): (f64, f64),
  mask: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  // it will make intermediate stops to make orthogonal pipes
  let first = route[0];
  let w = pipe_width / 2.0;

  let mut polygons = vec![];

  let mut leftpath = vec![];
  let mut rightpath = vec![];
  let mut stops = vec![];
  leftpath.push((first.0 - w, first.1));
  rightpath.push((first.0 + w, first.1));
  stops.push(first);

  let mut filling = vec![];
  let lines = coloring + 2;
  for i in 0..lines {
    let x = (i as f64) / (lines as f64 - 1.) - 0.5;
    let v = w * 2. * x;
    let p = (first.0 + v, first.1);
    filling.push(vec![p]);
  }

  for i in 0..route.len() - 1 {
    let p1 = route[i];
    let p2 = route[i + 1];
    let dispx = rand_displacementx
      .min(0.2 * (p2.0 - p1.0).abs())
      .min(0.2 * (p2.1 - p1.1).abs());
    let dispy = rand_displacementy
      .min(0.2 * (p2.0 - p1.0).abs())
      .min(0.2 * (p2.1 - p1.1).abs());
    let a1 = rng.gen_range(-PI, PI);
    let a2 = rng.gen_range(-PI, PI);
    let h1 = (
      p1.0 + dispx * a1.cos(),
      (p1.1 + p2.1) / 2.0 + dispy * a1.sin(),
    );
    let h2 = (p2.0 + dispx * a2.cos(), h1.1 + dispy * a2.sin());
    stops.push(h1);
    stops.push(h2);
    stops.push(p2);

    let l0 = leftpath[leftpath.len() - 1];
    let r0 = rightpath[rightpath.len() - 1];

    let hd = if p1.0 < p2.0 { w } else { -w };
    let l1 = (h1.0 - w, h1.1 + hd);
    let l2 = (h2.0 - w, h2.1 + hd);
    let l3 = (p2.0 - w, p2.1);

    leftpath.push(l1);
    leftpath.push(l2);
    leftpath.push(l3);

    let r1 = (h1.0 + w, h1.1 - hd);
    let r2 = (h2.0 + w, h2.1 - hd);
    let r3 = (p2.0 + w, p2.1);

    rightpath.push(r1);
    rightpath.push(r2);
    rightpath.push(r3);

    for i in 0..lines {
      let x = (i as f64) / (lines as f64 - 1.) - 0.5;
      let v = w * 2. * x;
      let l1 = (h1.0 + v, h1.1 - hd * v / w);
      let l2 = (h2.0 + v, h2.1 - hd * v / w);
      let l3 = (p2.0 + v, p2.1);
      filling[i].push(l1);
      filling[i].push(l2);
      filling[i].push(l3);
    }

    let poly = vec![l0, r0, r1, l1];
    polygons.push(poly);
    let poly = vec![l1, r1, r2, l2];
    polygons.push(poly);
    let poly = vec![l2, r2, r3, l3];
    polygons.push(poly);
  }

  let mut routes = vec![];
  for f in filling {
    routes.push((clr, f));
  }

  let is_outside = |p: (f64, f64)| mask.is_painted(p);

  routes = clip_routes_with_colors(&routes, &is_outside, 0.5, 5);

  for poly in &polygons {
    mask.paint_polygon(poly);
  }

  /*
  let mut routes = vec![];
  for poly in &polygons {
    let mut route = vec![];
    route.extend(poly.clone());
    route.push(poly[0]);
    routes.push(route);
  }
  */

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut routes = vec![];
  let mut mask = PaintMask::new(0.2, width, height);

  let mut rng = rng_from_seed(opts.seed);

  let is_gold_one = rng.gen_bool(0.4);
  let pgold = rng.gen_range(0.0, 0.1);
  let pfill = rng.gen_range(0.0, 0.5);
  let panomalyendpipe = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);

  let diamondeffect = rng.gen_range(-0.1f64, 0.5).max(0.0);
  let diamondeffect2 =
    rng.gen_range(-0.2f64, 0.8).max(0.0) * rng.gen_range(0.0, 1.0);

  let rand_displacement = (
    rng.gen_range(-0.4f64, 0.2).max(0.0) * pad,
    rng.gen_range(-0.3f64, 0.1).max(0.0) * pad,
  );
  let pdisplace = rng.gen_range(-0.5, 1.5);

  let pipe_size = rng.gen_range(3.0, 7.0);
  let pipe_pad = rng.gen_range(0.0, 2.0) * pipe_size;

  let filldist = rng.gen_range(0.5, 2.0);
  let fill = (pipe_size / filldist) as usize;

  let pipe_count = (width * 0.8 / (pipe_size + pipe_pad)) as usize;

  let mut connects: Vec<_> = (0..pipe_count).collect();
  rng.shuffle(&mut connects);
  let mut pipes: Vec<_> = connects
    .iter()
    .enumerate()
    .map(|(i, &to)| (i, to))
    .collect();
  rng.shuffle(&mut pipes);

  let ydivs = (5. + rng.gen_range(0., 60.) * rng.gen_range(0.0, 1.0)) as usize;
  let incrmax = rng.gen_range(1, ydivs);

  let xmul = (width - 2.0 * pad) / (pipe_count as f64);
  let ymul = (height - 2.0 * pad) / (ydivs as f64);

  let clr = if is_gold_one { 1 } else { rng.gen_range(0, 2) };
  let mut incr = 0.0;
  while incr < pipe_size {
    let y = pad + 0.5 * ymul - incr;
    routes.push((clr, vec![(pad, y), (width - pad, y)]));
    incr += filldist;
  }
  routes.push((
    clr,
    vec![
      (pad, pad + 0.5 * ymul),
      (pad, pad + 0.5 * ymul - pipe_size + filldist),
    ],
  ));
  routes.push((
    clr,
    vec![
      (width - pad, pad + 0.5 * ymul),
      (width - pad, pad + 0.5 * ymul - pipe_size + filldist),
    ],
  ));

  let lastcellistoi = rng.gen_bool(0.5);

  let extradrops = if rng.gen_bool(0.5) {
    rng.gen_range(0., 100.)
  } else {
    0.
  };

  let goldone_i = rng.gen_range(0, pipe_count);

  let droprate =
    rng.gen_range(0.0, 0.8) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  for (fromindex, toindex) in pipes {
    if rng.gen_bool(droprate) && !(is_gold_one && fromindex == goldone_i) {
      continue;
    }
    let dc = (fromindex as f64 / (pipe_count as f64) - 0.5).abs() * 2.0;
    let fromi = (dc * (ydivs as f64 * diamondeffect)) as usize;
    let dc = ((toindex as f64) / (pipe_count as f64) - 0.5).abs() * 2.0;
    let mut toi = ydivs - 1 - (dc * (ydivs as f64 * diamondeffect2)) as usize;
    let lastcell = if rng.gen_bool(panomalyendpipe) {
      rng.gen_range(fromi, toi)
    } else if lastcellistoi {
      toi
    } else {
      ydivs - 1
    };
    if lastcell > toi {
      toi = lastcell;
    }

    let mut xi = fromindex;
    let mut yi = 0;
    let mut route = vec![];

    loop {
      let mut dx = 0.0;
      let mut dy = 0.0;
      if yi > fromi && yi < toi {
        dx = rng.gen_range(-0.5, 0.5);
        dy = rng.gen_range(-0.5, 0.5);
      }
      let p = (
        pad + (xi as f64 + 0.5 + dx) * xmul,
        pad + (yi as f64 + 0.5 + dy) * ymul,
      );

      route.push(p);

      if yi == lastcell {
        break;
      }
      if yi == 0 && fromi > 0 {
        yi = fromi;
        continue;
      }
      if yi >= toi && toi != lastcell {
        xi = toindex;
        yi = lastcell;
        continue;
      }
      xi = rng.gen_range(0, pipe_count);
      yi += rng.gen_range(1, incrmax);
      if yi >= toi {
        xi = toindex;
        yi = toi;
      }
    }
    let clr = if is_gold_one {
      if fromindex == goldone_i {
        1
      } else {
        0
      }
    } else if rng.gen_bool(pgold) {
      1
    } else {
      0
    };
    let coloring = if clr == 1 {
      fill
    } else {
      if rng.gen_bool(pfill) {
        rng.gen_range(0, fill)
      } else {
        0
      }
    };
    let disp = pdisplace > 0.99 || pdisplace > 0. && rng.gen_bool(pdisplace);
    let pipes: Vec<(usize, Vec<(f64, f64)>)> = make_pipes(
      &mut rng,
      route,
      pipe_size,
      coloring,
      clr,
      if disp { rand_displacement } else { (0., 0.) },
      &mut mask,
    );
    routes.extend(pipes);

    let mut basey = rng.gen_range(0.6, 0.8); //rng.gen_range(0.5, 1.5);
    for i in 0..(1. + extradrops * rng.gen_range(0.0, 1.0)) as usize {
      let o = (
        pad
          + (toindex as f64
            + 0.5
            + if i > 0 {
              rng.gen_range(-0.5, 0.5)
                * rng.gen_range(0.0, 1.0)
                * rng.gen_range(0.0, 1.0)
                * rng.gen_range(0.0, 1.0)
            } else {
              0.
            })
            * xmul,
        pad + (lastcell as f64 + 0.5) * ymul + pipe_size * basey,
      );
      basey += i as f64 + rng.gen_range(0.0, 0.4) * rng.gen_range(0.0, 1.0);
      if o.1 > height - pad {
        break;
      }
      let sz = pipe_size * rng.gen_range(0.2, 0.5);
      routes.push((clr, make_drop(&mut rng, o, sz)));
    }
  }

  vec!["white", "#fc0"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.55, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "322.0")]
  seed: f64,
  #[clap(short, long, default_value = "297")]
  width: f64,
  #[clap(short, long, default_value = "420")]
  height: f64,
  #[clap(short, long, default_value = "30")]
  pad: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
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

// TODO more efficient algorithm would be to paint on a mask.
struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
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
    }
  }

  fn is_painted(&self, point: (f64, f64)) -> bool {
    // check out of bounds
    if point.0 <= 0.0
      || point.0 >= self.width
      || point.1 <= 0.0
      || point.1 >= self.height
    {
      return false;
    }
    let precision = self.precision;
    let width = self.width;
    let x = (point.0 / precision) as usize;
    let y = (point.1 / precision) as usize;
    let wi = (width / precision) as usize;
    self.mask[x + y * wi]
  }

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
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
        if polygon_includes_point(polygon, point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
}

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
