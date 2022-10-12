use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "148.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn crab<R: Rng>(
  origin: (f64, f64),
  scale: f64,
  rot: f64,
  rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = vec![];

  // MAKE THE BODY
  let bodycenter = (0.5, -0.6);
  let body = spiral_ovale(
    bodycenter.0,
    bodycenter.1,
    rng.gen_range(0.2, 0.3),
    0.7,
    rng.gen_range(0.3, 1.0) / scale,
    rng.gen_range(0.08, 0.18) / scale,
  );
  routes.push(body);

  // MAKE THE EYES
  let eyes = vec![rng.gen_range(-0.15, -0.05), rng.gen_range(0.05, 0.15)];
  for dx in eyes {
    routes.push(vec![(0.5 + dx, -0.8), (0.5 + dx * 2.0, -0.95)]);
  }

  // MAKE THE PAWS
  let paws = vec![-1.3, -1.0, -0.6, 0.6, 1.0, 1.3];
  for da in paws {
    let ang = PI / 2.0 + da + rng.gen_range(-0.3, 0.3);
    let ax = ang.cos();
    let ay = ang.sin();
    routes.push(vec![
      (bodycenter.0 + 0.2 * ax, bodycenter.1 + 0.2 * ay),
      (bodycenter.0 + 0.4 * ax, bodycenter.1 + 0.4 * ay),
      (
        bodycenter.0 + rng.gen_range(0.4, 0.6) * ax,
        (bodycenter.1 + rng.gen_range(0.4, 0.6) * ay + rng.gen_range(0.1, 0.3))
          .min(0.0),
      ),
    ]);
  }

  // MAKE THE PLIERS
  let ampincr = rng.gen_range(0.2, 0.3);
  let arms = vec![-1.0, 1.0];
  for arm in arms {
    let mut ang = -PI / 2.0 + arm + rng.gen_range(-0.2, 0.2);
    let mut route = Vec::new();
    let mut amp = rng.gen_range(0.0, 0.2);
    for _i in 0..3 {
      ang += rng.gen_range(-0.3, 0.3) * amp;
      let ax = ang.cos();
      let ay = ang.sin();
      route.push((bodycenter.0 + amp * ax, bodycenter.1 + amp * ay));
      amp = 1.1 * amp + ampincr;
    }
    routes.push(route.clone());
    routes.push(route.iter().map(|&p| (p.0, p.1 - 0.05)).collect());
    let mut last = route[route.len() - 1];
    last.1 -= 0.025;
    for i in 0..2 {
      let p = i as f64 - 0.5;
      let a = ang + p;
      let a2 = ang + rng.gen_range(0.05, 0.9) * p;
      let ax = a.cos();
      let ay = a.sin();
      let ax2 = a2.cos();
      let ay2 = a2.sin();
      let route = vec![
        last,
        (
          last.0 + rng.gen_range(0.2, 0.4) * ax,
          last.1 + rng.gen_range(0.2, 0.4) * ay,
        ),
        (
          last.0 + rng.gen_range(0.4, 0.6) * ax2,
          last.1 + rng.gen_range(0.4, 0.6) * ay2,
        ),
      ];
      // duplicate multiple time to make it bolder and random
      for _i in 0..rng.gen_range(2, 4) {
        routes.push(path_subdivide_to_curve(
          shake(route.clone(), 0.05, rng),
          1,
          0.8,
        ));
      }
    }
  }

  // scale, rotate & translate
  let routes: Vec<Vec<(f64, f64)>> = routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&p| {
          let p = p_r(p, rot);
          round_point((scale * p.0 + origin.0, scale * p.1 + origin.1), 0.01)
        })
        .collect()
    })
    .collect();

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  // Prepare all the random values
  let mut rng = rng_from_seed(opts.seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let precision = rng.gen_range(0.1, 0.3);
  let mut passage = Passage::new(0.5, width, height);
  let passage_threshold = 8;

  let mut height_map: Vec<f64> = Vec::new();
  let mut crabs = Vec::new();

  let plain_field_spawn_rate = if rng.gen_bool(0.2) {
    rng.gen_range(0.0, 0.01)
  } else {
    0.0
  };

  let expad = pad + 20.0;

  let mut crabs_passage = Passage::new(rng.gen_range(15., 30.), width, height);

  for _g in 0..((1. + rng.gen_range(0., 30.) * rng.gen_range(0., 1.)) as usize)
  {
    let peakfactor = rng.gen_range(-0.001, 0.001) * rng.gen_range(0.0, 1.0);
    let stopy =
      (0.5 + rng.gen_range(-0.4, 0.5) * rng.gen_range(0.0, 1.0)) * height;
    let ampfactor = rng.gen_range(0.0, 0.1);
    let ynoisefactor = rng.gen_range(0.0, 0.1);
    let yincr = rng.gen_range(0.5, 8.0);
    let xfreq = rng.gen_range(0.0, 0.03);
    let amp2 = rng.gen_range(0.0, 8.0);
    let offsetstrategy = rng.gen_range(0, 5);
    let offsetx = rng.gen_range(-0.5, 0.5)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height * 5.0;
    let mut miny = height;
    loop {
      if miny < stopy {
        break;
      }

      let mut route = Vec::new();
      let mut x = pad;
      let mut was_outside = true;
      loop {
        if x > width - pad {
          break;
        }
        let xv = (4.0 - base_y / height) * (x - width / 2. + offsetx * width);

        let amp = height * ampfactor;
        let xx = (x - pad) / (width - 2. * pad);
        let xborderd = xx.min(1.0 - xx);
        let displacement = amp * peakfactor * (xv * xv - (xborderd).powf(0.5));

        let mut y = base_y;

        if offsetstrategy == 0 {
          y += displacement;
        }

        y += -amp
          * perlin
            .get([
              //
              xv * xfreq + 9.9,
              y * 0.02 - 3.1,
              77.
                + opts.seed / 7.3
                + perlin.get([
                  //
                  -opts.seed * 7.3,
                  8.3 + xv * 0.015,
                  y * 0.1,
                ]),
            ])
            .abs();

        if offsetstrategy == 1 {
          y += displacement;
        }

        y += amp2
          * amp
          * perlin.get([
            //
            8.3 + xv * 0.008,
            88.1 + y * ynoisefactor,
            opts.seed * 97.3,
          ]);

        if offsetstrategy == 2 {
          y += displacement;
        }

        y += amp
          * perlin.get([
            //
            opts.seed * 9.3 + 77.77,
            xv * 0.08 + 9.33,
            y * 0.5,
          ])
          * perlin
            .get([
              //
              xv * 0.015 - 88.33,
              88.1 + y * 0.2,
              -opts.seed / 7.7 - 6.66,
            ])
            .min(0.0);

        if offsetstrategy == 3 {
          y += displacement;
        }

        y += 0.1
          * amp
          * (1.0 - miny / height)
          * perlin.get([
            //
            6666.6 + opts.seed * 1.3,
            8.3 + xv * 0.5,
            88.1 + y * 0.5,
          ]);

        if offsetstrategy == 4 {
          y += displacement;
        }

        if y < miny {
          miny = y;
        }
        let mut collides = false;
        let xi = (x / precision) as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let in_bounds =
          pad < x && x < width - pad && pad < y && y < height - pad;
        let inside = !collides && in_bounds;
        if inside && passage.get((x, y)) < passage_threshold {
          if was_outside {
            if route.len() > min_route {
              routes.push(route);
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push(round_point((x, y), 0.01));
          passage.count((x, y));
        } else {
          was_outside = true;
        }

        let in_extended_bounds = in_bounds
          && expad < x
          && x < width - expad
          && expad < y
          && y < height - expad;

        let p = (x, y);
        if in_extended_bounds
          && !collides
          && plain_field_spawn_rate > 0.0
          && rng.gen_bool(plain_field_spawn_rate)
          && crabs_passage.get(p) == 0
        {
          crabs_passage.count(p);
          let scale = rng.gen_range(8.0, 10.0);
          let rot = 0.3
            * rng.gen_range(-PI, PI)
            * rng.gen_range(0.0, 1.0)
            * rng.gen_range(0.0, 1.0);
          crabs.push(crab(p, scale, rot, &mut rng));
        }

        x += precision;
      }

      if route.len() > min_route {
        routes.push(route);
      }

      base_y -= yincr;
    }

    // We use a "smooth average" algorithm to ignore the sharp edges of the mountains
    let smooth = rng.gen_range(10, 40);
    let sf = smooth as f64;
    let mut sum = 0.0;
    let mut acc = Vec::new();
    let mut smooth_heights = Vec::new();
    for (i, h) in height_map.iter().enumerate() {
      if acc.len() == smooth {
        let avg = sum / sf;
        let xtheoric = (i as f64 - sf / 2.0) * precision;
        let l = smooth_heights.len();
        let b = (xtheoric, avg, 0.0);
        let a = if l > 2 { smooth_heights[l - 2] } else { b };
        let rot = -PI / 2.0 + (b.0 - a.0).atan2(b.1 - a.1);
        let p = (xtheoric, avg, rot);
        smooth_heights.push(p);
        let prev = acc.remove(0);
        sum -= prev;
      }
      acc.push(h);
      sum += h;
    }

    for _i in 0..((rng.gen_range(0., 10.0) * rng.gen_range(0.0, 1.0)) as usize)
    {
      let (x, y, a) = smooth_heights[rng.gen_range(0, smooth_heights.len())];
      let p = (x, y);
      let in_extended_bounds = expad < p.0
        && p.0 < width - expad
        && expad < p.1
        && p.1 < height - expad;
      if in_extended_bounds && crabs_passage.get(p) == 0 {
        crabs_passage.count(p);
        let scale = rng.gen_range(8.0, 10.0);
        let rot = a;
        crabs.push(crab(p, scale, rot, &mut rng));
      }
    }
  }

  // Border around the postcard
  let border_size = 8;
  let border_dist = 0.3;
  let mut route = Vec::new();
  for i in 0..border_size {
    let d = i as f64 * border_dist;
    route.push((pad + d, pad + d));
    route.push((pad + d, height - pad - d));
    route.push((width - pad - d, height - pad - d));
    route.push((width - pad - d, pad + d));
    route.push((pad + d, pad + d));
  }
  routes.push(route);

  let routes_second = crabs.concat();

  // Make the SVG
  vec![("black", routes, 0.35), ("#FB0", routes_second, 0.5)]
    /*
    // DEBUG
    let mut rng = rng_from_seed(opts.seed);
    let mut all = vec![];
    for i in 0..8 {
      for j in 0..6 {
        all.push(crab(
          (
            width * ((0.4 + i as f64) / 8.),
            height * ((1.25 + j as f64) / 7.),
          ),
          6.0,
          0.0,
          &mut rng,
        ));
      }
    }
    let routes = all.concat();
    vec![("black", routes, 0.5)]
    */
    .iter()
    .map(|(color, routes, stroke_width)| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(color);
      l = l.add(base_path(color, *stroke_width, data));
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
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

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

fn shake<R: Rng>(
  path: Vec<(f64, f64)>,
  scale: f64,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  path
    .iter()
    .map(|&(x, y)| {
      let dx = rng.gen_range(-scale, scale);
      let dy = rng.gen_range(-scale, scale);
      (x + dx, y + dy)
    })
    .collect()
}

fn spiral_ovale(
  x: f64,
  y: f64,
  radius: f64,
  wmul: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = (x + wmul * r * a.cos(), y + r * a.sin());
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < approx {
      break;
    }
  }
  route
}
