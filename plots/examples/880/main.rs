use std::f64::consts::PI;

use byteorder::*;
use clap::*;
use geo::lines_iter::LinesIter;
use geo::map_coords::MapCoords;
use geo::prelude::Area;
use geo::prelude::BoundingRect;
use geo::prelude::Contains;
use geo::*;
use geo_booleanop::boolean::*;
use noise::*;
use rand::prelude::*;
use rand::Rng;
use serde_json::{json, Map, Value};
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  pub seed: f64,
  pub hash: String,
  pub primary_name: String,
  pub secondary_name: String,
}

fn spawn_element<R: Rng>(
  (cx, cy): (f64, f64),
  angle: f64,
  mut rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();

  if rng.gen_bool(0.3) {
    // ghoul
    for i in 0..3 {
      let mut x = rng.gen_range(-2.0, 2.0);
      let dir = 4.0 * if x > 0.0 { -1.0 } else { 1.0 };
      let mut y = -0.15 * (i as f64);
      let mut base_route = Vec::new();
      let origin = (x, y);
      base_route.push(origin);
      let dt = rng.gen_range(0.8, 1.5);
      for _j in 0..((9. * dt) as usize) {
        base_route
          .push((x + rng.gen_range(-dt, dt), y - dt + rng.gen_range(-dt, dt)));
      }
      x = origin.0;
      y = origin.1;
      for _j in 0..rng.gen_range(2, 4) {
        base_route.push((x, y));
        x += dir * rng.gen_range(0.8, 1.2);
        y += rng.gen_range(-0.6, 0.2) * rng.gen_range(0.0, 1.0);
      }
      y += rng.gen_range(-4.0, 0.0) * rng.gen_range(0.0, 1.0);
      base_route.push((x, y));
      let route = path_subdivide_to_curve(base_route, 1, 0.7);
      routes.push(route);
    }
  } else {
    if angle.abs() < 0.1 {
      // Ship
      let curvy1dt = rng.gen_range(-2.0, 1.0);
      let curvy2dt = rng.gen_range(-2.0, 1.0);
      let w1base = rng.gen_range(5., 7.);
      let w2base = w1base + rng.gen_range(-1.0, 1.0);
      for i in 0..3 {
        let dy = i as f64 * 0.3 - 0.1;
        let w1 = w1base + rng.gen_range(-0.6, 0.6);
        let w2 = w2base + rng.gen_range(-0.3, 0.3);
        let h1 = 3.0 + 2.0 * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0);
        let h2 = 3.0 + 2.0 * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0);
        let curvy1 = curvy1dt + rng.gen_range(-1.0, 1.0);
        let curvy2 = curvy2dt + rng.gen_range(-1.0, 1.0);
        let base_route = vec![
          (-w1 + curvy1, dy - h1),
          (-w1, dy - h1),
          (-w1, dy),
          (w2, dy),
          (w2, dy - h2),
          (w2 - curvy2, dy - h2),
        ];

        let route =
          path_subdivide_to_curve(base_route, 2, rng.gen_range(0.72, 0.78));
        routes.push(route);
      }
    } else {
      // simple base
      for i in 0..3 {
        let dy = (i as f64 - 1.2) * 0.14;
        let from = 2.0;
        let to = 6.0;
        let route = vec![
          (rng.gen_range(-to, -from), dy),
          (rng.gen_range(from, to), dy),
        ];
        routes.push(route);
      }
    }

    // roots

    let rootcount =
      (1.0 + rng.gen_range(0.0, 2.4) * rng.gen_range(0.0, 1.0)) as usize;
    let rootdist = rng.gen_range(3.0, 6.0);
    for r in 0..rootcount {
      let rdx = if rootcount == 1 {
        0.0
      } else {
        rootdist * ((r as f64) / ((rootcount - 1) as f64) - 0.5)
      };
      let headcount = rng.gen_range(0, 3);
      let count = rng.gen_range(2, 7) - r;
      if count < 1 {
        continue;
      }
      for i in 0..count {
        let dx = rdx + 0.6 * ((i as f64) / (count as f64) - 0.5);
        let mut x = dx;
        let mut y = 0.0;
        let mut base_route = Vec::new();
        for _j in 0..rng.gen_range(2, 5) {
          base_route.push((x, y));
          x += rng.gen_range(-2.0, 2.0) * rng.gen_range(0.2, 1.0);
          y -= rng.gen_range(2.0, 3.0);
        }
        if i < headcount {
          let dt = 1.0;
          for _j in 0..8 {
            base_route
              .push((x + rng.gen_range(-dt, dt), y + rng.gen_range(-dt, dt)));
          }
        }
        let route = path_subdivide_to_curve(base_route, 2, 0.7);
        routes.push(route);
      }
    }
  }

  // Rotate
  let acos = angle.cos();
  let asin = angle.sin();
  routes = routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|p| (cx + acos * p.0 + asin * p.1, cy + acos * p.1 - asin * p.0))
        .collect()
    })
    .collect();

  /*
  fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
      (
          a.cos() * p.0 + a.sin() * p.1,
          a.cos() * p.1 - a.sin() * p.0,
      )
  }
  */
  routes
}

