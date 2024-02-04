use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  width: f64,
  #[clap(short, long, default_value = "210.0")]
  height: f64,
  #[clap(short, long, default_value = "20.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
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

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();
  let min = 3.0;
  let max = 4.0;
  let circles = packing(
    opts.seed,
    200000,
    5000,
    1,
    0.0,
    (pad, pad, width - pad, height - pad),
    &VCircle::new(width / 2., height / 2., height),
    min,
    max,
  );

  let perlin = Perlin::new();

  let freq = rng.gen_range(0.01, 0.05);
  let freq2 = rng.gen_range(0.01, 0.03);
  let angleamp = rng.gen_range(1.0, 2.0);

  for c in circles.clone() {
    let mut local: Vec<_> = vec![];
    let cr = 0.2;
    let border = 0.3;
    let size = c.r * 1.2;
    let arrowy = 0.2;
    let arrowx = 0.4;
    let n2 = perlin.get([
      // angle follow a noise field
      freq2 * c.x,
      freq2 * c.y,
      100. + opts.seed / 0.3794,
    ]);
    let n = perlin.get([
      // angle follow a noise field
      freq * c.x,
      freq * c.y,
      100. + opts.seed / 0.03794 + n2,
    ]);
    let ang = angleamp * n;
    let clr = if n2 > 0.0 { 0 } else { 1 };
    local.push(circle_route(
      (-1.0 + border + cr, 0.0),
      cr,
      8 + c.r as usize,
    ));
    local.push(vec![(-1.0 + border + 2. * cr, 0.0), (1.0 - border, 0.0)]);
    local.push(vec![
      (1.0 - border - arrowx, -arrowy),
      (1.0 - border, 0.0),
      (1.0 - border - arrowx, arrowy),
    ]);
    let acos = ang.cos();
    let asin = ang.sin();
    routes.extend(local.iter().map(|rt| {
      (
        clr,
        rt.iter()
          .map(|(x, y)| {
            // rotate
            let p = (x * acos - y * asin, x * asin + y * acos);
            // scale
            let p = (p.0 * size, p.1 * size);
            // translate
            let p = (p.0 + c.x, p.1 + c.y);
            p
          })
          .collect(),
      )
    }));
  }

  vec!["red", "blue"]
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let mut data = Data::new();
      for (ci, route) in routes.clone() {
        if i == ci {
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
