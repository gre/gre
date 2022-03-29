use clap::Clap;
use geo::*;
use gre::*;
use rand::Rng;
use std::cmp::Ordering;
use svg::node::element::path::Data;
use svg::node::element::Group;

#[derive(Clap)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
  height: f64,
}

fn samples_polygon_edge<R: Rng>(
  polygon: Polygon<f64>,
  samples: f64,
  borderdist: f64,
  rng: &mut R,
) -> (Vec<(f64, f64)>, Vec<usize>) {
  let mut length = 0.;
  let poly: Vec<Point<f64>> =
    polygon.exterior().points_iter().collect();
  let l = poly.len() - 1;
  let mut dists = Vec::new();
  let mut cx = 0.;
  let mut cy = 0.;
  for i in 0..l {
    let (x1, y1) = poly[i].x_y();
    cx += x1;
    cy += y1;
    let (x2, y2) = poly[i + 1].x_y();
    let dx = x1 - x2;
    let dy = y1 - y2;
    let d = (dx * dx + dy * dy).sqrt();
    length += d;
    dists.push(d);
  }
  cx /= l as f64;
  cy /= l as f64;
  let incr = length / samples;
  let mut points: Vec<(f64, f64)> = Vec::new();
  let mut groups: Vec<usize> = Vec::new();
  for i in 0..l {
    let (x1, y1) = poly[i].x_y();
    let (x2, y2) = poly[i + 1].x_y();
    let d = dists[i];
    let dx = x2 - x1;
    let dy = y2 - y1;
    let inc = incr / d;
    let mut v = 0.0;
    loop {
      if v > 1. {
        break;
      }
      let px = x1 + v * dx;
      let py = y1 + v * dy;
      let ddx = px - cx;
      let ddy = py - cy;
      let dist = (ddx * ddx + ddy * ddy).sqrt();
      let d = rng.gen_range(0.0, borderdist / dist);
      points.push((mix(px, cx, d), mix(py, cy, d)));
      groups.push(i);
      v += inc;
    }
  }
  return (points, groups);
}

fn sort_point(
  a: &(f64, (f64, f64), usize),
  b: &(f64, (f64, f64), usize),
) -> Ordering {
  ((if a.2 == b.2 { 8. } else { 1. }) * (b.0 - a.0))
    .partial_cmp(&(0.0))
    .unwrap()
}

fn randomize_across_groups<R: Rng>(
  points: Vec<(f64, f64)>,
  groups: Vec<usize>,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  let mut pts: Vec<(f64, (f64, f64), usize)> = points
    .iter()
    .enumerate()
    .map(|(i, &p)| (rng.gen_range(0.0, 1.0), p, groups[i]))
    .collect();
  pts.sort_by(sort_point);
  return pts.iter().map(|p| p.1).collect();
}

fn square(
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
) -> Polygon<f64> {
  polygon![
    (x1, y1).into(),
    (x2, y1).into(),
    (x2, y2).into(),
    (x1, y2).into(),
  ]
}

fn boundaries(
  groups: &Vec<usize>,
  i: usize,
  w: usize,
  h: usize,
) -> (usize, usize, usize, usize) {
  let xi = i % w;
  let yi = i / w;
  let g = groups[i];
  let mut minx = xi;
  let mut maxx = xi;
  let mut miny = yi;
  let mut maxy = yi;
  loop {
    if minx == 0 || groups[(minx - 1) + w * yi] != g {
      break;
    }
    minx -= 1;
  }
  loop {
    if maxx == w - 1 || groups[(maxx + 1) + w * yi] != g {
      break;
    }
    maxx += 1;
  }
  loop {
    if miny == 0 || groups[(miny - 1) * w + xi] != g {
      break;
    }
    miny -= 1;
  }
  loop {
    if maxy == h - 1 || groups[(maxy + 1) * w + xi] != g {
      break;
    }
    maxy += 1;
  }
  (minx, miny, maxx, maxy)
}

fn rects_from_grid<R: Rng>(
  rng: &mut R,
  w: usize,
  h: usize,
  iterations: usize,
  threshold: usize,
) -> Vec<(usize, usize, usize, usize)> {
  let l = w * h;
  let mut groups: Vec<usize> = (0..l).collect();
  let mut map: Vec<usize> = (0..l).collect();
  rng.shuffle(&mut map);
  for i in (0..iterations).map(|i| map[i % l]) {
    let (minx, miny, maxx, maxy) =
      boundaries(&groups, i, w, h);
    let l = (maxy - miny) + (maxx - minx);
    if l > threshold {
      continue;
    }
    // index of: top right bottom left
    let mut freedoms: Vec<usize> = (0..4)
      .filter(|i| match i {
        // top
        0 => minx == maxx && miny > 0,
        // right
        1 => miny == maxy && maxx < w - 2,
        // bottom
        2 => minx == maxx && maxy < h - 2,
        // left
        _ => miny == maxy && minx > 0,
      })
      .collect();
    rng.shuffle(&mut freedoms);
    let (x, y) = match freedoms.get(0) {
      Some(0) => (i % w, miny - 1),
      Some(1) => (maxx + 1, i / w),
      Some(2) => (i % w, maxy + 1),
      Some(3) => (minx - 1, i / w),
      _ => (i % w, i / w),
    };
    groups[x + w * y] = groups[i];
  }
  let mut rects = Vec::new();
  let mut seen = Vec::new();
  for (i, &g) in groups.iter().enumerate() {
    if seen.contains(&g) {
      continue;
    }
    seen.push(g);
    rects.push(boundaries(&groups, i, w, h));
  }
  rects
}

fn art(opts: &Opts) -> Vec<Group> {
  let color = "#000";
  let width = opts.width;
  let height = opts.height;
  let pad = 8.0;
  let stroke_width = 0.35;
  let borderdist = 4.0;
  let seed = opts.seed;
  let mut layers = Vec::new();
  let mut rng = rng_from_seed(seed);
  let xcount = rng.gen_range(8, 24);
  let ycount = xcount;
  let iterations = rng.gen_range(200, 1200);
  let samples_amp = rng.gen_range(1., 6.);
  let threadshold = (1.0
    + (xcount as f64) * rng.gen_range(0.0, 1.0))
    as usize;

  let routes: Vec<Vec<(f64, f64)>> = rects_from_grid(
    &mut rng,
    xcount,
    ycount,
    iterations,
    threadshold,
  )
  .iter()
  .map(|&(x1, y1, x2, y2)| {
    let w = width - 2. * pad;
    let h = height - 2. * pad;
    let x1 = pad + w * (x1 as f64 / ((xcount) as f64));
    let y1 = pad + h * (y1 as f64 / ((ycount) as f64));
    let x2 =
      pad + w * ((x2 + 1) as f64 / ((xcount) as f64));
    let y2 =
      pad + h * ((y2 + 1) as f64 / ((ycount) as f64));
    let rect = square(x1, y1, x2, y2);
    let samples =
      samples_amp * ((x2 - x1) * (y2 - y1)).powf(0.5);
    let (points, groups) = samples_polygon_edge(
      rect, samples, borderdist, &mut rng,
    );
    randomize_across_groups(points, groups, &mut rng)
  })
  .collect();

  let mut l = layer(color);
  let mut data = Data::new();
  for r in routes.clone() {
    data = render_route(data, r);
  }
  l = l.add(base_path(color, stroke_width, data));
  layers.push(l);
  layers
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document =
    base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
