/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – Golden Train
 */
mod utils;
use hex::FromHex;
use noise::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
  let opts = val.into_serde().unwrap();
  let doc = art(&opts);
  let str = doc.to_string();
  return str;
}

#[derive(Deserialize)]
pub struct Opts {
  pub hash: String,
  pub primary_name: String,
  pub secondary_name: String,
  pub gold_border: bool,
  pub debug: bool,
}

pub fn art(opts: &Opts) -> svg::Document {
  let height = 105.0;
  let width = 148.5;
  let pad = 5.0;

  // Prepare all the random values
  let mut rng = rng_from_hexhash(&opts.hash);

  // all the lines to draw are pushed here
  let mut primary = vec![];
  let mut secondary = vec![];
  let mut highlightedpart = vec![];
  let mut train_found = false;
  let mut total_cactus = 0;
  let mut total_eagles = 0;
  let mut total_carriages = 0;
  let mut cloud_density_factor = 0.0;
  let mut mountain_offbound_factor = 0.0;
  let mut train_slope = 0.0;
  let mut lowbridgey = 0.0;
  let mut bridge = String::from("");

  let offsetstrategy = rng.gen_range(0, 5);
  while !train_found {
    // reset
    total_cactus = 0;
    total_eagles = 0;
    total_carriages = 0;
    primary = vec![];
    secondary = vec![];
    highlightedpart = vec![];
    ///////////////////////////////

    let seed = rng.gen_range(0.0, 100000.0);
    let perlin = Perlin::new();
    let min_route = 2;
    let pf = rng.gen_range(0.0, 1.0);
    let peakfactor = mix(-0.0004, -0.0001, pf) * rng.gen_range(0.2, 1.0);
    let stopy = rng.gen_range(0.1, 0.3) * height;
    let ampfactor = mix(0.06, 0.2, pf);
    let ynoisefactor = rng.gen_range(0.05, 0.1);
    let yincr = rng.gen_range(0.4, 0.6);
    let xfreq = rng.gen_range(0.005, 0.01);
    let amp2 = rng.gen_range(1.0, 4.0);
    let precision = rng.gen_range(0.18, 0.25);

    let mut passage = Passage::new(0.5, width, height);
    let passage_threshold = 10;

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height * 5.0;
    let mut miny = height;
    let mut height_map: Vec<f64> = Vec::new();
    loop {
      if miny < stopy {
        break;
      }

      let mut route = Vec::new();
      let mut x = pad;
      let mut was_outside = true;
      loop {
        if x > width - pad {
          break;
        }
        let xv = (4.0 - base_y / height) * (x - width / 2.);

        let amp = height * ampfactor;
        let xx = (x - pad) / (width - 2. * pad);
        let xborderd = xx.min(1.0 - xx);
        let displacement = amp * peakfactor * (xv * xv - (xborderd).powf(0.5));

        let mut y = base_y;

        if offsetstrategy == 0 {
          y += displacement;
        }

        y += -amp
          * perlin
            .get([
              //
              xv * xfreq + 9.9,
              y * 0.02 - 3.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  -seed * 7.3,
                  8.3 + xv * 0.015,
                  y * 0.1,
                ]),
            ])
            .abs();

        if offsetstrategy == 1 {
          y += displacement;
        }

        y += amp2
          * amp
          * perlin.get([
            //
            8.3 + xv * 0.008,
            88.1 + y * ynoisefactor,
            seed * 97.3,
          ]);

        if offsetstrategy == 2 {
          y += displacement;
        }

        y += amp
          * perlin.get([
            //
            seed * 9.3 + 77.77,
            xv * 0.08 + 9.33,
            y * 0.5,
          ])
          * perlin
            .get([
              //
              xv * 0.015 - 88.33,
              88.1 + y * 0.2,
              -seed / 7.7 - 6.66,
            ])
            .min(0.0);

        if offsetstrategy == 3 {
          y += displacement;
        }

        y += 0.1
          * amp
          * (1.0 - miny / height)
          * perlin.get([
            //
            6666.6 + seed * 1.3,
            8.3 + xv * 0.5,
            88.1 + y * 0.5,
          ]);

        if offsetstrategy == 4 {
          y += displacement;
        }

        if y < miny {
          miny = y;
        }
        let mut collides = false;
        let xi = (x / precision).round() as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let inside = !collides
          && pad + 1.8 < x
          && x < width - pad - 1.8
          && pad + 1.8 < y
          && y < height - pad - 1.8;
        if inside && passage.get((x, y)) < passage_threshold {
          if was_outside {
            if route.len() > min_route {
              primary.push(route);
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push((x, y));
          passage.count((x, y));
        } else {
          was_outside = true;
        }

        x += precision;
      }

      if route.len() > min_route {
        primary.push(route);
      }

      base_y -= yincr;
    }

    // We use a "smooth average" algorithm to ignore the sharp edges of the mountains
    let smooth = 50;
    let sf = smooth as f64;
    let mut sum = 0.0;
    let mut acc = Vec::new();
    let mut smooth_heights = Vec::new();
    let mut offbound = 0;
    for (i, h) in height_map.iter().enumerate() {
      if acc.len() == smooth {
        let avg = sum / sf;
        if avg > height - pad {
          offbound += 1;
        }
        let xtheoric = (i as f64 - sf / 2.0) * precision;
        smooth_heights.push((xtheoric, avg));
        let prev = acc.remove(0);
        sum -= prev;
      }
      acc.push(h);
      sum += h;
    }

    mountain_offbound_factor = offbound as f64 / (smooth_heights.len() as f64);

    // We can then highlight the mountain tops with sorting:
    smooth_heights.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Algorith will now try to find the best bridge placement
    // Here are configurations:
    let tries = 100;
    let jump = 5; // how much "tops" to skip between tries. to not consider just the best tops.

    // - maximize the area under the bridge (the reason we make a bridge is there is a precipice)
    let min_area_per_dx = rng.gen_range(5.0, 20.0);

    // - try to find a certain length of the bridge:
    let min_bridge_width = 0.25 * width;
    let max_bridge_width = rng.gen_range(0.5, 0.75) * width;

    // - have an horizontal slope as much as possible:
    let min_ratio_threshold = rng.gen_range(4.0, 30.0); // dx/dy ratio of the slope to start accepting

    let threshold_bridge_height = 0.8 * height;

    for t in 0..tries {
      let i = (t * jump) % smooth_heights.len();
      let a = smooth_heights[i];
      if a.1 > threshold_bridge_height {
        continue;
      }
      let maybe_b = smooth_heights.iter().find(|&&b| {
        if b.1 > threshold_bridge_height {
          return false;
        }
        let d = (a.0 - b.0).abs();
        if min_bridge_width < d && d < max_bridge_width {
          let dx = (a.0 - b.0).abs();
          let dy = (a.1 - b.1).abs();
          if dy < 1.0 || dx / dy > min_ratio_threshold {
            let left = if a.0 < b.0 { a } else { b };
            let right = if a.0 > b.0 { a } else { b };
            let leftxi = (left.0 / precision).round() as usize;
            let rightxi = (right.0 / precision).round() as usize;
            let mut area = 0.0;
            let l = (rightxi - leftxi) as f64;
            for xi in leftxi..rightxi {
              let xp = (xi - leftxi) as f64 / l;
              let liney = mix(left.1, right.1, xp);
              let dy = height_map[xi] - liney;
              if dy < 0.0 {
                area += -dy * dy; // square of the era if it's traversing the bridge
              } else {
                area += dy;
              }
            }
            area *= precision;
            if area / dx > min_area_per_dx {
              return true;
            }
          }
        }
        return false;
      });

      // We may have found our bridge, from a to b:
      if let Some(&b) = maybe_b {
        let dx = (a.0 - b.0).abs();
        let dy = (a.1 - b.1).abs();
        train_slope = dy / dx;

        let angle = if a.0 < b.0 {
          (b.1 - a.1).atan2(b.0 - a.0)
        } else {
          (a.1 - b.1).atan2(a.0 - b.0)
        };

        let pos = |cx: f64, cy: f64, offx: f64, offy: f64| {
          // position a point offsetted from a center point rotating based on the angle
          let offx = offx * angle.cos() - offy * angle.sin();
          let offy = offx * angle.sin() + offy * angle.cos();
          (cx + offx, cy + offy)
        };

        let trainh = rng.gen_range(1.0, 3.2); // height scale reference for train

        passage.grow_passage(1.0);

        let (left, right) = if a.0 < b.0 { (a, b) } else { (b, a) };

        // Dig the mountain
        if offsetstrategy < 2 && rng.gen_bool(0.5) {
          // these mountain shape are suited to dig
          primary = dig_tunnel(
            &mut rng,
            &passage,
            a,
            b,
            primary,
            (pad, pad, width - pad, height - pad),
          );
        } else {
          // instead, we make tunnel entrance, if suitable
          for (dir, pos) in vec![(-1.0, left), (1.0, right)] {
            let max_search = 3.0;
            let height = rng.gen_range(2.0, 4.0) + trainh;
            let mut search = 0.0;
            loop {
              if search > max_search {
                break;
              }
              let x = pos.0 + dir * search;
              let i = (x / precision).round() as usize;
              if i > height_map.len() {
                // out of map
                break;
              }
              let y = height_map[i];
              let dy = pos.1 - y;
              if dy > height {
                // found a suitable place
                let x = pos.0 + height * dir * search / dy;
                let y = pos.1 - height;
                let count = rng.gen_range(4, 8);
                for i in 0..count {
                  let dx = (i as f64 / 3. - 0.5) * (-dir);
                  primary.push(vec![
                    (pos.0 + dx, pos.1),
                    (x + dx, y - dx),
                    (x + dx * 2.0, y - dx),
                  ]);
                }
                break;
              }
              search += 0.1;
            }
          }
        }

        // Build the Bridge Structure
        let doubledy = rng.gen_range(0.6, 0.8);
        let bridge_height_min =
          trainh + rng.gen_range(4.0, 16.0) * rng.gen_range(0.0, 1.0);
        let bridge_height_amp =
          rng.gen_range(0.0, 50f64.min(0.4 * dx)) * rng.gen_range(0.2, 1.0);
        let bridge_height_dir = if a.1.min(b.1)
          > pad + 1.0 + bridge_height_min + bridge_height_amp
          && rng.gen_bool(0.3)
        {
          -1.0
        } else {
          1.0
        };
        // bridge double line
        let extrax = rng.gen_range(1.0, 3.0);
        primary
          .push(vec![(left.0 - extrax, left.1), (right.0 + extrax, right.1)]);
        primary.push(vec![
          (left.0 - extrax, left.1),
          (left.0 - extrax / 2.0, left.1 - doubledy),
          (right.0 + extrax / 2.0, right.1 - doubledy),
          (right.0 + extrax, right.1),
        ]);

        let xstep = rng.gen_range(1.0, 8.0);
        let splits = ((b.0 - a.0).abs() / xstep) as usize; // nb of triangles of the bridge
        let mut route = Vec::new(); // triangles of the bridge structure
        let mut route2 = Vec::new(); // curve of the arche
        let mut route3 = Vec::new(); // second curve (doubled)

        if bridge_height_dir > 0.0 && rng.gen_bool(0.3) {
          let rep = rng.gen_range(2.0, 8.0);
          let mut y = rep;
          loop {
            if y > bridge_height_min {
              break;
            }
            primary.push(vec![(a.0, a.1 + y), (b.0, b.1 + y)]);

            y += rep;
          }
        }

        let bridge_steps = 1
          + (rng.gen_range(0.0, splits as f64 / 10.0) * rng.gen_range(0.0, 1.0))
            as usize;

        let mut bridge_traits = vec![];

        bridge_traits.push(if bridge_steps == 1 {
          "Regular"
        } else if bridge_steps == 2 {
          "Double"
        } else {
          "Complex"
        });

        if bridge_height_dir < 0.0 {
          bridge_traits.push("Reversed");
        }

        if bridge_height_amp < 5.0 {
          bridge_traits.push("Small");
        } else if bridge_height_amp > 30.0 {
          bridge_traits.push("Big");
        }

        bridge_traits.push("Bridge");

        bridge = bridge_traits.join(" ");

        let anchorx = rng.gen_range(5.0, 10.0);

        let sign = (a.0 - b.0).signum();

        let a_anchorx = (a.0 + sign * anchorx).max(pad).min(width - pad);
        let a_anchory = (height_map
          [(a_anchorx / precision).round() as usize % height_map.len()])
        .min(height - pad - 2.0)
        .max(pad + 2.0);
        let b_anchorx = (b.0 - sign * anchorx).max(pad).min(width - pad);
        let b_anchory = (height_map
          [(b_anchorx / precision).round() as usize % height_map.len()])
        .min(height - pad - 2.0)
        .max(pad + 2.0);

        if bridge_height_dir < 0.0 {
          route2.push((a_anchorx, a_anchory));
          route2.push((a_anchorx, a_anchory + doubledy));
        }

        let ratio_y_alignment =
          rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

        for i in 0..splits {
          let p = i as f64 / ((splits - 1) as f64);
          let x = mix(a.0, b.0, p);
          let y = mix(a.1, b.1, p);
          let amp = (bridge_steps as f64 * PI * (x - a.0) / (b.0 - a.0))
            .sin()
            .abs();

          let yratio = (y - b.1) / (a.1 - b.1);
          let h = bridge_height_amp * (1. - ratio_y_alignment * yratio);
          let dy = bridge_height_dir * (bridge_height_min + h * (1.0 - amp));

          let lowesty = (height_map[(x / precision).round() as usize]
            + rng.gen_range(0.0, 10.0))
          .min(height - pad - 2.0);
          let y2 = (y + dy).min(lowesty);

          if lowesty > y2 && amp < 0.2 {
            let doubledx = 0.6;
            primary.push(vec![(x, y2), (x, lowesty)]);
            primary.push(vec![(x - doubledx, y2), (x - doubledy, lowesty)]);
            primary.push(vec![(x + doubledx, y2), (x + doubledx, lowesty)]);
          }
          route.push((x, y));
          route.push((x, y2));
          if xstep < 2.0 {
            primary.push(route);
            route = vec![];
          }
          route2.push((x, y2));
          route3.push((x, y2 + doubledy));
          passage.count((x, y));
          passage.count((x, y2));
          passage.count((x, y2 + doubledy));
        }

        if bridge_height_dir < 0.0 {
          route2.push((b_anchorx, b_anchory));
          route2.push((b_anchorx, b_anchory + doubledy));
        }

        primary.push(route);
        primary.push(route2);
        primary.push(route3);

        // Add our train
        let headx = mix(a.0, b.0, rng.gen_range(0.2, 0.8));
        let basetrainy = |x| -1.0 + mix(a.1, b.1, (x - a.0) / (b.0 - a.0));
        let carriage_dist = rng.gen_range(0.5, 0.8);
        let lines_dist = rng.gen_range(0.3, 0.4);

        // railway
        let x1 = headx;
        let x2 = headx - trainh * 3.0;
        let y1 = basetrainy(x1);
        let y2 = basetrainy(x2);
        // base
        let mut dy = 0.0;
        loop {
          if dy > trainh {
            break;
          }
          let a = pos(x1, y1, 0., -dy);
          let b = pos(x2, y2, 0., -dy);
          passage.count(a);
          passage.count(b);
          secondary.push(vec![a, b]);
          dy += lines_dist;
        }
        // chimney
        let mut dx = 0.0;
        let chimneyx = mix(x1, x2, rng.gen_range(0.2, 0.3));
        let chimneyw = rng.gen_range(0.4, 0.6) * trainh;
        let chimneyh = rng.gen_range(0.6, 0.8) * trainh;
        let chimneyytop = basetrainy(chimneyx) - trainh - chimneyh;
        loop {
          if dx > chimneyw {
            break;
          }
          let x = chimneyx - chimneyw / 2.0 + dx;
          let y = basetrainy(x);
          let a = pos(x, y, 0., -trainh);
          let b = pos(x, y, 0., -trainh - chimneyh);
          passage.count(a);
          passage.count(b);
          secondary.push(vec![a, b]);
          dx += lines_dist;
        }

        // coal carriage

        let w = trainh * 1.8;
        let h = 0.7 * trainh;
        let x1 = x2 - carriage_dist;
        let x2 = x1 - w;
        let y1 = basetrainy(x1);
        let y2 = basetrainy(x2);
        // base
        let mut dy = 0.0;
        loop {
          if dy > h {
            break;
          }
          let a = pos(x1, y1, 0.0, -dy);
          let b = pos(x2, y2, 0.0, -dy);
          passage.count(a);
          passage.count(b);
          secondary.push(vec![a, b]);
          dy += lines_dist;
        }
        // coal
        let mut dy = 0.0;
        let coalx = mix(x1, x2, 0.5);
        let coalw = rng.gen_range(0.7, 0.8) * w;
        let coalh = rng.gen_range(0.2, 0.3) * trainh;
        loop {
          if dy > coalh {
            break;
          }
          let (x1, y1) = pos(coalx, basetrainy(coalx), -coalw / 2.0, -h - dy);
          let (x2, y2) = pos(coalx, basetrainy(coalx), coalw / 2.0, -h - dy);
          secondary.push(vec![(x1, y1), (x2, y2)]);
          passage.count((x1, y1));
          passage.count((x2, y2));
          dy += lines_dist;
        }

        // carriages

        let mut x1;
        let mut x2 = x2;
        for _i in 0..rng.gen_range(3, 12) {
          let w = 5.0 * trainh;
          let h = trainh;
          x1 = x2 - carriage_dist;
          x2 = x1 - w;

          let x2_is_on_bridge = a.0.min(b.0) < x2 && x2 < a.0.max(b.0);
          if !x2_is_on_bridge {
            break;
          }

          let y1 = basetrainy(x1);
          let y2 = basetrainy(x2);
          // base
          let mut dy = 0.0;
          loop {
            if dy > h {
              break;
            }
            let a = pos(x1, y1, 0., -dy);
            let b = pos(x2, y2, 0., -dy);
            secondary.push(vec![a, b]);
            passage.count(a);
            passage.count(b);
            dy += lines_dist;
          }

          total_carriages += 1;
        }

        // smoke

        for _j in 0..8 {
          let mut ang = -PI / 2.0;
          let incr = 0.5;
          let mut angdelta = 0.3;
          let mut amp = 0.3;
          let mut route = Vec::new();
          let mut lastp = (chimneyx, chimneyytop);
          let count =
            (rng.gen_range(20., 400.) * rng.gen_range(0.2, 1.0)) as usize;
          for i in 0..count {
            let v = rng.gen_range(-1.0, 1.0);
            let disp = (
              v * amp * (ang + PI / 2.0).cos(),
              v * amp * (ang + PI / 2.0).sin(),
            );
            let p = (
              lastp.0 + incr * ang.cos() + disp.0,
              lastp.1 + incr * ang.sin() + disp.1,
            );
            if p.0 < pad + 2.
              || p.0 > width - pad - 2.
              || p.1 < pad + 2.
              || p.1 > height - pad - 2.
              || p.1
                > height_map
                  [(p.0 / precision).round() as usize % height_map.len()]
            {
              break;
            }
            passage.count(p);
            route.push(p);
            ang -= angdelta;
            angdelta /= 1.3;
            amp = (amp * 1.01 + 0.01).min(0.7);
            lastp = p;
            if route.len() > 1 && rng.gen_bool(0.5) {
              if rng.gen_bool(0.7 - 0.6 * i as f64 / (count as f64)) {
                // randomly drop
                highlightedpart.push(route);
              }
              route = Vec::new();
            }
          }
          highlightedpart.push(route);
        }

        train_found = true;

        lowbridgey = a.1.min(b.1);

        // cactus
        let divisions = 1
          + (rng.gen_range(0.0, 32.0)
            * rng.gen_range(0.0, 1.0)
            * rng.gen_range(0.0, 1.0)) as usize;
        let pad = 10.0;
        let positions: Vec<(f64, f64)> = (0..divisions)
          .map(|i| {
            let x =
              (i as f64 + 0.5) / (divisions as f64) * (width - 2.0 * pad) + pad;
            let i = (x / precision).round() as usize;
            let y = height_map[i % height_map.len()];
            if y > height - pad - 2.0 {
              return None;
            }
            // check slope
            let yprev = height_map[(i - 1) % height_map.len()];
            let ynext = height_map[(i + 1) % height_map.len()];
            let dy = (ynext - yprev) / 2.0;
            if dy.abs() > 0.5 {
              return None;
            }
            // check proximity to train
            if (a.1 - y).abs().min(b.1 - y).abs() < 12.0 {
              return None;
            }

            Some((x, y))
          })
          .filter_map(|v| v)
          .collect();

        let proba = rng.gen_range(0.05, 0.95);
        for p in positions {
          if rng.gen_bool(proba) {
            continue;
          }
          total_cactus += 1;
          let h = rng.gen_range(4.0, 8.0);
          primary.extend(cactus(&mut rng, &mut passage, p, h));
        }
        break; // we found our bridge, stops
      }
    }

    if !train_found {
      continue;
    }

    passage.grow_passage(2.0);

    let bound = (pad, pad, width - pad, height - pad);

    let in_shape = |p: (f64, f64)| -> bool {
      passage.get(p) == 0 && strictly_in_boundaries(p, bound)
    };

    let does_overlap = |c: &VCircle| {
      in_shape((c.x, c.y))
        && circle_route((c.x, c.y), c.r, 8)
          .iter()
          .all(|&p| in_shape(p))
    };

    let count = (rng.gen_range(0.0, 32.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;

    let eagle_circles = packing(
      &mut rng,
      100000,
      count,
      1,
      2.0,
      bound,
      &does_overlap,
      3.0,
      4.0,
    );

    for c in eagle_circles.iter() {
      let rot = 0.2 * rng.gen_range(-PI, PI);
      let xreverse = rng.gen_bool(0.5);
      primary.extend(eagle(
        (c.x, c.y),
        c.r,
        rot,
        xreverse,
        &mut rng,
        &mut passage,
      ));
      total_eagles += 1;
    }

    // make clouds areas

    let max_clouds = 8.0;
    let count = (rng.gen_range(0.0, max_clouds)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;

    passage.grow_passage(1.0);

    let in_shape = |p: (f64, f64)| -> bool {
      passage.get(p) == 0 && strictly_in_boundaries(p, bound)
    };

    let does_overlap = |c: &VCircle| {
      c.y < lowbridgey
        && eagle_circles.iter().all(|eagle| c.dist(eagle) > 6.0)
        && in_shape((c.x, c.y))
        && circle_route((c.x, c.y), c.r, 8)
          .iter()
          .all(|&p| in_shape(p))
    };

    let circles = packing(
      &mut rng,
      50000,
      count,
      1,
      0.0,
      bound,
      &does_overlap,
      5.0,
      20.0,
    );

    let clouds: Vec<VCircle> = circles
      .iter()
      .flat_map(|c| {
        let (routes, circles) = cloud_in_circle(&mut rng, &c);
        primary.extend(routes);
        circles
      })
      .collect();

    let cloud_dy = rng.gen_range(2.0, 3.0);
    let freqx = 0.5 + rng.gen_range(0.5, 8.0) * rng.gen_range(0.0, 1.0);
    let freqy = freqx * rng.gen_range(1.0, 4.0);
    let threshold = rng.gen_range(-1.0, 1.0);

    let mut routes = vec![];
    let mut y = pad + 2.0 + cloud_dy;
    let maxy = lowbridgey;
    let mut total = 0;
    let mut covered = 0;
    loop {
      let percent_y = (y - pad) / (maxy - pad);
      let mut x = pad + 2.5;

      let mut route = vec![];
      loop {
        x += 0.5;

        let can_have_cloud = passage.get((x, y)) == 0
          && perlin.get([freqx * x / width, freqy * y / height, seed])
            - threshold
            > percent_y;

        if can_have_cloud && !clouds.iter().any(|c| c.includes((x, y))) {
          // draw
          if route.len() < 2 {
            route.push((x, y));
          } else {
            route[1] = (x, y);
          }
          covered += 1;
        } else {
          // save the stroke
          let l = route.len();
          if l > 1 {
            routes.push(route);
            route = vec![];
          } else if l != 0 {
            route = vec![];
          }

          // in that case, it's actually a cloud circles case
          if can_have_cloud {
            covered += 1;
          }
        }
        total += 1;

        if x > width - pad - 3.0 {
          break;
        }
      }

      let l = route.len();
      if l > 1 {
        routes.push(route);
      }

      y += cloud_dy;
      if y > maxy {
        break;
      }
    }
    cloud_density_factor = (covered as f64 / total as f64) * 0.9
      + 0.1 * (circles.len() as f64 / max_clouds);

    primary.extend(routes);
  }

  // Border around the postcard
  let border_size = 8;
  let border_dist = 0.3;
  let mut route = Vec::new();
  for i in 0..border_size {
    let d = i as f64 * border_dist;
    route.push((pad + d, pad + d));
    route.push((pad + d, height - pad - d));
    route.push((width - pad - d, height - pad - d));
    route.push((width - pad - d, pad + d));
    route.push((pad + d, pad + d));
  }

  if opts.gold_border {
    secondary.push(route);
  } else {
    primary.push(route);
  }

  let (layers, inks) = make_layers(vec![
    ("#0FF", opts.primary_name.clone(), primary),
    ("#F0F", opts.secondary_name.clone(), secondary),
    ("#FF0", opts.secondary_name.clone(), highlightedpart),
  ]);

  let mut traits = Map::new();
  traits.insert(String::from("Inks Count"), json!(inks.len()));
  traits.insert(String::from("Inks"), json!(inks.join(" + ")));
  traits.insert(String::from("Total Cactus"), json!(total_cactus));
  traits.insert(String::from("Total Carriages"), json!(total_carriages));
  traits.insert(String::from("Total Eagles"), json!(total_eagles));
  traits.insert(String::from("Mountain Kind"), json!(offsetstrategy));

  // slope
  traits.insert(
    String::from("Train Slope"),
    json!((if train_slope < 0.04 {
      "Flat"
    } else if train_slope < 0.08 {
      "Gentle"
    } else if train_slope < 0.16 {
      "Moderate"
    } else {
      "Steep"
    })
    .to_string()),
  );

  if total_cactus > 0 {
    traits.insert(
      String::from("Cactus Density"),
      json!((if total_cactus < 5 {
        "Low"
      } else if total_cactus < 10 {
        "High"
      } else {
        "Extreme"
      })
      .to_string()),
    );
  }

  if total_eagles > 0 {
    traits.insert(
      String::from("Eagle Density"),
      json!((if total_eagles < 5 {
        "Low"
      } else if total_eagles < 15 {
        "High"
      } else {
        "Extreme"
      })
      .to_string()),
    );
  }

  if cloud_density_factor > 0.0 {
    traits.insert(
      String::from("Cloud Density"),
      json!((if cloud_density_factor < 0.1 {
        "Low"
      } else if cloud_density_factor < 0.4 {
        "Medium"
      } else {
        "High"
      })
      .to_string()),
    );
  }

  traits.insert(
    String::from("Precipice"),
    json!((if mountain_offbound_factor < 0.1 {
      "Regular"
    } else if mountain_offbound_factor < 0.4 {
      "Moderate"
    } else if mountain_offbound_factor < 0.6 {
      "Deep"
    } else {
      "Very Deep"
    })
    .to_string()),
  );

  traits.insert(String::from("Bridge"), json!(bridge));

  if opts.gold_border || inks.len() == 1 && inks[0] == "Gold" {
    traits.insert(String::from("Gold Border"), json!("Yes"));
  }

  let mut document = svg::Document::new()
    .set("data-credits", "@greweb - 2023 - Golden Train".to_string())
    .set("data-hash", opts.hash.to_string())
    .set("data-traits", Value::Object(traits).to_string())
    .set("viewBox", (0, 0, width, height))
    .set("width", format!("{}mm", width))
    .set("height", format!("{}mm", height))
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

// The slime primitive =>

// Generic helper to simplify and clean up a path

// render helper

#[inline]
fn significant_str(f: f64) -> f64 {
  (f * 100.0).floor() / 100.0
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

fn rng_from_hexhash(hash: &String) -> impl Rng {
  let mut bs = [0; 32];
  bs.copy_from_slice(&Vec::<u8>::from_hex(hash).unwrap().as_slice());
  let rng = StdRng::from_seed(bs);
  return rng;
}

#[inline]
fn mix(a: f64, b: f64, x: f64) -> f64 {
  (1. - x) * a + x * b
}

#[derive(Clone)]
struct Passage {
  precision: f64,
  width: f64,
  height: f64,
  counters: Vec<usize>,
}
impl Passage {
  pub fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision).ceil() as usize;
    let hi = (height / precision).ceil() as usize;
    let counters = vec![0; wi * hi];
    Passage {
      precision,
      width,
      height,
      counters,
    }
  }

  fn index(self: &Self, (x, y): (f64, f64)) -> usize {
    let wi = (self.width / self.precision).ceil() as usize;
    let hi = (self.height / self.precision).ceil() as usize;
    let xi = ((x / self.precision).round() as usize).max(0).min(wi - 1);
    let yi = ((y / self.precision).round() as usize).max(0).min(hi - 1);
    yi * wi + xi
  }

  pub fn count(self: &mut Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    let v = self.counters[i] + 1;
    self.counters[i] = v;
    v
  }

  pub fn count_once(self: &mut Self, p: (f64, f64)) {
    let i = self.index(p);
    let v = self.counters[i];
    if v == 0 {
      self.counters[i] = 1;
    }
  }

  pub fn get(self: &Self, p: (f64, f64)) -> usize {
    let i = self.index(p);
    self.counters[i]
  }

  pub fn grow_passage(self: &mut Self, radius: f64) {
    let precision = self.precision;
    let width = self.width;
    let height = self.height;
    let counters: Vec<usize> = self.counters.iter().cloned().collect();
    let mut mask = Vec::new();
    // TODO, in future for even better perf, I will rewrite this
    // working directly with index integers instead of having to use index() / count_once()
    let mut x = -radius;
    loop {
      if x >= radius {
        break;
      }
      let mut y = -radius;
      loop {
        if y >= radius {
          break;
        }
        if x * x + y * y < radius * radius {
          mask.push((x, y));
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
      let mut y = 0.0;
      loop {
        if y >= height {
          break;
        }
        let index = self.index((x, y));
        if counters[index] > 0 {
          for &(dx, dy) in mask.iter() {
            self.count_once((x + dx, y + dy));
          }
        }
        y += precision;
      }
      x += precision;
    }
  }
}

fn make_layers(
  data: Vec<(&str, String, Vec<Vec<(f64, f64)>>)>,
) -> (Vec<Group>, Vec<String>) {
  let mut inks = Vec::new();
  let layers: Vec<Group> = data
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
      let opacity: f64 = 0.6;
      let opdiff = 0.15 / (routes.len() as f64);
      let mut trace = 0f64;
      for route in routes.clone() {
        trace += 1f64;
        let data = render_route(Data::new(), route);
        l = l.add(
          Path::new()
            .set(
              "opacity",
              (1000. * (opacity - trace * opdiff)).floor() / 1000.0,
            )
            .set("d", data),
        );
      }
      l
    })
    .collect();
  // remove inks that have no paths at all
  inks.sort();
  inks.dedup();
  if inks.len() == 2 && inks[0].eq(&inks[1]) {
    inks.remove(1);
  }
  (layers, inks)
}

fn cactus_branch(
  path: Vec<(f64, f64)>,
  divs: usize,
  width: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  for d in 0..divs {
    let maxw = ((d as f64 + 0.5) / (divs as f64)) * width;

    let mut prev = &path[0];
    let mut left_route = vec![];
    let mut right_route = vec![];
    for (i, p) in path.iter().skip(1).enumerate() {
      let w = (1.0 - 0.8 * ((i as f64) / (path.len() as f64)).powf(2.0)) * maxw;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let angle = dy.atan2(dx);
      let acos = angle.cos();
      let asin = angle.sin();
      let angle2 = angle + PI / 2.0;
      let a2cos = angle2.cos();
      let a2sin = angle2.sin();
      if i == 0 {
        let left = (prev.0 + a2cos * w, prev.1 + a2sin * w);
        let right = (prev.0 - a2cos * w, prev.1 - a2sin * w);
        left_route.push(left);
        right_route.push(right);
      }
      let left = (p.0 + acos * maxw + a2cos * w, p.1 + asin * maxw + a2sin * w);
      let right =
        (p.0 + acos * maxw - a2cos * w, p.1 + asin * maxw - a2sin * w);
      left_route.push(left);
      right_route.push(right);
      prev = p;
    }

    right_route.reverse();
    left_route.extend(right_route);
    routes.push(path_subdivide_to_curve(left_route, 2, 0.8));
  }

  routes
}

fn make_cactus_branch_path<R: Rng>(
  rng: &mut R,
  passage: &mut Passage,
  origin: (f64, f64),
  incr: f64,
  ang: f64,
  da: f64,
  randomangle: f64,
  height: f64,
) -> Vec<(f64, f64)> {
  let mut main_branch = vec![];
  let mut length = 0.0;
  let mut x = origin.0;
  let mut y = origin.1;
  let mut a = ang;
  main_branch.push((x, y));
  loop {
    if length > height {
      break;
    }
    x += a.cos() * incr;
    y += a.sin() * incr;
    let p = (x, y);
    main_branch.push(p);
    passage.count(p);
    a += da + rng.gen_range(-1.0, 1.0) * randomangle;
    length += incr;
  }
  main_branch
}

fn cactus<R: Rng>(
  rng: &mut R,
  passage: &mut Passage,
  origin: (f64, f64),
  height: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let incr = rng.gen_range(0.8, 2.0);
  let randomangle = rng.gen_range(0.0, 0.5);

  let main_branch = make_cactus_branch_path(
    rng,
    passage,
    origin,
    incr,
    -PI / 2.0,
    0.0,
    randomangle,
    height,
  );

  let idelta = rng.gen_range(0, 2);
  for i in 0..3 {
    let index = i * 2;
    if index >= main_branch.len() {
      break;
    }
    let incr = rng.gen_range(0.8, 1.2);
    let h = height * rng.gen_range(0.3, 0.6);

    let imod = ((i + idelta) % 2) as f64 - 0.5;
    let da = rng.gen_range(0.4, 0.6) * imod;
    let ang = -2.0 * imod - PI / 2.0;
    let p = main_branch[index];
    let o = (p.0 - imod, p.1);
    let branch =
      make_cactus_branch_path(rng, passage, o, incr, ang, da, 0.0, h);
    let w = rng.gen_range(0.07, 0.12) * height;
    let divs = rng.gen_range(2, 4);
    routes.extend(cactus_branch(branch, divs, w));
  }

  let w = rng.gen_range(0.1, 0.2) * height;
  let divs = rng.gen_range(3, 8);
  routes.extend(cactus_branch(main_branch, divs, w));

  routes
}

fn eagle<R: Rng>(
  origin: (f64, f64),
  sz: f64,
  rotation: f64,
  xreverse: bool,
  rng: &mut R,
  passage: &mut Passage,
) -> Vec<Vec<(f64, f64)>> {
  let scale = sz / 5.0;
  let xmul = if xreverse { -1.0 } else { 1.0 };
  let count = 2 + (scale * 3.0) as usize;
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let shaking = scale * 0.1;

  // body
  let bodyw = 5.0;
  let bodyh = 1.5;
  let headcompression = rng.gen_range(0.1, 0.5);
  let headoff = rng.gen_range(0.1, 0.5);
  for i in 0..count {
    let yp = (i as f64 - (count - 1) as f64 * 0.5) / (count as f64);
    let ybase = bodyh * yp;
    let route = shake(
      path_subdivide_to_curve(
        vec![
          (-rng.gen_range(0.4, 0.6) * bodyw, 1.5 * ybase),
          (-0.3 * bodyw, ybase),
          (0.2 * bodyw, ybase),
          (0.45 * bodyw, headcompression * ybase + headoff * bodyh),
        ],
        1,
        0.8,
      ),
      shaking,
      rng,
    );
    routes.push(route);
  }

  let count = 2 + (scale * rng.gen_range(4.0, 6.0)) as usize;

  // wings
  let wingw = 1.4;
  let wingh = 8.0;
  let dx1 = rng.gen_range(-4.0, 4.0) * rng.gen_range(0.0, 1.0);
  let dx2 = if rng.gen_bool(0.8) {
    -dx1
  } else {
    rng.gen_range(-3.0, 3.0)
  };
  let spread1 = 1.0 + rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let spread2 = 1.0 + rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let offset1 = rng.gen_range(-1.0, 0.6) * rng.gen_range(0.0, 1.0);
  let offset2 = rng.gen_range(-1.0, 0.6) * rng.gen_range(0.0, 1.0);
  let interp = 0.5;
  let wing1m = 1.0 - rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
  let wing2m = 1.0 - rng.gen_range(0.0, 0.5) * rng.gen_range(0.0, 1.0);
  let wing2up = rng.gen_bool(0.5);

  for i in 0..count {
    let xp = (i as f64 - (count - 1) as f64 * 0.5) / (count as f64);
    let xbase = wingw * xp;
    let wing1 = rng.gen_range(0.8, 1.1) * wing1m;
    let wing2 =
      rng.gen_range(0.8, 1.1) * wing2m * (if wing2up { -1.0 } else { 1.0 });
    let route = shake(
      path_subdivide_to_curve(
        vec![
          (
            xbase * spread1 + dx1 + wingw * offset1,
            -wingh * 0.5 * wing1,
          ),
          (xbase + dx1 * interp, -wingh * 0.5 * interp * wing1),
          (xbase, 0.0),
          (xbase + dx2 * interp, wingh * 0.5 * interp * wing2),
          (xbase * spread2 + dx2 + wingw * offset2, wingh * 0.5 * wing2),
        ],
        2,
        0.8,
      ),
      shaking,
      rng,
    );
    routes.push(route);
  }

  // scale, rotate & translate
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&p| {
          let p = p_r(p, rotation);
          let p = (xmul * scale * p.0 + origin.0, scale * p.1 + origin.1);
          passage.count(p);
          p
        })
        .collect()
    })
    .collect()
}

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

