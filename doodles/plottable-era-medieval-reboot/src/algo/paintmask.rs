use crate::algo::math2d::*;
use crate::algo::polygon::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

#[derive(Clone)]
pub struct PaintMask {
  mask: Vec<bool>,
  pub precision: f32,
  pub width: f32,
  pub height: f32,
}

impl PaintMask {
  pub fn clone_empty(&self) -> Self {
    let wi = (self.width / self.precision) as usize;
    let hi = (self.height / self.precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width: self.width,
      height: self.height,
      precision: self.precision,
    }
  }

  pub fn new(precision: f32, width: f32, height: f32) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    }
  }

  pub fn is_painted(&self, point: (f32, f32)) -> bool {
    let precision = self.precision;
    let wi = (self.width / precision) as usize;
    let hi = (self.height / precision) as usize;
    let x = ((point.0.max(0.) / precision) as usize).min(wi - 1);
    let y = ((point.1.max(0.) / precision) as usize).min(hi - 1);
    self.mask[x + y * wi]
  }

  pub fn grow(&mut self, growpad: f32) {
    let wi = (self.width / self.precision) as usize;

    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let data: Vec<bool> = self.mask.iter().cloned().collect();
    let mut pos = Vec::new();
    let mut x = -growpad;
    loop {
      if x >= growpad {
        break;
      }
      let mut y = -growpad;
      loop {
        if y >= growpad {
          break;
        }
        if x * x + y * y < growpad * growpad {
          pos.push((x, y));
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
      let xi = (x / precision) as usize;
      let mut y = 0.0;
      loop {
        if y >= height {
          break;
        }
        let index = xi + (y / precision) as usize * wi;
        if data[index] {
          for &(dx, dy) in pos.iter() {
            let x = x + dx;
            let y = y + dy;
            let i = (x / precision) as usize + (y / precision) as usize * wi;
            self.mask[i] = true;
          }
        }
        y += precision;
      }
      x += precision;
    }
  }

  pub fn paint(&mut self, other: &Self) {
    if other.width != self.width
      || other.height != self.height
      || other.precision != self.precision
    {
      panic!("PaintMask::paint: incompatible sizes");
    }
    for (i, &v) in other.mask.iter().enumerate() {
      if v {
        self.mask[i] = true;
      }
    }
  }

  pub fn paint_fn<F: Fn((f32, f32)) -> bool>(&mut self, f: F) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    for x in 0..wi {
      for y in 0..hi {
        let point = (x as f32 * precision, y as f32 * precision);
        if f(point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  pub fn reverse(&mut self) {
    for v in self.mask.iter_mut() {
      *v = !*v;
    }
  }

  pub fn intersects(&mut self, other: &Self) {
    if other.width != self.width
      || other.height != self.height
      || other.precision != self.precision
    {
      panic!("PaintMask::intersection: incompatible sizes");
    }
    for (i, &v) in other.mask.iter().enumerate() {
      if !v {
        self.mask[i] = false;
      }
    }
  }

  pub fn painted_boundaries(&self) -> (f32, f32, f32, f32) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    let mut minx = width;
    let mut miny = height;
    let mut maxx = 0.0f32;
    let mut maxy = 0.0f32;
    for x in 0..wi {
      for y in 0..hi {
        if self.mask[x + y * wi] {
          minx = minx.min(x as f32 * precision);
          miny = miny.min(y as f32 * precision);
          maxx = maxx.max(x as f32 * precision);
          maxy = maxy.max(y as f32 * precision);
        }
      }
    }
    if minx > maxx || miny > maxy {
      minx = 0.0;
      maxx = 0.0;
      miny = 0.0;
      maxy = 0.0;
    }
    (minx, miny, maxx, maxy)
  }

  /*
  paint.paint_columns_left_to_right(&|x| {
    let yridge = lookup_ridge(&self.ridge, x).min(yhorizon);
    yridge..yhorizon
  });
  */
  pub fn paint_columns_left_to_right<F: Fn(f32) -> std::ops::Range<f32>>(
    &mut self,
    f: F,
  ) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    for x in 0..wi {
      let range = f(x as f32 * precision);
      let miny = (range.start.max(0.) / precision) as usize;
      let maxy = ((range.end / precision) as usize).min(hi);
      for y in miny..maxy {
        self.mask[x + y * wi] = true;
      }
    }
  }

  pub fn paint_circle(&mut self, cx: f32, cy: f32, cr: f32) {
    let (minx, miny, maxx, maxy) = (
      (cx - cr).max(0.),
      (cy - cr).max(0.),
      (cx + cr).min(self.width),
      (cy + cr).min(self.height),
    );
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f32 * precision, y as f32 * precision);
        if euclidian_dist(point, (cx, cy)) < cr {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  pub fn paint_pixels(
    &mut self,
    topleft: (f32, f32),
    data: &Vec<u8>,
    datawidth: usize,
  ) {
    let precision = self.precision;
    let ox = (topleft.0 / self.precision).max(0.0) as usize;
    let oy = (topleft.1 / self.precision).max(0.0) as usize;
    let wi = (self.width / precision) as usize;
    let hi = (self.height / precision) as usize;
    for (i, &v) in data.iter().enumerate() {
      if v > 0 {
        let dx = i % datawidth;
        let dy = i / datawidth;
        let x = ox + dx;
        let y = oy + dy;
        if x < wi && y < hi {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  pub fn paint_rectangle(
    &mut self,
    minx: f32,
    miny: f32,
    maxx: f32,
    maxy: f32,
  ) {
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        self.mask[x + y * wi] = true;
      }
    }
  }

  pub fn paint_borders(&mut self, pad: f32) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
  }

  pub fn paint_polygon(&mut self, polygon: &Vec<(f32, f32)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f32 * precision, y as f32 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  pub fn paint_polyline(&mut self, polyline: &Vec<(f32, f32)>, strokew: f32) {
    if polyline.len() < 1 {
      return;
    }
    let first = polyline[0];
    let mut minx = first.0;
    let mut miny = first.1;
    let mut maxx = first.0;
    let mut maxy = first.1;
    for p in polyline.iter().skip(1) {
      minx = minx.min(p.0);
      miny = miny.min(p.1);
      maxx = maxx.max(p.0);
      maxy = maxy.max(p.1);
    }
    minx = (minx - strokew).max(0.0);
    miny = (miny - strokew).max(0.0);
    maxx = (maxx + strokew).min(self.width);
    maxy = (maxy + strokew).min(self.height);

    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f32 * precision, y as f32 * precision);
        for i in 0..polyline.len() - 1 {
          let a = polyline[i];
          let b = polyline[i + 1];
          if point_in_segment(point, a, b, strokew) {
            self.mask[x + y * wi] = true;
            break;
          }
        }
      }
    }
  }
}

fn point_in_segment(
  (px, py): (f32, f32),
  (ax, ay): (f32, f32),
  (bx, by): (f32, f32),
  strokew: f32,
) -> bool {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;
  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let h = (dot_pa_ba / dot_ba_ba).max(0.0).min(1.0);
  let h_x = ba_x * h;
  let h_y = ba_y * h;
  let dx = pa_x - h_x;
  let dy = pa_y - h_y;
  dx * dx + dy * dy < strokew * strokew
}