enum TilingTechnique {
  DivergingTunnel(usize, usize),
  SimpleTunnel(usize),
  SimpleVoronoi,
}

#[derive(Clone)]
struct Tile {
  polygon: MultiPolygon<f64>,
  color_pattern: usize,
  // TODO includes more params of the tile like the density and patterns it uses
}

struct TileProperties {
  xmirrors: usize,
  ymirrors: usize,
  ray_projection: RayProjection, // TODO more than one!
  debug_lines: Vec<((f64, f64), (f64, f64))>,
}

enum RayProjection {
  ToCenter((f64, f64)),
  Direction(f64),
}

fn bound_grow(
  (a, b, c, d): (f64, f64, f64, f64),
  v: f64,
) -> (f64, f64, f64, f64) {
  (a - v, b - v, c + v, d + v)
}
fn bound_points((a, b, c, d): (f64, f64, f64, f64)) -> Vec<(f64, f64)> {
  vec![(a, b), (c, b), (c, d), (a, d)]
}

fn tiling<R: Rng>(
  mut rng: &mut R,
  (x1, y1, x2, y2): (f64, f64, f64, f64),
) -> (Vec<Tile>, TileProperties) {
  let mut debug_lines = Vec::new();

  // randomly decide the main properties
  let xmirrors = (rng.gen_range(1.0, 4.5) * rng.gen_range(0.0, 1.0)) as usize; // 0 split means no mirroring, 1 split means 2 piece split on X that mirrors, 2 means 3 frames...
  let ymirrors = (rng.gen_range(1.0, 4.5) * rng.gen_range(0.0, 1.0)) as usize;

  let working_area = (
    x1,
    y1,
    x1 + (x2 - x1) / (xmirrors as f64 + 1.),
    y1 + (y2 - y1) / (ymirrors as f64 + 1.),
  );
  let proj_frame = bound_grow(working_area, -10.0);
  let working_area_points = bound_points(working_area);
  let working_area_poly = Polygon::new(working_area_points.into(), Vec::new());
  let outside_working_area_poly = Polygon::new(
    bound_points(bound_grow(working_area, 1.0)).into(),
    Vec::new(),
  );

  let two_pi = 2.0 * PI;

  let ray_projection = if rng.gen_bool(0.3) {
    RayProjection::Direction(rng.gen_range(0.0, two_pi))
  } else {
    let xd = 0.00001
      + rng.gen_range(0.0, 0.5)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
    RayProjection::ToCenter((
      x1 + (x2 - x1) * (if rng.gen_bool(0.8) { 0.5 + xd } else { -xd }),
      y1 + (y2 - y1)
        * (0.5
          + (if rng.gen_bool(0.5) { -1.0 } else { 1.0 })
            * rng.gen_range(0.0, 0.6)
            * rng.gen_range(0.0, 1.0)
            * rng.gen_range(0.0, 1.0)),
    ))
  };

  // determine a bunch of segments to apply on the workaround area to cut the tiles with
  let cutting_segments: Vec<((f64, f64), (f64, f64))> = match ray_projection {
    RayProjection::Direction(angle) => {
      let vx = angle.cos();
      let vy = angle.sin();
      let count = rng.gen_range(1, 8);

      let dist_threshold = 10.0;
      let amp = 10000.0;

      let mut segments = Vec::new();
      for p in sample_perimeter(rng, proj_frame, 100) {
        if segments.len() >= count {
          break;
        }
        let p1 = (p.0 + amp * vx, p.1 + amp * vy);
        let proj1 = outside_working_area_poly.lines_iter().find_map(|l| {
          let (a, b) = l.points();
          collides_segment(a.x_y(), b.x_y(), p, p1)
        });
        let p2 = (p.0 - amp * vx, p.1 - amp * vy);
        let proj2 = outside_working_area_poly.lines_iter().find_map(|l| {
          let (a, b) = l.points();
          collides_segment(a.x_y(), b.x_y(), p, p2)
        });
        if let Some(a) = proj1 {
          if let Some(b) = proj2 {
            if segments.iter().all(|&(c, d)| {
              euclidian_dist(a, c) > dist_threshold
                && euclidian_dist(a, d) > dist_threshold
                && euclidian_dist(b, c) > dist_threshold
                && euclidian_dist(b, d) > dist_threshold
            }) {
              segments.push((a, b));
            }
          }
        }
      }

      segments
    }
    RayProjection::ToCenter(center) => {
      let count = rng.gen_range(1, 8);

      let angle_threshold = 0.1;
      let mut angles = Vec::new();
      for p in sample_perimeter(rng, proj_frame, 100) {
        if angles.len() >= count {
          break;
        }
        let a = (p.1 - center.1).atan2(p.0 - center.0);
        if angles.iter().all(|b| {
          let diff = PI - ((b - a).abs() - PI).abs();
          diff.abs() > angle_threshold
        }) {
          angles.push(a);
        }
      }

      let amp = 1000.0;
      angles
        .iter()
        .map(|a| (center, (center.0 + amp * a.cos(), center.1 + amp * a.sin())))
        .collect()
    }
  };

  let mut polygons = vec![working_area_poly.clone()];

  for cut in cutting_segments {
    debug_lines.push(cut);
    polygons = polygons
      .iter()
      .flat_map(|poly| cut_polygon(&poly, cut.0, cut.1))
      .filter(|p| p.signed_area() > 5.0)
      .collect();
  }

  // TODO coloring avoiding algorithm
  let mut i = 0;
  let mut working_tiles = Vec::new();
  for polygon in polygons {
    working_tiles.push(Tile {
      polygon: MultiPolygon::new(vec![polygon]),
      color_pattern: i % 2,
    });
    i += 1;
  }

  // TODO merging the polygons that touches
  // if a point is touching the line of the edge of the mirror, then we connect the polygon
  // we have to keep the indexes

  let mut tiles = Vec::new();
  for t in working_tiles.iter() {
    tiles.push(t.clone());
  }

  for xm in 0..xmirrors {
    let w = working_area.2 - working_area.0;
    let x1 = working_area.0 + w * (xm as f64 + 1.0);
    let y1 = working_area.1;
    let x2 = working_area.2 + w * (xm as f64 + 1.0);
    let y2 = working_area.3;
    let bound = if xm % 2 == 0 {
      (x2, y1, x1, y2)
    } else {
      (x1, y1, x2, y2)
    };
    tiles = tiles
      .iter()
      .flat_map(|tile| {
        vec![
          tile.clone(),
          Tile {
            polygon: project_polygon(&tile.polygon, working_area, bound),
            color_pattern: tile.color_pattern,
          },
        ]
      })
      .collect();
    /*
    for tile in tiles.clone() {
        tiles.push(Tile {
            polygon: project_polygon(&tile.polygon, working_area, bound),
            color_pattern: tile.color_pattern,
        });
    }
    */
  }

  let newarea = (x1, y1, x2, working_area.3);
  for ym in 0..ymirrors {
    let h = newarea.3 - newarea.1;
    let x1 = newarea.0;
    let y1 = newarea.1 + h * (ym as f64 + 1.0);
    let x2 = newarea.2;
    let y2 = newarea.3 + h * (ym as f64 + 1.0);
    let bound = if ym % 2 == 0 {
      (x1, y2, x2, y1)
    } else {
      (x1, y1, x2, y2)
    };

    tiles = tiles
      .iter()
      .flat_map(|tile| {
        vec![
          tile.clone(),
          Tile {
            polygon: project_polygon(&tile.polygon, newarea, bound),
            color_pattern: tile.color_pattern,
          },
        ]
      })
      .collect();

    /*
    for tile in tiles.clone() {
        tiles.push(Tile {
            polygon: project_polygon(&tile.polygon, newarea, bound),
            color_pattern: tile.color_pattern,
        });
    }*/
  }

  let properties = TileProperties {
    xmirrors,
    ymirrors,
    ray_projection,
    debug_lines,
  };
  (tiles, properties)
}

