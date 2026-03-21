use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;
use svg::Document;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  width: f64,
  #[clap(short, long, default_value = "297.0")]
  height: f64,
  #[clap(short, long, default_value = "10.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

// ── Differential Growth ─────────────────────────────────────────────────────

struct DiffGrowthConfig {
  repulsion_radius: f64,
  repulsion_strength: f64,
  attraction_strength: f64,
  alignment_strength: f64,
  max_edge_length: f64,
  min_edge_length: f64,
  bounds: (f64, f64, f64, f64),
  boundary_strength: f64,
  max_speed: f64,
  max_nodes: usize,
  iterations: usize,
}

struct SpatialGrid {
  inv_cell_size: f64,
  cols: usize,
  rows: usize,
  cells: Vec<Vec<usize>>,
}

impl SpatialGrid {
  fn new(bounds: (f64, f64, f64, f64), cell_size: f64) -> Self {
    let cols = ((bounds.2 - bounds.0) / cell_size).ceil() as usize + 1;
    let rows = ((bounds.3 - bounds.1) / cell_size).ceil() as usize + 1;
    Self {
      inv_cell_size: 1.0 / cell_size,
      cols,
      rows,
      cells: vec![Vec::new(); cols * rows],
    }
  }

  fn clear(&mut self) {
    for cell in &mut self.cells {
      cell.clear();
    }
  }

  fn cell_index(&self, x: f64, y: f64) -> Option<usize> {
    let col = (x * self.inv_cell_size) as isize;
    let row = (y * self.inv_cell_size) as isize;
    if col >= 0 && col < self.cols as isize && row >= 0 && row < self.rows as isize {
      Some(row as usize * self.cols + col as usize)
    } else {
      None
    }
  }

  fn insert(&mut self, idx: usize, x: f64, y: f64) {
    if let Some(ci) = self.cell_index(x, y) {
      self.cells[ci].push(idx);
    }
  }

  fn query_neighbors_into(&self, x: f64, y: f64, result: &mut Vec<usize>) {
    result.clear();
    let col = (x * self.inv_cell_size) as isize;
    let row = (y * self.inv_cell_size) as isize;
    for dr in -1..=1 {
      for dc in -1..=1 {
        let c = col + dc;
        let r = row + dr;
        if c >= 0 && c < self.cols as isize && r >= 0 && r < self.rows as isize {
          let ci = r as usize * self.cols + c as usize;
          result.extend_from_slice(&self.cells[ci]);
        }
      }
    }
  }
}

fn euclidian_dist(a: (f64, f64), b: (f64, f64)) -> f64 {
  let dx = a.0 - b.0;
  let dy = a.1 - b.1;
  (dx * dx + dy * dy).sqrt()
}

fn circle_route(center: (f64, f64), radius: f64, count: usize) -> Vec<(f64, f64)> {
  (0..=count)
    .map(|i| {
      let a = 2.0 * PI * (i as f64) / (count as f64);
      (center.0 + radius * a.cos(), center.1 + radius * a.sin())
    })
    .collect()
}

