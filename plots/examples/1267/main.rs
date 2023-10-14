use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
// use rayon::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn make_pepper<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  o: (f64, f64),
  r: f64,
  ang: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  let spiral_fill_dr = rng.gen_range(0.45, 0.6);

  let mut barebone = vec![];
  let initialdir = rng.gen_range(-0.1, 0.1);
  let incr = rng.gen_range(0.8, 2.5);
  let maxlength = r * rng.gen_range(1.5, 2.5);
  let daincr = rng.gen_range(0.0, 0.04) * rng.gen_range(0.0, 1.0) * incr;
  let daincrdiffcorrection = rng.gen_range(0.0, 0.03) * incr;
  let damul = 1.0 + rng.gen_range(0.0, 0.05) * incr;
  let dacutoff = rng.gen_range(0.1, 0.3) * incr;
  let mut a = (1. + initialdir) * PI;
  let mut p: (f64, f64) = (r, 0.0);
  let mut da: f64 = 0.0;
  let mut l = 0.0;
  while euclidian_dist((0.0, 0.0), p) <= r && l < maxlength {
    barebone.push(p);

    // correct the direction to go back to center
    let angdiff = p.1.atan2(p.0);

    da += daincrdiffcorrection * angdiff.signum()
      + daincr * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0);
    da *= damul;

    if da.abs() > dacutoff {
      da = 0.;
    }

    p.0 += a.cos() * incr;
    p.1 += a.sin() * incr;
    l += incr;
    a += da;
  }

  let mut widths = vec![];
  let wfactor = rng.gen_range(0.5, 2.0);
  let modulo = rng.gen_range(5.0, 16.0);
  let ond = rng.gen_range(0.0, 2.0);
  for i in 0..barebone.len() {
    let f = (i as f64) / ((barebone.len() - 1) as f64);
    let c = 1.0 - 2.0 * (f - 0.5).abs();
    let m = 1. - 2. * ((f * modulo).fract() - 0.5).abs();
    let stick = 0.1;
    let w = (r / 22.0).min(1.2) * (
      // ondulating
      ond * m.abs().powf(2.0) + 
      // high to low
      4. * smoothstep(1.0, stick, f) * (if f < stick { 0.1 } else{ 1.0 }) +
      // center growing
      10.0 * c.powf(2.0)
    )
    .max(0.5);
    widths.push(wfactor * w);
  }

  let lpd = 2. * r;
  // let prec = 0.05;
  // let mut local_paint = PaintMask::new(prec, 4.0 * r, 4.0 * r);

  let mut fibers: Vec<Vec<(f64, f64)>> = vec![];
  let count = rng.gen_range(2, 6);
  for _ in 0..count {
    fibers.push(vec![]);
  }
  for i in 0..count {
    let df = (i as f64) / ((count - 1) as f64) - 0.5;
    for j in 0..barebone.len() {
      let a = if j > 0 {
        (barebone[j].1 - barebone[j - 1].1)
          .atan2(barebone[j].0 - barebone[j - 1].0)
      } else {
        (barebone[1].1 - barebone[0].1).atan2(barebone[1].0 - barebone[0].0)
      };
      let orthogonal = a + PI / 2.0;
      let dist = widths[j];
      let d = df * dist * 0.5;
      let p = barebone[j];
      let q = (p.0 + d * orthogonal.cos(), p.1 + d * orthogonal.sin());
      fibers[i].push(q);
    }
  }

  // TODO with fibers[0] and fibers[last], we can make polygons and fill local_paint with it
  let mut polys = vec![];
  let mut globalpolys = vec![];
  for i in 1..barebone.len() {
    let a = fibers[0][i - 1];
    let b = fibers[0][i];
    let c = fibers[count - 1][i];
    let d = fibers[count - 1][i - 1];

    
    {
      let a = (a.0 + lpd, a.1 + lpd);
      let b = (b.0 + lpd, b.1 + lpd);
      let c = (c.0 + lpd, c.1 + lpd);
      let d = (d.0 + lpd, d.1 + lpd);
      let polygon = vec![a, b, c, d];
      polys.push(polygon);
    }

    {
      let a = p_r(a, -ang);
      let a = (a.0 + o.0, a.1 + o.1);
      let b = p_r(b, -ang);
      let b = (b.0 + o.0, b.1 + o.1);
      let c = p_r(c, -ang);
      let c = (c.0 + o.0, c.1 + o.1);
      let d = p_r(d, -ang);
      let d = (d.0 + o.0, d.1 + o.1);
      globalpolys.push(vec![a, b, c, d]);
    }
  }

  let spiral = vec![(clr, spiral_optimized(r, 0.0, 2.*r, spiral_fill_dr, 0.1))];
  let local_painted = |p: (f64, f64)| {

    let q = (p.0+lpd, p.1+lpd);
    !polys.iter().any(|poly| polygon_includes_point(&poly, q))

    // !(euclidian_dist(p, (0.0, 0.0)) < lpd &&
    // local_paint.is_painted(q))
  };
  routes.extend(
    clip_routes_with_colors(&spiral, &local_painted, 0.4, 5),
  );


  // TODO then we can colorize the paint

  for fiber in fibers {
    routes.push((clr, fiber));
  }
  
  let greenclr = 2;
  if clr != greenclr {
    // green part to be splitted out on the routes
    let green = rng.gen_range(0.2, 0.3);
    let is_green = |p| {
      euclidian_dist((r, 0.), p) < green * r
    };
    let is_red = |p| !is_green(p);

    let green_routes = clip_routes_with_colors(&routes, &is_red, 0.4, 5);
    let red_routes = clip_routes_with_colors(&routes, &is_green, 0.4, 5);

    routes = vec![
      green_routes.iter().map(|(_, r)| (greenclr, r.clone())).collect(),
      red_routes
    ].concat();
  }
  // rotate everything
  let mut all = vec![];
  for (clr, route) in routes {
    let mut path = vec![];
    for p in route {
      let p = p_r(p, -ang);
      let p = (p.0 + o.0, p.1 + o.1);
      path.push(p);
    }
    all.push((clr, path));
  }

  let is_outside = |p| paint.is_painted(p);
  all = clip_routes_with_colors(&all, &is_outside, 0.3, 5);

  for poly in globalpolys {
    paint.paint_polygon(&poly);
  }

  all
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let extrapad = opts.extrapad;
  let bounds = (pad, pad, width - pad, height - pad);

  let mut routes = vec![];
  let mut rng = rng_from_seed(opts.seed);

  let f = rng.gen_range(0.0, 0.2);
  let amp = rng.gen_range(0.0, PI) * rng.gen_range(0.0, 1.0);

  
  let mut paint = PaintMask::new(0.2, width, height);

  let extrabound = (extrapad, extrapad, width-extrapad,height-extrapad);

  let overlap = |p| {
    strictly_in_boundaries(p, extrabound)
  };
  
  let does_overlap = |c: (f64, f64, f64)| {
    overlap((c.0, c.1))
      && circle_route((c.0, c.1), c.2, 10)
        .iter()
        .all(|&p| overlap(p))
  };

  let mut circles = vec![];

  let count = (1.4 + rng.gen_range(0.0, 3.0) * rng.gen_range(0.0, 1.0)) as usize;
  let desired_count = rng.gen_range(count+5, 100);

  for i in 0..count {
    let objpad: f64 = rng.gen_range(0.0, 20.0) * rng.gen_range(0.0, 1.0);
    let objsizemin = objpad + rng.gen_range(6.0, 12.0);
    let objsizemax = objsizemin + objsizemin * (1.0 + rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0));
    let optim = rng.gen_range(0, 3);
    circles.extend(
  packing(
    &mut rng,
    1000000,
    desired_count / (count - i),
    optim,
    objpad,
    (pad, pad, width - pad, height - pad),
    &does_overlap,
    objsizemin,
    objsizemax,
  ));
}

  let perlin = Perlin::new();

  let mut points = vec![];

  let lowp = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let grow_anomaly = rng.gen_range(0.0, 1.0);
  let clrmax = 3.0;

  for c in circles {
    let n = perlin.get([f * c.x, f * c.y, opts.seed / 7.7]);
    let ang = amp * n - PI / 2.0;
    let r = c.r * (1.0 + rng.gen_range(0.0, grow_anomaly) * rng.gen_range(0.0, 1.0));
    let r2 = 0.95 * r;
    points.push((c.x + r2 * ang.cos(), c.y + r2 * ang.sin()));
    let clr = (rng.gen_range(0.0, clrmax) * rng.gen_range(lowp, 1.0)) as usize;
    routes.extend(make_pepper(&mut rng, &mut paint, (c.x, c.y), r, ang, clr));
  }

  
  points.sort_by_key(|p| (p.1 * 1000.0) as i32);


  let lookup = rng.gen_range(2.0, 4.0);
  let mut pasts: Vec<(f64, f64)> = vec![];
  for p in points {
    let width = rng.gen_range(1.0, 2.0);
    let tracks_count = 1;
    let noiseamp = rng.gen_range(0.2, 0.3);
    let freq_mul = rng.gen_range(0.8, 2.0);
    let q = pasts.iter().cloned().find(|&q| {
      (q.0-p.0).abs() < lookup
    }).unwrap_or((p.0, p.1-height));

    let path = vec![p, q];
    
    let all = cordon(
      path,
      width,
      noiseamp,
      2.0,
      tracks_count,
      true,
      freq_mul,
      0.0,
    );
    let all = all.iter().map(|rt| (3, rt.clone())).collect();
    let is_outside = |p| out_of_boundaries(p, bounds) || paint.is_painted(p);
    let all = clip_routes_with_colors(&all, &is_outside, 0.4, 5);
    routes.extend(all);
    pasts.push(p);
  }
  
  
