use algo::tri::*;
use algo::{extrusion::build_polygonal_path, math1d::mix};
use clap::*;
use gre::*;
use kiss3d::nalgebra::{Matrix3, Perspective3, Point3, Rotation3, Vector3};
use noise::*;
use rand::prelude::*;
use std::f32::consts::PI;
use std::{
  fs::File,
  io::{BufWriter, Write},
};
use svg::node::element::path::Data;
mod algo;
mod primitives;
mod stlexport;

use stlexport::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "105.0")]
  width: f64,
  #[clap(short, long, default_value = "148.5")]
  height: f64,
  #[clap(short, long, default_value = "5.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

pub struct ArtResult {
  stl: Vec<u8>,
  routes: Vec<(usize, Vec<(f64, f64)>)>,
}
impl ArtResult {
  pub fn new(stl: Vec<u8>, routes: Vec<(usize, Vec<(f64, f64)>)>) -> ArtResult {
    ArtResult { stl, routes }
  }
  pub fn stl(&self) -> Vec<u8> {
    self.stl.clone()
  }
  pub fn routes(&self) -> Vec<(usize, Vec<(f64, f64)>)> {
    self.routes.clone()
  }
}

struct BranchConfig {
  stepping: f32,
  rotangmul: f32,
  edges: usize,
}

fn sprout(
  objects: &mut Vec<(usize, Vec<Tri>)>,
  config: &BranchConfig,
  rng: &mut SmallRng,
  origin: Point3<f32>,
  dir: Vector3<f32>,
  length: f32,
  thickness: f32,
  clr: usize,
) {
  let perlin = Perlin::new();
  let seed = rng.gen_range(0.0, 100.0);

  let subdivisions = (length / config.stepping) as usize;
  if subdivisions < 2 {
    return;
  }
  let rotang = config.rotangmul / subdivisions as f32;
  let f = 0.5;

  let mut path = vec![];
  let mut p = origin;
  let mut direction = dir;
  let mut orientation = Matrix3::identity();

  let mut orientations = vec![];

  for _ in 0..subdivisions + 1 {
    let l = length / subdivisions as f32;
    path.push(p);
    p += direction * l;

    let n1 = perlin.get([
      f * p.x as f64 + seed / 0.6,
      f * p.y as f64 + seed * 3.3358534342,
      f * p.z as f64 + seed,
    ]) as f32;

    let n2 = perlin.get([
      f * p.x as f64 + seed * 0.3,
      f * p.y as f64 + seed * 7.3358534342,
      f * p.z as f64 + seed,
    ]) as f32;

    let n3 = perlin.get([
      f * p.x as f64 + seed / 0.016,
      f * p.y as f64 + seed / 0.472,
      f * p.z as f64,
    ]) as f32;

    let roll = n1 * rotang;
    let pitch = n2 * rotang;
    let yaw = n3 * rotang;
    let rotation = Rotation3::from_euler_angles(roll, pitch, yaw);
    direction = rotation * direction;

    orientation = rotation * orientation;
    orientations.push(orientation);
  }

  let branch = build_polygonal_path(
    config.edges,
    &mut |i, j| {
      let ang = i as f32 * 2. * PI / (config.edges as f32);
      let rad = thickness;
      let rad = rad * mix(1.0, 0.1, (j as f32) / subdivisions as f32);
      (ang, rad)
    },
    &path,
  );
  objects.push((clr, branch));
}