fn shake<R: Rng>(
  path: Vec<(f64, f64)>,
  scale: f64,
  rng: &mut R,
) -> Vec<(f64, f64)> {
  path
    .iter()
    .map(|&(x, y)| {
      let dx = rng.gen_range(-scale, scale);
      let dy = rng.gen_range(-scale, scale);
      (x + dx, y + dy)
    })
    .collect()
}

#[inline]
fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
}

#[derive(Clone, Copy, Debug)]
struct VCircle {
  x: f64,
  y: f64,
  r: f64,
}
impl VCircle {
  fn new(x: f64, y: f64, r: f64) -> Self {
    VCircle { x, y, r }
  }
  fn includes(self: &Self, p: (f64, f64)) -> bool {
    euclidian_dist(p, (self.x, self.y)) < self.r
  }
  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
}

fn scaling_search<F: FnMut(f64) -> bool>(
  mut f: F,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let mut from = min_scale;
  let mut to = max_scale;
  loop {
    if !f(from) {
      return None;
    }
    if to - from < 0.1 {
      return Some(from);
    }
    let middle = (to + from) / 2.0;
    if !f(middle) {
      to = middle;
    } else {
      from = middle;
    }
  }
}

fn search_circle_radius(
  does_overlap: &dyn Fn(&VCircle) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap(&c) && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing<R: Rng>(
  rng: &mut R,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = Vec::new();
  let mut tries = Vec::new();
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(bound.0, bound.2);
    let y: f64 = rng.gen_range(bound.1, bound.3);
    if let Some(size) =
      search_circle_radius(&does_overlap, &circles, x, y, min_scale, max_scale)
    {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        tries = Vec::new();
      }
    }
    if circles.len() > desired_count {
      break;
    }
  }
  circles
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

