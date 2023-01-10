use clap::*;
use gre::*;
use ndarray::Array2;
use rayon::prelude::*;
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
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let bound = (pad, pad, width - pad, height - pad);

  let dim = 1000;
  let probability = 0.05;

  let clusters = 40;
  let clouds_count = 3;
  let samples = 12000;
  let seconds = 10;

  let ratio = (width - 2. * pad) / (height - 2. * pad);
  let f = |(x, y)| {
    probability * raytrace((x * ratio + (1.0 - ratio) / 2.0, y)).powf(2.0)
  };

  let clouds: Vec<Vec<(f64, f64)>> = (0..clouds_count)
    .into_par_iter()
    .flat_map(|i| {
      let mut rng = rng_from_seed(opts.seed + i as f64 / 0.013);
      let mut passage = Passage::new(1.0, opts.width, opts.height);
      let passage_max = 2;
      let samples = sample_2d_candidates_f64(&f, dim, samples, &mut rng);

      let points: Vec<(f64, f64)> = samples
        .iter()
        .map(|&p| project_in_boundaries(p, bound))
        .filter(|&p| passage.count(p) < passage_max)
        .collect();

      clusterize(points, clusters)
    })
    .collect();

  let routes: Vec<Vec<(f64, f64)>> = clouds
    .par_iter()
    .map(|pts| {
      let mut route = tsp(pts.clone(), time::Duration::seconds(seconds));
      route.push(route[0]);
      route
    })
    .collect();

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route_curve(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
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

// this is the "main" coloring function. for a given uv, returns a color.
fn raytrace(uv: Vec2) -> f64 {
  let (x, y) = uv;
  // raymarching
  let origin = (0.0, 0.0, -4.0);
  let dir = normalize3((x - 0.5, y - 0.5, 1.0));
  let mut t = 0.0;
  let mut hit = 99.0;
  for _i in 0..100 {
    let h = map(add3(origin, mul3f(dir, t)));
    t += h;
    if h.abs() < 0.001 {
      hit = h;
      break;
    }
  }
  let p = add3(origin, mul3f(dir, t));
  let n = normal(p);
  return lighting(hit, p, n, dir);
}

// this is our "3D scene" distance function:
// for a given point in space, tells the distance to closest object
fn map(mut p: Vec3) -> f64 {
  // x axis rotation
  let r = rot2((p.1, p.2), 0.8);
  p = (p.0, r.0, r.1);
  // y axis rotation
  let r = rot2((p.0, p.2), 0.8);
  p = (r.0, p.1, r.1);
  let k = 0.4;
  let d = 1.0;
  f_op_union_round(
    f_box(p, (0.5, 0.5, 0.5)),
    f_op_union_round(
      f_sphere(add3(p, (0.0, d, 0.0)), 0.3),
      f_op_union_round(
        f_sphere(add3(p, (0.0, 0.0, d)), 0.3),
        f_sphere(add3(p, (d, 0.0, 0.0)), 0.3),
        k,
      ),
      k,
    ),
    k,
  )
}

// distance to a sphere
fn f_sphere(p: Vec3, r: f64) -> f64 {
  length3(p) - r
}

// distance to a box
fn f_box(p: Vec3, b: Vec3) -> f64 {
  let d = add3(abs3(p), neg3(b));
  return length3(max3(d, (0.0, 0.0, 0.0))) + vmax3(min3(d, (0.0, 0.0, 0.0)));
}

// apply a rotation on 2d
fn rot2(p: Vec2, a: f64) -> Vec2 {
  add2(mul2f(p, (a).cos()), mul2f((p.1, -p.0), (a).sin()))
}

// this implements lighting of the 3D scene. 2 lights here.
fn lighting(_hit: f64, p: Vec3, n: Vec3, _dir: Vec3) -> f64 {
  let mut c = 0.0;
  let ldir = (-1.0, 1.0, -2.0);
  c += 0.1 + diffuse(p, n, ldir);
  let ldir = (1.0, 0.0, -1.0);
  c += 0.4 * (0.1 + diffuse(p, n, ldir));
  c = clamp(c, 0.0, 1.0);
  return c;
}

// a bunch of vectors helpers (in future, I need a library =D)
type Vec2 = (f64, f64);
type Vec3 = (f64, f64, f64);
fn length3((x, y, z): Vec3) -> f64 {
  (x * x + y * y + z * z).sqrt()
}
fn normalize3(p: Vec3) -> Vec3 {
  let l = length3(p);
  return (p.0 / l, p.1 / l, p.2 / l);
}
fn add2(a: Vec2, b: Vec2) -> Vec2 {
  (a.0 + b.0, a.1 + b.1)
}
fn add3(a: Vec3, b: Vec3) -> Vec3 {
  (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
fn neg3(a: Vec3) -> Vec3 {
  (-a.0, -a.1, -a.2)
}
fn mul3f(a: Vec3, f: f64) -> Vec3 {
  (a.0 * f, a.1 * f, a.2 * f)
}
fn mul2f(a: Vec2, f: f64) -> Vec2 {
  (a.0 * f, a.1 * f)
}
fn normal(p: Vec3) -> Vec3 {
  return normalize3((
    map(add3(p, (0.0005, 0.0, 0.0))) - map(add3(p, (-0.0005, 0.0, 0.0))),
    map(add3(p, (0.0, 0.0005, 0.0))) - map(add3(p, (0.0, -0.0005, 0.0))),
    map(add3(p, (0.0, 0.0, 0.0005))) - map(add3(p, (0.0, 0.0, -0.0005))),
  ));
}
fn clamp(a: f64, from: f64, to: f64) -> f64 {
  (a).max(from).min(to)
}
fn dot3(a: Vec3, b: Vec3) -> f64 {
  a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}
/*
fn cross3((x, y, z): Vec3, (a, b, c): Vec3) -> Vec3 {
  (y * c - z * b, z * a - x * c, x * b - y * a)
}
*/

fn abs3(a: Vec3) -> Vec3 {
  (a.0.abs(), a.1.abs(), a.2.abs())
}
fn diffuse(p: Vec3, n: Vec3, lpos: Vec3) -> f64 {
  let l = normalize3(add3(lpos, neg3(p)));
  let dif = clamp(dot3(n, l), 0.01, 1.);
  return dif;
}
fn vmax3(v: Vec3) -> f64 {
  (v.0).max(v.1).max(v.2)
}
fn min3(a: Vec3, b: Vec3) -> Vec3 {
  (a.0.min(b.0), a.1.min(b.1), a.2.min(b.2))
}
fn max3(a: Vec3, b: Vec3) -> Vec3 {
  (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2))
}

#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }
}

fn clusterize(
  points: Vec<(f64, f64)>,
  clusters: usize,
) -> Vec<Vec<(f64, f64)>> {
  let arr = Array2::from_shape_vec(
    (points.len(), 2),
    points.iter().flat_map(|&(x, y)| vec![x, y]).collect(),
  )
  .unwrap();

  let (means, clusters) = rkm::kmeans_lloyd(&arr.view(), clusters);

  means
    .outer_iter()
    .enumerate()
    .map(|(c, _coord)| {
      clusters
        .iter()
        .enumerate()
        .filter(|(_i, &cluster)| cluster == c)
        .map(|(i, _c)| points[i])
        .collect()
    })
    .collect()
}
