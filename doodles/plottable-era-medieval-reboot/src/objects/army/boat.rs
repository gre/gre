use crate::{
  algo::{
    clipping::{clip_routes_with_colors, regular_clip, regular_clip_polys},
    math1d::{mix, values_subdivide_to_curve},
    math2d::{angle_mirrored_on_x, lerp_point},
    paintmask::PaintMask,
    pathlookup::PathLookup,
    polygon::polygon_includes_point,
    polylines::{
      path_subdivide_to_curve, path_to_fibers, route_scale_translate_rotate,
      scale_translate_rotate, Polylines,
    },
  },
  objects::blazon::Blazon,
};
use rand::prelude::*;
use std::f32::consts::PI;

use super::{belierhead::BelierHead, flag::Flag};

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub struct Boat {
  pub x1: f32,
  pub x2: f32,
  pub origin: (f32, f32),
  pub size: f32,
  pub angle: f32,
  pub w: f32,
  pub xflip: bool,
  pub blazon: Blazon,
  pub clr: usize,
  pub blazonclr: usize,
  pub boatglobs: BoatGlobals,
  pub with_mast: bool,
  pub with_sailing: bool,
  pub with_sailing_colored: bool,
  pub with_flag: bool,
  pub with_ropes: bool,
  pub boat_bottom: Vec<(f32, f32)>,
  pub boat_middle: Vec<(f32, f32)>,
  pub boat_top: Vec<(f32, f32)>,
  pub head: BelierHead,
}

#[derive(Clone)]
pub struct BoatGlobals {
  pub mast_p: f64,
  pub sailing_p: f64,
  pub sailing_colored_p: f64,
  pub flag_p: f64,
  pub ropes_p: f64,
  pub decoration_stripes: usize,
  pub decoration_wide: f32,
  pub decoration_fibers_density: f32,
  pub sailing_divisions: usize,
  pub sailing_interp_n: usize,
}
impl BoatGlobals {
  pub fn rand<R: Rng>(rng: &mut R) -> Self {
    let stripes = if rng.gen_bool(0.2) {
      1
    } else {
      2 + (rng.gen_range(0.0..8.0) * rng.gen_range(0.0..1.0)) as usize
    };
    let wide = 1.0 + rng.gen_range(0.0..0.8) * rng.gen_range(0.0..1.0);
    let fibers_density = rng.gen_range(1.0..2.0);
    let sailing_divisions = rng.gen_range(3..12);
    let sailing_interp_n = if rng.gen_bool(0.8) { 1 } else { 0 };

    Self {
      mast_p: (1.2f64
        - 1.4 * rng.gen_range(0.0..1.0) * rng.gen_range(0.5..1.0))
      .max(0.00001)
      .min(0.99999),
      sailing_p: (1.5f64 - rng.gen_range(0.0..1.0) * rng.gen_range(0.5..1.0))
        .max(0.00001)
        .min(0.99999),
      sailing_colored_p: rng.gen_range(-2.0f64..2.0).max(0.00001).min(0.99999),
      flag_p: (1.2f64
        - 1.4 * rng.gen_range(0.0..1.0) * rng.gen_range(0.5..1.0))
      .max(0.00001)
      .min(0.99999),
      ropes_p: (1.2f64
        - 1.4 * rng.gen_range(0.0..1.0) * rng.gen_range(0.5..1.0))
      .max(0.00001)
      .min(0.99999),
      decoration_stripes: stripes,
      decoration_wide: wide,
      decoration_fibers_density: fibers_density,
      sailing_divisions,
      sailing_interp_n,
    }
  }
}

