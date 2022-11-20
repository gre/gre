use byteorder::*;
use contour::ContourBuilder;
use geo::*;
use geojson::Feature;
use image::gif::GifDecoder;
use image::io::Reader as ImageReader;
use image::AnimationDecoder;
use image::RgbaImage;
use ndarray::Array2;
use prelude::{BoundingRect, Contains};
use rand::prelude::*;
use std::f64::{consts::PI, INFINITY};
use std::fs::File;
use std::ops::RangeInclusive;
use svg::node::element::{path::Data, Rectangle};
use svg::node::element::{Circle, Group, Path};
use svg::Document;
use time::Duration;

pub mod line_intersection;
pub mod pi;

pub fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}

pub fn mix_point(a: (f64, f64), b: (f64, f64), x: f64) -> (f64, f64) {
  (mix(a.0, b.0, x), mix(a.1, b.1, x))
}

// usual scale is 1.0 for A4
pub fn signature(scale: f64, translation: (f64, f64), color: &str) -> Group {
  return layer("signature").add(Path::new().
        set("d", "m 15.815664,12.893319 c -1.445284,-3.0999497 -5.555449,-0.3575 -5.08537,2.32203 1.697826,2.92736 4.504013,-3.54417 4.40859,-2.58178 -1.548999,2.22986 0.741131,6.08701 3.012419,3.25791 2.532153,-2.82358 0.259001,-7.8326797 -3.488671,-7.9011197 -3.217272,0.056 -5.8863857,2.4603197 -7.9308737,4.7238797 -2.354585,2.46752 0.0048,5.887 2.757763,6.62143 3.2195457,1.10867 6.8759417,1.30834 9.9459317,-0.36585 2.270396,-1.12373 5.025949,-2.62031 5.680576,-5.20027 -2.108811,-3.66096 -6.038415,1.28356 -3.842036,3.67822 1.07278,0.89979 4.586982,-2.27037 3.201668,-2.73503 0.03094,3.24357 1.226854,6.37852 1.337023,9.60311 -0.672198,3.54892 -7.469251,0.32096 -4.637082,-2.5164 2.158436,-2.4193 5.610472,-2.84094 8.202925,-4.57369 0.993877,-1.40371 0.353413,-5.25046 3.182464,-3.48957 2.142923,1.43516 -2.250898,5.7532 1.723416,5.02339 1.661189,-0.71663 6.494946,-1.40457 4.601401,-3.95236 -4.205319,-0.68052 -1.190571,5.86505 1.665411,3.46881 1.929752,-0.9247 2.778055,-4.05119 1.423645,-5.35034 0.479155,1.8589 3.849911,7.52574 4.880369,3.32696 0.21201,-1.28088 0.40468,-3.80204 1.01246,-1.23041 0.5858,2.6865 3.83412,4.91909 4.56937,1.07383 0.65272,-1.00894 -0.2696,-4.02739 0.99929,-1.35746 1.10974,2.31613 6.32001,1.46113 6.147,-1.13059 -1.98394,-2.13868 -5.3717,1.45205 -3.78252,3.73454 2.57741,0.96208 6.69797,-0.21041 7.06275,-3.33983 0.41287,-2.63769 0.26643,-5.3430297 -0.11178,-7.9756197 0.67418,3.94149 1.24889,7.9380497 2.39963,11.7713397 2.10586,1.67977 5.7434,1.65022 7.74596,-0.23639 3.03149,-1.85431 -0.26637,-4.76925 -2.71777,-4.54025 -2.11577,0.0793 -5.36257,2.40772 -5.16868,3.85604 2.08262,-2.38818 5.55759,-1.22628 8.30726,-1.6832 3.182,-0.26596 6.46546,-0.57372 9.54494,-1.18158 0.24171,0.4199 -0.27752,0.54338 -0.43067,0.17453")
        .set("fill","none")
        .set("stroke", color)
        .set("stroke-width", 1)
        .set("transform", format!("translate({},{}) scale({})", translation.0, translation.1, 0.3 * scale)));
}

pub fn grayscale((r, g, b): (f64, f64, f64)) -> f64 {
  return 0.299 * r + 0.587 * g + 0.114 * b;
}

pub fn lerp(a: f64, b: f64, x: f64) -> f64 {
  if a == b {
    a
  } else {
    (x - a) / (b - a)
  }
}

pub fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
  let k = ((x - a) / (b - a)).max(0.0).min(1.0);
  return k * k * (3.0 - 2.0 * k);
}

// see also https://en.wikipedia.org/wiki/CMYK_color_model
pub fn rgb_to_cmyk((r, g, b): (f64, f64, f64)) -> (f64, f64, f64, f64) {
  let k = 1.0 - r.max(g).max(b);
  let c = (1.0 - r - k) / (1.0 - k);
  let m = (1.0 - g - k) / (1.0 - k);
  let y = (1.0 - b - k) / (1.0 - k);
  return (c, m, y, k);
}

