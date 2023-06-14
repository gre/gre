use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes(
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

  for (c, input_route) in input_routes.iter() {
    if input_route.len() < 2 {
      continue;
    }
    let clr = *c;

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

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "400.0")]
  pub width: f64,
  #[clap(short, long, default_value = "500.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "200.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);

  let mut mask = PaintMask::new(0.1, width, height);

  let min = 3.0;
  let max = min + 40.0;
  let internal_pad = pad + mix(min, max, 0.4);
  let diff = min * rng.gen_range(0.5, 1.5);

  // centers of the squares
  let mut centers = vec![];
  let mut x = internal_pad;
  loop {
    if x > width - internal_pad {
      break;
    }
    let mut y = internal_pad;
    loop {
      if y > height - internal_pad {
        break;
      }
      centers.push((x, y));
      y += diff;
    }
    x += diff;
  }
  rng.shuffle(&mut centers);

  // we use perlin noise for the randomness on context of this 'Noise' theme
  let perlin = Perlin::new();
  let f = rng.gen_range(0.5, 3.0) * rng.gen_range(0.2, 1.0);
  let f2 = rng.gen_range(0.0, 0.2);
  let amp2 = rng.gen_range(5.0, 15.0);
  let angoff = rng.gen_range(-PI, PI);
  let img = image_get_color(
    "images/greweb_abstract_artist_using_computers_and_pens_hat_glasses.png",
  )
  .unwrap();

  let vratio = rng.gen_range(0.0, 0.8) * rng.gen_range(0.0, 1.0);

  let mut routes = Vec::new(); // all the paths to draw are stored here
  for c in centers {
    let v =
      grayscale(img((-0.1 + 1.2 * c.0 / width, -0.2 + 1.4 * c.1 / height)));
    let size = mix(
      min,
      max,
      mix(
        (0.5
          + 0.5
            * perlin.get([
              666. + 30.1 * opts.seed,
              f * c.0 / width,
              f * c.1 / width,
            ]))
        .powf(2.0),
        v.powf(2.0),
        vratio,
      ),
    );
    let clr = 0;
    let g =
      grayscale(img((-0.05 + 1.1 * c.0 / width, -0.2 + 1.2 * c.1 / height)));

    let square = (-size / 2., -size / 2., size / 2., size / 2.);
    let mut all = vec![];

    let dist_to_edge = (c.0 - internal_pad)
      .min(c.1 - internal_pad)
      .min(width - c.0 - internal_pad)
      .min(height - c.1 - internal_pad);

    let dist_to_edge_norm = dist_to_edge / ((width - 2. * internal_pad) / 2.);

    let ang = dist_to_edge_norm
      * (angoff
        + amp2
          * PI
          * perlin.get([f2 * c.0 / width, f2 * c.1 / width, opts.seed / 7.7]));

    let poly = vec![
      (square.0, square.1),
      (square.0, square.3),
      (square.2, square.3),
      (square.2, square.1),
      (square.0, square.1),
    ]
    .iter()
    .map(|&p| {
      let p = p_r(p, ang);
      (p.0 + c.0, p.1 + c.1)
    })
    .collect::<Vec<(f64, f64)>>();

    // we draw the square border
    // all.push((clr, poly.clone()));

    // we sometimes fill it completely with zig-zag lines
    if g < 0.9 {
      let mut route = vec![];
      let incr = 0.3 + g.powf(1.2) * 2.0;
      let pad = 0.0;
      let mut y = square.1 + pad;
      let mut reverse = false;
      while y < square.3 - pad / 2.0 {
        if !reverse {
          route.push((square.0 + pad, y));
          route.push((square.2 - pad, y));
        } else {
          route.push((square.2 - pad, y));
          route.push((square.0 + pad, y));
        }
        y += incr;
        reverse = !reverse;
      }
      route = route
        .iter()
        .map(|&p| {
          let p = p_r(p, ang);
          (p.0 + c.0, p.1 + c.1)
        })
        .collect::<Vec<(f64, f64)>>();
      all.push((clr, route));
    }

    // used for collision detection
    let is_outside =
      |p: (f64, f64)| !strictly_in_boundaries(p, bound) || mask.is_painted(p);
    routes.extend(clip_routes(&all, &is_outside, 1., 4));

    mask.paint_polygon(&poly);
  }

  vec!["black"]
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
      l = l.add(base_path(color, 0.3, data));
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