fn differential_growth(
  initial_nodes: &[(f64, f64)],
  config: &DiffGrowthConfig,
) -> Vec<(f64, f64)> {
  let mut nodes = initial_nodes.to_vec();

  let grid_bounds = (
    0.0,
    0.0,
    config.bounds.2 - config.bounds.0,
    config.bounds.3 - config.bounds.1,
  );
  let offset_x = config.bounds.0;
  let offset_y = config.bounds.1;

  let mut grid = SpatialGrid::new(grid_bounds, config.repulsion_radius);
  let mut neighbor_buf = Vec::with_capacity(64);

  for _iter in 0..config.iterations {
    let n = nodes.len();
    let mut forces: Vec<(f64, f64)> = vec![(0.0, 0.0); n];

    grid.clear();
    for (i, &(x, y)) in nodes.iter().enumerate() {
      grid.insert(i, x - offset_x, y - offset_y);
    }

    // 1. Repulsion
    let rep_r2 = config.repulsion_radius * config.repulsion_radius;
    for i in 0..n {
      let (ax, ay) = nodes[i];
      grid.query_neighbors_into(ax - offset_x, ay - offset_y, &mut neighbor_buf);
      for &j in &neighbor_buf {
        if j <= i {
          continue;
        }
        let (bx, by) = nodes[j];
        let dx = ax - bx;
        let dy = ay - by;
        let d2 = dx * dx + dy * dy;
        if d2 < rep_r2 && d2 > 0.0001 {
          let d = d2.sqrt();
          let force = config.repulsion_strength * (config.repulsion_radius - d) / d;
          let fx = dx * force;
          let fy = dy * force;
          forces[i].0 += fx;
          forces[i].1 += fy;
          forces[j].0 -= fx;
          forces[j].1 -= fy;
        }
      }
    }

    // 2. Attraction + 3. Alignment
    for i in 0..n {
      let prev = if i == 0 { n - 1 } else { i - 1 };
      let next = if i == n - 1 { 0 } else { i + 1 };

      let (px, py) = nodes[prev];
      let (cx, cy) = nodes[i];
      let (nx, ny) = nodes[next];

      let d_prev = euclidian_dist((cx, cy), (px, py));
      if d_prev > config.min_edge_length {
        forces[i].0 += (px - cx) * config.attraction_strength;
        forces[i].1 += (py - cy) * config.attraction_strength;
      }
      let d_next = euclidian_dist((cx, cy), (nx, ny));
      if d_next > config.min_edge_length {
        forces[i].0 += (nx - cx) * config.attraction_strength;
        forces[i].1 += (ny - cy) * config.attraction_strength;
      }

      let mid_x = (px + nx) * 0.5;
      let mid_y = (py + ny) * 0.5;
      forces[i].0 += (mid_x - cx) * config.alignment_strength;
      forces[i].1 += (mid_y - cy) * config.alignment_strength;
    }

    // 4. Boundary constraints
    for i in 0..n {
      let (x, y) = nodes[i];
      let (bx0, by0, bx1, by1) = config.bounds;
      let margin = config.repulsion_radius * 0.5;

      if x < bx0 + margin {
        forces[i].0 += config.boundary_strength * (bx0 + margin - x);
      }
      if x > bx1 - margin {
        forces[i].0 -= config.boundary_strength * (x - (bx1 - margin));
      }
      if y < by0 + margin {
        forces[i].1 += config.boundary_strength * (by0 + margin - y);
      }
      if y > by1 - margin {
        forces[i].1 -= config.boundary_strength * (y - (by1 - margin));
      }
    }

    // Apply forces with speed limit
    for i in 0..n {
      let (fx, fy) = forces[i];
      let mag = (fx * fx + fy * fy).sqrt();
      let (fx, fy) = if mag > config.max_speed {
        let s = config.max_speed / mag;
        (fx * s, fy * s)
      } else {
        (fx, fy)
      };
      nodes[i].0 += fx;
      nodes[i].1 += fy;
      nodes[i].0 = nodes[i].0.clamp(config.bounds.0, config.bounds.2);
      nodes[i].1 = nodes[i].1.clamp(config.bounds.1, config.bounds.3);
    }

    // 5. Split long edges
    if n < config.max_nodes {
      let safe_r2 = config.repulsion_radius * config.repulsion_radius * 0.5;
      let mut new_nodes = Vec::with_capacity(n + n / 4);
      for i in 0..n {
        new_nodes.push(nodes[i]);
        if new_nodes.len() < config.max_nodes {
          let next = if i == n - 1 { 0 } else { i + 1 };
          let d = euclidian_dist(nodes[i], nodes[next]);
          if d > config.max_edge_length {
            let mid = (
              (nodes[i].0 + nodes[next].0) * 0.5,
              (nodes[i].1 + nodes[next].1) * 0.5,
            );
            grid.query_neighbors_into(
              mid.0 - offset_x,
              mid.1 - offset_y,
              &mut neighbor_buf,
            );
            let mut safe = true;
            for &j in &neighbor_buf {
              if j == i
                || j == next
                || j == (if i == 0 { n - 1 } else { i - 1 })
                || j == (if next == n - 1 { 0 } else { next + 1 })
              {
                continue;
              }
              let dx = mid.0 - nodes[j].0;
              let dy = mid.1 - nodes[j].1;
              if dx * dx + dy * dy < safe_r2 {
                safe = false;
                break;
              }
            }
            if safe {
              new_nodes.push(mid);
            }
          }
        }
      }
      nodes = new_nodes;
    }
  }

  // Close the loop
  if nodes.len() > 1 {
    let first = nodes[0];
    nodes.push(first);
  }

  nodes
}

