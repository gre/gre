use clap::*;
use gre::*;
use isosurface::{marching_cubes::MarchingCubes, source::Source};
use kiss3d::nalgebra::{Perspective3, Point3, Rotation3, Vector2, Vector3};
use rand::prelude::*;
use std::f32::consts::PI;
use std::ops::{Add, Mul, Sub};
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
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

// We use sign distance function paradigm.
// inspiration https://www.shadertoy.com/view/wlXSD7

fn sd_link(p: Vector3<f32>, le: f32, r1: f32, r2: f32) -> f32 {
  let q = Vector3::new(p.x, (p.y.abs() - le).max(0.0), p.z);
  let d = Vector2::new(q.xy().norm() - r1, q.z).norm() - r2;
  d
}

trait BinaryOps<T> {
  fn intersect(&self, other: T) -> T;
  fn difference(&self, other: T) -> T;
  fn union(&self, other: T) -> T;
  fn smooth_intersect(&self, k: T, other: T) -> T;
  fn smooth_difference(&self, k: T, other: T) -> T;
  fn smooth_union(&self, k: T, other: T) -> T;
}

impl BinaryOps<f32> for f32 {
  fn intersect(&self, other: f32) -> f32 {
    self.max(other)
  }
  fn difference(&self, other: f32) -> f32 {
    self.max(-other)
  }
  fn union(&self, other: f32) -> f32 {
    self.min(other)
  }

  fn smooth_intersect(&self, k: f32, other: f32) -> f32 {
    let h = (0.5 - 0.5 * (self - other) / k).max(0.0).min(1.0);
    mix(*self, other, h) + k * h * (1.0 - h)
  }

  fn smooth_difference(&self, k: f32, other: f32) -> f32 {
    let h = (0.5 - 0.5 * (other + self) / k).max(0.0).min(1.0);
    mix(*self, -other, h) + k * h * (1.0 - h)
  }

  fn smooth_union(&self, k: f32, other: f32) -> f32 {
    let h = (0.5 + 0.5 * (self - other) / k).max(0.0).min(1.0);
    mix(*self, other, h) - k * h * (1.0 - h)
  }
}

fn mix(a: f32, b: f32, x: f32) -> f32 {
  (1. - x) * a + x * b
}

struct Shape {
  seed: f64,
  rots: Vec<Rotation3<f32>>,
  scale_base: f32,
  scale_mul: f32,
  count: usize,
  offsetmul: f32,
}
impl Source for Shape {
  fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
    let mut rng = rng_from_seed(self.seed);

    let mut p = Vector3::new(x, y, z);
    p.x -= 0.5;
    p.y -= 0.5;
    p.z -= 0.5;
    let origin = p;

    let mut s = 999f32;

    let rots = self.rots.clone();
    let scale_base = self.scale_base;
    let offsetmul = self.offsetmul;

    for _i in 0..self.count {
      let index = (rng.gen_range(0f32, rots.len() as f32)
        * rng.gen_range(0f32, 1.)) as usize;
      let rot = rots[index];

      let mut p = origin.clone();

      p.y += offsetmul * rng.gen_range(-0.1, 0.1);

      p.x += offsetmul * rng.gen_range(-0.4, 0.4) * rng.gen_range(0.5, 1.0);
      p.z += offsetmul * rng.gen_range(-0.2, 0.2) * rng.gen_range(0.0, 1.0);

      p = rot * p;

      let scale = rng.gen_range(scale_base, self.scale_mul * scale_base);

      p.x *= scale;
      p.y *= scale;
      p.z *= scale;

      p.y += scale;

      let le = rng.gen_range(0.1, 0.15);
      let r1 = rng.gen_range(0.18, 0.2);
      let r2 = rng.gen_range(0.08, 0.09);

      let mut a = p.clone();
      a.y = a.y.fract() - 0.5;

      let mut b = p.clone();
      b.y = (b.y + 0.5).fract() - 0.5;
      let z = b.z;
      b.z = b.x;
      b.x = z;

      s = s.min(sd_link(a, le, r1, r2)).min(sd_link(b, le, r1, r2));
    }

    s
  }
}

fn make_triangles_from_vertices_indices(
  vert: &Vec<f32>,
  idx: &Vec<u32>,
) -> Vec<Tri> {
  let mut triangles = vec![];
  for face in idx.chunks(3) {
    let i1 = face[0] as usize;
    let i2 = face[1] as usize;
    let i3 = face[2] as usize;
    let v1 = Point3::new(vert[i1 * 3], vert[i1 * 3 + 1], vert[i1 * 3 + 2]);
    let v2 = Point3::new(vert[i2 * 3], vert[i2 * 3 + 1], vert[i2 * 3 + 2]);
    let v3 = Point3::new(vert[i3 * 3], vert[i3 * 3 + 1], vert[i3 * 3 + 2]);
    triangles.push(Tri::new(v3, v2, v1));
  }
  triangles
}

#[derive(Debug, Clone)]
struct Tri {
  v1: Point3<f32>,
  v2: Point3<f32>,
  v3: Point3<f32>,
}

impl Sub<Vector3<f32>> for Tri {
  type Output = Tri;

  fn sub(self, v: Vector3<f32>) -> Self::Output {
    Tri {
      v1: self.v1 - v,
      v2: self.v2 - v,
      v3: self.v3 - v,
    }
  }
}