pub fn art(opts: &Opts) -> Document {
  let seed = opts.seed;
  let pad = 10.0;
  let width = 210.;
  let height = 297.;
  let pincr = 0.38;
  let perlin = Perlin::new();
  let mut rng = rng_from_seed(seed);
  let mut layer_primary = Vec::new();
  let mut layer_secondary = Vec::new();

  let f = |p| 0.5 - euclidian_dist(p, (0.5, 0.5));

  /*
  let tiling = TilingTechnique::SimpleTunnel(rng.gen_range(2, 8));

  // TODO add a "direction" on which we use to know where the shape look at
  let polys: Vec<(bool, MultiPolygon<_>)> = match tiling {
      TilingTechnique::SimpleTunnel(angle_splits) => {
          let center = (
              rng.gen_range(0.45, 0.55) * width,
              rng.gen_range(0.4, 0.6) * height,
          );
          let mut data = Vec::new();
          let angles_right: Vec<_> = (0..angle_splits)
              .map(|i| PI * rng.gen_range(0.0, 1.0) * ((i % 2) as f64 - 0.5))
              .collect();

          // TODO remove angle too close to each other
          let mut angles = angles_right.clone();
          for a in angles_right.iter() {
              let acos = a.cos();
              let asin = a.sin();
              let newa = asin.atan2(-acos);
              angles.push(newa);
          }
          angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
          let bound_points = vec![
              (pad, pad),
              (width - pad, pad),
              (width - pad, height - pad),
              (pad, height - pad),
          ];
          let bounds_len = bound_points.len();
          let bounds_poly = Polygon::new(bound_points.into(), vec![]);
          let edge_points: Vec<(f64, f64)> = angles
              .iter()
              .map(|a| {
                  let ray_proj = (center.0 + a.cos() * 99999.0, center.1 + a.sin() * 99999.0);
                  return ray_proj;
                  /*
                  for i in 0..bounds_len {
                    let j = (i + 1) % bounds_len;
                    let collision = collides_segment(center, ray_proj, bound_points[i], bound_points[j]);
                    if collision.is_some() {
                      return collision.unwrap();
                    }
                  }
                  return center;
                  */
              })
              .collect();

          let len = edge_points.len();
          for (i, &edge1) in edge_points.iter().enumerate() {
              let edge2 = edge_points[(i + 1) % len];
              let poly = Polygon::new(vec![edge1, edge2, center].into(), vec![]);
              let res = poly.intersection(&bounds_poly);
              //for poly in res {
              let r = poly.signed_area();
              if r < 10.0 {
                  continue;
              }
              data.push((i % 2 == 0, res));
              //}
          }
          data
      }
      TilingTechnique::DivergingTunnel(angle_splits, depth_splits) => {
          let mut data = Vec::new();
          let angles_right: Vec<_> = (0..angle_splits)
              .map(|i| PI * rng.gen_range(0.1, 1.0) * ((i % 2) as f64 - 0.5))
              .collect();
          // TODO remove angle too close to each other
          let mut angles = angles_right.clone();
          for a in angles_right.iter() {
              let acos = a.cos();
              let asin = a.sin();
              let newa = asin.atan2(-acos);
              angles.push(newa);
          }
          angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

          let mut center = (
              rng.gen_range(0.45, 0.55) * width,
              rng.gen_range(0.4, 0.6) * height,
          );
          let calc_bounds = |d| {
              let p = d as f64 / (depth_splits as f64);
              let x1 = pad;
              let x2 = width - pad;
              let y1 = pad;
              let y2 = height - pad;
              let phalf = p / 2.0;
              let points: Vec<(f64, f64)> = vec![
                  (mix(x1, x2, phalf), mix(y1, y2, phalf)),
                  (mix(x2, x1, phalf), mix(y1, y2, phalf)),
                  (mix(x2, x1, phalf), mix(y2, y1, phalf)),
                  (mix(x1, x2, phalf), mix(y2, y1, phalf)),
              ];
              let poly = Polygon::new(points.clone().into(), vec![]);
              (points, poly)
          };

          let mut bounds = calc_bounds(0);
          for d in 0..depth_splits {
              let edge_points: Vec<(f64, f64)> = angles
                  .iter()
                  .map(|a| (center.0 + a.cos() * 99999.0, center.1 + a.sin() * 99999.0))
                  .collect();

              let next_bounds = calc_bounds(d + 1);

              let len = edge_points.len();
              for (i, &edge1) in edge_points.iter().enumerate() {
                  let edge2 = edge_points[(i + 1) % len];
                  let poly = Polygon::new(vec![edge1, edge2, center].into(), vec![]);
                  let mut res = poly.intersection(&bounds.1);
                  if d < depth_splits - 1 {
                      res = res.difference(&next_bounds.1);
                  }

                  //for poly in res {
                  let r = poly.signed_area();
                  if r < 10.0 {
                      continue;
                  }
                  data.push((i % 2 == 0, res));
                  //}
              }

              let next_center = (
                  mix(
                      next_bounds.0[0].0,
                      next_bounds.0[2].0,
                      rng.gen_range(0.3, 0.6),
                  ),
                  mix(
                      next_bounds.0[0].1,
                      next_bounds.0[2].1,
                      rng.gen_range(0.3, 0.6),
                  ),
              );
              angles = edge_points
                  .iter()
                  .map(|&edge| {
                      for i in 0..4 {
                          let j = (i + 1) % 4;
                          let collision =
                              collides_segment(center, edge, next_bounds.0[i], next_bounds.0[j]);
                          if collision.is_some() {
                              return collision.unwrap();
                          }
                      }
                      return center;
                  })
                  .map(|p| (p.1 - next_center.1).atan2(p.0 - next_center.0))
                  .collect();

              bounds = next_bounds;
              center = next_center;
          }
          data
      }
      TilingTechnique::SimpleVoronoi => {
          let voronoi_count = rng.gen_range(3, 16);
          let samples = sample_2d_candidates_f64(&f, 200, voronoi_count, &mut rng);
          let mut points = Vec::new();
          for c in samples {
              points.push(voronoi::Point::new(width * c.0, height * c.1));
          }
          let dcel = voronoi::voronoi(points, height);
          voronoi::make_polygons(&dcel)
              .iter()
              .map(|pts| {
                  (
                      rng.gen_bool(0.5),
                      MultiPolygon::new(vec![Polygon::new(
                          pts.iter()
                              .map(|p| (p.x(), p.y()))
                              .collect::<Vec<_>>()
                              .into(),
                          vec![],
                      )]),
                  )
              })
              .collect()
      }
      _ => vec![],
  };
  */

  let (tiles, tileproperties) =
    tiling(&mut rng, (pad, pad, width - pad, height - pad));

  for (polyi, tile) in tiles.iter().enumerate() {
    let horizontal = tile.color_pattern == 0;
    let poly = tile.polygon.clone();
    let polyseed = 3.3 * opts.seed + 7.7 * (polyi as f64);
    let bounds = poly.bounding_rect().unwrap();
    let min = bounds.min();
    let pw = bounds.width();
    let ph = bounds.height();
    let middle_increase_lineincr = rng.gen_bool(0.1);
    let edge_full_mode = rng.gen_bool(0.1);
    let edge_none_mode = rng.gen_bool(0.2);

    let pstart = if horizontal {
      min.x.min(width - pad).max(pad)
    } else {
      min.y.min(height - pad).max(pad)
    };
    let pend = if horizontal {
      (min.x + pw).min(width - pad).max(pad)
    } else {
      (min.y + ph).min(height - pad).max(pad)
    };

    let mut linepos = if horizontal {
      min.y.min(height - pad).max(pad)
    } else {
      min.x.min(width - pad).max(pad)
    };
    let linestart = linepos;
    let linemax = if horizontal {
      (min.y + ph).min(height - pad).max(pad)
    } else {
      (min.x + pw).min(width - pad).max(pad)
    };
    let lineincrbase = 1.1 + rng.gen_range(-0.3, 0.6) * rng.gen_range(0.0, 1.0);

    /*
    // TODO
    let mut openings = Vec::new();
    let opening_size = 3.0;
    let estimated_count = ((linemax - linepos) / lineincrbase) as usize;
    if rng.gen_bool(0.2) {
      let opening_count = (1. + rng.gen_range(0., 6.) * rng.gen_range(0.0, 1.0)) as usize;
      let offset_open = rng.gen_range(0.4, 0.6);
      for i in 0..opening_count {
        openings.push(
          (estimated_count as f64 * (i as f64 + offset_open) / (opening_count as f64)) as usize,
        );
      }
    }
    */

    loop {
      let mut lineincr = lineincrbase;
      if middle_increase_lineincr {
        lineincr *= mix(
          1.0,
          2.0,
          smoothstep(
            0.2,
            0.0,
            ((linemax - linepos) / (linemax - linestart) - 0.5).abs(),
          ),
        );
      }
      if linepos > linemax {
        break;
      }
      let mut p = pstart;
      loop {
        if p > pend {
          break;
        }
        let (x, y) = if horizontal {
          (p, linepos)
        } else {
          (linepos, p)
        };

        // TODO polygon can be concave
        if poly.contains(&geo::Point::new(x, y)) {
          break;
        }
        p += pincr;
      }
      let localpstart = p;
      let mut route = Vec::new();
      loop {
        if p > pend {
          break;
        }
        let (x, y) = if horizontal {
          (p, linepos)
        } else {
          (linepos, p)
        };

        route.push((x, y));

        if !poly.contains(&geo::Point::new(x, y)) {
          break;
        }
        p += pincr;
      }
      let localpend = p;

      if route.len() > 1 {
        // duplicate the same lines a few times & displace it
        let xdiramp = if horizontal { 0.0 } else { 1.0 };
        let ydiramp = if horizontal { 1.0 } else { 0.0 };

        let edge_threshold = rng.gen_range(16.0, 32.0);
        // TODO have a parameter for the position of the line
        let edge_amount = if edge_full_mode {
          1.0
        } else if edge_none_mode {
          0.0
        } else {
          2.0
            * ((perlin.get([opts.seed, linepos / 0.07, polyi as f64 * 1.3])
              - 0.2)
              * 4.0)
              .max(0.0)
              .min(0.5)
        };

        let len = route.len() as f64 * pincr;
        let noise_amp1 = rng.gen_range(0.0, 0.5);
        let noise_amp2 = rng.gen_range(0.2, 0.4);
        let disp_amp1 = rng.gen_range(0.1, 0.3);
        let split_values: Vec<(f64, f64)> = route
          .iter()
          .enumerate()
          .map(|(i, &(x, y))| {
            let p = i as f64 * pincr;
            let n1 = noise_amp1
              * smoothstep(
                0.3,
                0.48,
                perlin.get([linepos * 3.3, 0.07 * p, polyseed]),
              )
              + noise_amp2
                * perlin.get([polyseed, p / 7., linepos / 5.6]).abs();
            let edge =
              smoothstep(edge_threshold, 0.0, p.min(len - p)).powf(2.0);
            let splitamount = (n1 + edge_amount * edge).min(0.9);
            let displacement = disp_amp1
              * (0.4 * perlin.get([x / 7.7, polyseed, y / 7.7])
                + 0.6 * perlin.get([polyseed, x / 19.7, y / 19.7]));
            (splitamount, displacement)
          })
          .collect();

        let repetitions = 4;
        let repetitions_f = repetitions as f64;
        let split_amp = (repetitions_f / (repetitions_f + 1.)) * lineincr;
        for i in 0..repetitions {
          let delta = split_amp * (i as f64 / (repetitions_f - 1.) - 0.5);
          let mut instance = Vec::new();
          for (j, p) in route.iter().enumerate() {
            let (splitamount, displacement) = split_values[j];
            let n = splitamount * delta + lineincr * displacement;
            let x = p.0 + xdiramp * n;
            let y = p.1 + ydiramp * n;
            instance.push((x, y));
          }
          if i % 2 == 1 {
            instance.reverse();
          }
          layer_primary.push(instance);
        }

        // check for opportunity to spawn body
        let spawn_body_positions = if rng.gen_bool(0.01) {
          vec![
            localpstart + rng.gen_range(0.2, 0.8) * (localpend - localpstart),
          ]
        } else {
          vec![]
        };

        for spawn in spawn_body_positions {
          let p = if horizontal {
            (spawn, linepos)
          } else {
            (linepos, spawn)
          };

          let direction = if !horizontal { PI / 2.0 } else { 0.0 };
          layer_primary =
            vec![layer_primary, spawn_element(p, direction, &mut rng)].concat();
        }
      }

      linepos += lineincr;
    }
  }

  for (a, b) in tileproperties.debug_lines {
    layer_secondary.push(vec![a, b]);
  }

  let mut inks = Vec::new();
  let layers: Vec<Group> = vec![
    ("#0FF", opts.primary_name.clone(), layer_primary),
    ("#F0F", opts.secondary_name.clone(), layer_secondary),
  ]
  .iter()
  .filter(|(_color, _label, routes)| routes.len() > 0)
  .map(|(color, label, routes)| {
    inks.push(label.clone());
    let mut l = Group::new()
      .set("inkscape:groupmode", "layer")
      .set("inkscape:label", label.clone())
      .set("fill", "none")
      .set("stroke", color.clone())
      .set("stroke-linecap", "round")
      .set("stroke-width", 0.35);

    let opacity: f64 = 0.5;
    for route in routes.clone() {
      let data = render_route(Data::new(), route);
      l = l.add(
        Path::new()
          .set(
            "opacity", // TODO? randomize a bit it
            opacity,
          )
          .set("d", data),
      );
    }

    l
  })
  .collect();

  inks.sort();
  if inks.len() == 2 && inks[0].eq(&inks[1]) {
    inks.remove(1);
  }

  let mut map = Map::new();
  map.insert(String::from("Inks Count"), json!(inks.len()));
  let traits = Value::Object(map);

  let mut document = svg::Document::new()
    .set("viewBox", (0, 0, 210, 297))
    .set("data-hash", opts.hash.to_string())
    .set("data-traits", traits.to_string())
    .set("width", "210mm")
    .set("height", "297mm")
    .set("style", "background:white")
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("xmlns", "http://www.w3.org/2000/svg");
  for l in layers {
    document = document.add(l);
  }
  document
}