fn arc(
  center: (f64, f64),
  r: f64,
  start: f64,
  end: f64,
  count: usize,
) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  for i in 0..(count + 1) {
    let a = start + (end - start) * i as f64 / (count as f64);
    let x = center.0 + r * a.cos();
    let y = center.1 + r * a.sin();
    route.push((x, y));
  }
  return route;
}

#[inline]
pub fn strictly_in_boundaries(
  p: (f64, f64),
  boundaries: (f64, f64, f64, f64),
) -> bool {
  p.0 > boundaries.0
    && p.0 < boundaries.2
    && p.1 > boundaries.1
    && p.1 < boundaries.3
}

fn dig_tunnel<R: Rng>(
  rng: &mut R,
  passage: &Passage,
  a: (f64, f64),
  b: (f64, f64),
  input_routes: Vec<Vec<(f64, f64)>>,
  boundary: (f64, f64, f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  let dy = 0.5;
  let tunnelh = 5.0;

  let (left, right) = if a.0 < b.0 { (a, b) } else { (b, a) };

  let mut bounds = vec![];
  let mut tunnel_routes = vec![];

  let tunnelw = rng.gen_range(5.0, 40.0) * rng.gen_range(0.2, 1.0);
  let mut b = (
    (left.0 - tunnelw).max(boundary.0),
    left.1 - tunnelh - dy,
    left.0,
    left.1 - dy,
  );
  tunnel_routes.push(vec![(b.2, b.3), (b.0, b.3), (b.0, b.1), (b.2, b.1)]);
  tunnel_routes.push(vec![
    (b.2, b.3 + 0.7),
    (b.0, b.3 + 0.7),
    (b.0, b.1),
    (b.2, b.1),
  ]);
  b.2 += 5.0;
  bounds.push(b);

  let tunnelw = rng.gen_range(0.0, 40.0) * rng.gen_range(0.0, 1.0);
  let mut b = (
    right.0,
    right.1 - tunnelh - dy,
    (right.0 + tunnelw).min(boundary.2),
    right.1 - dy,
  );
  tunnel_routes.push(vec![(b.0, b.1), (b.2, b.1), (b.2, b.3), (b.0, b.3)]);
  tunnel_routes.push(vec![
    (b.0, b.1),
    (b.2, b.1),
    (b.2, b.3 + 0.7),
    (b.0, b.3 + 0.7),
  ]);
  b.0 -= 5.0;
  bounds.push(b);

  let should_crop =
    |p| bounds.iter().any(|&bound| strictly_in_boundaries(p, bound));

  let mut cutted_points = vec![];
  let proba_skip = rng.gen_range(-0.1f64, 1.3).max(0.001).min(0.999)
    * rng.gen_range(0.0, 1.0)
    * rng.gen_range(0.0, 1.0);

  let mut routes = crop_routes_with_predicate_rng(
    rng,
    proba_skip,
    input_routes,
    &should_crop,
    &mut cutted_points,
  );

  if proba_skip < 0.5 {
    for route in tunnel_routes.clone() {
      let route = subdivide(route, 3)
        .iter()
        .take_while(|p| passage.get((p.0, p.1)) > 0)
        .map(|p| *p)
        .collect::<Vec<(f64, f64)>>();

      if route.len() > 1 {
        routes.push(route);
      }
    }
  }

  routes
}

fn crop_routes_with_predicate_rng<R: Rng>(
  rng: &mut R,
  proba_skip: f64,
  input_routes: Vec<Vec<(f64, f64)>>,
  should_crop: &dyn Fn((f64, f64)) -> bool,
  cutted_points: &mut Vec<(f64, f64)>,
) -> Vec<Vec<(f64, f64)>> {
  let search = |a_, b_, n| {
    let mut a = a_;
    let mut b = b_;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if should_crop(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes {
    if input_route.len() < 2 {
      continue;
    }
    if proba_skip > 0.0 && rng.gen_bool(proba_skip) {
      routes.push(input_route);
      continue;
    }
    let mut prev = input_route[0];
    let mut route = vec![];
    if !should_crop(prev) {
      // prev is not to crop. we can start with it
      route.push(prev);
    } else {
      if !should_crop(input_route[1]) {
        // prev is to crop, but [1] is outside. we start with the exact intersection
        let intersection = search(input_route[1], prev, 7);
        prev = intersection;
        cutted_points.push(intersection);
        route.push(intersection);
      } else {
        cutted_points.push(prev);
      }
    }
    // cut segments with crop logic
    for &p in input_route.iter().skip(1) {
      // TODO here, we must do step by step to detect crop inside the segment (prev, p)

      if should_crop(p) {
        if route.len() > 0 {
          // prev is outside, p is to crop
          let intersection = search(prev, p, 7);
          cutted_points.push(intersection);
          route.push(intersection);
          routes.push(route);
          route = vec![];
        } else {
          cutted_points.push(p);
        }
      } else {
        // nothing to crop
        route.push(p);
      }
      prev = p;
    }
    if route.len() >= 2 {
      routes.push(route);
    }
  }

  routes
}

fn cloud_in_circle<R: Rng>(
  rng: &mut R,
  circle: &VCircle,
) -> (Vec<Vec<(f64, f64)>>, Vec<VCircle>) {
  let mut routes = vec![];

  let mut circles: Vec<VCircle> = vec![];

  let stretchy = rng.gen_range(0.2, 1.0);

  let count = rng.gen_range(16, 40);
  for _i in 0..count {
    let radius = circle.r * rng.gen_range(0.3, 0.5) * rng.gen_range(0.2, 1.0);
    let angle = rng.gen_range(0.0, 2.0 * PI);
    let x = circle.x + angle.cos() * (circle.r - radius);
    let y = circle.y
      + angle.sin() * (circle.r - radius) * rng.gen_range(0.5, 1.0) * stretchy;
    let circle = VCircle::new(x, y, radius);

    let should_crop = |p| circles.iter().any(|c| c.includes(p));

    let mut input_routes = vec![];
    let mut r = radius;
    let dr = rng.gen_range(1.0, 3.0);
    loop {
      if r < 1.0 {
        break;
      }
      let count = (r * 2.0 + 10.0) as usize;
      let amp = rng.gen_range(0.5 * PI, 1.2 * PI);
      let ang = angle
        + PI
          * rng.gen_range(-1.0, 1.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0);
      let start = ang - amp / 2.0;
      let end = ang + amp / 2.0;
      input_routes.push(arc((x, y), r, start, end, count));
      r -= dr;
    }

    routes.extend(crop_routes_with_predicate_rng(
      rng,
      0.0,
      input_routes,
      &should_crop,
      &mut vec![],
    ));

    circles.push(circle);
  }

  (routes, circles)
}

fn subdivide(path: Vec<(f64, f64)>, n: usize) -> Vec<(f64, f64)> {
  if n <= 0 || path.len() < 2 {
    return path;
  }
  let mut last = path[0];
  let mut route = vec![last];
  for &p in path.iter().skip(1) {
    let a = lerp_point(last, p, 0.5);
    route.push(a);
    route.push(p);
    last = p;
  }
  for _i in 0..n {
    route = subdivide(route, n - 1);
  }
  route
}
