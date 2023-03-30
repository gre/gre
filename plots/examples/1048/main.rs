use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "148.5")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "30.0")]
  pub size: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

enum FloorPattern {
  Empty,
  Fill,
}

enum StructurePattern {
  X,
  RevV,
  V,
  None,
}

struct Floor {
  position: (f64, f64),
  height: f64,
  pattern: FloorPattern,
  structure_pattern: StructurePattern,
}

// TODO: this function should return possible positions where people can be
// FIXME: better roof
// IDEA: wheel on the top for the people to be on. connect the rope of the bridge to it.
// IDEA: a ladder in the middle?
// ?? IDEA: 2d plank style like in drawings https://www.cathares.org/dossiers-histoire-patrimoine/assets/images/machine-de-guerre-moyen-age-beffroi-tour-mobile-bois-dessin-philippe-contal-1-800x600.jpg + dot for the nails
fn belfry<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  height: f64,
  bridge_width: f64,
  bridge_opening: f64,
  xflip: bool,
  // FIXME: the mountain will have a slope. how to handle that?
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();

  let pole_in_front = rng.gen_bool(0.5);

  let fill_step = rng.gen_range(0.5, 2.0);

  let xmul = if xflip { -1.0 } else { 1.0 };

  let wheel_radius = 1.0;

  let w = rng.gen_range(0.4, 0.7) * height;

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
  let count = rng.gen_range(3, 7);
  let xrandomfactor = rng.gen_range(-1.0f64, 0.6).max(0.0);
  let mut last_x = x1;
  let floors = (0..count)
    .map(|i| {
      let ipercent = i as f64 / count as f64;
      let y = mix(y1, y2, ipercent);
      let mut h = height / count as f64;
      let is_roof = i == count - 1;
      if is_roof {
        h *= rng.gen_range(0.5, 1.0);
      }
      let x = mix(
        mix(x1, x2, ipercent),
        if last_x < x2 {
          rng.gen_range(last_x, x2)
        } else {
          rng.gen_range(x2, last_x)
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
          match rng.gen_range(0, 2) {
            0 => StructurePattern::RevV,
            _ => StructurePattern::V,
          }
        } else {
          match rng.gen_range(0, 4) {
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

  let mut bridge_h = rng.gen_range(0.7, 0.85) * height;
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
        x - rng.gen_range(0.0, 0.3 * w) * xmul
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
        y - h * rng.gen_range(0.3f64, 1.2).min(1.0)
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
  let bw = rng.gen_range(0.7, 0.9) * bridge_width;
  let anchor2 = (origin.0 + bw * acos * xmul, origin.1 - bridge_h + bw * asin);
  routes.push(vec![anchor1, anchor2]);

  for i in 0..bridge_count {
    let m = i as f64 * spacing;
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

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;

  let mut rng = rng_from_seed(opts.seed);
  let mut routes = Vec::new();
  let mut x = pad + 0.15 * size;
  while x < width - pad - size {
    let mut y = pad;
    let mut xflip = false;
    while y < height - pad - size {
      let origin = (pad + x + size / 2.0, pad + y + size);
      let sz = size - pad * 2.0;
      let bridge_width = rng.gen_range(0.3, 0.6) * sz;
      let bridge_opening = rng.gen_range(0.0f64, 1.5).min(1.0);
      let belfry_routes =
        belfry(&mut rng, origin, sz, bridge_width, bridge_opening, xflip);
      routes.extend(belfry_routes);
      y += size;
      xflip = !xflip;
    }
    x += size;
  }

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn circle_route(center: (f64, f64), r: f64, count: usize) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = 2. * PI * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}
