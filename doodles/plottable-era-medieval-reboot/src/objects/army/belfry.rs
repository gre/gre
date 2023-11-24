use crate::algo::{
  clipping::regular_clip, math1d::mix, paintmask::PaintMask,
  polylines::Polylines, renderable::Renderable, shapes::circle_route,
};
use rand::prelude::*;
use std::f32::consts::PI;

/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era: (II) Medieval
 */

pub enum FloorPattern {
  Empty,
  Fill,
}

pub enum StructurePattern {
  X,
  RevV,
  V,
  None,
}

pub struct Floor {
  pub position: (f32, f32),
  pub height: f32,
  pub pattern: FloorPattern,
  pub structure_pattern: StructurePattern,
}

pub struct Belfry {
  pub floors: Vec<Floor>,
  pub routes: Polylines,
  pub origin: (f32, f32),
}

impl Belfry {
  // FIXME: better roof
  // IDEA: wheel on the top for the people to be on. connect the rope of the bridge to it.
  // IDEA: a ladder in the middle?
  // ?? IDEA: 2d plank style like in drawings https://www.cathares.org/dossiers-histoire-patrimoine/assets/images/machine-de-guerre-moyen-age-beffroi-tour-mobile-bois-dessin-philippe-contal-1-800x600.jpg + dot for the nails
  pub fn init<R: Rng>(
    rng: &mut R,
    clr: usize,
    origin: (f32, f32),
    height: f32,
    bridge_width: f32,
    bridge_opening: f32,
    xflip: bool,
  ) -> Self {
    let mut routes = Vec::new();

    let pole_in_front = rng.gen_bool(0.5);

    let fill_step = rng.gen_range(0.5..2.0);

    let xmul = if xflip { -1.0 } else { 1.0 };

    let wheel_radius = 1.0;

    let w = rng.gen_range(0.4..0.7) * height;

    let y = origin.1;

    // wheels
    let wheels = vec![origin, (origin.0 - w * xmul, origin.1)];
    // TODO middle wheel?
    for wheel in wheels {
      let route = circle_route(wheel, wheel_radius, 12);
      routes.push(route.clone());
      for (i, &r) in route.iter().enumerate() {
        if i % 2 == 0 {
          routes.push(vec![r, wheel]);
        }
      }
    }

    // building the floors
    let y1 = y;
    let x1 = origin.0 - w * xmul;
    let y2 = origin.1 - height;
    let x2 = origin.0;
    let count = rng.gen_range(3..7);
    let xrandomfactor = rng.gen_range(-1.0f32..0.6).max(0.0);
    let mut last_x = x1;
    let floors = (0..count)
      .map(|i| {
        let ipercent = i as f32 / count as f32;
        let y = mix(y1, y2, ipercent);
        let mut h = height / count as f32;
        let is_roof = i == count - 1;
        if is_roof {
          h *= rng.gen_range(0.5..1.0);
        }
        let x = mix(
          mix(x1, x2, ipercent),
          if last_x < x2 {
            rng.gen_range(last_x..x2)
          } else {
            rng.gen_range(x2..last_x)
          },
          if i == 0 { 0.0 } else { xrandomfactor },
        );
        last_x = x;
        let pattern = if !is_roof && rng.gen_bool(0.8) {
          FloorPattern::Fill
        } else {
          FloorPattern::Empty
        };
        let structure_pattern = if is_roof {
          StructurePattern::None
        } else {
          if !pole_in_front {
            match rng.gen_range(0..2) {
              0 => StructurePattern::RevV,
              _ => StructurePattern::V,
            }
          } else {
            match rng.gen_range(0..4) {
              0 => StructurePattern::X,
              1 => StructurePattern::RevV,
              2 => StructurePattern::V,
              _ => StructurePattern::None,
            }
          }
        };

        Floor {
          position: (x, y),
          height: h,
          pattern,
          structure_pattern,
        }
      })
      .collect::<Vec<_>>();

    // each time a floor is too x distance to the previous, we spawn a pole between that floor and the ground

    let mut bridge_h = rng.gen_range(0.7..0.85) * height;
    let bridgey = origin.1 - bridge_h;
    for f in floors.iter().rev() {
      if f.position.1 > bridgey {
        bridge_h = origin.1 - f.position.1;
        break;
      }
    }

    let main_pole_x = if pole_in_front { origin.0 } else { last_x };

    let dx = 0.1;
    routes.push(vec![
      (main_pole_x - dx, y1),
      (main_pole_x - dx, y2),
      (main_pole_x + dx, y2),
      (main_pole_x + dx, y1),
    ]);

    for (i, floor) in floors.iter().enumerate() {
      let (x, y) = floor.position;
      let h = floor.height;
      let is_roof = i == floors.len() - 1;

      if !is_roof || pole_in_front {
        let nextfloorx = if is_roof {
          x - rng.gen_range(0.0..0.3 * w) * xmul
        } else {
          floors[i + 1].position.0
        };

        let mut route = Vec::new();
        let y1 = y;
        let x1 = x;
        route.push((x, y));
        let x2 = origin.0;
        route.push((x2, y));
        let y2 = y - h;
        route.push((x2, y2));
        let y3 = if is_roof {
          y - h * rng.gen_range(0.3f32..1.2).min(1.0)
        } else {
          y - h
        };
        route.push((nextfloorx, y3));
        route.push((x, y));
        routes.push(route);

        match floor.pattern {
          FloorPattern::Empty => {}
          FloorPattern::Fill => {
            // horizontal lines
            let mut route = vec![];
            let mut rev = false;
            let step = fill_step;
            let mut y = y1 - step;
            while y > y2 {
              let xinterp = mix(nextfloorx, x1, (y - y2) / (y1 - y2));
              if rev {
                route.push((xinterp, y));
                route.push((x2, y));
              } else {
                route.push((x2, y));
                route.push((xinterp, y));
              }
              y -= step;
              rev = !rev;
            }
            routes.push(route);
          } // TODO we could have different plank orientations?
        }
        match floor.structure_pattern {
          StructurePattern::None => {}
          StructurePattern::X => {
            routes.push(vec![(x1, y1), (x2, y2)]);
            routes.push(vec![(x2, y1), (nextfloorx, y3)]);
          }
          StructurePattern::V => {
            routes.push(vec![(main_pole_x, y1), (x2, y2)]);
            routes.push(vec![(main_pole_x, y1), (nextfloorx, y3)]);
          }
          StructurePattern::RevV => {
            routes.push(vec![(main_pole_x, y2), (x1, y1)]);
            routes.push(vec![(main_pole_x, y3), (x2, y1)]);
          }
        }
      }

      if is_roof && !pole_in_front {}
    }

    // main border
    /*
    let mut route = Vec::new();
    route.push((origin.0, y));
    route.push((origin.0, origin.1 - height));
    route.push((origin.0 - w2 * xmul, origin.1 - height));
    route.push((origin.0 - w * xmul, y));
    routes.push(route.clone());
    */

    // bridge

    let bridge_count = 3;
    let spacing = 0.2;
    let dx = mix(0.2, -0.5, bridge_opening) * xmul;
    let ang = mix(-PI / 2.0, 0.0, bridge_opening);
    let asin = ang.sin();
    let acos = ang.cos();

    // TODO should we have anchor0 from where the rope is attached on a wheel?
    // TODO where is anchor1?
    let anchor1 = (main_pole_x, origin.1 - height);
    let bw = rng.gen_range(0.7..0.9) * bridge_width;
    let anchor2 =
      (origin.0 + bw * acos * xmul, origin.1 - bridge_h + bw * asin);
    routes.push(vec![anchor1, anchor2]);

    for i in 0..bridge_count {
      let m = i as f32 * spacing;
      let disp = (-m * asin, m * acos);
      let mut route = Vec::new();
      let bridge_from =
        (origin.0 + dx + disp.0 * xmul, origin.1 - bridge_h + disp.1);
      let bridge_to = (
        bridge_from.0 + bridge_width * acos * xmul,
        bridge_from.1 + bridge_width * asin,
      );
      route.push(bridge_from);
      route.push(bridge_to);
      routes.push(route);
    }

    let mut out = vec![];
    for route in routes {
      out.push((clr, route));
    }

    Self {
      floors,
      routes: out,
      origin,
    }
  }
}

impl<R: Rng> Renderable<R> for Belfry {
  fn render(&self, _rng: &mut R, paint: &mut PaintMask) -> Polylines {
    let routes = regular_clip(&self.routes, paint);
    for (_, rt) in &routes {
      paint.paint_polyline(rt, 1.0);
    }
    routes
  }
  fn yorder(&self) -> f32 {
    self.origin.1
  }
}
