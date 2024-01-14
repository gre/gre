use clap::*;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn sdf_circle(p: (f64, f64), center: (f64, f64), r: f64) -> f64 {
  let ux = p.0 - center.0;
  let uy = p.1 - center.1;
  (ux * ux + uy * uy).sqrt() - r
}

fn sdf_ndot(a: (f64, f64), b: (f64, f64)) -> f64 {
  a.0 * b.0 - a.1 * b.1
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
  if x < min {
    min
  } else if x > max {
    max
  } else {
    x
  }
}

fn dot(a: (f64, f64), b: (f64, f64)) -> f64 {
  a.0 * b.0 + a.1 * b.1
}

fn length(a: (f64, f64)) -> f64 {
  let (x, y) = a;
  (x * x + y * y).sqrt()
}

// based on https://iquilezles.org/articles/distfunctions2d/
fn sdf_rhombus(p: (f64, f64), b: (f64, f64)) -> f64 {
  let p = (p.0.abs(), p.1.abs());
  let h = clamp(
    sdf_ndot((b.0 - 2. * p.0, b.1 - 2. * p.1), b) / dot(b, b),
    -1.0,
    1.0,
  );
  let d = length((p.0 - 0.5 * b.0 * (1.0 - h), p.1 - 0.5 * b.1 * (1.0 + h)));
  d * (p.0 * b.1 + p.1 * b.0 - b.0 * b.1).signum()
}

trait BinaryOps<T> {
  fn intersect(&self, other: T) -> T;
  fn difference(&self, other: T) -> T;
  fn union(&self, other: T) -> T;
  fn smooth_intersect(&self, k: T, other: T) -> T;
  fn smooth_difference(&self, k: T, other: T) -> T;
  fn smooth_union(&self, k: T, other: T) -> T;
}

impl BinaryOps<f64> for f64 {
  fn intersect(&self, other: f64) -> f64 {
    self.max(other)
  }
  fn difference(&self, other: f64) -> f64 {
    self.max(-other)
  }
  fn union(&self, other: f64) -> f64 {
    self.min(other)
  }

  fn smooth_intersect(&self, k: f64, other: f64) -> f64 {
    let h = (0.5 - 0.5 * (self - other) / k).max(0.0).min(1.0);
    mix(*self, other, h) + k * h * (1.0 - h)
  }

  fn smooth_difference(&self, k: f64, other: f64) -> f64 {
    let h = (0.5 - 0.5 * (other + self) / k).max(0.0).min(1.0);
    mix(*self, -other, h) + k * h * (1.0 - h)
  }

  fn smooth_union(&self, k: f64, other: f64) -> f64 {
    let h = (0.5 + 0.5 * (self - other) / k).max(0.0).min(1.0);
    mix(*self, other, h) - k * h * (1.0 - h)
  }
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();

  let line_d = 0.7;
  let stripediv = 2;
  let available_height = height - 2. * pad;
  let available_width = width - 2. * pad;
  let gridw = rng.gen_range(30, 50);
  let gridh = (available_height / available_width * (gridw as f64)) as usize;

  let cellh = available_height / (gridh as f64);
  let cellw = available_width / (gridw as f64);

  let mut stripes = vec![false; gridw * gridh];
  let mut grid1 = vec![0; gridw * gridh];
  let mut grid2 = vec![0; gridw * gridh];

  let mut circles = vec![];
  for _ in 0..rng.gen_range(2, 7) {
    let x = rng.gen_range(-0.1, 0.1);
    let y = rng.gen_range(-0.2, 0.2);
    let r = rng.gen_range(0.05, 0.1);
    circles.push(((x, y), r));
  }

  let smoothk = 0.1;