fn art(opts: &Opts) -> ArtResult {
  let mut objects = vec![];

  let mut rng = rng_from_seed_smallrng(opts.seed);

  /*
  objects.push((
    0,
    lowpolyball::low_poly_ball(&mut rng, 80, Point3::origin(), 10.0),
  ));
  */

  let center = (0.5, 0.8);
  let maxx = rng.gen_range(10.0, 20.0);
  let maxy = rng.gen_range(20.0, 120.0);
  let cellw = 5.0;
  let cellh = rng.gen_range(5.0, 12.0);
  let steppingbase = 2.0 + rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0);

  let direction = Vector3::new(0.0, 1.0, 1.0);

  let mut y = -maxy;
  loop {
    if y > maxy {
      break;
    }
    let mut x = -maxx;
    loop {
      if x > maxx {
        break;
      }

      let config = BranchConfig {
        stepping: steppingbase * rng.gen_range(0.8, 1.2),
        edges: rng.gen_range(4, 12),
        rotangmul: 20.0,
      };

      let branch_length = rng.gen_range(20.0, 80.0);
      let thickness = rng.gen_range(4.0, 6.0);
      let p = Point3::new(x, y, 0.0);
      sprout(
        &mut objects,
        &config,
        &mut rng,
        p,
        direction,
        branch_length,
        thickness,
        0,
      );
      x += cellw;
    }
    y += cellh;
  }

  /*
  // plane for debug
  let a = Point3::new(-20.0, 0.0, -20.0);
  let b = Point3::new(20.0, 0.0, -20.0);
  let c = Point3::new(20.0, 0.0, 20.0);
  let d = Point3::new(-20.0, 0.0, 20.0);
  objects.push((1, vec![Tri::new(a, b, d), Tri::new(b, c, d)]));
  */

  // export to stl
  let palette = vec![
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
  ];
  let mut bw = BufWriter::new(Vec::new());
  stl_export(&mut bw, &objects, &palette);

  // GENERATE svg
  // project triangles to 2D with a camera
  let dist = 200.0;
  let fovy = 1.4;
  let cam = Camera::new((opts.width / opts.height) as f32, fovy, 1.0, 400.0);

  // z ordering
  let mut triangles = objects
    .iter()
    .flat_map(|(clr, tris)| tris.iter().map(move |tri| (clr, tri)))
    .flat_map(|(clr, tri)| {
      let t = tri.clone() + Vector3::new(-0.5, -0.5, -0.5);
      let t = t + Vector3::new(0., 0., -dist);
      let tri = cam.project(&t);
      let z = tri.a.z + tri.b.z + tri.c.z;
      if !z.is_finite() {
        return None;
      }
      let c = *clr;
      Some((tri.clone(), z, c))
    })
    .collect::<Vec<_>>();
  triangles.sort_by(|(_, a, _), (_, b, _)| a.partial_cmp(b).unwrap());

  // clipping
  let mut routes = vec![];
  let mut paint = PaintMask::new(0.2, opts.width, opts.height);
  paint.paint_borders(opts.pad);
  for (tri, _z, clr) in triangles {
    let points: Vec<(f64, f64)> = vec![tri.a, tri.b, tri.c, tri.a]
      .iter()
      .map(|p| {
        (
          (p.x as f64 + center.0) * opts.width,
          (-p.y as f64 + center.1) * opts.height,
        )
      })
      .collect();

    let mut rts = vec![(clr, points.clone())];
    rts = regular_clip_polys(&rts, &mut paint, &vec![points]);
    routes.extend(rts);
  }

  return ArtResult::new(bw.into_inner().unwrap(), routes);
}

fn main() {
  let opts: Opts = Opts::parse();
  let res = art(&opts);

  let mut document = base_document("white", opts.width, opts.height);
  for (i, color) in vec!["black", "red"].iter().enumerate() {
    let mut data = Data::new();
    for (ci, route) in res.routes() {
      if i == ci {
        data = render_route(data, route);
      }
    }
    let mut l = layer(color);
    l = l.add(base_path(color, 0.35, data));
    document = document.add(l);
  }
  svg::save("image.svg", &document).unwrap();

  let f = File::create("result.stl").unwrap();
  let mut bw = BufWriter::new(f);
  let bytes = res.stl();
  bw.write_all(&bytes).unwrap();
}

struct Camera {
  aspect: f32,
  fovy: f32,
  znear: f32,
  zfar: f32,
}

impl Camera {
  fn new(aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Self {
    Camera {
      aspect,
      fovy,
      znear,
      zfar,
    }
  }
  fn project(&self, tri: &Tri) -> Tri {
    let proj = Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);
    Tri {
      a: proj.project_point(&tri.a),
      b: proj.project_point(&tri.b),
      c: proj.project_point(&tri.c),
    }
  }
}

fn regular_clip(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let is_outside = |p| paint.is_painted(p);
  clip_routes_with_colors(&routes, &is_outside, 0.3, 5)
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

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
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
        let point = (x as f64 * precision, y as f64 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }
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