impl Add<Vector3<f32>> for Tri {
  type Output = Tri;

  fn add(self, v: Vector3<f32>) -> Self::Output {
    Tri {
      v1: self.v1 + v,
      v2: self.v2 + v,
      v3: self.v3 + v,
    }
  }
}

impl Mul<Tri> for f32 {
  type Output = Tri;

  fn mul(self, tri: Tri) -> Self::Output {
    Tri {
      v1: self * tri.v1,
      v2: self * tri.v2,
      v3: self * tri.v3,
    }
  }
}

impl Mul<Tri> for Rotation3<f32> {
  type Output = Tri;

  fn mul(self, tri: Tri) -> Self::Output {
    Tri {
      v1: self * tri.v1,
      v2: self * tri.v2,
      v3: self * tri.v3,
    }
  }
}

impl Tri {
  fn new(v1: Point3<f32>, v2: Point3<f32>, v3: Point3<f32>) -> Self {
    Tri { v1, v2, v3 }
  }
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
      v1: proj.project_point(&tri.v1),
      v2: proj.project_point(&tri.v2),
      v3: proj.project_point(&tri.v3),
    }
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let precision = 0.2;

  let mut rng = rng_from_seed(opts.seed);
  let mut paint = PaintMask::new(precision, width, height);
  paint.paint_borders(pad);
  let grid_size = rng.gen_range(120, 200);

  let rotscount = (1. + rng.gen_range(0.0, 4.0)) as usize;

  let mut rots = (0..rotscount)
    .map(|_i| {
      Rotation3::from_axis_angle(&Vector3::z_axis(), rng.gen_range(-PI, PI))
        * Rotation3::from_axis_angle(&Vector3::y_axis(), rng.gen_range(-PI, PI))
        * Rotation3::from_axis_angle(
          &Vector3::x_axis(),
          0.2 * rng.gen_range(-PI, PI),
        )
    })
    .collect::<Vec<_>>();

  // project triangles to 2D with a camera
  let dist = 1.2;
  let cam = Camera::new((width / height) as f32, 1.0, 1.0, 6.0);

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = vec![];

  for j in 0..2 {
    let i = 1 - j;
    let count = if i == 1 { 1 } else { rng.gen_range(20, 80) };

    if rng.gen_bool(0.3) {
      rots.push(
        Rotation3::from_axis_angle(&Vector3::z_axis(), rng.gen_range(-PI, PI))
          * Rotation3::from_axis_angle(
            &Vector3::y_axis(),
            rng.gen_range(-PI, PI),
          )
          * Rotation3::from_axis_angle(
            &Vector3::x_axis(),
            rng.gen_range(-PI, PI),
          ),
      );
    }

    let scale_base = rng.gen_range(4.0, 8.0) / ((i + 1) as f32);

    let scale_mul = if i == 0 {
      rng.gen_range(4.0, 20.0)
    } else {
      2.0
    };

    let offsetmul = if i == 1 { rng.gen_range(0.0, 0.1) } else { 1.0 };

    let source = Shape {
      seed: opts.seed + i as f64 / 0.037,
      rots: rots.clone(),
      scale_base,
      scale_mul,
      count,
      offsetmul,
    };

    let mut vertices = vec![];
    let mut indices = vec![];
    let mut marching = MarchingCubes::new(grid_size / (i + 1));
    marching.extract(&source, &mut vertices, &mut indices);
    let triangles = make_triangles_from_vertices_indices(&vertices, &indices);

    let mut projected = triangles
      .iter()
      .map(|tri| {
        let t = tri.clone() + Vector3::new(-0.5, -0.5, -0.5);
        let t = t + Vector3::new(0., 0., -dist);
        cam.project(&t)
      })
      .collect::<Vec<_>>();

    // sort by z-index
    let mut data = projected
      .iter()
      .map(|tri| {
        let z = tri.v1.z + tri.v2.z + tri.v3.z;
        (tri.clone(), z)
      })
      .collect::<Vec<(Tri, f32)>>();
    data.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    projected = data.iter().map(|(tri, _)| tri.clone()).collect::<Vec<_>>();

    for tri in projected {
      let points: Vec<(f64, f64)> = vec![tri.v1, tri.v2, tri.v3, tri.v1]
        .iter()
        .map(|p| ((p.x as f64 + 0.5) * width, (p.y as f64 + 0.5) * height))
        .collect();

      let mut rts = vec![(i, points.clone())];
      // TODO FUTURE: we have to grow the polygon to allow more space for the clip
      rts = regular_clip_polys(&rts, &mut paint, &vec![points]);
      routes.extend(rts);
    }
  }

  let sc = (width / 2.0, 2.0 * height);
  let scd = 4.0 * height;
  let d = rng.gen_range(4.0, 8.0);

  routes.extend(regular_clip(
    &vec![(0, spiral_optimized(sc.0, sc.1, scd, d, 0.1))],
    &mut paint,
  ));

  vec!["white", "gold"]
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
      l = l.add(base_path(color, 0.5, data));
      l
    })
    .collect()
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

fn regular_clip(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let is_outside = |p| paint.is_painted(p);
  clip_routes_with_colors(&routes, &is_outside, 0.5, 3)
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

fn curve_length(path: &Vec<(f64, f64)>) -> f64 {
  let mut len = 0.0;
  for i in 0..path.len() - 1 {
    len += euclidian_dist(path[i], path[i + 1]);
  }
  len
}