pub fn rgb_to_cmyk_vec(c: (f64, f64, f64)) -> Vec<f64> {
  let (c, m, y, k) = rgb_to_cmyk(c);
  vec![c, m, y, k]
}

pub fn preserve_ratio_inside(
  (x, y): (f64, f64),
  (w, h): (f64, f64),
) -> (f64, f64) {
  let m = w.min(h);
  let ratio = (w / m, h / m);
  (0.5 + (x - 0.5) * ratio.0, 0.5 + (y - 0.5) * ratio.1)
}

pub fn preserve_ratio_outside(
  (x, y): (f64, f64),
  (w, h): (f64, f64),
) -> (f64, f64) {
  let m = w.max(h);
  let ratio = (w / m, h / m);
  (0.5 + (x - 0.5) * ratio.0, 0.5 + (y - 0.5) * ratio.1)
}

// point is normalized in 0..1
// returned value is a rgb tuple in 0..1 range
pub fn image_get_color(
  path: &str,
) -> Result<impl Fn((f64, f64)) -> (f64, f64, f64), image::ImageError> {
  let img = ImageReader::open(path)?.decode()?;
  return Ok(dynamic_image_get_color(img.to_rgba8()));
}

pub fn image_gif_get_color(
  path: &str,
  index: usize,
) -> Result<impl Fn((f64, f64)) -> (f64, f64, f64), image::ImageError> {
  let file_in = File::open(path)?;
  let decoder = GifDecoder::new(file_in).unwrap();
  let frames = decoder.into_frames();
  let frames = frames.collect_frames()?;
  let img = frames.get(index % frames.len()).unwrap();
  let buffer = img.buffer();
  return Ok(dynamic_image_get_color(buffer.clone()));
}

pub fn dynamic_image_get_color(
  img: RgbaImage,
) -> impl Fn((f64, f64)) -> (f64, f64, f64) {
  let (width, height) = img.dimensions();
  return move |(x, y): (f64, f64)| {
    // quadratic implementation
    let xi: f64 = x.max(0.0).min(1.0) * ((width - 1) as f64);
    let yi: f64 = y.max(0.0).min(1.0) * ((height - 1) as f64);
    let x1 = xi.floor() as u32;
    let x2 = xi.ceil() as u32;
    let y1 = yi.floor() as u32;
    let y2 = yi.ceil() as u32;
    let p1 = img.get_pixel(x1, y1);
    let p2 = img.get_pixel(x2, y1);
    let p3 = img.get_pixel(x2, y2);
    let p4 = img.get_pixel(x1, y2);
    let xp = xi - xi.floor();
    let yp = yi - yi.floor();
    let r = (mix(
      mix(p1[0] as f64, p2[0] as f64, xp),
      mix(p4[0] as f64, p3[0] as f64, xp),
      yp,
    )) / 255.0;
    let g = (mix(
      mix(p1[1] as f64, p2[1] as f64, xp),
      mix(p4[1] as f64, p3[1] as f64, xp),
      yp,
    )) / 255.0;
    let b = (mix(
      mix(p1[2] as f64, p2[2] as f64, xp),
      mix(p4[2] as f64, p3[2] as f64, xp),
      yp,
    )) / 255.0;
    return (r, g, b);
  };
}

pub fn layer(id: &str) -> Group {
  return Group::new()
    .set("inkscape:groupmode", "layer")
    .set("inkscape:label", id);
}

pub fn base_document(bg: &str, width: f64, height: f64) -> Document {
  Document::new()
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("viewBox", (0, 0, width, height))
    .set("width", format!("{}mm", width))
    .set("height", format!("{}mm", height))
    .set("style", format!("background:{}", bg))
}

pub fn base_a3_portrait(bg: &str) -> Document {
  base_document(bg, 297., 420.)
}
pub fn base_a3_landscape(bg: &str) -> Document {
  base_document(bg, 420., 297.)
}
pub fn base_a4_portrait(bg: &str) -> Document {
  base_document(bg, 210., 297.)
}
pub fn base_a4_landscape(bg: &str) -> Document {
  base_document(bg, 297., 210.)
}
pub fn base_a4_square(bg: &str) -> Document {
  base_document(bg, 210., 210.)
}
pub fn base_a5_landscape(bg: &str) -> Document {
  base_document(bg, 210., 297. / 2.)
}
pub fn base_24x30_portrait(bg: &str) -> Document {
  base_document(bg, 240., 300.)
}
pub fn base_24x30_landscape(bg: &str) -> Document {
  base_document(bg, 300., 240.)
}

pub fn euclidian_rgb_distance(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
  let r = a.0 - b.0;
  let g = a.1 - b.1;
  let b = a.2 - b.2;
  (r * r + g * g + b * b).sqrt()
}

pub fn base_path(color: &str, stroke_width: f64, data: Data) -> Path {
  Path::new()
    .set("fill", "none")
    .set("stroke", color)
    .set("stroke-width", stroke_width)
    .set("d", data)
    .set("style", "mix-blend-mode: multiply;")
}