/*
  
  // TODO instead of chunking, drop some curves down, that form trees
  let chunking = rng.gen_range(5, 20);
  let chunks: Vec<_> = points
    .chunks(chunking)
    .map(|chunk| chunk.iter().cloned().collect::<Vec<_>>())
    .collect();

  routes.extend(
    chunks
      .par_iter()
      .flat_map(|candidates| {
        if candidates.len() < 3 {
          return vec![];
        }
        let first = candidates[0];
        let mut rng = rng_from_seed(opts.seed + first.0 + first.1 * 73.777);
        let width = rng.gen_range(0.7, 1.0);
        let tracks_count = 1;
        let noiseamp = rng.gen_range(0.2, 0.3);
        let freq_mul = rng.gen_range(0.8, 2.0);
        let path =
          tsp(candidates.clone(), time::Duration::seconds(opts.seconds));
        // let path = path_subdivide_to_curve(path, 2, 0.9);
        let all = cordon(
          path,
          width,
          noiseamp,
          2.0,
          tracks_count,
          true,
          freq_mul,
          0.0,
        );
        let all = all.iter().map(|rt| (3, rt.clone())).collect();
        let is_outside = |p| paint.is_painted(p);
        let all = clip_routes_with_colors(&all, &is_outside, 0.4, 5);
        all
      })
      .collect::<Vec<_>>(),
  );
  */
  

  vec!["#c23", "#f92", "#7a8", "#ccc"]
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
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "297")]
  width: f64,
  #[clap(short, long, default_value = "420")]
  height: f64,
  #[clap(short, long, default_value = "20")]
  pad: f64,
  #[clap(short, long, default_value = "30")]
  extrapad: f64,
  #[clap(short, long, default_value = "1")]
  seconds: i64,
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

