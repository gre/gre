use crate::algo::clipping::*;
use crate::algo::math2d::*;
use crate::algo::polygon::*;
use rand::prelude::*;
use std::cmp::Ordering;
use std::f32::consts::PI;

use super::paintmask::PaintMask;
use super::polylines::Polylines;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

// it's like a Renderable item, but with a focus on polygon like shapes that can be slices and explored into parts (for the destruction logic)
#[derive(Clone)]
pub struct RenderItem {
  pub routes: Polylines,
  pub polygons: Vec<Vec<(f32, f32)>>,
  pub zorder: f32,
  // sometimes, the objects are not designed to fit into the polygons realm / uses diff paradigm,
  // so to connect both worlds, a special object will store the object with an id.
  // it's a ForeignObject in order to track the possible translation and rotation
  pub foreign: Option<ForeignObject>,
}

#[derive(Clone)]
pub struct ForeignObject {
  pub id: usize,
  pub current_absolute_pos: (f32, f32),
  // accumulated translation and rotation
  pub translation: (f32, f32),
  pub rotation: f32,
}

impl ForeignObject {
  pub fn new(id: usize, current_absolute_pos: (f32, f32)) -> Self {
    Self {
      id,
      current_absolute_pos,
      translation: (0., 0.),
      rotation: 0.,
    }
  }
}

impl PartialEq for RenderItem {
  fn eq(&self, other: &Self) -> bool {
    self.zorder == other.zorder
  }
}

impl Eq for RenderItem {}

impl PartialOrd for RenderItem {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.zorder.partial_cmp(&self.zorder)
  }
}

impl Ord for RenderItem {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap_or(Ordering::Equal)
  }
}

impl RenderItem {
  pub fn new(
    routes: Polylines,
    polygons: Vec<Vec<(f32, f32)>>,
    zorder: f32,
  ) -> Self {
    Self {
      routes,
      polygons,
      zorder,
      foreign: None,
    }
  }

  pub fn from_foreign(
    foreign_id: usize,
    zorder: f32,
    current_absolute_pos: (f32, f32),
  ) -> Self {
    Self {
      routes: vec![],
      polygons: vec![],
      zorder,
      foreign: Some(ForeignObject::new(foreign_id, current_absolute_pos)),
    }
  }

  pub fn render(&self, paint: &mut PaintMask) -> Polylines {
    let rts = regular_clip_polys(&self.routes, paint, &self.polygons);
    rts
  }
}

pub fn multicut_along_line<R: Rng>(
  rng: &mut R,
  items_in: &Vec<RenderItem>,
  clr: usize,
  from: (f32, f32),
  to: (f32, f32),
  mut increment_f: impl FnMut(&mut R) -> f32,
  mut angle_delta_f: impl FnMut(&mut R) -> f32,
  mut sliding_f: impl FnMut(&mut R) -> f32,
  // TODO we want to be able to make pushback only apply on one side (to the up on our side). so maybe we need 2 values
  mut pushback_f: impl FnMut(&mut R) -> f32,
  mut pushback_rotation_f: impl FnMut(&mut R) -> f32,
) -> Vec<RenderItem> {
  let mut items = items_in.clone();
  let initial = increment_f(rng) / 2.0;
  let mut d = initial;
  let l = euclidian_dist(from, to);
  let dx = to.0 - from.0;
  let dy = to.1 - from.1;
  let a = dy.atan2(dx);
  while d < l - initial {
    let p = lerp_point(from, to, d / l);
    let ang = a + PI / 2.0 + angle_delta_f(rng);
    let sliding = sliding_f(rng);
    let pushback = pushback_f(rng);
    let pushback_rotation = pushback_rotation_f(rng);
    items = binary_cut_and_slide(
      &items,
      p,
      ang,
      sliding,
      pushback,
      pushback_rotation,
      clr,
    );
    d += increment_f(rng);
  }

  items
}

pub fn binary_cut_and_slide(
  items_in: &Vec<RenderItem>,
  center: (f32, f32),
  ang: f32,
  sliding: f32,
  pushback: f32,
  pushback_rotation: f32,
  clr: usize,
) -> Vec<RenderItem> {
  let dx = ang.cos();
  let dy = ang.sin();
  let amp = 1000.0;
  let a = (center.0 + amp * dx, center.1 + amp * dy);
  let b = (center.0 - amp * dx, center.1 - amp * dy);

  let is_left =
    |(x, y)| (x - center.0) * (b.1 - a.1) - (y - center.1) * (b.0 - a.0) > 0.0;
  let is_right = |p| !is_left(p);

  let project = |(x, y), leftmul| {
    let local = (x - center.0, y - center.1);
    let local = p_r(local, pushback_rotation * leftmul);
    (
      center.0 + local.0 + (sliding * dx - pushback * dy) * leftmul,
      center.1 + local.1 + (sliding * dy + pushback * dx) * leftmul,
    )
  };

  let mut items = vec![];
  for item in items_in.iter() {
    let mut polygons_left = vec![];
    let mut polygons_right = vec![];
    let foreign = item.foreign.clone().map(|f| {
      let mut f = f.clone();
      let pos = f.current_absolute_pos;
      let leftmul = if is_left(pos) { 1.0 } else { -1.0 };
      f.rotation += pushback_rotation * leftmul;
      let newp = project(pos, leftmul);
      let diff = (newp.0 - pos.0, newp.1 - pos.1);
      f.translation.0 += diff.0;
      f.translation.1 += diff.1;
      f.current_absolute_pos = newp;
      f
    });

    for poly in &item.polygons {
      let out: Vec<Vec<(f32, f32)>> = cut_polygon(&poly, a, b);
      for p in out {
        let mut c = (0., 0.);
        for point in p.iter() {
          c.0 += point.0;
          c.1 += point.1;
        }
        let len = p.len() as f32;
        c = (c.0 / len, c.1 / len);

        let left = is_left(c);
        let leftmul = if left { 1.0 } else { -1.0 };
        let p = p.iter().map(|&p| project(p, leftmul)).collect();
        if left {
          polygons_left.push(p);
        } else {
          polygons_right.push(p);
        }
      }
    }

    let mut left_routes =
      clip_routes_with_colors(&item.routes, &is_right, 0.5, 4);
    let mut right_routes =
      clip_routes_with_colors(&item.routes, &is_left, 0.5, 4);

    let out_of_polys = |p| !polygons_includes_point(&item.polygons, p);

    let cut_routes =
      clip_routes_with_colors(&vec![(clr, vec![a, b])], &out_of_polys, 1.0, 3);

    left_routes.extend(cut_routes.clone());
    right_routes.extend(cut_routes.clone());

    for (_, rt) in &mut left_routes {
      for p in rt {
        *p = project(*p, 1.0);
      }
    }

    for (_, rt) in &mut right_routes {
      for p in rt {
        *p = project(*p, -1.0);
      }
    }

    let zorder = item.zorder;

    if left_routes.len() > 0 {
      items.push(RenderItem {
        routes: left_routes,
        polygons: polygons_left,
        zorder,
        foreign: None,
      });
    }

    if right_routes.len() > 0 {
      items.push(RenderItem {
        routes: right_routes,
        polygons: polygons_right,
        zorder,
        foreign: None,
      });
    }

    if foreign.is_some() {
      items.push(RenderItem {
        routes: vec![],
        polygons: vec![],
        zorder,
        foreign,
      });
    }
  }

  items
}