fn main() {
  let opts: Opts = Opts::parse();
  let document = art(&opts);
  svg::save(opts.file, &document).unwrap();
}

fn render_route(data: Data, route: Vec<(f64, f64)>) -> Data {
  if route.len() == 0 {
    return data;
  }
  let first_p = route[0];
  let mut d =
    data.move_to((significant_str(first_p.0), significant_str(first_p.1)));
  for p in route {
    d = d.line_to((significant_str(p.0), significant_str(p.1)));
  }
  return d;
}

#[inline]
fn significant_str(f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
}

fn rng_from_seed(s: f64) -> impl Rng {
  let mut bs = [0; 16];
  bs.as_mut().write_f64::<BigEndian>(s).unwrap();
  let mut rng = SmallRng::from_seed(bs);
  // run it a while to have better randomness
  for _i in 0..50 {
    rng.gen::<f64>();
  }
  return rng;
}

fn sample_2d_candidates_f64<R: Rng>(
  f: &dyn Fn((f64, f64)) -> f64,
  dim: usize,
  samples: usize,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  let mut candidates = Vec::new();
  for x in 0..dim {
    for y in 0..dim {
      let p = ((x as f64) / (dim as f64), (y as f64) / (dim as f64));
      if f(p) > rng.gen_range(0.0, 1.0) {
        candidates.push(p);
      }
    }
  }
  rng.shuffle(&mut candidates);
  candidates.truncate(samples);
  return candidates;
}

