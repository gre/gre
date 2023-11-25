use std::{f32::consts::PI, ops::Range};

use super::{
  math2d::{euclidian_dist, lerp_point},
  polylines::Polyline,
};
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct PathLookup {
  path: Vec<(f32, f32)>,
  local_length: Vec<f32>,
  partial_length: Vec<f32>,
  length: f32,
}

impl PathLookup {
  pub fn init(path: Vec<(f32, f32)>) -> Self {
    let mut length = 0.0;
    let mut partial_length = vec![];
    let mut local_length = vec![];
    for i in 0..path.len() - 1 {
      partial_length.push(length);
      let l = euclidian_dist(path[i], path[i + 1]);
      local_length.push(l);
      length += l;
    }
    partial_length.push(length);

    Self {
      path,
      length,
      local_length,
      partial_length,
    }
  }

  pub fn length(&self) -> f32 {
    self.length
  }

  pub fn build_path(&self, range: Range<f32>, offset: f32) -> Vec<(f32, f32)> {
    let mut interesting_points = vec![range.start];
    interesting_points.extend(
      self
        .partial_length
        .iter()
        .filter(|p| range.contains(*p))
        .cloned()
        .collect::<Vec<_>>(),
    );
    interesting_points.push(range.end);

    let mut path = vec![];
    for p in interesting_points {
      let pos = self.lookup_pos(p);
      let ang = self.lookup_angle(p) + PI / 2.0;
      let pos = (offset * ang.cos() + pos.0, offset * ang.sin() + pos.1);
      path.push(pos);
    }
    path
  }

  pub fn lookup_pos(&self, l: f32) -> (f32, f32) {
    let path = &self.path;
    if l < 0.0 {
      return path[0];
    }
    for i in 0..path.len() - 1 {
      let pl = self.partial_length[i + 1];
      if l < pl {
        let a = path[i];
        let b = path[i + 1];
        let m = (pl - l) / self.local_length[i];
        return lerp_point(b, a, m);
      }
    }
    return path[path.len() - 1];
  }

  pub fn slice_before(&self, l: f32) -> Polyline {
    let mut out = vec![];
    let path = &self.path;
    if l < 0.0 {
      return out;
    }
    for i in 0..path.len() - 1 {
      let pl = self.partial_length[i + 1];
      let a = path[i];
      out.push(a);
      if l < pl {
        let b = path[i + 1];
        let m = (pl - l) / self.local_length[i];
        out.push(lerp_point(b, a, m));
        return out;
      }
    }
    out.push(path[path.len() - 1]);
    out
  }

  pub fn lookup_angle(&self, l: f32) -> f32 {
    let path = &self.path;
    if l < 0.0 {
      return angle2(path[0], path[1]);
    }
    for i in 0..path.len() - 1 {
      let pl = self.partial_length[i + 1];
      if l < pl {
        let a = path[i];
        let b = path[i + 1];
        let angle = angle2(a, b);
        return angle;
      }
    }
    let len = path.len();
    return angle2(path[len - 2], path[len - 1]);
  }
}

fn angle2(p1: (f32, f32), p2: (f32, f32)) -> f32 {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  let dx = x2 - x1;
  let dy = y2 - y1;
  dy.atan2(dx)
}
