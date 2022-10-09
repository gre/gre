use clap::*;
use geo::prelude::*;
use geo::*;
use gre::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut routes = Vec::new(); // where we put all our line paths

  // randomness
  let mut rng = rng_from_seed(opts.seed);
  let layering = 3;
  let voronoi_size =
    (3.0 + rng.gen_range(0.0, 100.0) * rng.gen_range(0.0, 1.0)) as usize;
  let cpad = rng.gen_range(0.8, 1.4);
  let min_scale = cpad + rng.gen_range(1.2, 1.6);
  let max_scale = min_scale + rng.gen_range(2.0, 6.0);
  let bound = (pad, pad, width - pad, height - pad);
  let border_effect = height * rng.gen_range(0.1, 0.3);
  let threshold_factor = rng.gen_range(0.0, 1.0);
  let cpad_mul_ext = rng.gen_range(1.2, 1.5);
  let cpad_mul_int = rng.gen_range(0.7, 0.9);

  // STEP 1: we sample random points

  let candidates = sample_2d_candidates_f64(
    &|p| 1.2 - euclidian_dist((0.5, 0.5), p), // more likely to sample on center
    800,
    voronoi_size,
    &mut rng,
  );

  // STEP 2: we generate voronoi with these points

  let mut polys = sample_square_voronoi_polys(candidates, 0.05);
  polys = polys
    .iter()
    .map(|p| p.map_coords(|c| (c.x * width, c.y * height).into()))
    .collect();

  // STEP 3: we layer many times the packing

  for i in 0..layering {
    // variable padding between circles: we increase the padding on the edges
    let pad_f = |(x, y): (f64, f64)| {
      mix(
        cpad_mul_ext * cpad,
        cpad_mul_int * cpad,
        ((x - pad)
          .min(y - pad)
          .min(width - pad - x)
          .min(height - pad - y)
          / border_effect)
          .min(1.0)
          .max(0.0),
      )
    };

    // variable maximum radius of these circles: we make circles bigger on the edge of Y axis (portrait)
    let max_scale_f = |(_x, y): (f64, f64)| {
      mix(
        max_scale,
        min_scale,
        ((y - pad).min(height - pad - y) / border_effect)
          .min(1.0)
          .max(0.0),
      )
    };

    // STEP 4: generate random circle packing
    let circles = packing(
      opts.seed * 7.7 + i as f64 / 3.0,
      1000000,
      5000,
      1,
      &pad_f,
      bound,
      min_scale,
      &max_scale_f,
    );

    // this drives the "blurryness" of the generation
    let threshold = threshold_factor * width * rng.gen_range(0.2, 0.4);

    // STEP 5: draw circle-spirals with a color randomly depending on voronoi polygon positions
    for c in circles {
      // we will chose a color based on the distance to closest voronoi polygon
      let mut distances: Vec<(usize, f64)> = polys
        .iter()
        .map(|poly| poly.euclidean_distance(&Point::new(c.x, c.y)))
        .enumerate()
        .collect();
      distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
      distances.retain(|e| e.1 < threshold);

      if distances.len() == 0 {
        continue; // no match, skip
      }
      let first = distances[0];
      let clr = if distances.len() == 1 {
        first.0 // ONE match, we're deep inside the voronoi polygon and we use its color
      } else {
        // on this case, we need to randomly chose a color, weighted by distance to nearby polygons
        let mut value = first.0;
        for &(index, dist) in distances.iter() {
          if rng.gen_bool(0.5 - 0.5 * dist / threshold) {
            value = index;
            break;
          }
        }
        value
      };

      // Finally we can draw the circle + spiral.
      let res = c.r as usize * 4 + 12;
      routes.push((clr, circle_route((c.x, c.y), c.r, res)));
      routes.push((clr, spiral_optimized(c.x, c.y, c.r, 1.0, 0.2)));
    }
  }

  // All our inks
  let mut colors = vec![
    ("Amber", "#fa0"),
    ("Pink", "#f0c"),
    ("Red", "#d00"),
    ("Indigo", "#447"),
    ("Soft Mint", "#0a6"),
    ("Turquoise", "#04e"),
    ("Skull and Roses", "#106"),
    ("Violet", "#70a"),
  ];
  rng.shuffle(&mut colors);

  // convert the lines into <path>, grouped by the chosen color
  colors
    .iter()
    .enumerate()
    .map(|(ci, (color, css))| {
      let mut data = Data::new();
      for (clr, route) in routes.clone() {
        if clr % colors.len() == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(css, 0.35, data));
      l
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "420.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
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

fn circle_route(center: (f64, f64), r: f64, count: usize) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 3. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
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
  pad_f: &dyn Fn((f64, f64)) -> f64,
  bound: (f64, f64, f64, f64),
  min_scale: f64,
  max_scale_f: &dyn Fn((f64, f64)) -> f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    let max_scale = max_scale_f((x, y));
    let pad = pad_f((x, y));
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