pub fn sample_2d_candidates(
  f: &dyn Fn((f64, f64)) -> bool,
  dim: usize,
  samples: usize,
  rng: &mut impl Rng,
) -> Vec<(f64, f64)> {
  let mut candidates = Vec::new();
  for x in 0..dim {
    for y in 0..dim {
      let p = ((x as f64) / (dim as f64), (y as f64) / (dim as f64));
      if f(p) {
        candidates.push(p);
      }
    }
  }
  rng.shuffle(&mut candidates);
  candidates.truncate(samples);
  return candidates;
}

pub struct Passage2DCounter {
  granularity: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage2DCounter {
  pub fn new(granularity: f64, width: f64, height: f64) -> Self {
    let wi = (width / granularity).ceil() as usize;
    let hi = (height / granularity).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage2DCounter {
      granularity,
      width,
      height,
      counters,
    }
  }
  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.granularity).ceil() as usize;
    let hi = (self.height / self.granularity).ceil() as usize;
    let xi = ((x / self.granularity).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.granularity).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }
  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }
  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    self.counters[self.index(p)]
  }
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

#[inline]
fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
  let k = ((x - a) / (b - a)).max(0.0).min(1.0);
  return k * k * (3.0 - 2.0 * k);
}

#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}

#[inline]
fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn path_subdivide_to_curve_it(
  path: Vec<(f64, f64)>,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let l = path.len();
  if l < 3 {
    return path;
  }
  let mut route = Vec::new();
  let mut first = path[0];
  let mut last = path[l - 1];
  let looped = euclidian_dist(first, last) < 0.1;
  if looped {
    first = lerp_point(path[1], first, interpolation);
  }
  route.push(first);
  for i in 1..(l - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, interpolation);
    let p2 = lerp_point(path[i + 1], p, interpolation);
    route.push(p1);
    route.push(p2);
  }
  if looped {
    last = lerp_point(path[l - 2], last, interpolation);
  }
  route.push(last);
  if looped {
    route.push(first);
  }
  route
}