impl Boat {
  pub fn init<R: Rng>(
    rng: &mut R,
    origin: (f32, f32),
    size: f32,
    angle: f32,
    w: f32,
    xflip: bool,
    blazon: Blazon,
    clr: usize,
    blazonclr: usize,
    boatglobs: &BoatGlobals,
  ) -> Self {
    let x1 = -w * rng.gen_range(0.3..0.45);
    let x2 = w * rng.gen_range(0.3..0.4);

    let xdir = if xflip { -1.0 } else { 1.0 };
    let scale = (xdir, 1.0);

    let h = size;
    let yleft = -h * rng.gen_range(0.6..1.0);
    let yright = -h * rng.gen_range(0.8..1.0);

    let dy_edge = 0.3;
    // boat bottom
    let mut route = Vec::new();
    route.push((-w / 2.0 - dy_edge, yleft + dy_edge));
    route.push((x1, 0.0));
    route.push((x2, 0.0));
    route.push((w / 2.0 + dy_edge, yright + dy_edge));
    route = path_subdivide_to_curve(&route, 2, 0.8);
    route = route_scale_translate_rotate(&route, scale, origin, angle);
    let boat_bottom = route.clone();

    // boat in between
    let mut route = Vec::new();
    let y = -0.15 * h;
    route.push((-w / 2.0, yleft));
    route.push((x1, y));
    route.push((x2, y));
    route.push((w / 2.0, yright));
    route = path_subdivide_to_curve(&route, 2, 0.8);
    route = route_scale_translate_rotate(&route, scale, origin, angle);
    let boat_middle = route.clone();

    // boat top
    let mut route = Vec::new();
    let y = -0.3 * h;
    route.push((-w / 2.0 + dy_edge, yleft - dy_edge));
    route.push((x1, y));
    route.push((x2, y));
    route.push((w / 2.0 - dy_edge, yright - dy_edge));
    route = path_subdivide_to_curve(&route, 2, 0.8);
    route = route_scale_translate_rotate(&route, scale, origin, angle);
    let boat_top = route.clone();

    // make a boat head
    let o =
      scale_translate_rotate((w / 2.0, yright), (xdir, 1.), origin, angle);
    let s = 0.5 * size;
    let a = angle - PI / 4.0;

    let head = BelierHead::init(rng, clr, o, s, a, xflip);

    let boatglobs = boatglobs.clone();

    let with_mast = boatglobs.mast_p >= 1.0
      || boatglobs.mast_p > 0.0 && rng.gen_bool(boatglobs.mast_p);

    let with_sailing = with_mast
      && (boatglobs.sailing_p >= 1.0
        || boatglobs.sailing_p > 0.0 && rng.gen_bool(boatglobs.sailing_p));

    let with_sailing_colored = with_sailing
      && (boatglobs.sailing_colored_p >= 1.0
        || boatglobs.sailing_colored_p > 0.0
          && rng.gen_bool(boatglobs.sailing_colored_p));

    let with_flag = with_mast
      && (boatglobs.flag_p >= 1.0
        || boatglobs.flag_p > 0.0 && rng.gen_bool(boatglobs.flag_p));

    let with_ropes = with_sailing
      || with_mast
        && (boatglobs.ropes_p >= 1.0
          || boatglobs.ropes_p > 0.0 && rng.gen_bool(boatglobs.ropes_p));

    Self {
      x1,
      x2,
      origin,
      size,
      angle,
      w,
      xflip,
      blazon,
      clr,
      blazonclr,
      boatglobs,
      with_mast,
      with_sailing,
      with_sailing_colored,
      with_flag,
      with_ropes,
      boat_bottom,
      boat_middle,
      boat_top,
      head,
    }
  }

  pub fn render_main_only(
    &self,
    mask: &mut PaintMask,
    clr: usize,
  ) -> Polylines {
    let size = self.size;

    let mut out = vec![];
    let mut routes = vec![];

    routes.push((clr, self.boat_bottom.clone()));
    routes.push((clr, self.boat_middle.clone()));
    routes.push((clr, self.boat_top.clone()));
    out.extend(self.head.render(mask));

    routes = regular_clip(&routes, mask);
    for (_clr, route) in &routes {
      mask.paint_polyline(route, 0.1 * size);
    }
    out.extend(routes);

    out
  }