#[derive(Clone)]
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

  /*
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
  */

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx-precision).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny-precision).max(0.).min(self.height) / precision) as usize;
    let maxx = ((maxx+precision).max(0.).min(self.width) / precision) as usize;
    let maxy = ((maxy+precision).max(0.).min(self.height) / precision) as usize;
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
  /*
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist(p, (self.x, self.y)) < self.r
  }
  */
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
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
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y, size)) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing<R: Rng>(
  rng: &mut R,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn((f64, f64, f64)) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = vec![];
  let mut tries = Vec::new();
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) =
      search_circle_radius(&does_overlap, &circles, x, y, min_scale, max_scale)
    {
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

/*
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

fn path_subdivide_to_curve(
  path: Vec<(f64, f64)>,
  n: usize,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}
*/

fn cordon(
  path: Vec<(f64, f64)>,
  width: f64,
  noiseamp: f64,
  corner_pad: f64,
  tracks_count: usize,
  reconnect: bool,
  freq_mul: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let precision = 0.5;
  let r = precision;
  let mut pindex = 0;
  let mut p = path[pindex];
  let perlin = Perlin::new();
  let mut tracks = Vec::new();
  for _xi in 0..tracks_count {
    tracks.push(Vec::new());
  }
  for &next in path.iter().skip(1) {
    let dx = next.0 - p.0;
    let dy = next.1 - p.1;
    let a = dy.atan2(dx);
    let mut i = 0.0;
    let acos = a.cos();
    let asin = a.sin();
    let mut dist = (dx * dx + dy * dy).sqrt();
    if pindex != 0 {
      dist -= corner_pad;
      p.0 += corner_pad * acos;
      p.1 += corner_pad * asin;
    }
    if pindex == path.len() - 1 {
      dist -= corner_pad;
    }
    loop {
      if i >= dist {
        p = next;
        break;
      }
      p.0 += r * acos;
      p.1 += r * asin;
      for xi in 0..tracks_count {
        let variation = ((xi as f64 + (tracks_count as f64 * phase))
          % (tracks_count as f64)
          - ((tracks_count - 1) as f64 / 2.0))
          / (tracks_count as f64);
        let mut delta = variation * width;
        let noisefreq = freq_mul * (0.1 + 0.2 * (0.5 - variation.abs()));
        delta += noiseamp
          * perlin.get([
            //
            noisefreq * p.0,
            noisefreq * p.1,
            10.0 * xi as f64,
          ]);
        let a2 = a + PI / 2.0;
        let q = (p.0 + delta * a2.cos(), p.1 + delta * a2.sin());
        tracks[xi].push(q);
      }
      i += r;
    }
    pindex += 1;
  }
  for track in tracks {
    let mut track_copy = track.clone();
    if reconnect {
      track_copy.push(track[0]);
    }
    routes.push(track_copy);
  }
  routes
}