fn path_subdivide_to_curve(
  path: Vec<(f64, f64)>,
  n: usize,
  interpolation: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_subdivide_to_curve_it(route, interpolation);
  }
  route
}

// collides segments (p0,p1) with (p2,p3)
fn collides_segment(
  p0: (f64, f64),
  p1: (f64, f64),
  p2: (f64, f64),
  p3: (f64, f64),
) -> Option<(f64, f64)> {
  let s10_x = p1.0 - p0.0;
  let s10_y = p1.1 - p0.1;
  let s32_x = p3.0 - p2.0;
  let s32_y = p3.1 - p2.1;
  let d = s10_x * s32_y - s32_x * s10_y;
  if d.abs() < 0.000001 {
    return None;
  }
  let s02_x = p0.0 - p2.0;
  let s02_y = p0.1 - p2.1;
  let s_numer = s10_x * s02_y - s10_y * s02_x;
  if (s_numer < 0.) == (d > 0.) {
    return None;
  }
  let t_numer = s32_x * s02_y - s32_y * s02_x;
  if (t_numer < 0.) == (d > 0.) {
    return None;
  }
  if (s_numer > d) == (d > 0.) || (t_numer > d) == (d > 0.) {
    return None;
  }
  let t = t_numer / d;
  return Some((p0.0 + t * s10_x, p0.1 + t * s10_y));
}

