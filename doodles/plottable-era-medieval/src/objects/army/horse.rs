use crate::{
  algo::{
    clipping::{regular_clip, regular_clip_polys},
    math1d::mix,
    paintmask::PaintMask,
    polygon::make_wireframe_from_vertexes,
    polylines::{
      grow_as_rectangle, path_subdivide_to_curve, route_scale_translate_rotate,
      Polyline, Polylines,
    },
    renderable::Renderable,
    wormsfilling::WormsFilling,
  },
  global::GlobalCtx,
};
use rand::prelude::*;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Horse {
  pub origin: (f32, f32),
  pub size: f32,
  pub angle: f32,
  pub xflip: bool,
  pub mainclr: usize,
  pub blazonclr: usize,
  pub decorationratio: f32,
  pub foot_offset: f32,
}

impl Horse {
  pub fn init(
    origin: (f32, f32),
    size: f32,
    angle: f32,
    xflip: bool,
    mainclr: usize,
    blazonclr: usize,
    decorationratio: f32,
    foot_offset: f32,
  ) -> Self {
    Self {
      origin,
      size,
      angle,
      xflip,
      mainclr,
      blazonclr,
      decorationratio,
      foot_offset,
    }
  }

  pub fn render<R: Rng>(&self, rng: &mut R, mask: &mut PaintMask) -> Polylines {
    let mut out = vec![];
    let beforemask = mask.clone();

    let xflip = self.xflip;
    let origin = self.origin;
    let size = self.size;
    let angle = self.angle;
    let mainclr = self.mainclr;
    let blazonclr = self.blazonclr;
    let foot_offset = self.foot_offset;

    let mut highlight_paint = mask.clone_empty_rescaled(1.0);
    let mut highlighted: Vec<Polyline> = vec![];

    let xdir = if xflip { -1.0 } else { 1.0 };

    let x0 = -size * rng.gen_range(0.4..0.5);
    let x1 = -size * rng.gen_range(0.3..0.4);
    let x2 = size * rng.gen_range(0.25..0.35);
    let x3 = size * rng.gen_range(0.4..0.5);
    let yleft = size * rng.gen_range(0.0..0.1);
    let yright = -size * rng.gen_range(0.6..0.8);

    let dy_edge = 0.3;

    let scale = (xdir, 1.0);

    // HORSE HEAD

    let mut headroutes = vec![];
    let a = (x3 - rng.gen_range(0.0..0.05) * size, yright);
    let b = (x3 + rng.gen_range(0.15..0.3) * size, yright + 0.3 * size);
    let pts = route_scale_translate_rotate(&vec![a, b], scale, origin, angle);
    let a = pts[0];
    let b = pts[1];
    let rect: Vec<(f32, f32)> = grow_as_rectangle(a, b, 0.1 * size);
    let ythreshold = a.1.max(b.1);
    let mut topline = rect
      .iter()
      .filter(|(_, y)| *y < ythreshold)
      .cloned()
      .collect::<Vec<_>>();
    topline.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    highlighted.push(topline);
    headroutes.push((mainclr, rect.clone()));
    headroutes = regular_clip(&headroutes, mask);
    mask.paint_polygon(&rect);
    highlight_paint.paint_polygon(&rect);
    out.extend(headroutes);

    // HORSE BODY

    let splits = 1;
    // horse body bottom
    let mut route = Vec::new();
    route.push((x0, yleft + dy_edge));
    route.push((x1, 0.0));
    route.push((x2, 0.0));
    route.push((x3 + 0.05 * size, yright + dy_edge + 0.15 * size));
    route = route_scale_translate_rotate(&route, scale, origin, angle);
    route = path_subdivide_to_curve(&route, splits, 0.8);
    let body_bottom = route;
    // horse body top
    let mut route = Vec::new();
    let y = -0.3 * size;
    route.push((x0, yleft - dy_edge));
    route.push((x1, y));
    route.push((x2, y));
    route.push((x3 - 0.05 * size, yright - dy_edge));
    route = route_scale_translate_rotate(&route, scale, origin, angle);
    route = path_subdivide_to_curve(&route, splits, 0.8);
    let body_top = route;
    highlighted.push(body_top.clone());

    let wireframe = make_wireframe_from_vertexes(&body_top, &body_bottom);

    let mut body = vec![];
    body.push((mainclr, body_bottom));
    body.push((mainclr, body_top));
    body = regular_clip_polys(&body, mask, &wireframe);
    for poly in wireframe.iter() {
      highlight_paint.paint_polygon(poly);
    }

    // HORSE FOOT
    let footw = 0.04 * size;
    for (footw, a, b) in vec![
      // make horse left feet
      (
        footw,
        (x1 + 0.1 * size, y + 0.2 * size),
        (
          x1 + mix(0.1, rng.gen_range(-0.1..0.1), foot_offset) * size,
          y + 0.5 * size,
        ),
      ),
      // make horse right feet
      (
        footw,
        (x3 - 0.1 * size, y),
        (
          x3 + mix(-0.1, rng.gen_range(-0.2..0.3), foot_offset) * size,
          y + mix(0.5, 0.4, foot_offset) * size,
        ),
      ),
    ] {
      let localrect = grow_as_rectangle(a, b, footw);
      let route =
        route_scale_translate_rotate(&localrect, scale, origin, angle);
      let poly = route.clone();
      let rts = vec![(mainclr, route)];
      let rts = regular_clip(&rts, mask);
      mask.paint_polygon(&poly);
      if self.decorationratio >= 1.0 {
        // a high decoration ratio indicates we want to colorize foot too
        highlight_paint.paint_polygon(&poly);
      }
      out.extend(rts);
    }
    out.extend(body);

    let mut highlight2_paint = highlight_paint.clone_empty();
    let highlightw = self.decorationratio * size / 2.0;
    for rt in highlighted {
      highlight2_paint.paint_polyline(&rt, highlightw);
    }
    highlight_paint.intersects(&highlight2_paint);
    let filling = WormsFilling::rand(rng);
    let bound = highlight_paint.painted_boundaries();
    let iterations = 500;
    let density = 2.0;
    out.extend(regular_clip(
      &filling.fill_in_paint(
        rng,
        &highlight_paint,
        blazonclr,
        density,
        bound,
        iterations,
      ),
      &beforemask,
    ));

    out
  }
}

impl<R: Rng> Renderable<R> for Horse {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut GlobalCtx,
    paint: &mut PaintMask,
  ) -> Polylines {
    self.render(rng, paint)
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