  pub fn render_background_only<R: Rng>(
    &self,
    rng: &mut R,
    mask: &mut PaintMask,
    clr: usize,
  ) -> Polylines {
    let size = self.size;
    let origin = self.origin;
    let angle = self.angle;
    let w = self.w;
    let xflip = self.xflip;
    let blazonclr = self.blazonclr;

    let mut out = vec![];

    let xdir = if xflip { -1.0 } else { 1.0 };
    let scale = (xdir, 1.0);

    // mast
    if self.with_mast {
      let main_h = (rng.gen_range(2.8..3.6) * size).min(self.w * 2.0);
      let second_w = rng
        .gen_range((if self.with_sailing { 0.25 } else { 0.1 })..0.4)
        * self.w.min(4.0 * size);
      let y = -main_h;
      let y2 = rng.gen_range(0.8..1.0) * y;

      if self.with_sailing {
        let sailingclr = if self.with_sailing_colored {
          blazonclr
        } else {
          clr
        };
        let mut routes = vec![];
        let mut polys = vec![];
        let yfrom = y2;
        let yto = rng.gen_range(0.1..0.35) * y;
        let yincr =
          (yto - yfrom).abs() / (self.boatglobs.sailing_divisions as f32);
        let mut y = yfrom;
        let mut path1 = vec![];
        let mut path2 = vec![];
        let wfrom = rng.gen_range(0.5..1.0) * second_w;
        let wto = rng.gen_range(0.2..1.0) * wfrom;
        let ampcurve = rng.gen_range(-0.1f32..0.2).max(0.0) * w;
        let noisef = rng.gen_range(-0.1f32..0.08).max(0.0) * w;
        let curveinversion = rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0);
        let mut full_widths = vec![];
        while y <= yto {
          let f = (y - yfrom) / (yto - yfrom);
          let w = mix(wfrom, wto, f);
          let xoff =
            ampcurve * (PI * f).sin() + noisef * rng.gen_range(-0.5..0.5);
          let x1 = -w + xoff * 2.0 * (0.5 - curveinversion);
          let x2 = w + xoff;
          full_widths.push(w);
          path1.push((x1, y));
          path2.push((x2, y));
          y += yincr;
        }
        let n = self.boatglobs.sailing_interp_n;
        let interpolation = 0.8;
        let full_widths =
          values_subdivide_to_curve(&full_widths, n, interpolation);
        let path1 = path_subdivide_to_curve(&path1, n, interpolation);
        let path1 = route_scale_translate_rotate(&path1, scale, origin, angle);

        let path2 = path_subdivide_to_curve(&path2, n, interpolation);
        let path2 = route_scale_translate_rotate(&path2, scale, origin, angle);

        let poly: Vec<(f32, f32)> =
          path1.iter().chain(path2.iter().rev()).cloned().collect();
        polys.push(poly.clone());
        let mut route = poly.clone();
        route.push(route[0]);
        routes.push((sailingclr, route));

        // decoration
        let stripes = self.boatglobs.decoration_stripes;
        let wide = self.boatglobs.decoration_wide;
        let fibers_density = self.boatglobs.decoration_fibers_density;
        if stripes > 0 {
          let fibers = (wide * fibers_density * full_widths[0]
            / (1.0 + stripes as f32 * 2.0)) as usize;
          let widths = full_widths
            .iter()
            .map(|w| wide * w / stripes as f32)
            .collect::<Vec<_>>();
          let lastw = widths[widths.len() - 1];
          let widths = vec![vec![widths[0]], widths, vec![lastw]].concat();
          for i in 0..stripes {
            let f = (i + 1) as f32 / (stripes + 1) as f32;
            let path = path1
              .iter()
              .zip(path2.iter())
              .map(|(a, b)| lerp_point(*a, *b, f))
              .collect::<Vec<_>>();
            let mut o = path[0];
            o.1 -= 0.2 * size;
            let mut last = path[path.len() - 1];
            last.1 += 0.2 * size;
            let path = vec![vec![o], path, vec![last]].concat();
            let all = path_to_fibers(&path, &widths, fibers);
            let decorations = all
              .iter()
              .map(|rt| (sailingclr, rt.clone()))
              .collect::<Vec<_>>();

            let is_outside = |p| !polygon_includes_point(&poly, p);

            let decorations =
              clip_routes_with_colors(&decorations, &is_outside, 0.5, 3);

            routes.extend(decorations);
          }
        }

        // clip things
        routes = regular_clip_polys(&routes, mask, &polys);
        out.extend(routes);
      }

      {
        let mut routes = vec![];
        let mut polys = vec![];

        // main mast
        let mastw = rng.gen_range(0.08..0.14) * size;
        let poly1 = vec![
          (-mastw / 2.0, 0.0),
          (mastw / 2.0, 0.0),
          (mastw / 2.0, y),
          (-mastw / 2.0, y),
        ];
        let poly1 = route_scale_translate_rotate(&poly1, scale, origin, angle);
        polys.push(poly1.clone());

        let mut route = poly1.clone();
        route.push(route[0]);
        routes.push((clr, route));

        // clip things
        routes = regular_clip_polys(&routes, mask, &polys);
        out.extend(routes);
      }

      {
        let mut routes = vec![];
        let mut polys = vec![];

        // second mast
        let mastw = rng.gen_range(0.05..0.1) * size;
        let poly2 = vec![
          (-second_w, y2 - mastw / 2.),
          (second_w, y2 - mastw / 2.),
          (second_w, y2 + mastw / 2.),
          (-second_w, y2 + mastw / 2.),
        ];
        let poly2 = route_scale_translate_rotate(&poly2, scale, origin, angle);
        polys.push(poly2.clone());

        let mut route = poly2.clone();
        route.push(route[0]);
        routes.push((clr, route));

        // clip things
        routes = regular_clip_polys(&routes, mask, &polys);
        out.extend(routes);
      }

      if self.with_flag {
        // flag
        let o =
          route_scale_translate_rotate(&vec![(0.0, y)], scale, origin, angle)
            [0];

        let cloth_height_factor = rng.gen_range(0.4..0.7);
        let cloth_len_factor = rng.gen_range(0.5..1.0);
        let flagtoleft = !xflip;
        let mut a = angle - PI / 2.0;
        if !xflip {
          a = angle_mirrored_on_x(a);
        }
        let flag = Flag::init(
          rng,
          clr,
          blazonclr,
          o,
          size,
          a,
          flagtoleft,
          cloth_height_factor,
          cloth_len_factor,
          false,
        );
        out.extend(flag.render(mask));
      }

      if self.with_ropes {
        let mut routes = vec![];
        // ropes
        let crossp =
          route_scale_translate_rotate(&vec![(0.0, y2)], scale, origin, angle)
            [0];
        let lookup = PathLookup::init(self.boat_middle.clone());
        let count = rng.gen_range(3..8);
        let restr = rng.gen_range(0.0..0.3);
        for i in 0..count {
          let p = mix(
            restr,
            1.0 - restr,
            (i as f32 + rng.gen_range(-0.5..0.5) * rng.gen_range(0.0..1.0))
              / (count as f32 - 1.0),
          );
          let p = lookup.lookup_percentage(p);
          routes.push((clr, vec![crossp, p]));
        }

        // clip things
        routes = regular_clip(&routes, mask);
        out.extend(routes);
      }
    }

    out
  }
}

impl<R: Rng> super::Renderable<R> for Boat {
  fn render(
    &self,
    rng: &mut R,
    _ctx: &mut crate::global::GlobalCtx,
    mask: &mut PaintMask,
  ) -> Polylines {
    let mut out = vec![];
    out.extend(self.render_main_only(mask, self.clr));
    out.extend(self.render_background_only(rng, mask, self.clr));
    out
  }

  fn zorder(&self) -> f32 {
    self.origin.1
  }
}
