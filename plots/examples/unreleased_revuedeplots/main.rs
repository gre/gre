use clap::*;
use gre::*;
use rand::Rng;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::*;
use svg::parser::Event;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "148.")]
  pub width: f64,
  #[clap(short, long, default_value = "105.")]
  pub height: f64,
  #[clap(short, long, default_value = "8.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut red_data = Vec::new();
  let mut routes = Vec::new();
  let precision = 0.1;
  let mut passage = Passage::new(precision, width, height);
  let bound = (pad, pad, width - pad, height - pad);

  let logo_get_color = image_get_color("../public/logo.jpg").unwrap();
  let f = |p| logo_get_color(p).0;
  let res = contour(100, 100, f, &vec![0.5]);
  let mut logoroutes = features_to_routes(res, 1.0);
  logoroutes = crop_routes(&logoroutes, (0.1, 0.1, 99.9, 99.9));

  let get_color = image_get_color("images/revuedeplots.jpg").unwrap();
  let mut x = 0.0;
  loop {
    if x >= width {
      break;
    }
    let mut y = 0.0;
    loop {
      if y >= height {
        break;
      }
      let (r, g, b) = get_color((x / width, y / height));
      if r < 0.99 {
        passage.count((x, y));
      }
      y += precision;
    }
    x += precision;
  }

  let radius = 6.0;

  passage.grow_passage(radius);

  let does_overlap =
    |p| strictly_in_boundaries(p, bound) && passage.get(p) == 0;
  let cpad = 0.0;
  let min_scale = cpad + 4.0;
  let max_scale = cpad + 12.0;

  let circles = packing(
    opts.seed,
    1000000,
    2000,
    1,
    cpad,
    bound,
    &does_overlap,
    min_scale,
    max_scale,
  );

  for c in circles {
    for route in logoroutes.clone() {
      let mut newroute = Vec::new();
      for p in route {
        let x = c.x + c.r * (p.0 / 100.0 - 0.5);
        let y = c.y + c.r * (p.1 / 100.0 - 0.5);
        newroute.push((x, y));
      }
      routes.push(newroute);
    }
    // routes.push(spiral(c.x, c.y, c.r, 1.0));
  }

  let mut content = String::new();
  for event in svg::open("images/revuedeplots.svg", &mut content).unwrap() {
    match event {
      Event::Tag(Path, _, attributes) => {
        let data = attributes.get("d");
        if let Some(data) = data {
          let data = Data::parse(data).unwrap();
          red_data.push(data.clone());
        }
      }
      _ => {}
    }
  }

  let mut data = Data::new();
  for route in routes.clone() {
    data = render_route(data, route);
  }

  vec![(vec![data], "black"), (red_data, "#900")]
    .iter()
    .enumerate()
    .map(|(i, (all_data, color))| {
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      for data in all_data {
        l = l.add(base_path(color, 0.35, data.clone()));
      }
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

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }

  pub fn count_once(self: &mut Self, p: (f64, f64)) {
    let i = self.index(p);
    let v = self.counters[i];
    if v == 0 {
      self.counters[i] = 1;
    }
  }

  pub fn grow_passage(self: &mut Self, radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters: Vec<usize> = self.counters.iter().cloned().collect();
    let mut mask = Vec::new();
    // TODO, in future for even better perf, I will rewrite this
    // working directly with index integers instead of having to use index() / count_once()
    let mut x = -radius;
    loop {
      if x >= radius {
        break;
      }
      let mut y = -radius;
      loop {
        if y >= radius {
          break;
        }
        if x * x + y * y < radius * radius {
          mask.push((x, y));
        }
        y += precision;
      }
      x += precision;
    }

    let mut x = 0.0;
    loop {
      if x >= width {
        break;
      }
      let mut y = 0.0;
      loop {
        if y >= height {
          break;
        }
        let index = self.index((x, y));
        if counters[index] > 0 {
          for &(dx, dy) in mask.iter() {
            self.count_once((x + dx, y + dy));
          }
        }
        y += precision;
      }
      x += precision;
    }
  }
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y)) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
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