pub fn base_rect(color: &str, stroke_width: f64) -> Rectangle {
  Rectangle::new()
    .set("fill", "none")
    .set("stroke", color)
    .set("stroke-width", stroke_width)
    .set("style", "mix-blend-mode: multiply;")
}

pub fn project_in_boundaries(
  p: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> (f64, f64) {
  (
    p.0 * (boundaries.2 - boundaries.0) + boundaries.0,
    p.1 * (boundaries.3 - boundaries.1) + boundaries.1,
  )
}

pub fn normalize_in_boundaries(
  p: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> (f64, f64) {
  (
    (p.0 - boundaries.0) / (boundaries.2 - boundaries.0),
    (p.1 - boundaries.1) / (boundaries.3 - boundaries.1),
  )
}

pub fn out_of_boundaries(
  p: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> bool {
  p.0 < boundaries.0
    || p.0 > boundaries.2
    || p.1 < boundaries.1
    || p.1 > boundaries.3
}

pub fn strictly_in_boundaries(
  p: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> bool {
  p.0 > boundaries.0
    && p.0 < boundaries.2
    && p.1 > boundaries.1
    && p.1 < boundaries.3
}

pub fn render_polygon_stroke(data: Data, poly: Polygon<f64>) -> Data {
  let mut first = true;
  let mut d = data;
  for p in poly.exterior().points() {
    if first {
      first = false;
      d = d.move_to(p.x_y());
    } else {
      d = d.line_to(p.x_y());
    }
  }
  d
}

pub fn samples_polygon(
  poly: &Polygon<f64>,
  samples: usize,
  sz: usize,
  rng: &mut impl Rng,
) -> Vec<(f64, f64)> {
  let bounds = poly.bounding_rect().unwrap();
  let mut candidates = Vec::new();
  for x in 0..sz {
    for y in 0..sz {
      let o: Point<f64> = bounds.min().into();
      let p: Point<f64> = o
        + point!(x: x as f64 * bounds.width(), y: y as f64 * bounds.height())
          / (sz as f64);
      if poly.contains(&p) {
        candidates.push(p.x_y());
      }
    }
  }
  rng.shuffle(&mut candidates);
  candidates.truncate(samples);
  return candidates;
}

pub fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
  let mut first = true;
  let mut d = data;
  for p in route {
    if first {
      first = false;
      d = d.move_to(p);
    } else {
      d = d.line_to(p);
    }
  }
  return d;
}

pub fn render_route_collide(
  data: Data,
  route: Vec<(f64, f64)>,
  // when lines are not to be drawn, you can Some(from)
  // otherwise you can give the precise collision point of the line
  // if None we assume we can draw the full line
  collides: &dyn Fn((f64, f64), (f64, f64)) -> Option<(f64, f64)>,
) -> Data {
  let mut first = true;
  let mut up = false;
  let mut last = (0.0, 0.0);
  let mut d = data;
  for p in route {
    if first {
      first = false;
      d = d.move_to(p);
    } else {
      if let Some(c) = collides(last, p) {
        if c.0 == last.0 && c.1 == last.1 || c.0 == p.0 && c.1 == p.1 {
          // nothing to draw
          up = true;
        } else {
          if up {
            d = d.move_to(last);
          }
          d = d.line_to(c);
          up = true;
        }
      } else {
        if up {
          up = false;
          d = d.move_to(last);
        }
        d = d.line_to(p);
      }
    }
    last = p;
  }
  return d;
}

pub fn render_route_when<F: FnMut((f64, f64), (f64, f64)) -> bool>(
  data: Data,
  route: Vec<(f64, f64)>,
  mut should_draw_line: F,
) -> Data {
  let mut first = true;
  let mut up = false;
  let mut last = (0.0, 0.0);
  let mut d = data;
  for p in route {
    if first {
      if should_draw_line(p, p) {
        first = false;
        d = d.move_to(p);
      }
    } else {
      if should_draw_line(last, p) {
        if up {
          up = false;
          d = d.move_to(last);
        }
        d = d.line_to(p);
      } else {
        up = true;
      }
    }
    last = p;
  }
  return d;
}

pub fn tsp(candidates: Vec<(f64, f64)>, duration: Duration) -> Vec<(f64, f64)> {
  let tour =
    travelling_salesman::simulated_annealing::solve(&candidates, duration);
  return tour.route.iter().map(|&i| candidates[i]).collect();
}

pub fn render_tsp(
  data: Data,
  candidates: Vec<(f64, f64)>,
  duration: Duration,
) -> Data {
  return render_route(data, tsp(candidates, duration));
}

pub fn render_polygon_fill_tsp(
  data: Data,
  poly: Polygon<f64>,
  samples: usize,
  rng: &mut impl Rng,
  duration: Duration,
) -> Data {
  let candidates = samples_polygon(&poly, samples, 32, rng);
  return render_tsp(data, candidates, duration);
}

pub fn route_spiral(candidates: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
  if candidates.len() == 0 {
    return candidates;
  }
  let mut result = Vec::new();
  let mut list = candidates.clone();
  let mut p = *(candidates
    .iter()
    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    .unwrap());
  let mut a = 0.0;
  result.push(p);
  loop {
    list = list.into_iter().filter(|&x| x != p).collect();

    let maybe_match = list.iter().min_by_key(|q| {
      let qp_angle = (p.1 - q.1).atan2(p.0 - q.0);
      // HACK!!! no Ord for f64 :(
      return (1000000.0 * ((2. * PI + qp_angle - a) % (2.0 * PI))) as i32;
    });
    if let Some(new_p) = maybe_match {
      a = (p.1 - new_p.1).atan2(p.0 - new_p.0);
      p = *new_p;
      result.push(p);
    } else {
      break;
    }
  }
  result
}

pub fn render_fill_spiral(data: Data, candidates: Vec<(f64, f64)>) -> Data {
  let result = route_spiral(candidates);
  return render_route(data, result);
}

pub fn render_polygon_fill_spiral(
  data: Data,
  poly: Polygon<f64>,
  samples: usize,
  rng: &mut impl Rng,
) -> Data {
  let candidates = samples_polygon(&poly, samples, 32, rng);
  render_fill_spiral(data, candidates)
}

pub fn sample_2d_candidates(
  f: &dyn Fn((f64, f64)) -> bool,
  dim: usize,
  samples: usize,
  rng: &mut impl Rng,
) -> Vec<(f64, f64)> {
  let mut candidates = Vec::new();
  for x in 0..dim {
    for y in 0..dim {
      let p = ((x as f64) / (dim as f64), (y as f64) / (dim as f64));
      if f(p) {
        candidates.push(p);
      }
    }
  }
  rng.shuffle(&mut candidates);
  candidates.truncate(samples);
  return candidates;
}

// f returns a value from 0.0 to 1.0. if 0 the point is not considered, if 1 it's always taken in samples candidate. otherwise it's randomly filtered
pub fn sample_2d_candidates_f64<R: Rng>(
  f: &dyn Fn((f64, f64)) -> f64,
  dim: usize,
  samples: usize,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  let mut candidates = Vec::new();
  for x in 0..dim {
    for y in 0..dim {
      let p = ((x as f64) / (dim as f64), (y as f64) / (dim as f64));
      if f(p) > rng.gen_range(0.0, 1.0) {
        candidates.push(p);
      }
    }
  }
  rng.shuffle(&mut candidates);
  candidates.truncate(samples);
  return candidates;
}

pub fn render_debug_samples(pts: Vec<(f64, f64)>) -> Group {
  let mut g = Group::new();
  for p in pts {
    g = g.add(
      Circle::new()
        .set("cx", p.0)
        .set("cy", p.1)
        .set("r", 1)
        .set("fill", "black"),
    );
  }
  return g;
}

// formula from https://www.youtube.com/watch?v=aNR4n0i2ZlM
pub fn heart_distance(p: (f64, f64)) -> f64 {
  let x = p.0;
  let y = 4.0 + 1.2 * p.1 - x.abs() * ((20.0 - x.abs()) / 15.0).sqrt();
  x * x + y * y - 10.0
}

pub fn boundaries_route(boundaries: (f64, f64, f64, f64)) -> Vec<(f64, f64)> {
  vec![
    (boundaries.0, boundaries.1),
    (boundaries.2, boundaries.1),
    (boundaries.2, boundaries.3),
    (boundaries.0, boundaries.3),
    (boundaries.0, boundaries.1),
  ]
}

// get in mm the side length of the bounding square that contains the polygon
pub fn poly_bounding_square_edge(poly: &Polygon<f64>) -> f64 {
  let bounds = poly.bounding_rect().unwrap();
  bounds.width().max(bounds.height())
}

pub fn sample_square_voronoi_polys(
  candidates: Vec<(f64, f64)>,
  pad: f64,
) -> Vec<Polygon<f64>> {
  let mut points = Vec::new();
  for c in candidates {
    points.push(voronoi::Point::new(
      pad + (1.0 - 2.0 * pad) * c.0,
      pad + (1.0 - 2.0 * pad) * c.1,
    ));
  }
  let dcel = voronoi::voronoi(points, 1.0);
  let polys = voronoi::make_polygons(&dcel)
    .iter()
    .map(|pts| {
      Polygon::new(
        pts
          .iter()
          .map(|p| (p.x(), p.y()))
          .collect::<Vec<_>>()
          .into(),
        vec![],
      )
    })
    .collect();
  polys
}

pub fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

pub fn group_by_proximity(
  candidates: Vec<(f64, f64)>,
  threshold: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut groups: Vec<Vec<(f64, f64)>> = Vec::new();
  let list = candidates.clone();

  for item in list {
    let mut found = false;
    for group in &mut groups {
      let matches = group.iter().any(|p| euclidian_dist(*p, item) < threshold);
      if matches {
        found = true;
        group.push(item);
        break;
      }
    }
    if !found {
      let group = vec![item];
      groups.push(group);
    }
  }

  return groups;
}

pub fn render_route_curve(data: Data, route: Vec<(f64, f64)>) -> Data {
  if route.len() == 0 {
    return data;
  }
  let mut first = true;
  let mut d = data;
  let mut last = route[0];
  for p in route {
    if first {
      first = false;
      d = d.move_to(p);
    } else {
      d = d.quadratic_curve_to((
        last.0,
        last.1,
        (p.0 + last.0) / 2.,
        (p.1 + last.1) / 2.,
      ));
    }
    last = p;
  }
  return d;
}

pub fn group_with_kmeans(
  samples: Vec<(f64, f64)>,
  n: usize,
) -> Vec<Vec<(f64, f64)>> {
  let arr = Array2::from_shape_vec(
    (samples.len(), 2),
    samples.iter().flat_map(|&(x, y)| vec![x, y]).collect(),
  )
  .unwrap();

  let (means, clusters) = rkm::kmeans_lloyd(&arr.view(), n);

  let all: Vec<Vec<(f64, f64)>> = means
    .outer_iter()
    .enumerate()
    .map(|(c, _coord)| {
      clusters
        .iter()
        .enumerate()
        .filter(|(_i, &cluster)| cluster == c)
        .map(|(i, _c)| samples[i])
        .collect()
    })
    .collect();

  all
}

pub fn round_point((x, y): (f64, f64), precision: f64) -> (f64, f64) {
  (
    (x / precision).round() * precision,
    (y / precision).round() * precision,
  )
}

pub fn round_route(route: Vec<(f64, f64)>, precision: f64) -> Vec<(f64, f64)> {
  route.iter().map(|&p| round_point(p, precision)).collect()
}

pub fn follow_angle(o: (f64, f64), a: f64, amp: f64) -> (f64, f64) {
  (o.0 + amp * a.cos(), o.1 + amp * a.sin())
}

/// Get the relationship between this line segment and another.
pub fn collides_segment(
  from_1: (f64, f64),
  to_1: (f64, f64),
  from_2: (f64, f64),
  to_2: (f64, f64),
) -> Option<(f64, f64)> {
  // see https://stackoverflow.com/a/565282
  let p = from_1;
  let q = from_2;
  let r = (to_1.0 - p.0, to_1.1 - p.1);
  let s = (to_2.0 - q.0, to_2.1 - q.1);

  let r_cross_s = cross(r, s);
  let q_minus_p = (q.0 - p.0, q.1 - p.1);
  let q_minus_p_cross_r = cross(q_minus_p, r);

  // are the lines are parallel?
  if r_cross_s == 0.0 {
    // are the lines collinear?
    if q_minus_p_cross_r == 0.0 {
      // the lines are collinear
      None
    } else {
      // the lines are parallel but not collinear
      None
    }
  } else {
    // the lines are not parallel
    let t = cross(q_minus_p, div(s, r_cross_s));
    let u = cross(q_minus_p, div(r, r_cross_s));

    // are the intersection coordinates both in range?
    let t_in_range = 0.0 <= t && t <= 1.0;
    let u_in_range = 0.0 <= u && u <= 1.0;

    if t_in_range && u_in_range {
      // there is an intersection
      Some((p.0 + t * r.0, p.1 + t * r.1))
    } else {
      // there is no intersection
      None
    }
  }
}

fn cross(a: (f64, f64), b: (f64, f64)) -> f64 {
  a.0 * b.1 - a.1 * b.0
}

fn div(a: (f64, f64), b: f64) -> (f64, f64) {
  (a.0 / b, a.1 / b)
}

// assume the collision is on 1D (same line) so we can just do logic on one dimension
pub fn find_best_collision_1d(
  from: (f64, f64),
  others: Vec<(f64, f64)>,
) -> Option<(f64, f64)> {
  let mut best_dx = INFINITY;
  let mut best_intersection = None;
  for q in others {
    let dx = (q.0 - from.0).abs();
    if dx < best_dx {
      best_intersection = Some(q);
      best_dx = dx;
    }
  }
  return best_intersection;
}

pub fn collide_route_segment(
  route: &Vec<(f64, f64)>,
  from: (f64, f64),
  to: (f64, f64),
) -> Option<(f64, f64)> {
  // TODO: things could be way more performant with a quad tree
  let mut last = route[0];
  let mut best_dx = INFINITY;
  let mut best_intersection = None;
  for i in 1..route.len() {
    let p = route[i];
    let intersection = collides_segment(from, to, last, p);
    if let Some(q) = intersection {
      let dx = (q.0 - from.0).abs();
      if dx < best_dx {
        best_intersection = intersection;
        best_dx = dx;
      }
    }
    last = p;
  }
  return best_intersection;
}

pub fn collide_segment_boundaries(
  from: (f64, f64),
  to: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> Option<(f64, f64)> {
  let from_in = strictly_in_boundaries(from, boundaries);
  let to_in = strictly_in_boundaries(to, boundaries);

  if from_in && to_in {
    return None;
  }
  if !from_in && !to_in {
    return Some(from); // convention to return from when both are out
  }
  let segment = line_intersection::LineInterval::line_segment(Line {
    start: from.into(),
    end: to.into(),
  });
  vec![
    line_intersection::LineInterval::line_segment(Line {
      start: (boundaries.0, boundaries.1).into(),
      end: (boundaries.2, boundaries.1).into(),
    }),
    line_intersection::LineInterval::line_segment(Line {
      start: (boundaries.0, boundaries.1).into(),
      end: (boundaries.0, boundaries.3).into(),
    }),
    line_intersection::LineInterval::line_segment(Line {
      start: (boundaries.2, boundaries.1).into(),
      end: (boundaries.2, boundaries.3).into(),
    }),
    line_intersection::LineInterval::line_segment(Line {
      start: (boundaries.0, boundaries.3).into(),
      end: (boundaries.2, boundaries.3).into(),
    }),
  ]
  .iter()
  .find_map(|edge| segment.relate(edge).unique_intersection().map(|p| p.x_y()))
}

/*
// collide routes: make a route stop as soon as it collides another

// sequential: means the routes are ordered by priority.
// we do one route after the other in the routes order.
pub fn collide_routes_sequential(
    routes: Vec<Vec<(f64, f64)>>,
) -> Vec<Vec<(f64, f64)>> {
    let mut acc = Vec::new();
    for route in routes {
        let mut copy = Vec::new();
        let mut cur = route[0];
        copy.push(cur);
        for &next in route.iter().skip(1) {
            let collision = acc.iter().find_map(|r| {
                collide_route_segment(r, cur, next)
            });
            if let Some(point) = collision {
                copy.push(point);
                break;
            }
            cur = next;
            copy.push(next);
        }
        if copy.len() > 1 {
            acc.push(copy);
        }
    }
    acc
}

// parallel: means the routes are going to be equally considered but their "length" determines the priority.
// we give equal chance to all routes (they progress at the same time)
pub fn collide_routes_parallel(
    routes: Vec<Vec<(f64, f64)>>,
) -> Vec<Vec<(f64, f64)>> {
    let mut acc = Vec::new();
    let mut finished = Vec::new();
    for route in routes.iter() {
        let mut v: Vec<(f64, f64)> = Vec::new();
        v.push(route[0]);
        acc.push(v);
        finished.push(false);
    }

    let mut i = 1;
    loop {
        let mut continues = false;
        for (j, route) in routes.iter().enumerate() {
            if route.len() <= i || finished[j] {
                continue;
            }
            let cur = acc[j][i - 1];
            let next = route[i];
            let collision = acc
                .iter()
                .enumerate()
                .find_map(|(k, r)| {
                    if k == j {
                        None
                    } else {
                        collide_route_segment(r, cur, next)
                    }
                });
            if let Some(point) = collision {
                acc[j].push(point);
                finished[j] = true;
            } else {
                continues = true;
                acc[j].push(next);
            }
        }
        if !continues {
            break;
        }
        i += 1;
    }

    acc = acc
        .iter()
        .filter(|r| r.len() > 1)
        .map(|r| r.clone())
        .collect();

    return acc;
}
*/

pub fn build_routes<
  F: FnMut(
    // last position
    (f64, f64),
    // index of the route position to build
    usize,
    // index of the route in routes
    usize,
  ) -> Option<((f64, f64), bool)>,
>(
  initial_positions: Vec<(f64, f64)>,
  // returns None if there is no point to build anymore
  // returns Some((point, ends)) where point is the next point and ends tells if it's the last terminating point.
  mut build_route: F,
) -> Vec<Vec<(f64, f64)>> {
  let mut acc = Vec::new();
  for (j, &origin) in initial_positions.iter().enumerate() {
    let mut v: Vec<(f64, f64)> = Vec::new();
    v.push(origin);
    let mut i = 1;
    let mut cur = origin;
    loop {
      if let Some((next, ends)) = build_route(cur, i, j) {
        if ends {
          v.push(next);
          break;
        } else {
          v.push(next);
        }
        i += 1;
        cur = next;
      } else {
        break;
      }
    }
    acc.push(v);
  }
  acc = acc
    .iter()
    .filter(|r| r.len() > 1)
    .map(|r| r.clone())
    .collect();
  acc
}

pub fn build_routes_with_collision_seq<
  F: FnMut(
    // last position
    (f64, f64),
    // index of the route position to build
    usize,
    // index of the route in routes
    usize,
  ) -> Option<((f64, f64), bool)>,
>(
  initial_positions: Vec<(f64, f64)>,
  // returns None if there is no point to build anymore
  // returns Some((point, ends)) where point is the next point and ends tells if it's the last terminating point.
  mut build_route: F,
) -> Vec<Vec<(f64, f64)>> {
  let mut acc = Vec::new();
  for (j, &origin) in initial_positions.iter().enumerate() {
    let mut v: Vec<(f64, f64)> = Vec::new();
    v.push(origin);
    let mut i = 1;
    let mut cur = origin;
    loop {
      if let Some((next, ends)) = build_route(cur, i, j) {
        let collision = find_best_collision_1d(
          cur,
          acc
            .iter()
            .enumerate()
            .filter_map(|(k, r)| {
              if k == j {
                None
              } else {
                collide_route_segment(r, cur, next)
              }
            })
            .collect(),
        );
        if let Some(point) = collision {
          v.push(point);
          break;
        } else if ends {
          v.push(next);
          break;
        } else {
          v.push(next);
        }

        i += 1;
        cur = next;
      } else {
        break;
      }
    }
    acc.push(v);
  }
  acc = acc
    .iter()
    .filter(|r| r.len() > 1)
    .map(|r| r.clone())
    .collect();
  acc
}

pub fn build_routes_with_collision_par<
  F: FnMut(
    // last position
    (f64, f64),
    // index of the route position to build
    usize,
    // index of the route in routes
    usize,
  ) -> Option<((f64, f64), bool)>,
>(
  initial_positions: Vec<(f64, f64)>,
  // returns None if there is no point to build anymore
  // returns Some((point, ends)) where point is the next point and ends tells if it's the last terminating point.
  mut build_route: F,
) -> Vec<Vec<(f64, f64)>> {
  let len = initial_positions.len();
  let mut acc = Vec::new();
  let mut finished = Vec::new();
  for origin in initial_positions {
    let mut v: Vec<(f64, f64)> = Vec::new();
    v.push(origin);
    acc.push(v);
    finished.push(false);
  }

  let mut i = 1;
  loop {
    let mut continues = false;
    for j in 0..len {
      if finished[j] {
        continue;
      }
      let cur = acc[j][i - 1];
      if let Some((next, ends)) = build_route(cur, i, j) {
        let collision = find_best_collision_1d(
          cur,
          acc
            .iter()
            .enumerate()
            .filter_map(|(k, r)| {
              if k == j {
                None
              } else {
                collide_route_segment(r, cur, next)
              }
            })
            .collect(),
        );
        if let Some(point) = collision {
          acc[j].push(point);
          finished[j] = true;
        } else if ends {
          acc[j].push(next);
          finished[j] = true;
        } else {
          continues = true;
          acc[j].push(next);
        }
      } else {
        finished[j] = true;
      }
    }
    if !continues {
      break;
    }
    i += 1;
  }

  acc = acc
    .iter()
    .filter(|r| r.len() > 1)
    .map(|r| r.clone())
    .collect();

  return acc;
}

// TODO remove a polygon shape on a route

/**
 * utility to count the number of items passing by a position in order to limit too much passage.
 */
#[derive(Clone)]
pub struct Passage2DCounter {
  granularity: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage2DCounter {
  pub fn new(granularity: f64, width: f64, height: f64) -> Self {
    let wi = (width / granularity).ceil() as usize;
    let hi = (height / granularity).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage2DCounter {
      granularity,
      width,
      height,
      counters,
    }
  }
  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.granularity).ceil() as usize;
    let hi = (self.height / self.granularity).ceil() as usize;
    let xi = ((x / self.granularity).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.granularity).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }
  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }
  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    self.counters[self.index(p)]
  }
}

pub fn rng_from_seed(s: f64) -> impl Rng {
  let mut bs = [0; 16];
  bs.as_mut().write_f64::<BigEndian>(s).unwrap();
  let mut rng = SmallRng::from_seed(bs);
  // run it a while to have better randomness
  for _i in 0..50 {
    rng.gen::<f64>();
  }
  return rng;
}

pub fn rasterize_1d<F: FnMut((f64, f64)) -> f64>(
  width: u32,
  height: u32,
  mut f: F,
) -> Vec<f64> {
  (0..height)
    .flat_map(|y| {
      (0..width)
        .map(|x| f((x as f64 / width as f64, y as f64 / height as f64)))
        .collect::<Vec<f64>>()
    })
    .collect::<Vec<f64>>()
}

pub fn contour<F: FnMut((f64, f64)) -> f64>(
  width: u32,
  height: u32,
  mut f: F,
  thresholds: &Vec<f64>,
) -> Vec<Feature> {
  let c = ContourBuilder::new(width, height, true);
  let values = rasterize_1d(width, height, &mut f);
  c.contours(&values, &thresholds).unwrap_or(Vec::new())
}

pub fn features_to_routes(
  features: Vec<Feature>,
  precision: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  for f in features {
    for g in f.geometry {
      let value = g.value;
      match value {
        geojson::Value::MultiPolygon(all) => {
          for poly in all {
            for lines in poly {
              let mut points = lines
                .iter()
                .map(|p| (precision * p[0], precision * p[1]))
                .collect::<Vec<(f64, f64)>>();
              let len = points.len();
              if len < 3 {
                continue;
              }
              if euclidian_dist(points[0], points[len - 1]) <= precision {
                points.push(points[0]);
              }
              routes.push(points);
            }
          }
        }
        _ => {}
      }
    }
  }
  routes
}

pub fn crop_route(
  route: &Vec<(f64, f64)>,
  boundaries: (f64, f64, f64, f64),
) -> Option<Vec<(f64, f64)>> {
  if route.len() < 2
    || route
      .iter()
      .all(|&p| !strictly_in_boundaries(p, boundaries))
  {
    return None;
  }
  return Some(route.clone());
}

pub fn crop_routes(
  routes: &Vec<Vec<(f64, f64)>>,
  boundaries: (f64, f64, f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  return routes
    .iter()
    .filter_map(|route| crop_route(&route, boundaries))
    .collect();
}

pub fn sdf_box2((x, y): (f64, f64), (w, h): (f64, f64)) -> f64 {
  let dx = x.abs() - w;
  let dy = y.abs() - h;
  euclidian_dist((0., 0.), (dx.max(0.), dy.max(0.)))
    + dx.min(0.).max(dy.min(0.))
}

pub fn length(l: (f64, f64)) -> f64 {
  (l.0 * l.0 + l.1 * l.1).sqrt()
}

pub fn f_op_union_round(a: f64, b: f64, r: f64) -> f64 {
  r.max(a.min(b)) - length(((r - a).max(0.), (r - b).max(0.)))
}

pub fn f_op_intersection_round(a: f64, b: f64, r: f64) -> f64 {
  (-r).min(a.max(b)) + length(((r + a).max(0.), (r + b).max(0.)))
}

pub fn f_op_difference_round(a: f64, b: f64, r: f64) -> f64 {
  f_op_intersection_round(a, -b, r)
}

pub fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

pub fn spiral(x: f64, y: f64, radius: f64, dr: f64) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = round_point((x + r * a.cos(), y + r * a.sin()), 0.01);
    route.push(p);
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.1 {
      break;
    }
  }
  route
}