  let sdivy = (2.
    + rng.gen_range(0.0, 20.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;
  let sdivx = (2.
    + rng.gen_range(0.0, 20.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;
  let sdivy2 = (2.
    + rng.gen_range(0.0, 20.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;
  let sdivx2 = (2.
    + rng.gen_range(0.0, 20.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;
  let sk1 = (2. + rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0)) as usize;
  let sk2 = (2. + rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0)) as usize;

  let threshold1 =
    (1. + rng.gen_range(0., sk1 as f64) * rng.gen_range(0.0, 1.0)) as usize;
  let threshold2 =
    (1. + rng.gen_range(0., sk2 as f64) * rng.gen_range(0.0, 1.0)) as usize;

  let mut clrs = (0..4).collect::<Vec<_>>();
  rng.shuffle(&mut clrs);

  let mut off = 0;
  let pc2offset = rng.gen_range(-0.02f64, 0.03)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);

  for x in 0..gridw {
    for y in 0..gridh {
      let i = x + y * gridw;

      let p = (
        x as f64 / (gridw as f64) - 0.5,
        y as f64 / (gridh as f64) - 0.5,
      );

      let c2d1 = rng.gen_range(1, 7);
      let c2d2 = rng.gen_range(1, 7);
      let c2d3 = rng.gen_range(1, 7);
      let c2k = rng.gen_range(2, 5);

      let mut d1 = 9999f64;
      for c in &circles {
        d1 = d1.smooth_union(smoothk, sdf_circle(p, c.0, c.1));
      }

      let d2 = sdf_rhombus(p, (0.15, 0.3)) - 0.15;

      let d3 = sdf_circle((p.0, p.1.abs()), (0.0, 0.5), 0.2);

      let cap = d3 < 0.0 && d2 < 0.0;

      let c = if cap {
        clrs[0]
      } else if d1 < 0. {
        clrs[1]
      } else if d2 < 0. {
        clrs[2]
      } else {
        clrs[3]
      };

      let mut c1 = c % 3;
      let mut c2 = if c == 3 {
        (i / c2d1 + x / c2d2 + y / c2d3) % c2k
      } else {
        (c1 + off) % 4
      };

      if pc2offset > 0. && rng.gen_bool(pc2offset) {
        if off > 0 && rng.gen_bool(0.5) {
          off -= 1;
        } else {
          off += 1;
        }
      }

      if rng.gen_bool(0.01) {
        c2 = c1;
      } else if rng.gen_bool(0.01) {
        c1 = c2;
      }

      grid1[i] = c1;
      grid2[i] = c2;
      stripes[i] = (x / sdivx + y / sdivy) % sk1 < threshold1
        && (x / sdivx2 + y / sdivy2) % sk2 < threshold2;
    }
  }

  let mut x = pad;
  while x < width - pad {
    let xi = (gridw as f64 * (x - pad) / (width - 2. * pad)) as usize;
    for yi in 1..(gridh + 1) {
      let y1 = pad + (yi - 1) as f64 * cellh;
      let y2 = pad + yi as f64 * cellh;
      let clr = grid1[xi + (yi - 1) * gridw];
      routes.push((clr, vec![(x, y1), (x, y2)]));
    }
    x += line_d;
  }

  let mut y = pad;
  let mut linei = 0;
  while y < height - pad {
    let yi = (gridh as f64 * (y - pad) / (height - 2. * pad)) as usize;
    for xi in 1..(gridw + 1) {
      let x1 = pad + (xi - 1) as f64 * cellw;
      let x2 = pad + xi as f64 * cellw;
      let stripe = stripes[xi - 1 + yi * gridw];
      let clr = grid2[xi - 1 + yi * gridw];
      let alt = (linei / stripediv) % 2 == 0;
      let clr = if stripe && alt { (clr + 1) % 4 } else { clr };
      routes.push((clr, vec![(x1, y), (x2, y)]));
    }
    y += line_d;
    linei += 1;
  }

  let mut colors = vec![
    "#0f9", // soft mint
    // "#bbb", // moonstone
    // "#f60", // pumpkin
    "#fc2", // amber
    // "#224", // indigo
    "#f33", // poppy red
    "#00c", // blue
  ];
  rng.shuffle(&mut colors);
  colors.truncate(3);

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
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
