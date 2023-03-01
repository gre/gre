use clap::*;
use gre::letters::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "148.5")]
  pub width: f64,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "5.0")]
  pub size: f64,
  #[clap(short, long, default_value = "0.1")]
  pub letter_precision: f64,
  #[clap(short, long, default_value = "1.0")]
  pub non_attached_pad: f64,
  #[clap(short, long, default_value = "0.3")]
  pub density_mul: f64,
  #[clap(short, long, default_value = "10")]
  pub seconds: i64,
  #[clap(short, long, default_value = "images/letters.svg")]
  letters_file: String,
  #[clap(
    short,
    long,
    default_value = "can an experiment exist without the intent to prove something "
  )]
  text: String,
  #[clap(short, long)]
  debug: bool,
}

fn main() {
  let opts: Opts = Opts::parse();
  let letters_ref = LetterSvgReferential::new(
    opts.letters_file.clone(),
    opts.letter_precision,
    opts.non_attached_pad,
  );
  let groups = art(&opts, &letters_ref);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn art(opts: &Opts, letters_ref: &LetterSvgReferential) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;

  let mut rng = rng_from_seed(opts.seed);

  let mut circles = packing(
    3.3 * opts.seed,
    1000000,
    1000,
    rng.gen_range(1, 4),
    0.0,
    (pad, pad, width - pad, height - pad),
    8.0,
    opts.width,
  );

  let points: Vec<(f64, f64)> = circles.iter().map(|c| (c.x, c.y)).collect();

  let tour = travelling_salesman::simulated_annealing::solve(
    &points,
    time::Duration::seconds(opts.seconds),
  );

  circles = tour.route.iter().map(|&i| circles[i]).collect();

  let route: Vec<(f64, f64)> = circles
    .iter()
    .flat_map(|circle| {
      let s = opts.seed + circle.x * 3.1 + circle.y / 9.8;
      let mut rng = rng_from_seed(s);
      let samples = 1 + ((circle.r * opts.density_mul) as usize);
      shape_strokes_random(&mut rng, circle, samples)
    })
    .collect();

  let len = curve_length(&route);

  let mut routes = vec![];

  let text = opts.text.clone();
  let yoffset = -size * 0.7;
  let mut i = 0.0;
  loop {
    let measure = measure_text(&letters_ref, text.clone(), size);
    if i + measure > len {
      break;
    }
    let res = draw_text(&letters_ref, text.clone(), size, i, yoffset, &route);

    routes.extend(res.0);

    i += measure;
  }

  vec![("black", routes)]
    .iter()
    .map(|(color, routes)| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

pub fn abs_angle(a: f64) -> f64 {
  ((2.0 * PI) + (a % (2.0 * PI))) % (2.0 * PI)
}
pub fn angle_delta(a: f64, b: f64) -> f64 {
  let delta = (abs_angle(a) - abs_angle(b)).abs();
  let sign = if abs_angle(a) > abs_angle(b) || delta >= PI {
    -1.0
  } else {
    1.0
  };
  (PI - (delta - PI).abs()) * sign
}

pub fn slice_on_sharp_edges(
  path: &Vec<(f64, f64)>,
  max_angle: f64,
) -> Vec<Vec<(f64, f64)>> {
  if path.len() < 3 {
    return vec![path.clone()];
  }
  let mut slices = Vec::new();
  let mut slice = Vec::new();
  let (x1, y1) = path[0];
  let (x2, y2) = path[1];
  let mut last_angle = (y2 - y1).atan2(x2 - x1);
  slice.push(path[0]);
  let mut prev = path[0];
  for &p in path.iter().skip(1) {
    let (x1, y1) = prev;
    let (x2, y2) = p;
    let angle = (y2 - y1).atan2(x2 - x1);
    let diff = angle_delta(angle, last_angle);
    if diff.abs() > max_angle {
      if slice.len() > 1 {
        slices.push(slice);
      }
      slice = vec![prev, p];
    } else {
      slice.push(p);
    }
    last_angle = angle;
    prev = p;
  }
  if slice.len() > 1 {
    slices.push(slice);
  }
  slices
}

pub fn draw_text(
  letter_ref: &LetterSvgReferential,
  text: String,           // text to draw
  size: f64,              // font size
  xstart: f64,            // x move on the path
  yoffset: f64,           // make diff baseline
  path: &Vec<(f64, f64)>, // curve to follow
) -> (Vec<Vec<(f64, f64)>>, f64) {
  let mut routes = Vec::new();
  let mut x = 0.;
  let mut y = 0.;
  let mut can_attach = true;
  let mut last: Vec<(f64, f64)> = vec![];
  for c in text.chars() {
    if let Some(letter) = letter_ref.get_letter(&c.to_string()) {
      let (rts, (dx, dy)) = letter.render((x, y), size, false);
      if letter.can_attach && can_attach {
        let mut rts = rts.clone();

        let mut add = rts.pop().unwrap();
        // interpolate curve to attach more smoothly
        if last.len() > 0 {
          let lastp = last[last.len() - 1];
          let firstp = add[0];
          // ygap between last and first
          let ygap = firstp.1 - lastp.1;
          let mut i = 1;
          let mut maxlen = 0.5 * size;
          while i < add.len() {
            if maxlen < 0. {
              break;
            }
            let l = euclidian_dist(add[i - 1], add[i]);
            if ygap > 0.0 {
              if add[i].1 < lastp.1 {
                break;
              }
            } else {
              if add[i].1 > lastp.1 {
                break;
              }
            }
            i += 1;
            maxlen -= l;
          }
          if i == add.len() {
            i -= 1;
          }
          let stopi = i;
          add = add
            .iter()
            .enumerate()
            .map(|(i, &p)| {
              if i <= stopi {
                let y = p.1 - ygap * (1.0 - i as f64 / stopi as f64);
                (p.0, y)
              } else {
                p
              }
            })
            .collect();
        }

        last.extend(add);

        routes.extend(rts); // ° on i and j
      } else {
        if last.len() > 0 {
          routes.push(last);
          last = vec![];
        }
        routes.extend(rts);
      }
      can_attach = letter.can_attach;
      x += dx;
      y += dy;
    } else {
      println!("letter not found: {}", c);
    }
  }
  if last.len() > 0 {
    routes.push(last);
  }

  // rotate with angle and translate to origin all routes
  let mut proj_routes = Vec::new();
  for route in routes {
    let mut proj_route = Vec::new();
    for (x, y) in route {
      // use x to find position in path and project x,y
      let (origin, a) = lookup_curve_point_and_angle(&path, x + xstart);

      let y = y + yoffset;
      let disp = (-y * a.sin(), y * a.cos());

      let p = (origin.0 + disp.0, origin.1 + disp.1);

      proj_route.push(p);
    }
    proj_routes.push(proj_route);
  }

  (proj_routes, x)
}

fn angle2(p1: (f64, f64), p2: (f64, f64)) -> f64 {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  let dx = x2 - x1;
  let dy = y2 - y1;
  dy.atan2(dx)
}

fn curve_length(path: &Vec<(f64, f64)>) -> f64 {
  let mut len = 0.0;
  for i in 0..path.len() - 1 {
    len += euclidian_dist(path[i], path[i + 1]);
  }
  len
}

fn measure_text(
  letter_ref: &LetterSvgReferential,
  text: String,
  size: f64,
) -> f64 {
  let mut x = 0.;
  for c in text.chars() {
    if let Some(letter) = letter_ref.get_letter(&c.to_string()) {
      let (dx, _dy) = letter.render((x, 0.0), size, false).1;
      x += dx;
    }
  }
  x
}

fn lookup_curve_point_and_angle(
  path: &Vec<(f64, f64)>,
  l: f64,
) -> ((f64, f64), f64) {
  let mut i = 0;
  if l < 0.0 {
    return (path[0], angle2(path[0], path[1]));
  }
  let mut len = 0.0;
  while i < path.len() - 1 {
    let l1 = euclidian_dist(path[i], path[i + 1]);
    if len + l1 > l {
      let r = (l - len) / l1;
      let x = path[i].0 + r * (path[i + 1].0 - path[i].0);
      let y = path[i].1 + r * (path[i + 1].1 - path[i].1);
      let angle = angle2(path[i], path[i + 1]);
      return ((x, y), angle);
    }
    len += l1;
    i += 1;
  }
  return (
    path[path.len() - 1],
    angle2(path[path.len() - 2], path[path.len() - 1]),
  );
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
  bound: (f64, f64, f64, f64),
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    bound.0 < c.x - c.r
      && c.x + c.r < bound.2
      && bound.1 < c.y - c.r
      && c.y + c.r < bound.3
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
  bound: (f64, f64, f64, f64),
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
      search_circle_radius(bound, &circles, x, y, min_scale, max_scale)
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

fn shape_strokes_random<R: Rng>(
  rng: &mut R,
  c: &VCircle,
  samples: usize,
) -> Vec<(f64, f64)> {
  let samples = sample_2d_candidates_f64(
    &|p| {
      let dx = p.0 - 0.5;
      let dy = p.1 - 0.5;
      let d2 = dx * dx + dy * dy;
      if d2 > 0.25 {
        0.0
      } else {
        d2
      }
    },
    (6. * c.r) as usize,
    samples,
    rng,
  );
  samples
    .iter()
    .map(|(x, y)| (2.0 * c.r * (x - 0.5) + c.x, 2.0 * c.r * (y - 0.5) + c.y))
    .collect()
}