pub fn spiral_optimized(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  approx: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = 0f64;
  loop {
    let p = round_point((x + r * a.cos(), y + r * a.sin()), 0.01);
    let l = route.len();
    if l == 0 || euclidian_dist(route[l - 1], p) > approx {
      route.push(p);
    }
    let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.05 {
      break;
    }
  }
  route
}

// adapted from library "ramer_douglas_peucker"
/// Given a set of points and an epsilon, returns a list of indexes to keep.
/// If the first and last points are the same, then the points are treated as a closed polygon
pub fn rdp(points: &Vec<(f64, f64)>, epsilon: f64) -> Vec<(f64, f64)> {
  if points.len() < 3 {
    return points.clone();
  }
  let mut ranges = Vec::<RangeInclusive<usize>>::new();

  let mut results = Vec::new();
  results.push(0); // We always keep the starting point

  // Set of ranges to work through
  ranges.push(0..=points.len() - 1);

  while let Some(range) = ranges.pop() {
    let range_start = *range.start();
    let range_end = *range.end();

    let start = points[range_start];
    let end = points[range_end];

    // Caches a bit of the calculation to make the loop quicker
    let line = LineDistance::new(start, end);

    let (max_distance, max_index) =
      points[range_start + 1..range_end].iter().enumerate().fold(
        (0.0_f64, 0),
        move |(max_distance, max_index), (index, &point)| {
          let distance = match line.to(point) {
            Some(distance) => distance,
            None => {
              let base = point.0 - start.0;
              let height = point.1 - start.1;
              base.hypot(height)
            }
          };

          if distance > max_distance {
            // new max distance!
            // +1 to the index because we start enumerate()ing on the 1st element
            return (distance, index + 1);
          }

          // no new max, pass the previous through
          (max_distance, max_index)
        },
      );

    // If there is a point outside of epsilon, subdivide the range and try again
    if max_distance > epsilon {
      // We add range_start to max_index because the range needs to be in
      // the space of the whole vector and not the range
      let division_point = range_start + max_index;

      let first_section = range_start..=division_point;
      let second_section = division_point..=range_end;

      // Process the second one first to maintain the stack
      // The order of ranges and results are opposite, hence the awkwardness
      let should_keep_second_half = division_point - range_start > 2;
      if should_keep_second_half {
        ranges.push(second_section);
      }

      if division_point - range_start > 2 {
        ranges.push(first_section);
      } else {
        results.push(division_point);
      }

      if !should_keep_second_half {
        results.push(range_end);
      }
    } else {
      // Keep the end point for the results
      results.push(range_end);
    }
  }

  results.iter().map(|&i| points[i]).collect()
}

// adapted from "legasea_line"
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LineDistance {
  a: f64,
  b: f64,
  c: f64,
  pub length: f64,
}

impl LineDistance {
  pub fn new(p1: (f64, f64), p2: (f64, f64)) -> Self {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let a = y2 - y1;
    let b = x2 - x1;
    let c = (x2 * y1) - (y2 * x1);
    let length = euclidian_dist(p1, p2);
    Self { a, b, c, length }
  }
  pub fn to(&self, point: (f64, f64)) -> Option<f64> {
    let Self { a, b, c, length } = self;
    if 0.0 == *length {
      None
    } else {
      // https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
      Some(((a * point.0) - (b * point.1) + c).abs() / length)
    }
  }
}