fn cut_polygon(
  poly: &Polygon<f64>,
  a: (f64, f64),
  b: (f64, f64),
) -> Vec<Polygon<f64>> {
  let mut prev: Option<Point<f64>> = None;
  let mut first = vec![];
  let mut second = vec![];
  let mut on_first = true;
  let mut pts: Vec<_> = poly.exterior().points().collect();
  //pts.push(pts[0]);
  for p in pts {
    if let Some(prev) = prev {
      let c = collides_segment(prev.x_y(), p.x_y(), a, b);
      if let Some(c) = c {
        first.push(c.into());
        second.push(c.into());
        on_first = !on_first;
      }
    }
    if on_first {
      first.push(p);
    } else {
      second.push(p);
    }
    prev = Some(p);
  }
  if second.len() < 2 {
    vec![poly.clone()]
  } else {
    vec![
      Polygon::new(first.into(), Vec::new()),
      Polygon::new(second.into(), Vec::new()),
    ]
  }
}

fn sample_perimeter<R: Rng>(
  mut rng: &mut R,
  (ax1, ax2, ay1, ay2): (f64, f64, f64, f64),
  count: usize,
) -> Vec<(f64, f64)> {
  let aw = ax2 - ax1;
  let ah = ay2 - ay1;
  let perimeter = aw * 2.0 + ah * 2.0;
  (0..count)
    .map(|_i| {
      let mut v = rng.gen_range(0.0, perimeter);
      if v < aw {
        (v, ay1)
      } else {
        v -= aw;
        if v < ah {
          (ax2, v)
        } else {
          v -= ah;
          if v < aw {
            (ax2 - v, ay2)
          } else {
            (ax1, ay2 - v)
          }
        }
      }
    })
    .collect()
}

fn project_polygon(
  poly: &MultiPolygon<f64>,
  from: (f64, f64, f64, f64),
  to: (f64, f64, f64, f64),
) -> MultiPolygon<f64> {
  poly.map_coords(|&(px, py)| {
    (
      mix(to.0, to.2, (px - from.0) / (from.2 - from.0)),
      mix(to.1, to.3, (py - from.1) / (from.3 - from.1)),
    )
  })
}
