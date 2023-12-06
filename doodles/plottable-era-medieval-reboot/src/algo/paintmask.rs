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

  pub fn clone_rescaled(&self, precision: f32) -> Self {
    if precision == self.precision {
      return self.clone();
    }
    let width = self.width;
    let height = self.height;
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    let mut next = Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    };
    for x in 0..wi {
      for y in 0..hi {
        let j = x + y * wi;
        let xf = x as f32 * precision;
        let yf = y as f32 * precision;
        next.mask[j] = self.is_painted((xf, yf));
      }
    }
    next
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

  pub fn is_painted(&self, (x, y): (f32, f32)) -> bool {
    let precision = self.precision;
    let wi = (self.width / precision) as usize;
    let hi = (self.height / precision) as usize;
    let xi = ((x / precision) as usize).min(wi - 1);
    let yi = ((y / precision) as usize).min(hi - 1);
    self.mask[xi + yi * wi]
  }

  pub fn manhattan_distance(&self) -> Vec<usize> {
    let precision = self.precision;
    let width = (self.width / precision) as usize;
    let height = (self.height / precision) as usize;
    let mut distances = vec![usize::MAX / 2; self.mask.len()];
    // Forward pass
    for y in 0..height {
      for x in 0..width {
        let idx = x + y * width;
        if self.mask[idx] {
          distances[idx] = 0;
        } else {
          if x > 0 {
            let i = x - 1 + y * width;
            distances[idx] = distances[idx].min(distances[i] + 1);
          }
          if y > 0 {
            let i = x + (y - 1) * width;
            distances[idx] = distances[idx].min(distances[i] + 1);
          }
        }
      }
    }
    // Backward pass
    for y in (0..height).rev() {
      for x in (0..width).rev() {
        let idx = x + y * width;
        if x < width - 1 {
          let i = x + 1 + y * width;
          distances[idx] = distances[idx].min(distances[i] + 1);
        }
        if y < height - 1 {
          let i = x + (y + 1) * width;
          distances[idx] = distances[idx].min(distances[i] + 1);
        }
      }
    }
    distances
  }

  pub fn dilate_manhattan(&mut self, radius: f32) {
    let distances = self.manhattan_distance();
    self.assign_data_lower_than_threshold(&distances, radius);
  }

  pub fn assign_data_lower_than_threshold(
    &mut self,
    data: &Vec<usize>,
    radius: f32,
  ) {
    let threshold = (radius / self.precision) as usize;
    let wi = (self.width / self.precision) as usize;
    let hi = (self.height / self.precision) as usize;
    for y in 0..hi {
      for x in 0..wi {
        let i = x + y * wi;
        if data[i] <= threshold {
          self.mask[i] = true;
        }
      }
    }
  }

  pub fn paint(&mut self, other: &Self) {
    if other.width != self.width
      || other.height != self.height
      || other.precision != self.precision
    {
      // alternative less efficient way when the sizes are different
      let wi = (self.width / self.precision) as usize;
      let hi = (self.height / self.precision) as usize;
      for x in 0..wi {
        let xf = x as f32 * self.precision;
        for y in 0..hi {
          let yf = y as f32 * self.precision;
          if other.is_painted((xf, yf)) {
            let i: usize = x + y * wi;
            self.mask[i] = true;
          }
        }
      }
    } else {
      // regular way
      for (i, &v) in other.mask.iter().enumerate() {
        if v {
          self.mask[i] = true;
        }
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
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let point = (x as f32 * precision, y as f32 * precision);
        if f(point) {
          self.mask[j] = true;
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
      panic!("PaintMask::paint: incompatible sizes");
    }

    let len = self.mask.len();
    let mut i = 0;
    while i < len {
      if !other.mask[i] {
        self.mask[i] = false;
      }
      i += 1;
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
    // TODO we just need to calculate the boundaries. there are ways to iter more faster without having to check all cells
    for x in 0..wi {
      for y in 0..hi {
        if self.mask[x + y * wi] {
          let xf = x as f32 * precision;
          let yf = y as f32 * precision;
          minx = minx.min(xf);
          miny = miny.min(yf);
          maxx = maxx.max(xf);
          maxy = maxy.max(yf);
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
    let cr2 = cr * cr;
    for x in minx..maxx {
      for y in miny..maxy {
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let point = (x as f32 * precision, y as f32 * precision);
        let dx = point.0 - cx;
        let dy = point.1 - cy;
        if dx * dx + dy * dy < cr2 {
          self.mask[j] = true;
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
    self.paint_rectangle_v(minx, miny, maxx, maxy, true);
  }

  pub fn paint_rectangle_v(
    &mut self,
    minx: f32,
    miny: f32,
    maxx: f32,
    maxy: f32,
    v: bool,
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
        self.mask[x + y * wi] = v;
      }
    }
  }

  pub fn paint_borders(&mut self, pad: f32) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
  }

  pub fn unpaint_borders(&mut self, pad: f32) {
    self.paint_rectangle_v(0., 0., self.width, pad, false);
    self.paint_rectangle_v(0., 0., pad, self.height, false);
    self.paint_rectangle_v(
      0.,
      self.height - pad,
      self.width,
      self.height,
      false,
    );
    self.paint_rectangle_v(
      self.width - pad,
      0.,
      self.width,
      self.height,
      false,
    );
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
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let point = (x as f32 * precision, y as f32 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[j] = true;
        }
      }
    }
  }

  pub fn paint_polyline(&mut self, polyline: &Vec<(f32, f32)>, strokew: f32) {
    let len = polyline.len();
    if len < 1 {
      return;
    }
    let first = polyline[0];
    let mut minx = first.0;
    let mut miny = first.1;
    let mut maxx = first.0;
    let mut maxy = first.1;
    let mut i = 1;
    while i < len {
      let (x, y) = polyline[i];
      if x < minx {
        minx = x;
      }
      if x > maxx {
        maxx = x;
      }
      if y < miny {
        miny = y;
      }
      if y > maxy {
        maxy = y;
      }
      i += 1;
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
      let xf = x as f32 * precision;
      for y in miny..maxy {
        let j = x + y * wi;
        if self.mask[j] {
          continue;
        }
        let yf = y as f32 * precision;
        let point = (xf, yf);
        let mut i = 1;
        let mut prev = polyline[0];
        while i < len {
          let next = polyline[i];
          if point_in_segment(point, prev, next, strokew) {
            self.mask[j] = true;
            break;
          }
          i += 1;
          prev = next;
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
