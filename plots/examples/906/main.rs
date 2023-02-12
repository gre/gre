use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
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
  #[clap(short, long, default_value = "185.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  let mut rng = rng_from_seed(opts.seed);

  let min = rng.gen_range(2.0, 8.0);
  let max = min + rng.gen_range(5.0, 40.0) * rng.gen_range(0.1, 1.0);
  let internal_pad = max + 2.3;
  let diff = rng.gen_range(3.0f64, 8.0);

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
  let f = rng.gen_range(0.01, 0.2) * rng.gen_range(0.0, 1.0);
  let f2 = rng.gen_range(0.01, 0.8) * rng.gen_range(0.0, 1.0);
  let f3 = rng.gen_range(0.01, 1.0) * rng.gen_range(0.5, 1.0);
  let fill_threshold = 0.3;

  let mut squares = vec![]; // used for collision detection of the clipping logic
  let mut routes = Vec::new(); // all the paths to draw are stored here
  for c in centers {
    let size = mix(
      min,
      max,
      (0.5 + 0.5 * perlin.get([666. + 30.1 * opts.seed, f * c.0, f * c.1]))
        .powf(2.0),
    );
    let clr = mix(
      0.0,
      2.0,
      0.5 + 0.5 * perlin.get([44. + opts.seed / 0.3, f2 * c.0, f2 * c.1]),
    ) as usize
      % 2;
    let fill = perlin.get([opts.seed, f3 * c.0, f3 * c.1]) > fill_threshold;

    let square = (
      c.0 - size / 2.,
      c.1 - size / 2.,
      c.0 + size / 2.,
      c.1 + size / 2.,
    );
    let mut all = vec![];

    // we draw the square border
    all.push((
      clr,
      vec![
        (square.0, square.1),
        (square.0, square.3),
        (square.2, square.3),
        (square.2, square.1),
        (square.0, square.1),
      ],
    ));
    // we sometimes fill it completely with zig-zag lines
    if fill {
      let mut route = vec![];
      let incr = 0.5;
      let pad = 0.2;
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
      all.push((clr, route));
    }

    // used for collision detection
    let is_outside = |p: (f64, f64)| {
      !strictly_in_boundaries(p, bound)
        || squares.iter().any(|s| strictly_in_boundaries(p, *s))
    };
    routes.extend(clip_routes(&all, &is_outside, 1., 4));
    squares.push(square);
  }

  vec!["silver", "gold"]
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