// ── RDP simplification ──────────────────────────────────────────────────────

fn rdp(points: &[(f64, f64)], epsilon: f64) -> Vec<(f64, f64)> {
  if points.len() < 3 {
    return points.to_vec();
  }
  let mut ranges = Vec::<std::ops::RangeInclusive<usize>>::new();
  let mut results = Vec::new();
  results.push(0);
  ranges.push(0..=points.len() - 1);

  while let Some(range) = ranges.pop() {
    let range_start = *range.start();
    let range_end = *range.end();
    let start = points[range_start];
    let end = points[range_end];

    let (max_distance, max_index) = points[range_start + 1..range_end]
      .iter()
      .enumerate()
      .fold((0.0_f64, 0), |(max_distance, max_index), (index, &point)| {
        let distance = line_point_distance(start, end, point);
        if distance > max_distance {
          (distance, index + 1)
        } else {
          (max_distance, max_index)
        }
      });

    if max_distance > epsilon {
      let division_point = range_start + max_index;
      let should_keep_second_half = division_point - range_start > 2;
      if should_keep_second_half {
        ranges.push(division_point..=range_end);
      }
      if division_point - range_start > 2 {
        ranges.push(range_start..=division_point);
      } else {
        results.push(division_point);
      }
      if !should_keep_second_half {
        results.push(range_end);
      }
    } else {
      results.push(range_end);
    }
  }

  results.iter().map(|&i| points[i]).collect()
}

fn line_point_distance(
  p1: (f64, f64),
  p2: (f64, f64),
  point: (f64, f64),
) -> f64 {
  let length = euclidian_dist(p1, p2);
  if length == 0.0 {
    return euclidian_dist(p1, point);
  }
  let a = p2.1 - p1.1;
  let b = p2.0 - p1.0;
  let c = (p2.0 * p1.1) - (p2.1 * p1.0);
  ((a * point.0) - (b * point.1) + c).abs() / length
}

// ── Art ─────────────────────────────────────────────────────────────────────

fn art(opts: &Opts) -> Document {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let precision = 0.2;

  let mut rng = SmallRng::from_seed({
    let mut seed = [0u8; 16];
    let bytes = (opts.seed as u64).to_le_bytes();
    seed[..8].copy_from_slice(&bytes);
    seed
  });

  let cx = width / 2.0;
  let cy = height / 2.0;
  let initial_radius = 10.0 + rng.gen_range(0.0, 10.0);
  let initial_nodes = circle_route((cx, cy), initial_radius, 40);

  let config = DiffGrowthConfig {
    repulsion_radius: 3.0 + rng.gen_range(0.0, 2.0),
    repulsion_strength: 1.0 + rng.gen_range(0.0, 0.5),
    attraction_strength: 0.2 + rng.gen_range(0.0, 0.15),
    alignment_strength: 0.3 + rng.gen_range(0.0, 0.3),
    max_edge_length: 1.5 + rng.gen_range(0.0, 0.5),
    min_edge_length: 0.3,
    bounds: (pad, pad, width - pad, height - pad),
    boundary_strength: 1.5,
    max_speed: 0.3 + rng.gen_range(0.0, 0.2),
    max_nodes: 20000,
    iterations: 1500,
  };

  let curve = differential_growth(&initial_nodes, &config);
  let curve = rdp(&curve, precision * 0.25);

  let mut data = Data::new();
  if !curve.is_empty() {
    data = data.move_to(curve[0]);
    for &p in &curve[1..] {
      data = data.line_to(p);
    }
  }

  let path = Path::new()
    .set("fill", "none")
    .set("stroke", "black")
    .set("stroke-width", 0.35)
    .set("d", data);

  let mut l = layer("0-diffgrowth");
  l = l.add(path);

  let mut document = base_a4_portrait("white");
  document = document.add(l);
  document
}

fn main() {
  let opts = Opts::parse();
  let document = art(&opts);
  svg::save(opts.file.clone(), &document).unwrap();
}
