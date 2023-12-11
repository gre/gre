use super::bandpattern::BandPattern;
use crate::{
  algo::{paintmask::PaintMask, wormsfilling::WormsFilling},
  global::GlobalCtx,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */
pub fn framing<R: Rng>(
  rng: &mut R,
  ctx: &GlobalCtx,
  paint: &mut PaintMask,
  clr: usize,
  bound: (f32, f32, f32, f32),
  // pattern that will be colored for the framing
  pattern: Option<(Box<dyn BandPattern>, f32)>,
  // padding inside the frame
  padding: f32,
  // marging to exclude external
  margin: f32,
  // stroke width for the pattern
  wmul: f32,
  // density of the coloring
  density: f32,
  // nb of iteration of coloring logic
  iterations: usize,
) -> Vec<(usize, Vec<(f32, f32)>)> {
  let mut routes = vec![];

  if let Some((pattern, sw)) = pattern {
    let strokew = sw * wmul;
    // outer
    routes.push((
      clr,
      vec![
        (bound.0 + strokew, bound.1 + strokew),
        (bound.2 - strokew, bound.1 + strokew),
        (bound.2 - strokew, bound.3 - strokew),
        (bound.0 + strokew, bound.3 - strokew),
        (bound.0 + strokew, bound.1 + strokew),
      ],
    ));
    // inner
    routes.push((
      clr,
      vec![
        (bound.0 + padding - strokew, bound.1 + padding - strokew),
        (bound.2 - padding + strokew, bound.1 + padding - strokew),
        (bound.2 - padding + strokew, bound.3 - padding + strokew),
        (bound.0 + padding - strokew, bound.3 - padding + strokew),
        (bound.0 + padding - strokew, bound.1 + padding - strokew),
      ],
    ));

    let hp = padding / 2.;
    let bandw = hp - strokew;

    // top
    routes.extend(pattern.render_band(
      clr,
      (bound.0 + padding, bound.1 + hp),
      (bound.2 - padding, bound.1 + hp),
      bandw,
    ));
    // topleft
    routes.extend(pattern.render_corner(
      clr,
      (bound.0 + hp, bound.1 + hp),
      0.0,
      bandw,
    ));

    // right
    routes.extend(pattern.render_band(
      clr,
      (bound.2 - hp, bound.1 + padding),
      (bound.2 - hp, bound.3 - padding),
      bandw,
    ));
    // topright
    routes.extend(pattern.render_corner(
      clr,
      (bound.2 - hp, bound.1 + hp),
      -0.5 * PI,
      bandw,
    ));

    // bottom
    routes.extend(pattern.render_band(
      clr,
      (bound.2 - padding, bound.3 - hp),
      (bound.0 + padding, bound.3 - hp),
      bandw,
    ));
    // bottomright
    routes.extend(pattern.render_corner(
      clr,
      (bound.2 - hp, bound.3 - hp),
      -PI,
      bandw,
    ));

    // left
    routes.extend(pattern.render_band(
      clr,
      (bound.0 + hp, bound.3 - padding),
      (bound.0 + hp, bound.1 + padding),
      bandw,
    ));
    // bottomleft
    routes.extend(pattern.render_corner(
      clr,
      (bound.0 + hp, bound.3 - hp),
      -1.5 * PI,
      bandw,
    ));

    // strokes -> fill -> strokes. will create nice textures!
    let mut drawings = paint.clone_empty();
    for (_clr, route) in routes.iter() {
      drawings.paint_polyline(route, strokew);
    }
    let mut filling = WormsFilling::rand(rng);
    let ink = ctx.palette.inks[clr];
    let p = ink.3 * rng.gen_range(1.0..1.5);
    filling.precision = p;
    filling.step = p;
    routes =
      filling.fill_in_paint(rng, &drawings, clr, density, bound, iterations);
  }

  // we paint the mask for the paint to include our frame.

  // left
  paint.paint_rectangle(
    bound.0 - margin,
    bound.1 - margin,
    bound.0 + padding,
    bound.3 + margin,
  );
  // right
  paint.paint_rectangle(
    bound.2 - padding,
    bound.1 - margin,
    bound.2 + margin,
    bound.3 + margin,
  );
  // top
  paint.paint_rectangle(
    bound.0 - margin,
    bound.1 - margin,
    bound.2 + margin,
    bound.1 + padding,
  );
  // bottom
  paint.paint_rectangle(
    bound.0 - margin,
    bound.3 - padding,
    bound.2 + margin,
    bound.3 + margin,
  );

  routes
}
