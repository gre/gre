/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2023 – Plottable Era (II) Medieval
 */
mod utils;
use fontdue::layout::*;
use fontdue::*;
use noise::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::iter;
use std::ops::RangeInclusive;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use wasm_bindgen::prelude::*;

// Function called from JS to get the SVG document
#[wasm_bindgen]
pub fn render(val: &JsValue) -> String {
  let opts = val.into_serde().unwrap();
  let (doc, _) = art(&opts, true);
  let str = doc.to_string();
  return str;
}

// Input to the art function
#[derive(Deserialize)]
pub struct Opts {
  pub hash: String,
  pub width: f64,
  pub height: f64,
  pub pad: f64,
  pub fontdata: Vec<u8>,
}

// Feature tells caracteristics of a given art variant
// It is returned in the .SVG file
#[derive(Clone, Serialize)]
pub struct Feature {
  // which inks are used
  pub inks: String,
  // how much inks are used
  pub inks_count: usize,
  // which paper is used
  pub paper: String,
}

#[derive(Clone, Copy, Serialize)]
pub struct Ink(&'static str, &'static str, &'static str, f64);
#[derive(Clone, Copy, Serialize)]
pub struct Paper(&'static str, &'static str, bool);

// This is also returned in the SVG to have more metadata for the JS side to render a digital version
#[derive(Clone, Serialize)]
pub struct Palette {
  pub primary: Ink,
  pub secondary: Ink,
  pub third: Ink,
  pub paper: Paper,
}

// This is the main art function that will render the generative art piece
pub fn art(opts: &Opts, mask_mode: bool) -> (svg::Document, Feature) {
  let height = opts.height;
  let width = opts.width;
  let pad = opts.pad;
  let bounds = (pad, pad, width - pad, height - pad);

  let mut font =
    Font::from_bytes(opts.fontdata.clone(), FontSettings::default()).unwrap();

  // rng utilities
  let mut rng = rng_from_fxhash(&opts.hash);
  let perlin = Perlin::new();

  // Prepare all the colors

  let red_gel = Ink("Red Gel", "#BF738C", "#D880A6", 0.35);
  let orange_gel = Ink("Orange Gel", "#B27333", "#E68C4D", 0.35);
  let blue_gel = Ink("Blue Gel", "#338CFF", "#4D8CFF", 0.35);
  let green_gel = Ink("Green Gel", "#00B2A6", "#19CCBF", 0.35);

  let gold_gel = Ink("Gold Gel", "#D8B240", "#FFE38C", 0.6);
  let silver_gel = Ink("Silver Gel", "#CCCCCC", "#FFFFFF", 0.6);
  let white_gel = Ink("White Gel", "#E5E5E5", "#FFFFFF", 0.35);
  let black = Ink("Black", "#1A1A1A", "#000000", 0.35);
  let seibokublue = Ink("Sailor Sei-boku", "#1060a3", "#153a5d", 0.35);
  let inaho = Ink("iroshizuku ina-ho", "#ba6", "#7f6a33", 0.35);
  let imperial_purple = Ink("Imperial Purple", "#4D0066", "#260F33", 0.35);
  let sherwood_green = Ink("Sherwood Green", "#337239", "#194D19", 0.35);
  let evergreen = Ink("Evergreen", "#4D6633", "#263319", 0.35);
  let soft_mint = Ink("Soft Mint", "#33E0CC", "#19B299", 0.35);
  let turquoise = Ink("Turquoise", "#00B4E6", "#005A8C", 0.35);
  let sargasso_sea = Ink("Sargasso Sea", "#162695", "#111962", 0.35);
  let indigo = Ink("Indigo", "#667599", "#334D66", 0.35);
  let aurora_borealis = Ink("Aurora Borealis", "#009999", "#004D66", 0.35);
  let pumpkin = Ink("Pumpkin", "#FF8033", "#E54D00", 0.35);
  let pink = Ink("Pink", "#fd728e", "#E5604D", 0.35);
  let hope_pink = Ink("Hope Pink", "#fc839b", "#E53399", 0.35);
  let amber = Ink("Amber", "#FFC745", "#FF8000", 0.35);
  let poppy_red = Ink("Poppy Red", "#E51A1A", "#80001A", 0.35);
  let red_dragon = Ink("Red Dragon", "#9e061a", "#5b0a14", 0.35);
  let fire_and_ice = Ink("Fire And Ice", "#00BEDE", "#006478", 0.35);
  let bloody_brexit = Ink("Bloody Brexit", "#05206B", "#2E0033", 0.35);

  // ideas for color:
  // monochrome => any color
  // bicolor => 0 is mostly always black
  // bicolor => maybe seilor seiboku + ina ho
  // bicolor => maybe grey + ina ho
  // bicolor => black + grey

  let white_paper = Paper("White", "#fff", false);
  let black_paper = Paper("Black", "#202020", true);
  let grey_paper = Paper("Grey", "#959fa8", true);
  // TODO blue paper
  // TODO red paper??

  let precision = 0.2;
  let mut mask = PaintMask::new(precision, width, height);

  let prob = 0.12;
  // colors
  // 0 : mountains & objects
  // 1 : sun
  // 2 : human lights / fire -> MAYBE IT'S THE SAME COLOR!
  let (mut colors, paper) = (vec![black, amber, poppy_red], white_paper);

  // Prepare the generative code
  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = vec![];
  let mut reflectables: Vec<(usize, Vec<(f64, f64)>)> = vec![];

  // statistics of ink density usage
  let mut passage = Passage::new(0.5, width, height);
  let passage_threshold = 20;

  // FRAME

  let p = 10.0;
  let m = 10.0;
  let (pattern, strokew): (Box<dyn BandPattern>, f64) =
    match rng.gen_range(0, 5) {
      0 => (Box::new(MedievalBandLRectPattern::new()), 0.08 * p),
      1 => (
        Box::new(MedievalBandFeatherTrianglePattern::new()),
        0.06 * p,
      ),
      2 => (Box::new(MedievalBandForkPattern::new()), 0.06 * p),
      3 => (Box::new(MedievalBandComb::new()), 0.04 * p),
      4 => (Box::new(MedievalBandCurvePattern::new()), 0.04 * p),
      _ => (Box::new(MedievalBandConcentric::new(2)), 0.08 * p),
    };
  routes.extend(framing(
    &mut rng,
    &mut mask,
    0,
    (pad, pad, width - pad, height - pad),
    pattern.as_ref(),
    p,
    m,
    strokew,
    3.0,
    20000,
  ));

  // SHAPE THE MOUNTAINS

  let min_route = 2;
  let yincr = 0.8;
  let horizon_yincr_factor = 0.3;

  // when the reflection is not middle, should it be stretched?
  let yhorizon = height * 0.6;
  let horizon_factor_amp = height * 0.1;

  // store the high position on the mountains

  let mut height_map: Vec<f64> = Vec::new();

  let mut middle_level_map: Vec<f64> = Vec::new();

  let levels = 3;
  let mid_level = 1;
  for level in 0..levels {
    let seed = rng.gen_range(0.0, 100000.0);

    let level_p = level as f64 / (levels as f64 - 1.0);

    let ampfactor = mix(0.05, 0.2, level_p);
    let xfreq = mix(0.02, 0.01, level_p);
    let yfreq = 0.1;

    let stopy = mix(0.4, 0.25, level_p) * height;
    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height * 0.8;
    let mut miny = height;

    loop {
      if miny < stopy {
        break;
      }

      let mut route = Vec::new();
      let mut x = pad;
      let mut was_outside = true;
      let horizon_factor =
        (1.0 - (base_y - yhorizon).abs() / horizon_factor_amp).max(0.0);

      let amp = (1.0 - horizon_factor) * height * ampfactor;

      loop {
        if x > width - pad {
          break;
        }
        let xv = x;
        let mut y = base_y;

        y += -amp
          * perlin.get([
            //
            xv * xfreq + 9.9,
            y * yfreq - 3.1,
            77.
              + seed / 7.3
              + 0.8
                * perlin.get([
                  //
                  -seed * 7.3,
                  8.3 + xv * xfreq * 1.5,
                  y * 0.05,
                ]),
          ]);

        // shaking
        y += 0.01 * amp * perlin.get([xv * 0.1, y, 77. + seed / 0.3]);

        // TODO better mountains

        /*
        y += 2.0
          * amp
          * perlin.get([
            //
            0.3 * xv * xfreq,
            y * 0.01,
            7. + seed / 0.17,
          ]);
          */

        if y < miny {
          miny = y;
        }
        let mut collides = false;
        let xi = ((x - pad) / precision).round() as usize;
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
          && y < yhorizon; // TODO horizon to not strictly be in the middle, maybe small y displacement?

        // TODO we should count only once the same passage position
        if inside && passage.get((x, y)) < passage_threshold {
          if was_outside {
            if route.len() > min_route {
              reflectables.push((0, route.clone()));
              routes.push((0, rdp(&route, 0.1)));
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
        reflectables.push((0, route.clone()));
        routes.push((0, rdp(&route, 0.1)));
      }

      base_y -= mix(
        yincr,
        horizon_yincr_factor * yincr,
        horizon_factor.powf(2.0),
      );
    }

    if level == mid_level {
      middle_level_map = height_map.clone();
    }
  }

  // TODO IDEA: Forest?

  // TODO IDEA: Harbour?

  let mut castle_positions = vec![];

  // DEFENDERS
  // calculate a moving average
  let smooth = 40;
  let sf = smooth as f64;
  let mut sum = 0.0;
  let mut acc = Vec::new();
  let mut smooth_heights: Vec<(f64, f64, f64)> = Vec::new();
  for (i, h) in height_map.iter().enumerate() {
    if acc.len() == smooth {
      let avg = sum / sf;
      let xtheoric = pad + (i as f64 - sf / 2.0) * precision;

      let l = smooth_heights.len();
      let b = (xtheoric, avg, 0.0);
      let a = if l > 2 { smooth_heights[l - 2] } else { b };
      let rot = -PI / 2.0 + (b.0 - a.0).atan2(b.1 - a.1);
      let p = (xtheoric, avg, rot);
      smooth_heights.push(p);
      let prev = acc.remove(0);
      sum -= prev;
    }
    acc.push(h);
    sum += h;
  }

  smooth_heights.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

  let sizebase = rng.gen_range(20.0, 25.0);
  let castle_target = 1;
  // (1.5 + rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0)) as usize;
  let mut castles = Vec::new();
  let mut ranges = Vec::new();
  let mut i = 0;
  loop {
    if i > smooth_heights.len() * 2 {
      break;
    }
    if castles.len() >= castle_target {
      break;
    }
    let highest = smooth_heights[i % smooth_heights.len()];
    i += rng.gen_range(1, 11);
    let x = highest.0;
    let mut w = sizebase + rng.gen_range(-6.0, 10.0) * rng.gen_range(0.3, 1.0);
    let scale = w / 28.0;
    w += rng.gen_range(0.0, 20.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
    let left = x - w / 2.0 - 2.0;
    let right = x + w / 2.0 + 2.0;
    if left < pad + 4.0 || right > width - pad - 4.0 {
      continue;
    }

    if ranges.iter().any(|&(a, b)| {
      a < left && left < b
        || a < right && right < b
        || left < a && a < right
        || left < b && b < right
    }) {
      continue;
    }

    let mut peaks = Vec::new();
    let divisions = 10;
    for i in 0..divisions {
      let px = x + w * ((i as f64) / (divisions - 1) as f64 - 0.5);
      let py = height_map[((px - pad) / precision) as usize % height_map.len()];
      if py > height - pad - 5.0 {
        continue;
      }
      peaks.push((px, py, 0.0));
    }

    ranges.push((left, right));
    let (rts, polygons, center) = castle(&peaks, scale, &mut rng, &mut passage);
    for poly in polygons {
      mask.paint_polygon(&poly);
    }
    castle_positions.push(center);
    castles.push(rts);
  }

  for all in castles {
    for r in all {
      routes.push((0, r));
    }
  }

  // ATTACKERS

  // trebuchets
  for _i in 0..2 {
    // TODO figure out a flat area? instead of offset
    let yoffset = rng.gen_range(0.0, 20.0);

    let x = rng.gen_range(0.2, 0.8) * width;
    let y =
      yoffset + middle_level_map[((x - pad) / precision).round() as usize];

    let pos = (x, y);
    let h = mix(12.0, 20.0, smoothstep(0.2 * height, yhorizon, y));
    let action = rng.gen_range(0.0, 1.0);
    let xflip = false;

    let rts = trebuchet(&mut rng, pos, h, action, xflip);
    let colored = rts
      .iter()
      .map(|route| (0, route.clone()))
      .collect::<Vec<_>>();
    reflectables.extend(colored.clone());
    routes.extend(colored);

    // projectile
    if rng.gen_bool(0.5) {
      let castlecenter =
        castle_positions[rng.gen_range(0, castle_positions.len())];

      let curveh = 0.8 * (castlecenter.1 - pad);

      let mut destination = castlecenter;

      destination.0 += rng.gen_range(-0.5, 0.5) * 5.0;
      destination.1 += rng.gen_range(-0.5, 0.5) * 5.0;

      let position_percent = rng.gen_range(0.3, 0.9);

      let projectile = fireball_projectile(
        &mut rng,
        pos, // TODO use the actual trebuchet origin pos
        destination,
        curveh,
        2.0,
        position_percent,
      );

      routes.extend(projectile.clone());
      reflectables.extend(projectile);
    }
  }

  // bowmen are going to place themselves together down the mountain
  // spearman with shields in front

  let seed = rng.gen_range(-100.0, 100.0);
  let freq = 0.02;

  for _i in 0..rng.gen_range(100, 500) {
    // TODO figure out a flat area? instead of offset
    let yoffset = rng.gen_range(0.0, 50.0);

    // FIXME avoid going in the water please

    let x = rng.gen_range(0.05, 0.95) * width;
    let y =
      yoffset + middle_level_map[((x - pad) / precision).round() as usize];

    if y > yhorizon - 10.0 {
      continue;
    }

    if perlin.get([freq * x, freq * y, seed]) > 0.2 {
      continue;
    }

    let pos = (x, y);
    let size = 5.0;
    let mut rts = bowman(&mut rng, pos, size);

    let castlecenter =
      castle_positions[rng.gen_range(0, castle_positions.len())];

    let curveh = 0.6 * (castlecenter.1 - pad);

    let mut destination = castlecenter;

    destination.0 += rng.gen_range(-0.5, 0.5) * 5.0;
    destination.1 += rng.gen_range(-0.5, 0.5) * 5.0;

    let position_percent = rng.gen_range(0.1, 0.9) * rng.gen_range(0.5, 1.0);

    let projectile =
      arrow_projectile(pos, destination, curveh, 2.0, position_percent);

    routes.extend(projectile.clone());
    reflectables.extend(projectile);

    let colored = rts
      .iter()
      .map(|route| (0, route.clone()))
      .collect::<Vec<_>>();
    reflectables.extend(colored.clone());
    routes.extend(colored);
  }

  // MAKE THE SKY

  // EAGLES
  passage.grow_passage(4.0);

  let in_sky_area = |p: (f64, f64)| -> bool {
    p.1 < yhorizon && passage.get(p) == 0 && strictly_in_boundaries(p, bounds)
  };

  let does_overlap = |c: &VCircle| {
    in_sky_area((c.x, c.y))
      && circle_route((c.x, c.y), c.r, 8)
        .iter()
        .all(|&p| in_sky_area(p))
  };

  let count = rng.gen_range(0, 10);

  let eagle_circles = packing(
    &mut rng,
    100000,
    count,
    1,
    2.0,
    bounds,
    &does_overlap,
    3.0,
    4.0,
  );

  for c in eagle_circles.iter() {
    let rot = 0.2 * rng.gen_range(-PI, PI);
    let xreverse = rng.gen_bool(0.5);
    routes.extend(
      eagle((c.x, c.y), c.r, rot, xreverse, &mut rng, &mut passage)
        .iter()
        .map(|route| (0, route.clone())),
    );
  }

  // CLOUDS SHAPES

  passage.grow_passage(1.0);

  let in_shape = |p: (f64, f64)| -> bool {
    passage.get(p) == 0 && strictly_in_boundaries(p, bounds)
  };

  let does_overlap = |c: &VCircle| {
    c.y < yhorizon
      && in_shape((c.x, c.y))
      && circle_route((c.x, c.y), c.r, 8)
        .iter()
        .all(|&p| in_shape(p))
  };

  // FIXME figure out the right number of clouds
  // i think it's good to have a lot but have areas of exclusions?
  // notably we should have wider padding around the castle and mountains

  let count = rng.gen_range(0, 2);
  let layers_count = rng.gen_range(1, 4);

  let circles = (0..layers_count)
    .flat_map(|_i| {
      packing(
        &mut rng,
        50000,
        count,
        1,
        0.0,
        bounds,
        &does_overlap,
        5.0,
        20.0,
      )
    })
    .collect::<Vec<_>>();

  let clouds: Vec<VCircle> = circles
    .iter()
    .flat_map(|c| {
      let (rts, circles) = cloud_in_circle(&mut rng, &c);
      let is_outside = |p| mask.is_painted(p);
      let rts = clip_routes(&rts, &is_outside, 0.3, 7);
      routes.extend(rts.iter().map(|r| (0, r.clone())));
      for c in circles.clone() {
        mask.paint_circle(&c);
      }
      circles
    })
    .collect();

  // CLOUDS LINES

  // TODO clip the clouds shapes

  let cloud_add = rng.gen_range(-0.7, 1.8) * rng.gen_range(0.0, 1.0);
  let seed = rng.gen_range(0.0, 100000.0);

  let dy = 1.0;
  let dx = 1.0;
  let thresholds = vec![0.0, 0.4, 0.2, 0.6];
  let mut y = pad;
  let mut yi = 0;
  loop {
    let mut route = Vec::new();
    if y > yhorizon {
      break;
    }
    let ythreshold = thresholds[yi % thresholds.len()];

    let mut x = pad;
    loop {
      if x > width - pad {
        break;
      }

      let xi = ((x - pad) / precision) as usize;
      let lowy = height_map[xi];

      let p = (x, y);

      let should_draw = !mask.is_painted(p)
        && (0.2 * cloud_add
          + 0.7
            * perlin.get([
              seed
                + perlin.get([
                  //
                  0.02 * x,
                  7.7 * seed,
                  0.01 * y,
                ]),
              0.005 * x,
              0.02 * y,
            ])
          + 0.3 * perlin.get([0.002 * x, 0.1 * y, seed / 5.]))
          * (0.5 * cloud_add
            + 2.0 * perlin.get([-3. - seed / 7., 0.001 * x, 0.001 * y]))
          * (cloud_add + perlin.get([5. * seed / 7., 0.004 * x, 0.004 * y]))
          > ythreshold
            + 0.001 * (route.len() as f64)
            + 0.3 * smoothstep(0.0, yhorizon, y);

      if y < lowy - 1.0 && should_draw {
        route.push(p);
      } else {
        if route.len() > 1 {
          routes.push((0, route));
        }
        route = Vec::new();
      }

      x += dx;
    }

    if route.len() > 1 {
      routes.push((0, route));
    }

    y += dy;
    yi += 1;
  }

  // PLACE THE SUN

  let max_sun_radius: f64 = width * 30. / 210.;
  let sun_density = 12.;

  // find the lowest point of the mountain
  let mut lowxi = 0;
  let mut lowy = 0.0;
  let padend = (2. * (pad / precision)) as usize;
  let padxi = (height_map.len() as f64 * rng.gen_range(0.2, 0.4)) as usize;
  for xi in padxi..(height_map.len() - padend - padxi) {
    let y = height_map[xi];
    if y > lowy {
      lowy = y;
      lowxi = xi;
    }
  }
  let lowx = pad + lowxi as f64 * precision;

  let radius = max_sun_radius * rng.gen_range(0.2, 1.0);
  let sunset_position = rng.gen_range(-2.0, 0.5) * radius;
  let center = (lowx, lowy + sunset_position);

  if radius > 5.0 {
    let mut route = Vec::new();
    let spins = sun_density;
    let mut rbase = radius + 0.5;
    let mut a: f64 = 0.0;
    loop {
      if rbase < 0.05 {
        break;
      }
      let r = rbase.min(radius);
      let aincr = precision / (r + 1.0);
      let rincr = (0.9 * aincr) / spins;
      let p = (center.0 + r * a.cos(), center.1 + r * a.sin());
      let xi = ((p.0 - pad) / precision) as usize;
      let h = height_map[xi];
      // TODO we want to do proper clipping instead of stopping the spiral in place.
      // FIXME the sun clipping logic is not accurate enough
      // TODO we must use a mask to hide the sun behind clouds

      let collides = p.1 < h && !mask.is_painted(p);

      if collides {
        route.push(p);
      } else {
        if route.len() > 1 {
          reflectables.push((1, route.clone()));
          routes.push((1, route));
        }
        route = Vec::new();
      }
      rbase -= rincr;
      a += aincr;
    }

    if route.len() > 1 {
      reflectables.push((1, route.clone()));
      routes.push((1, route));
    }
  }

  // TODO random rays when sun light traverse sky?

  // TODO we need to clip the reflections away from the boat
  for _i in 0..rng.gen_range(1, 3) {
    let close_to_horizon: f64 = rng.gen_range(0.0, 0.7);
    let size = mix(10.0, 5.0, close_to_horizon);
    let xflip = false; //rng.gen_bool(0.5);
    let w = width * size / rng.gen_range(20.0, 50.0);
    let origin = (
      width / 2.0 + rng.gen_range(-1.0, 1.0) * 0.2 * width,
      height * mix(0.9, 0.55, close_to_horizon),
    );

    routes.extend(
      boat_with_army(&mut rng, origin, 0.0, size, w, xflip)
        .iter()
        .map(|route| (0, route.clone())),
    );
  }

  // make the title. TODO find the best place for it / draw it first with clipping.

  let txt = epic_title(&mut rng);
  let fontsize = width / 28.0;
  let iterations = 500000;
  let density = 4.0;
  // TODO make the filling having more resolution
  routes.extend(draw_font(
    &mut rng,
    &mut font,
    &mut mask,
    fontsize,
    (
      pad + pad + 0.5 * fontsize,
      height - pad - pad - 1.5 * fontsize,
    ),
    txt.as_str(),
    0,
    iterations,
    density,
  ));

  // REFLECT THE OBJECTS ON THE SEA

  // TODO clipping of the boat and the reflections. how to implement? strict clipping VS halo effect? => boat to export masks and then we can clip reflections?
  routes.extend(reflect_shapes(
    &mut rng,
    &reflectables,
    &mut passage,
    0.5,
    3.0,
    yhorizon,
    bounds,
    5,
  ));

  // Infer the features from the generated pieces

  let colors_count = colors.len();
  let mut color_presence = vec![false; colors_count];
  for (i, _) in routes.iter() {
    color_presence[*i] = true;
  }
  let mut inks = vec![];
  for (i, &present) in color_presence.iter().enumerate() {
    if present && !inks.contains(&colors[i].0) {
      inks.push(colors[i].0);
    }
  }

  inks.sort();
  let inks_length = inks.len();

  let feature = Feature {
    inks: inks.join(", "),
    inks_count: inks_length,
    paper: paper.0.to_string(),
  };

  let feature_json = serde_json::to_string(&feature).unwrap();
  let palette_json = serde_json::to_string(&Palette {
    paper,
    primary: colors[0],
    secondary: colors[1],
    third: colors[2],
  })
  .unwrap();

  let mask_colors = vec!["#0FF", "#F0F", "#FF0"];

  // TODO optimise lines with rdp(&route, 0.1)

  let layers = make_layers(
    colors
      .iter()
      .enumerate()
      .map(|(i, c)| {
        (
          if mask_mode { mask_colors[i] } else { c.1 },
          c.0.to_string(),
          c.3,
          routes
            .iter()
            .filter_map(
              |(ci, routes)| {
                if *ci == i {
                  Some(routes.clone())
                } else {
                  None
                }
              },
            )
            .collect(),
        )
      })
      .collect(),
  );

  let mut document = svg::Document::new()
    .set(
      "data-credits",
      "@greweb - 2023 - Plottable Era (II) Medieval".to_string(),
    )
    .set("data-hash", opts.hash.to_string())
    .set("data-traits", feature_json)
    .set("data-palette", palette_json)
    .set("viewBox", (0, 0, width, height))
    .set("width", format!("{}mm", width))
    .set("height", format!("{}mm", height))
    .set(
      "style",
      if mask_mode {
        "background:white".to_string()
      } else {
        format!("background:{}", paper.1)
      },
    )
    .set(
      "xmlns:inkscape",
      "http://www.inkscape.org/namespaces/inkscape",
    )
    .set("xmlns", "http://www.w3.org/2000/svg");
  for l in layers {
    document = document.add(l);
  }

  (document, feature)
}

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

fn rng_from_fxhash(hash: &String) -> impl Rng {
  let mut bs = [0; 32];
  bs58::decode(hash.chars().skip(2).take(43).collect::<String>())
    .into(&mut bs)
    .unwrap();
  let rng = StdRng::from_seed(bs);
  return rng;
}

fn make_layers(
  data: Vec<(&str, String, f64, Vec<Vec<(f64, f64)>>)>,
) -> Vec<Group> {
  let layers: Vec<Group> = data
    .iter()
    .filter(|(_color, _label, _stroke_width, routes)| routes.len() > 0)
    .enumerate()
    .map(|(ci, (color, label, stroke_width, routes))| {
      let mut l = Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", format!("{} {}", ci, label.clone()))
        .set("fill", "none")
        .set("stroke", color.clone())
        .set("stroke-linecap", "round")
        .set("stroke-width", *stroke_width);
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
  layers
}

#[inline]
fn euclidian_dist((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
  let dx = x1 - x2;
  let dy = y1 - y2;
  return (dx * dx + dy * dy).sqrt();
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

// adapted from library "ramer_douglas_peucker"
/// Given a set of points and an epsilon, returns a list of indexes to keep.
/// If the first and last points are the same, then the points are treated as a closed polygon
pub fn rdp(points: &Vec<(f64, f64)>, epsilon: f64) -> Vec<(f64, f64)> {
  if points.len() < 3 {
    return points.clone();
  }
  let mut ranges = Vec::<RangeInclusive<usize>>::new();

  let mut results = Vec::new();
  results.push(0); // We always keep the starting point

  // Set of ranges to work through
  ranges.push(0..=points.len() - 1);

  while let Some(range) = ranges.pop() {
    let range_start = *range.start();
    let range_end = *range.end();

    let start = points[range_start];
    let end = points[range_end];

    // Caches a bit of the calculation to make the loop quicker
    let line = LineDistance::new(start, end);

    let (max_distance, max_index) =
      points[range_start + 1..range_end].iter().enumerate().fold(
        (0.0_f64, 0),
        move |(max_distance, max_index), (index, &point)| {
          let distance = match line.to(point) {
            Some(distance) => distance,
            None => {
              let base = point.0 - start.0;
              let height = point.1 - start.1;
              base.hypot(height)
            }
          };

          if distance > max_distance {
            // new max distance!
            // +1 to the index because we start enumerate()ing on the 1st element
            return (distance, index + 1);
          }

          // no new max, pass the previous through
          (max_distance, max_index)
        },
      );

    // If there is a point outside of epsilon, subdivide the range and try again
    if max_distance > epsilon {
      // We add range_start to max_index because the range needs to be in
      // the space of the whole vector and not the range
      let division_point = range_start + max_index;

      let first_section = range_start..=division_point;
      let second_section = division_point..=range_end;

      // Process the second one first to maintain the stack
      // The order of ranges and results are opposite, hence the awkwardness
      let should_keep_second_half = division_point - range_start > 2;
      if should_keep_second_half {
        ranges.push(second_section);
      }

      if division_point - range_start > 2 {
        ranges.push(first_section);
      } else {
        results.push(division_point);
      }

      if !should_keep_second_half {
        results.push(range_end);
      }
    } else {
      // Keep the end point for the results
      results.push(range_end);
    }
  }

  results.iter().map(|&i| points[i]).collect()
}

// adapted from "legasea_line"
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LineDistance {
  a: f64,
  b: f64,
  c: f64,
  pub length: f64,
}

impl LineDistance {
  pub fn new(p1: (f64, f64), p2: (f64, f64)) -> Self {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let a = y2 - y1;
    let b = x2 - x1;
    let c = (x2 * y1) - (y2 * x1);
    let length = euclidian_dist(p1, p2);
    Self { a, b, c, length }
  }
  pub fn to(&self, point: (f64, f64)) -> Option<f64> {
    let Self { a, b, c, length } = self;
    if 0.0 == *length {
      None
    } else {
      // https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Line_defined_by_two_points
      Some(((a * point.0) - (b * point.1) + c).abs() / length)
    }
  }
}

fn reflect_shapes<R: Rng>(
  rng: &mut R,
  reflectables: &Vec<(usize, Vec<(f64, f64)>)>,
  // TODO use passage to not have too much density
  passage: &mut Passage,
  probability: f64,
  stroke_len_base: f64,
  ycenter: f64,
  boundaries: (f64, f64, f64, f64),
  max_passage: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut new_shapes = Vec::new();

  let min_stroke_length = 0.5 * stroke_len_base;
  let max_stroke_length = stroke_len_base;
  let xdisplacement = 30.0;
  let ydisplacement = 70.0;

  let exact_balance = 0.9;

  for (clr, route) in reflectables.clone() {
    for route in
      slice_polylines(&route, rng.gen_range(1.0, 2.0) * stroke_len_base)
    {
      if !rng.gen_bool(exact_balance * probability) {
        continue;
      }
      let dispy = rng.gen_range(0.0, 0.2 * ydisplacement)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
      let projection = route
        .iter()
        .map(|p| {
          let x = p.0;
          let mut y = 2.0 * ycenter - p.1;
          y += dispy;
          (x, y)
        })
        .collect();
      new_shapes.push((clr, projection));
    }

    for p in route {
      if !rng.gen_bool((1.0 - exact_balance) * probability) {
        continue;
      }
      let sx = (min_stroke_length
        + (max_stroke_length - min_stroke_length)
          * rng.gen_range(0f64, 1.0).powi(2))
        / 2.0;
      let sy = 0.3 * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0);
      let x = p.0
        + rng.gen_range(0.0, xdisplacement)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(-1.0, 1.0);
      let y = 2.0 * ycenter - p.1
        + rng.gen_range(0.0, ydisplacement)
          * rng.gen_range(0.5, 1.0)
          * rng.gen_range(-1.0, 1.0);
      if y > ycenter && y < boundaries.3 {
        let x1 = (x - sx).max(boundaries.0).min(boundaries.2);
        let x2 = (x + sx).max(boundaries.0).min(boundaries.2);
        if x2 - x1 > min_stroke_length {
          // TODO do it with as many point as needed between x1 and x2, if any of these have too much passage, we skip
          if passage.get((x, y)) > max_passage {
            continue;
          }
          passage.count((x, y));
          new_shapes.push((clr, vec![(x1, y - sy), (x2, y + sy)]));
        }
      }
    }
  }
  new_shapes
}

#[derive(Clone, Copy)]
pub struct HumanJointAngles {
  body_angle: f64,
  head_angle: f64,
  // shoulders (left, right)
  shoulder_right_angle: f64,
  shoulder_left_angle: f64,
  // elbows (left, right)
  elbow_right_angle: f64,
  elbow_left_angle: f64,
  // hips
  hip_right_angle: f64,
  hip_left_angle: f64,
  // knees (left, right)
  knee_right_angle: f64,
  knee_left_angle: f64,

  left_arm_bend: f64,
  left_leg_bend: f64,
  right_arm_bend: f64,
  right_leg_bend: f64,
}

#[derive(Clone, Copy)]
pub struct HumanBody {
  joints: HumanJointAngles,
  height: f64,
  hip: (f64, f64),
  shoulder: (f64, f64),
  shoulder_right: (f64, f64),
  shoulder_left: (f64, f64),
  elbow_right: (f64, f64),
  elbow_left: (f64, f64),
  hip_right: (f64, f64),
  hip_left: (f64, f64),
  knee_right: (f64, f64),
  knee_left: (f64, f64),
  head: (f64, f64),
}

impl HumanBody {
  pub fn head_pos_angle(&self) -> ((f64, f64), f64) {
    (self.head, self.joints.head_angle)
  }
  pub fn hand_left_pos_angle(&self) -> ((f64, f64), f64) {
    (self.elbow_left, self.joints.elbow_left_angle)
  }
  pub fn hand_right_pos_angle(&self) -> ((f64, f64), f64) {
    (self.elbow_right, self.joints.elbow_right_angle)
  }
  pub fn foot_left_pos_angle(&self) -> ((f64, f64), f64) {
    (self.knee_left, self.joints.knee_left_angle)
  }
  pub fn foot_right_pos_angle(&self) -> ((f64, f64), f64) {
    (self.knee_right, self.joints.knee_right_angle)
  }
  pub fn get_size(&self) -> f64 {
    self.height
  }

  pub fn new(
    origin: (f64, f64),
    height: f64,
    joints: HumanJointAngles,
  ) -> Self {
    let h = height;
    let j = joints;
    let mut hip = origin;

    // TODO how to position the origin properly?
    hip.1 -= 0.5 * h;

    let shoulder = proj_point(hip, j.body_angle, 0.4 * h);

    let shoulder_right =
      proj_point(shoulder, j.shoulder_right_angle, j.right_arm_bend * 0.3 * h);
    let shoulder_left =
      proj_point(shoulder, j.shoulder_left_angle, j.left_arm_bend * 0.3 * h);

    let elbow_right = proj_point(
      shoulder_right,
      j.elbow_right_angle,
      j.right_arm_bend * 0.3 * h,
    );
    let elbow_left =
      proj_point(shoulder_left, j.elbow_left_angle, j.left_arm_bend * 0.3 * h);

    let hip_right =
      proj_point(hip, j.hip_right_angle, j.right_leg_bend * 0.3 * h);
    let hip_left = proj_point(hip, j.hip_left_angle, j.left_leg_bend * 0.3 * h);

    let knee_right =
      proj_point(hip_right, j.knee_right_angle, j.right_leg_bend * 0.3 * h);
    let knee_left =
      proj_point(hip_left, j.knee_left_angle, j.left_leg_bend * 0.3 * h);

    let head = proj_point(shoulder, j.head_angle, 0.3 * h);

    Self {
      joints,
      height,
      hip,
      shoulder,
      shoulder_right,
      shoulder_left,
      elbow_right,
      elbow_left,
      hip_right,
      hip_left,
      knee_right,
      knee_left,
      head,
    }
  }

  fn render(&self) -> Vec<Vec<(f64, f64)>> {
    let mut routes = Vec::new();
    let hip = self.hip;
    let shoulder = self.shoulder;
    let shoulder_right = self.shoulder_right;
    let shoulder_left = self.shoulder_left;
    let elbow_right = self.elbow_right;
    let elbow_left = self.elbow_left;
    let hip_right = self.hip_right;
    let hip_left = self.hip_left;
    let knee_right = self.knee_right;
    let knee_left = self.knee_left;
    let head = self.head;

    routes.push(vec![hip, shoulder, head]);

    routes.push(vec![shoulder, shoulder_right, elbow_right]);
    routes.push(vec![shoulder, shoulder_left, elbow_left]);

    routes.push(vec![hip, hip_right, knee_right]);
    routes.push(vec![hip, hip_left, knee_left]);

    routes
  }
}

fn helmet(
  origin: (f64, f64),
  angle: f64,
  size: f64,
  xreverse: bool,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = 0.4 * size;

  // head
  routes.push(vec![(-dx, 0.0), (-dx, -h), (dx, -h), (dx, 0.0), (-dx, 0.0)]);

  routes.push(vec![
    (-dx, -h * 0.7),
    (-dx, -h * 0.8),
    (dx, -h * 0.8),
    (dx, -h * 0.7),
    (-dx, -h * 0.7),
  ]);

  // TODO implement

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&(x, y)| {
          let x = if xreverse { -x } else { x };
          let (x, y) = p_r((x, y), ang);
          (x + origin.0, y + origin.1)
        })
        .collect()
    })
    .collect()
}

fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
}

trait MonochromeStrokable {
  fn render(&self) -> Vec<Vec<(f64, f64)>>;
}

trait PointCheckable {
  fn includes_point(&self, point: (f64, f64)) -> bool;
}

#[derive(Clone)]
struct StrokesWithPolygonsBound {
  strokes: Vec<Vec<(f64, f64)>>,
  polygons: Vec<Vec<(f64, f64)>>,
}

impl StrokesWithPolygonsBound {
  fn new(
    strokes: Vec<Vec<(f64, f64)>>,
    polygons: Vec<Vec<(f64, f64)>>,
  ) -> Self {
    Self { strokes, polygons }
  }
}

impl MonochromeStrokable for StrokesWithPolygonsBound {
  fn render(&self) -> Vec<Vec<(f64, f64)>> {
    self.strokes.clone()
  }
}

fn polygon_includes_point(
  polygon: &Vec<(f64, f64)>,
  point: (f64, f64),
) -> bool {
  let mut c = false;
  for i in 0..polygon.len() {
    let j = (i + 1) % polygon.len();
    if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
      && (point.0
        < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
          / (polygon[j].1 - polygon[i].1)
          + polygon[i].0)
    {
      c = !c;
    }
  }
  c
}

// TODO more efficient algorithm would be to paint on a mask.
struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
}

impl PaintMask {
  fn clone_empty(&self) -> Self {
    Self {
      mask: vec![false; self.mask.len()],
      precision: self.precision,
      width: self.width,
      height: self.height,
    }
  }

  fn new(precision: f64, width: f64, height: f64) -> Self {
    let wi = (width / precision) as usize;
    let hi = (height / precision) as usize;
    Self {
      mask: vec![false; wi * hi],
      width,
      height,
      precision,
    }
  }

  fn is_painted(&self, point: (f64, f64)) -> bool {
    // check out of bounds
    if point.0 <= 0.0
      || point.0 >= self.width
      || point.1 <= 0.0
      || point.1 >= self.height
    {
      return false;
    }
    let precision = self.precision;
    let width = self.width;
    let x = (point.0 / precision) as usize;
    let y = (point.1 / precision) as usize;
    let wi = (width / precision) as usize;
    self.mask[x + y * wi]
  }

  fn paint_rectangle(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
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

  fn paint_borders(&mut self, pad: f64) {
    self.paint_rectangle(0., 0., self.width, pad);
    self.paint_rectangle(0., 0., pad, self.height);
    self.paint_rectangle(0., self.height - pad, self.width, self.height);
    self.paint_rectangle(self.width - pad, 0., self.width, self.height);
  }

  fn paint_polygon(&mut self, polygon: &Vec<(f64, f64)>) {
    let (minx, miny, maxx, maxy) = polygon_bounds(polygon);
    let precision = self.precision;
    let width = self.width;
    let minx = (minx / precision) as usize;
    let miny = (miny / precision) as usize;
    let maxx = (maxx / precision) as usize;
    let maxy = (maxy / precision) as usize;
    let wi = (width / precision) as usize;
    for x in minx..maxx {
      for y in miny..maxy {
        let point = (x as f64 * precision, y as f64 * precision);
        if polygon_includes_point(polygon, point) {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  fn paint_circle(&mut self, circle: &VCircle) {
    let (minx, miny, maxx, maxy) = (
      circle.x - circle.r,
      circle.y - circle.r,
      circle.x + circle.r,
      circle.y + circle.r,
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
        let point = (x as f64 * precision, y as f64 * precision);
        if euclidian_dist(point, (circle.x, circle.y)) < circle.r {
          self.mask[x + y * wi] = true;
        }
      }
    }
  }

  fn paint_polyline(&mut self, polyline: &Vec<(f64, f64)>, strokew: f64) {
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
        let point = (x as f64 * precision, y as f64 * precision);
        for i in 0..polyline.len() - 1 {
          let a = polyline[i];
          let b = polyline[i + 1];
          if sd_segment(point, a, b) < strokew {
            self.mask[x + y * wi] = true;
            break;
          }
        }
      }
    }
  }

  fn paint_pixels(
    &mut self,
    topleft: (f64, f64),
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
}

fn polygon_bounds(polygon: &Vec<(f64, f64)>) -> (f64, f64, f64, f64) {
  let mut minx = f64::MAX;
  let mut miny = f64::MAX;
  let mut maxx = f64::MIN;
  let mut maxy = f64::MIN;
  for &(x, y) in polygon {
    minx = minx.min(x);
    miny = miny.min(y);
    maxx = maxx.max(x);
    maxy = maxy.max(y);
  }
  (minx, miny, maxx, maxy)
}

// TODO we can optim something as we just need a "point_in_segment"

fn sd_segment(
  (px, py): (f64, f64),
  (ax, ay): (f64, f64),
  (bx, by): (f64, f64),
) -> f64 {
  let pa_x = px - ax;
  let pa_y = py - ay;
  let ba_x = bx - ax;
  let ba_y = by - ay;

  let dot_pa_ba = pa_x * ba_x + pa_y * ba_y;
  let dot_ba_ba = ba_x * ba_x + ba_y * ba_y;
  let h = (dot_pa_ba / dot_ba_ba).max(0.0).min(1.0);

  let h_x = ba_x * h;
  let h_y = ba_y * h;

  ((pa_x - h_x) * (pa_x - h_x) + (pa_y - h_y) * (pa_y - h_y)).sqrt()
}

impl PointCheckable for StrokesWithPolygonsBound {
  fn includes_point(&self, point: (f64, f64)) -> bool {
    self
      .polygons
      .iter()
      .any(|polygon| polygon_includes_point(polygon, point))
  }
}

fn route_translate_rotate(
  route: &Vec<(f64, f64)>,
  origin: (f64, f64),
  angle: f64,
) -> Vec<(f64, f64)> {
  route
    .iter()
    .map(|&(x, y)| {
      let (x, y) = p_r((x, y), angle);
      (x + origin.0, y + origin.1)
    })
    .collect()
}

fn shield<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  size: f64,
  angle: f64,
  shape1: f64,
  shape2: f64,
) -> StrokesWithPolygonsBound {
  let mut routes = Vec::new();
  let dx = 0.2 * size;
  let dy = 0.4 * size;
  let mut route = vec![];
  let mut route2 = vec![];
  for v in vec![
    (0.0, -dy),
    (0.5 * dx, -dy),
    (dx, -(1.0 - shape1 * shape1) * dy),
    (dx, 0.0),
    (dx, shape2 * dy),
    (0.0, dy),
  ] {
    route.push(v);
    route2.push((-v.0, v.1));
  }
  route2.reverse();
  route.extend(route2);

  route = route_translate_rotate(&route, origin, angle);
  let polygons = vec![route.clone()];
  routes.push(route);

  let tick = rng.gen_range(0.2, 0.3);
  let y = rng.gen_range(-0.2, 0.0) * dy;
  routes.push(route_translate_rotate(
    &vec![(0.0, -tick * dy + y), (tick * dx, y), (0.0, tick * dy + y)],
    origin,
    angle,
  ));

  StrokesWithPolygonsBound::new(routes, polygons)
}

fn proj_point(origin: (f64, f64), angle: f64, distance: f64) -> (f64, f64) {
  let (x, y) = origin;
  let s = angle.sin();
  let c = angle.cos();
  (x + distance * c, y + distance * s)
}

fn boat_with_army<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  angle: f64,
  size: f64, // reference size (height of the boat)
  w: f64,
  xflip: bool,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = vec![];
  let xdir = if xflip { -1.0 } else { 1.0 };

  let h = size;
  let x1 = -w * rng.gen_range(0.3, 0.45);
  let x2 = w * rng.gen_range(0.3, 0.4);
  let yleft = -h * rng.gen_range(0.6, 1.0);
  let yright = -h * rng.gen_range(0.8, 1.0);

  let dy_edge = 0.3;
  // boat bottom
  let mut route = Vec::new();
  route.push((-w / 2.0 - dy_edge, yleft + dy_edge));
  route.push((x1, 0.0));
  route.push((x2, 0.0));
  route.push((w / 2.0 + dy_edge, yright + dy_edge));
  route = path_subdivide_to_curve(route, 2, 0.8);
  routes.push(route);

  // boat in between
  let mut route = Vec::new();
  let y = -0.15 * h;
  route.push((-w / 2.0, yleft));
  route.push((x1, y));
  route.push((x2, y));
  route.push((w / 2.0, yright));
  route = path_subdivide_to_curve(route, 2, 0.8);
  // TODO route will be used to clip people
  routes.push(route);

  // boat top
  let mut route = Vec::new();
  let y = -0.3 * h;
  route.push((-w / 2.0 + dy_edge, yleft - dy_edge));
  route.push((x1, y));
  route.push((x2, y));
  route.push((w / 2.0 - dy_edge, yright - dy_edge));
  route = path_subdivide_to_curve(route, 2, 0.8);
  // TODO route will be used to clip people
  routes.push(route.clone());
  let boat_top = route;

  // make a boat head
  let o = (w / 2.0, yright);
  let mut route = vec![];
  for _i in 0..8 {
    let angle = rng.gen_range(-PI, PI);
    let amp = rng.gen_range(0.1, 0.2) * size;
    route.push((o.0 + amp * angle.cos(), o.1 + amp * angle.sin()));
  }
  route.push(route[0]);
  routes.push(route);

  // humans

  let mut foreground_routes = Vec::new();
  let mask_origin = (3.0 * w, 3.0 * h);
  let mut foreground_mask =
    PaintMask::new(0.5, 2.0 * mask_origin.0, 2.0 * mask_origin.1);

  let shape1 = rng.gen_range(0.0, 1.0);
  let shape2 = rng.gen_range(0.0, 1.0);
  let mut x = x1;
  while x < x2 {
    let joints = HumanJointAngles {
      body_angle: -PI / 2.0,
      head_angle: -PI / 2.0,
      shoulder_right_angle: rng.gen_range(0.0, PI / 4.0),
      shoulder_left_angle: rng.gen_range(3.0 * PI / 4.0, PI),
      elbow_right_angle: 0.3,
      elbow_left_angle: PI / 2.0 + 0.3,
      hip_right_angle: PI / 2.0 - 0.5,
      hip_left_angle: PI / 2.0 + 0.5,
      knee_right_angle: PI / 2.0,
      knee_left_angle: PI / 2.0,

      left_arm_bend: 0.5,
      right_arm_bend: 0.4,
      left_leg_bend: 1.0,
      right_leg_bend: 1.0,
    };
    let humansize = size * 0.5;
    let y = rng.gen_range(-0.1 * size, 0.0);
    let human = HumanBody::new((x, y), humansize, joints);

    let human_body = human.render();
    // clip human body with boat top
    let is_outside = |p| {
      let (x, y) = p;
      let mut inside = false;
      for i in 0..boat_top.len() - 1 {
        let (x1, y1) = boat_top[i];
        let (x2, y2) = boat_top[i + 1];
        if (y1 < y && y2 > y) || (y1 > y && y2 < y) {
          let x3 = x1 + (x2 - x1) * (y - y1) / (y2 - y1);
          if x3 < x {
            inside = !inside;
          }
        }
      }
      !inside
    };
    let human_body = clip_routes(&human_body, &is_outside, 1.0, 6);

    routes.extend(human_body);

    // stick
    let angle = -PI * rng.gen_range(0.3, 0.4);
    let amp1 = -0.4 * size;
    let amp2 = rng.gen_range(0.4, 0.8) * size;
    let stick = vec![
      (x + amp1 * angle.cos(), y + amp1 * angle.sin()),
      (x + amp2 * angle.cos(), y + amp2 * angle.sin()),
    ];
    routes.push(stick);

    let (headpos, headangle) = human.head_pos_angle();
    let h = helmet(headpos, headangle, humansize, false);
    routes.extend(h);

    let shield_p = human.elbow_right;

    let s = shield(rng, shield_p, size * 0.6, 0.0, shape1, shape2);

    let is_colliding_shield = |point: (f64, f64)| s.includes_point(point);

    foreground_routes =
      clip_routes(&foreground_routes, &is_colliding_shield, 1.0, 5);

    foreground_routes.extend(s.render());

    for poly in s.polygons.iter() {
      foreground_mask.paint_polygon(
        &poly
          .iter()
          .map(|p| {
            let (x, y) = p;
            let x = x + mask_origin.0;
            let y = y + mask_origin.1;
            (x, y)
          })
          .collect::<Vec<_>>(),
      );
    }

    let has_foreground = |p: (f64, f64)| {
      foreground_mask.is_painted((p.0 + mask_origin.0, p.1 + mask_origin.1))
    };

    routes = clip_routes(&routes, &has_foreground, 1.0, 5);

    x += rng.gen_range(0.15, 0.25) * size;
  }

  routes.extend(foreground_routes.clone());

  // translate routes
  routes = routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&(x, y)| {
          let x = xdir * x;
          let (x, y) = p_r((x, y), angle);
          (x + origin.0, y + origin.1)
        })
        .collect()
    })
    .collect();
  routes
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

#[inline]
fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
  let k = ((x - a) / (b - a)).max(0.0).min(1.0);
  return k * k * (3.0 - 2.0 * k);
}

fn clip_routes(
  input_routes: &Vec<Vec<(f64, f64)>>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<Vec<(f64, f64)>> {
  // locate the intersection where inside and outside cross
  let search = |inside: (f64, f64), outside: (f64, f64), n| {
    let mut a = inside;
    let mut b = outside;
    for _i in 0..n {
      let middle = lerp_point(a, b, 0.5);
      if is_outside(middle) {
        b = middle;
      } else {
        a = middle;
      }
    }
    return lerp_point(a, b, 0.5);
  };

  let mut routes = vec![];

  for input_route in input_routes.iter() {
    if input_route.len() < 2 {
      continue;
    }

    let mut prev = input_route[0];
    let mut prev_is_outside = is_outside(prev);
    let mut route = vec![];
    if !prev_is_outside {
      // prev is not to crop. we can start with it
      route.push(prev);
    }

    for &p in input_route.iter().skip(1) {
      // we iterate in small steps to detect any interruption
      let static_prev = prev;
      let dx = p.0 - prev.0;
      let dy = p.1 - prev.1;
      let d = (dx * dx + dy * dy).sqrt();
      let vx = dx / d;
      let vy = dy / d;
      let iterations = (d / stepping).ceil() as usize;
      let mut v = 0.0;
      for _i in 0..iterations {
        v = (v + stepping).min(d);
        let q = (static_prev.0 + vx * v, static_prev.1 + vy * v);
        let q_is_outside = is_outside(q);
        if prev_is_outside != q_is_outside {
          // we have a crossing. we search it precisely
          let intersection = if prev_is_outside {
            search(q, prev, dichotomic_iterations)
          } else {
            search(prev, q, dichotomic_iterations)
          };

          if q_is_outside {
            // we close the path
            route.push(intersection);
            if route.len() > 1 {
              // we have a valid route to accumulate
              routes.push(route);
            }
            route = vec![];
          } else {
            // we open the path
            route.push(intersection);
          }
          prev_is_outside = q_is_outside;
        }

        prev = q;
      }

      // prev should be == p
      if !prev_is_outside {
        // prev is not to crop. we can start with it
        route.push(p);
      }
    }

    if route.len() > 1 {
      // we have a valid route to accumulate
      routes.push(route);
    }
  }

  routes
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

fn cloud_in_circle<R: Rng>(
  rng: &mut R,
  circle: &VCircle,
) -> (Vec<Vec<(f64, f64)>>, Vec<VCircle>) {
  // FIXME the clouds have a weird issue on the fact we don't always see the edges

  let mut routes = vec![];

  let mut circles: Vec<VCircle> = vec![];

  let stretchy = rng.gen_range(0.2, 1.0);

  let count = rng.gen_range(40, 80);
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
    let dr = rng.gen_range(0.5, 2.0);
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

// TODO rework with clip_routes
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

// TODO this function needs to return where the payload position is
fn trebuchet<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  height: f64,
  action_percent: f64,
  xflip: bool,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();

  let xmul = if xflip { -1.0 } else { 1.0 };

  let w = 0.6 * height;

  let line_width = 0.04 * height;
  let line_dist = 0.3;

  // make the base plank
  let mut route = Vec::new();
  let mut l = 0.0;
  let mut rev = false;
  while l < line_width {
    let a = (origin.0 - w / 2.0, origin.1 - l);
    let b = (origin.0 + w / 2.0, origin.1 - l);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    l += line_dist;
    rev = !rev;
  }
  routes.push(route);

  let frame_h = height * 0.5;
  let pivot = (origin.0, origin.1 - height * 0.45);

  // main stick
  let mut route = Vec::new();
  let mut l = 0.0;
  let mut rev = false;
  while l < line_width {
    let a = (origin.0 + l - line_width / 2.0, origin.1);
    let b = (origin.0 + l - line_width / 2.0, origin.1 - frame_h);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    l += line_dist;
    rev = !rev;
  }
  routes.push(route);

  let line_width = 0.03 * height;

  let possible_positions = vec![0.3, 0.5, 0.7, 1.0];
  let mut indexes = (0..possible_positions.len()).collect::<Vec<_>>();
  rng.shuffle(&mut indexes);
  let count = rng.gen_range(1, indexes.len());

  // structure frames
  let mut frames = vec![];
  for i in &indexes[..count] {
    let hf = possible_positions[*i];
    let wf = rng.gen_range(0.3, 0.5) - 0.2 * hf;
    frames.push((wf * w, hf * frame_h));
  }
  for (dx, dy) in frames {
    let mut route = Vec::new();
    let mut l = 0.0;
    let mut rev = false;
    while l < line_width {
      let a = (origin.0 - dx, origin.1 - l);
      let b = (origin.0, origin.1 - dy - l);
      let c = (origin.0 + dx, origin.1 - l);
      if rev {
        route.push(a);
        route.push(b);
        route.push(c);
      } else {
        route.push(c);
        route.push(b);
        route.push(a);
      }
      l += line_dist;
      rev = !rev;
    }
    routes.push(route);
  }

  // beam
  let size_factor = rng.gen_range(0.0, 1.0);
  let beam_main_length = mix(0.5, 0.8, size_factor) * height;
  let beam_second_length = 0.2 * height;
  let angle = mix(mix(2.5, 3.0, size_factor), 6.0, action_percent);
  let acos = angle.cos();
  let asin = angle.sin();

  let pivot1 = (
    pivot.0 + xmul * beam_main_length * acos,
    pivot.1 + beam_main_length * asin,
  );

  let pivot2 = (
    pivot.0 - xmul * beam_second_length * acos,
    pivot.1 - beam_second_length * asin,
  );

  let mut route = Vec::new();
  let mut l = 0.0;
  let mut rev = false;
  while l < line_width {
    let m = l - line_width / 2.0;
    let disp = (-asin * m, acos * m);
    let a = (pivot1.0 + disp.0, pivot1.1 + disp.1);
    let b = (pivot2.0 + disp.0, pivot2.1 + disp.1);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    l += line_dist;
    rev = !rev;
  }
  routes.push(route);

  // counterweight parts
  let f = rng.gen_range(0.0, 1.0);
  let cw_height = mix(0.15, 0.25, 1.0 - f) * height;
  let cw_width = rng.gen_range(0.1, 0.25) * height;
  let stickh = mix(0.01, 0.1, f) * height;

  // counterweight stick
  let mut route = Vec::new();
  let mut l = 0.0;
  let mut rev = false;
  while l < line_width {
    let a = (pivot2.0 + l - line_width / 2.0, pivot2.1);
    let b = (pivot2.0 + l - line_width / 2.0, pivot2.1 + stickh);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    l += line_dist;
    rev = !rev;
  }
  routes.push(route);

  // counterweight block
  let dy = rng.gen_range(0.0, 1.0) * stickh;
  let center = (pivot2.0, pivot2.1 + dy);
  let rad = dy + cw_height * rng.gen_range(0.95, 1.1);
  let anglestart = PI / 4.0;
  let angleeng = 3.0 * PI / 4.0;

  let square = (
    pivot2.0 - cw_width / 2.0,
    pivot2.1 + stickh,
    pivot2.0 + cw_width / 2.0,
    pivot2.1 + stickh + cw_height,
  );

  let line_dist = 0.4;
  let mut route = Vec::new();
  let mut x = square.0;
  let mut rev = false;
  while x < square.2 {
    let mut y = if rev { square.3 } else { square.1 };
    let mut horizontal_points_count = 0;
    loop {
      if rev {
        if y < square.1 {
          break;
        }
      } else {
        if y > square.3 {
          break;
        }
      }

      let dx = x - center.0;
      let dy = y - center.1;
      let d = (dx * dx + dy * dy).sqrt();
      let is_inside_circle = d < rad;
      let a = dy.atan2(dx);
      let is_inside_angle = a > anglestart && a < angleeng;
      let is_inside_counterweight = is_inside_circle && is_inside_angle;

      if is_inside_counterweight {
        if horizontal_points_count < 2 {
          route.push((x, y));
          horizontal_points_count += 1;
        } else {
          let l = route.len();
          route[l - 1] = (x, y);
        }
      } else {
        horizontal_points_count = 0;
        if route.len() > 1 {
          routes.push(route);
          route = Vec::new();
        } else if route.len() > 0 {
          route = Vec::new();
        }
      }

      y += if rev { -line_dist } else { line_dist };
    }
    x += line_dist;
    rev = !rev;
  }
  if route.len() > 1 {
    routes.push(route);
  }
  // TODO contouring of the counterweight

  if rng.gen_bool(0.5) {
    // triangle structure on the counterweight
    let mainsz = rng.gen_range(0.1, 0.16);

    // vertical
    let mut l = 0.0;
    let mut rev = false;
    while l < 0.04 * height {
      let mut route = Vec::new();
      let sz = mainsz * height;
      let a = (pivot2.0, pivot2.1 + stickh - l);
      let b = (pivot2.0 + xmul * sz, pivot2.1 + stickh - l);
      if rev {
        route.push(b);
        route.push(a);
      } else {
        route.push(a);
        route.push(b);
      }
      routes.push(route);
      l += line_dist;
      rev = !rev;
    }

    // triangle side
    let mut l = 0.0;
    let mut rev = false;
    while l < 0.03 * height {
      let mut route = Vec::new();
      let sz = 0.1 * height;
      let a = (pivot2.0, pivot2.1 + cw_height / 2.0 + stickh - l);
      let b = (pivot2.0 + xmul * sz, pivot2.1 + stickh - l);
      if rev {
        route.push(b);
        route.push(a);
      } else {
        route.push(a);
        route.push(b);
      }
      routes.push(route);
      l += 1.4 * line_dist;
      rev = !rev;
    }

    // tip
    let mut l = 0.0;
    let mut rev = false;
    while l < 0.02 * height {
      let mut route = Vec::new();
      let sz = mainsz * height;
      let h = 0.03 * height;
      let a = (
        pivot2.0 + xmul * (sz + l),
        pivot2.1 + stickh - 0.04 * height,
      );
      let b = (pivot2.0 + xmul * (sz + l), pivot2.1 + stickh + h);
      if rev {
        route.push(b);
        route.push(a);
      } else {
        route.push(a);
        route.push(b);
      }
      routes.push(route);
      l += line_dist;
      rev = !rev;
    }
  }

  // sling
  let length = rng.gen_range(0.3, 0.5) * height;
  let inity = pivot1.1 + length;
  let miny = origin.1 - 0.06 * height;
  let dx = (inity - miny).max(0.0);
  let center = (pivot1.0 + dx, inity.min(miny));
  let angle = 2.5 * PI * action_percent.powf(1.5) * xmul;
  // rotate center around pivot1 by angle
  let dx = center.0 - pivot1.0;
  let dy = center.1 - pivot1.1;
  let acos = angle.cos();
  let asin = angle.sin();
  let center = (
    pivot1.0 + xmul * dx * acos - dy * asin,
    pivot1.1 + xmul * dx * asin + dy * acos,
  );
  let dt = 0.04 * height;
  let center1 = (center.0 + dt * acos, center.1 + dt * asin);
  let center2 = (center.0 - dt * acos, center.1 - dt * asin);
  let p = (mix(center.0, pivot1.0, 0.5), mix(center.1, pivot1.1, 0.5));
  routes.push(vec![pivot1, p]);
  routes.push(vec![center2, p, center1]);

  let mut r = line_width;
  while r > line_dist / 2.0 {
    routes.push(circle_route(center, r, 16));
    r -= 0.8;
  }

  // rope to attach the beam on a wheel

  let wheel_radius = 0.04 * height;
  let wheel_center = (
    origin.0 - 0.2 * xmul * height,
    origin.1 - wheel_radius - 0.06 * height,
  );
  routes.push(vec![
    (wheel_center.0, origin.1),
    wheel_center,
    (wheel_center.0 - 0.1 * xmul * height, origin.1),
  ]);

  let mut r = 0.3;
  while r < wheel_radius {
    routes.push(circle_route(wheel_center, r, 10));
    r += 0.5;
  }

  let beam_anchor = (mix(pivot1.0, pivot.0, 0.5), mix(pivot1.1, pivot.1, 0.5));
  let beam_anchor_half = (
    mix(beam_anchor.0, wheel_center.0, 0.5),
    mix(beam_anchor.1, wheel_center.1, 0.5),
  );
  let beam_anchor1 = (mix(pivot1.0, pivot.0, 0.3), mix(pivot1.1, pivot.1, 0.3));
  let beam_anchor2 = (mix(pivot1.0, pivot.0, 0.7), mix(pivot1.1, pivot.1, 0.7));

  let mut ropes = vec![beam_anchor1, beam_anchor_half, beam_anchor2];

  if action_percent < 0.1 {
    let a = (wheel_center.0 + 0.5 * wheel_radius * xmul, wheel_center.1);
    let b = (wheel_center.0 - 0.5 * wheel_radius * xmul, wheel_center.1);
    routes.push(vec![a, beam_anchor_half, b]);
  } else {
    let left = ropes[0];
    ropes[1].1 -= rng.gen_range(0.1, 0.2) * height;
    let right = ropes[2];
    ropes = path_subdivide_to_curve_it(ropes, 0.8);
    ropes = shake(ropes, 0.1 * height, rng);
    ropes = path_subdivide_to_curve_it(ropes, 0.75);
    ropes = path_subdivide_to_curve_it(ropes, 0.7);

    ropes[0] = left;
    let l = ropes.len();
    ropes[l - 1] = right;
  }

  routes.push(ropes);

  routes
}

fn spear<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  size: f64,
  angle: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let spear_len = rng.gen_range(1.8, 2.2) * size;
  let spear_w = 0.06 * size;

  let blade_w = 0.15 * size;
  let blade_len = 0.3 * size;

  let line_dist = 0.3;

  routes.push(grow_stroke_zigzag(
    (-spear_len / 2.0, 0.0),
    (spear_len / 2.0, 0.0),
    spear_w,
    line_dist,
  ));

  let mut route = Vec::new();
  route.push((spear_len / 2.0, -blade_w / 2.0));
  route.push((spear_len / 2.0 + blade_len, 0.0));
  route.push((spear_len / 2.0, blade_w / 2.0));
  route.push(route[0]);
  routes.push(route);

  // translate routes
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&(x, y)| {
          let (x, y) = p_r((x, y), angle);
          (x + origin.0, y + origin.1)
        })
        .collect()
    })
    .collect()
}

fn arrow(origin: (f64, f64), size: f64, angle: f64) -> Vec<Vec<(f64, f64)>> {
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let w = 0.15 * size;
  let l = 0.3 * size;

  routes.push(vec![(0.0, 0.0), (size, 0.0)]);

  let mut route = Vec::new();
  route.push((size, -w / 2.0));
  route.push((size + l, 0.0));
  route.push((size, w / 2.0));
  route.push(route[0]);
  routes.push(route);

  // translate routes
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&(x, y)| {
          let (x, y) = p_r((x, y), angle);
          (x + origin.0, y + origin.1)
        })
        .collect()
    })
    .collect()
}

fn sword<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  size: f64,
  angle: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

  let sword_len = rng.gen_range(0.8, 1.2) * size;
  let handle_len = 0.12 * size;
  let handle_w = 0.06 * size;
  let hilt_size = 0.2 * size;
  let hilt_w = 0.05 * size;
  let blade_w = 0.08 * size;

  // draw the swords: =||>==--

  let line_dist = 0.3;

  routes.push(grow_stroke_zigzag(
    (0.0, 0.0),
    (handle_len, 0.0),
    handle_w,
    line_dist,
  ));

  routes.push(grow_stroke_zigzag(
    (handle_len, -hilt_size / 2.0),
    (handle_len, hilt_size / 2.0),
    hilt_w,
    line_dist,
  ));

  let mut route = Vec::new();
  route.push((0.0, -blade_w / 2.0));
  route.push((sword_len, 0.0));
  route.push((0.0, blade_w / 2.0));
  routes.push(route);

  // translate routes
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&(x, y)| {
          let (x, y) = p_r((x, y), angle);
          (x + origin.0, y + origin.1)
        })
        .collect()
    })
    .collect()
}

struct LongBow {
  routes: Vec<Vec<(f64, f64)>>,
  arrow_start: (f64, f64),
  arrow_angle: f64,
}

impl MonochromeStrokable for LongBow {
  fn render(&self) -> Vec<Vec<(f64, f64)>> {
    self.routes.clone()
  }
}

impl LongBow {
  fn new<R: Rng>(
    rng: &mut R,
    origin: (f64, f64),
    size: f64,
    angle: f64,
    phase: f64,
  ) -> Self {
    let mut routes: Vec<Vec<(f64, f64)>> = Vec::new();

    // arc au repos
    let dy = 0.5 * size;
    let dx = 0.5 * dy;
    let bow_w = 0.1 * size;

    let max_allonge = 0.8 * size;
    let allonge = mix(dx, max_allonge, phase);

    let mut route = vec![];
    route.push((-dx, -dy));
    route.push((0.0, 0.0));
    route.push((-dx, dy));
    let bow = path_subdivide_to_curve(route, 2, 0.8);

    routes.push(grow_path_zigzag(bow, angle, bow_w, 0.3));

    let string = vec![(-dx, -dy), (-allonge, 0.0), (-dx, dy)];

    routes.push(string);

    // translate routes
    routes = routes
      .iter()
      .map(|route| {
        route
          .iter()
          .map(|&(x, y)| {
            let (x, y) = p_r((x, y), angle);
            (x + origin.0, y + origin.1)
          })
          .collect()
      })
      .collect();

    let arrow_angle = angle;
    let arrow_start = proj_point(origin, -angle, -allonge);

    Self {
      routes,
      arrow_start,
      arrow_angle,
    }
  }
}

fn grow_path_zigzag(
  path: Vec<(f64, f64)>,
  angle: f64,
  width: f64,
  line_dist: f64,
) -> Vec<(f64, f64)> {
  let mut route: Vec<(f64, f64)> = Vec::new();
  let dx = angle.cos();
  let dy = angle.sin();
  let incr_dx = -dy;
  let incr_dy = dx;

  let mut route = Vec::new();
  let count = (width / line_dist).ceil() as usize;
  let delta_i = if count < 2 { 0.0 } else { count as f64 / 2.0 };
  let mut rev = false;
  for i in 0..count {
    let mul = (i as f64 - delta_i) / (count as f64);
    let w = width * mul;
    let it: Vec<&(f64, f64)> = if rev {
      path.iter().rev().collect()
    } else {
      path.iter().collect()
    };
    for p in it {
      let (x, y) = p;
      let a = (x + incr_dx * w, y + incr_dy * w);
      route.push(a);
    }
    rev = !rev;
  }

  route
}

fn grow_stroke_zigzag(
  from: (f64, f64),
  to: (f64, f64),
  width: f64,
  line_dist: f64,
) -> Vec<(f64, f64)> {
  let mut route: Vec<(f64, f64)> = Vec::new();
  let (x0, y0) = from;
  let (x1, y1) = to;
  let (dx, dy) = (x1 - x0, y1 - y0);
  let len = (dx * dx + dy * dy).sqrt();
  let incr_dx = -dy / len;
  let incr_dy = dx / len;

  let mut route = Vec::new();
  let count = (width / line_dist).ceil() as usize;
  let delta_i = if count < 2 { 0.0 } else { count as f64 / 2.0 };
  let mut rev = false;
  for i in 0..count {
    let mul = (i as f64 - delta_i) / (count as f64);
    let w = width * mul;
    let a = (from.0 + incr_dx * w, from.1 + incr_dy * w);
    let b = (to.0 + incr_dx * w, to.1 + incr_dy * w);
    if rev {
      route.push(b);
      route.push(a);
    } else {
      route.push(a);
      route.push(b);
    }
    rev = !rev;
  }

  route
}

fn head(origin: (f64, f64), angle: f64, size: f64) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = 0.4 * size;
  routes.push(vec![(-dx, 0.0), (-dx, -h), (dx, -h), (dx, 0.0), (-dx, 0.0)]);

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&(x, y)| {
          let (x, y) = p_r((x, y), ang);
          (x + origin.0, y + origin.1)
        })
        .collect()
    })
    .collect()
}

fn bowman<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  size: f64,
) -> Vec<Vec<(f64, f64)>> {
  let phase = rng.gen_range(0.0, 1.0);
  let shoulder_right_angle = mix(0.0, -PI / 4.0, phase);
  let elbow_right_angle = shoulder_right_angle;

  let joints = HumanJointAngles {
    body_angle: -PI / 2.0,
    head_angle: -PI / 2.0,
    shoulder_right_angle,
    shoulder_left_angle: rng.gen_range(3.0 * PI / 4.0, PI),
    elbow_right_angle,
    elbow_left_angle: PI / 2.0 + 0.3,
    hip_right_angle: PI / 2.0 - 0.5,
    hip_left_angle: PI / 2.0 + 0.5,
    knee_right_angle: PI / 2.0,
    knee_left_angle: PI / 2.0,

    left_arm_bend: 0.5,
    right_arm_bend: 1.0,
    left_leg_bend: 1.0,
    right_leg_bend: 1.0,
  };
  let humansize = size * 0.5;
  let xcenter = origin.0 - size * 0.5;
  let human = HumanBody::new((xcenter, origin.1), humansize, joints);
  let mut new_routes = vec![];

  new_routes.extend(human.render());
  let (headpos, headangle) = human.head_pos_angle();
  let h = head(headpos, headangle, humansize);
  new_routes.extend(h);

  let (pos, angle) = human.hand_right_pos_angle();

  let bow = LongBow::new(rng, pos, size * 0.5, -angle, phase);
  new_routes.extend(bow.render());

  new_routes
}

fn merlon(
  route: &mut Vec<(f64, f64)>,
  leftx: f64,
  lefty: f64,
  rightx: f64,
  _righty: f64,
  h: f64,
) {
  let mut count = ((rightx - leftx) / h).ceil();
  count = (count / 2.0).floor() * 2.0 + 1.0;
  let w = (rightx - leftx) / count;
  let mut x = leftx;
  let mut alt = false;
  loop {
    if x > rightx - w / 2.0 {
      break;
    }
    let y = lefty; // TODO interpolate lefty righty
    x += w;
    if alt {
      route.push((x, y + h));
      route.push((x, y));
    } else {
      route.push((x, y));
      route.push((x, y + h));
    }
    alt = !alt;
  }
}

fn wall_shadow<R: Rng>(
  rng: &mut R,
  path: Vec<(f64, f64)>,
  stroke_len: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  if path.len() < 2 {
    return routes;
  }
  let mut prev = path[0];
  let mut current_l = euclidian_dist(prev, path[1]);
  let mut direction = (-1.0, 0.0);
  let mut i = 0;
  let mut l = 0.0;
  loop {
    while l > current_l {
      l -= current_l;
      prev = path[i];
      i += 1;
      if i >= path.len() {
        return routes;
      }
      current_l = euclidian_dist(prev, path[i]);
      let dx = path[i].0 - prev.0;
      let dy = path[i].1 - prev.1;
      direction = (-dy / current_l, dx / current_l);
    }
    let p = lerp_point(prev, path[i], l / current_l);
    let slen = stroke_len * rng.gen_range(0.8, 1.2);
    routes.push(vec![
      p,
      (p.0 + slen * direction.0, p.1 + slen * direction.1),
    ]);

    l += rng.gen_range(0.8, 1.2);
  }
}

fn castle<R: Rng>(
  peaks: &Vec<(f64, f64, f64)>,
  scale: f64,
  rng: &mut R,
  passage: &mut Passage,
) -> (Vec<Vec<(f64, f64)>>, Vec<Vec<(f64, f64)>>, (f64, f64)) {
  let mut routes = Vec::new();
  let mut polys = Vec::new();
  if peaks.len() == 0 {
    return (routes, polys, (0.0, 0.0));
  }

  let polypad = 0.8;

  /*
  let intersects_routes =
    |a: (f64, f64), b: (f64, f64)| -> Option<(f64, f64)> {
      routes
        .iter()
        .find_map(|(_ci, route)| collide_route_segment(route, a, b))
    };
    */

  let ybase = |xsearch: f64| -> f64 {
    let mut lastpeak = peaks[0];
    for &p in peaks.iter() {
      if xsearch < p.0 {
        if p.0 == lastpeak.0 {
          return p.1;
        }
        return mix(
          lastpeak.1,
          p.1,
          (xsearch - lastpeak.0) / (p.0 - lastpeak.0),
        );
      }
      lastpeak = p;
    }
    return lastpeak.1;
  };

  let wallcenter = peaks[peaks.len() / 2];
  let mut maxy = 0.0;
  for p in peaks.iter() {
    if p.1 > maxy {
      maxy = p.1;
    }
  }
  let wallheighty = wallcenter.1 - scale * rng.gen_range(2.0, 14.0);
  let towerwidth = scale * rng.gen_range(3.0, 5.0);
  let maint_height = scale * rng.gen_range(14.0, 24.0);
  let maint_width = scale * rng.gen_range(4.0, 8.0);
  let maint_roof_height = scale * rng.gen_range(4.0, 14.0);
  let merlonh = scale * rng.gen_range(1.0, 2.2);

  let d1 = scale * rng.gen_range(0.0, 3.0);
  let h1 = scale * rng.gen_range(3.0, 5.0);

  let leftpeak = peaks[0];
  let leftpeak2 = (leftpeak.0 + towerwidth, ybase(leftpeak.0 + towerwidth));
  let rightpeak = peaks[peaks.len() - 1];
  let rightpeak2 = (rightpeak.0 - towerwidth, ybase(rightpeak.0 - towerwidth));

  // wall top
  let mut route = Vec::new();
  polys.push(vec![
    (leftpeak2.0 - polypad, leftpeak2.1 + polypad),
    (leftpeak2.0 - polypad, wallheighty - polypad),
    (rightpeak2.0 + polypad, wallheighty - polypad),
    (rightpeak2.0 + polypad, rightpeak2.1 + polypad),
  ]);
  route.push(leftpeak2);
  route.push((leftpeak2.0, wallheighty));
  merlon(
    &mut route,
    leftpeak2.0 + 0.01,
    wallheighty,
    rightpeak2.0 - 0.01,
    wallheighty,
    merlonh,
  );
  route.push(rightpeak2);
  routes.push(route);

  for (a, b) in vec![
    // Left tower
    ((leftpeak.0, leftpeak.1), leftpeak2),
    // Right tower
    (rightpeak2, (rightpeak.0, rightpeak.1)),
  ] {
    if rng.gen_bool(0.1) {
      continue;
    }
    let towerheighty = wallheighty
      - scale * rng.gen_range(1.0, 3.0)
      - scale
        * rng.gen_range(0.0, 16.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);

    let mut route: Vec<(f64, f64)> = Vec::new();
    route.push(a);
    route.push((a.0, towerheighty));
    route.push((a.0 - d1, towerheighty - d1));
    route.push((a.0 - d1, towerheighty - d1 - h1));
    merlon(
      &mut route,
      a.0 - d1,
      towerheighty - d1 - h1,
      b.0 + d1,
      towerheighty - d1 - h1,
      merlonh,
    );
    route.push((b.0 + d1, towerheighty - d1 - h1));
    route.push((b.0 + d1, towerheighty - d1));
    route.push((b.0, towerheighty));
    route.push(b);

    // boundaries of the tower body
    polys.push(vec![
      (a.0 - polypad, a.1 + polypad),
      (b.0 + polypad, b.1 + polypad),
      (b.0 + polypad, towerheighty - polypad),
      (a.0 - polypad, towerheighty - polypad),
    ]);

    // boundaries of the tower head
    polys.push(vec![
      (a.0 - polypad, towerheighty + polypad),
      (b.0 + polypad, towerheighty + polypad),
      (b.0 + polypad + d1, towerheighty - d1 + polypad),
      (b.0 + polypad + d1, towerheighty - h1 - merlonh - polypad),
      (a.0 - polypad - d1, towerheighty - h1 - merlonh - polypad),
      (a.0 - polypad - d1, towerheighty - d1 + polypad),
    ]);

    let right_side_path = vec![
      (b.0 + d1, towerheighty - d1 - h1),
      (b.0 + d1, towerheighty - d1),
      (b.0, towerheighty),
      b,
    ];
    for shadow in wall_shadow(rng, right_side_path, 1.0) {
      routes.push(shadow);
    }
    routes.push(route);

    // windows
    let mut y = towerheighty;
    let w = scale * 0.25;
    let h = scale * rng.gen_range(1.0, 1.2);
    loop {
      let x = mix(a.0, b.0, rng.gen_range(0.4, 0.6));
      let lowesty = ybase(x);
      if y > lowesty - 3.0 * h {
        break;
      }
      routes.push(vec![
        (x - w, y - h),
        (x + w, y - h),
        (x + w, y + h),
        (x - w, y + h),
        (x - w, y - h),
      ]);
      y += 4.0 * h;
    }
  }

  // chapel
  if rng.gen_bool(0.5) {
    let mut route = Vec::new();

    let x = wallcenter.0 + maint_width / 2.0;
    route.push((x, wallheighty));
    route.push((x, wallcenter.1 - maint_height));
    for shadow in wall_shadow(rng, route.clone(), -1.0) {
      routes.push(shadow);
    }
    let x = wallcenter.0 - maint_width / 2.0;
    route.push((x, wallcenter.1 - maint_height));
    route.push((x, wallheighty));
    routes.push(route);

    // boundaries of chapel body
    polys.push(vec![
      (
        wallcenter.0 + maint_width / 2.0 - polypad,
        wallheighty + polypad,
      ),
      (
        wallcenter.0 + maint_width / 2.0 + polypad,
        wallheighty + polypad,
      ),
      (
        wallcenter.0 + maint_width / 2.0 + polypad,
        wallcenter.1 - maint_height - polypad,
      ),
      (
        wallcenter.0 + maint_width / 2.0 - polypad,
        wallcenter.1 - maint_height - polypad,
      ),
    ]);

    let w = maint_width * rng.gen_range(0.5, 0.55);
    let h = maint_roof_height;
    let y = wallcenter.1 - maint_height;
    routes.push(vec![
      (wallcenter.0 - w, y),
      (wallcenter.0, y - h),
      (wallcenter.0 + w, y),
    ]);

    // boundaries of chapel roof
    polys.push(vec![
      (wallcenter.0 - w - polypad, y),
      (wallcenter.0, y - h - polypad),
      (wallcenter.0 + w + polypad, y),
    ]);
    let mut l = 0.0;
    loop {
      if l > 2.0 * w {
        break;
      }
      routes.push(vec![(wallcenter.0, y - h), (wallcenter.0 + w - l, y)]);
      l += scale * rng.gen_range(0.3, 0.7) + l / w;
    }

    // cross
    let x = wallcenter.0;
    let y = y - h - 2.0;
    routes.push(vec![(x - scale * 0.8, y), (x + scale * 0.8, y)]);
    routes.push(vec![(x, y - scale * 1.0), (x, y + scale * 2.0)]);

    // window
    let x = wallcenter.0;
    let y = mix(
      wallcenter.1 - maint_height,
      wallheighty,
      rng.gen_range(0.2, 0.3),
    );
    let w = scale * 0.4;
    let h = scale * 0.6;
    routes.push(vec![
      (x - w, y - h),
      (x + w, y - h),
      (x + w, y + h),
      (x - w, y + h),
      (x - w, y - h),
    ]);
  }

  // wall texture
  let xrep = scale * rng.gen_range(2.6, 3.2);
  let yrep = scale * rng.gen_range(1.2, 1.6);
  let mut alt = false;
  let mut y = wallheighty + merlonh + yrep;
  loop {
    if y > maxy {
      break;
    }
    let mut x = leftpeak2.0;
    if alt {
      x += xrep / 2.0;
    }
    loop {
      if x > rightpeak2.0 {
        break;
      }
      let strokel = scale * rng.gen_range(1.3, 1.5);
      let dx = scale * rng.gen_range(-0.2, 0.2);
      let dy = scale * rng.gen_range(-0.1, 0.1);
      let x1 = (x + dx).max(leftpeak.0).min(rightpeak2.0);
      let x2 = (x + dx + strokel).max(leftpeak.0).min(rightpeak2.0);
      let y1 = y + dy;
      if y1 < ybase(x1) && y1 < ybase(x2) && rng.gen_bool(0.95) {
        routes.push(vec![(x1, y + dy), (x2, y + dy)]);
      }
      x += xrep;
    }
    y += yrep;
    alt = !alt;
  }

  for r in routes.iter() {
    for p in path_subdivide_to_curve(r.clone(), 2, 0.8) {
      // TODO custom code to do all the lines properly
      passage.count(p);
    }
  }

  (routes, polys, (wallcenter.0, wallheighty))
}

fn lookup_projectile_curve(
  origin: (f64, f64),
  destination: (f64, f64),
  curveh: f64,
  position_percentage: f64,
) -> (f64, f64) {
  let mut p = lerp_point(origin, destination, position_percentage);
  let dy = curveh
    * (2.0 * (0.5 - (0.5 - position_percentage.max(0.0).min(1.0)).abs()))
      .sqrt();
  p.1 -= dy;
  p
}

fn make_projectile_pos_ang(
  origin: (f64, f64),
  destination: (f64, f64),
  curveh: f64,
  position_percentage: f64,
) -> ((f64, f64), f64) {
  let p =
    lookup_projectile_curve(origin, destination, curveh, position_percentage);

  let s1 = lookup_projectile_curve(
    origin,
    destination,
    curveh,
    position_percentage - 0.001,
  );
  let s2 = lookup_projectile_curve(
    origin,
    destination,
    curveh,
    position_percentage + 0.001,
  );
  let angle = (s2.1 - s1.1).atan2(s2.0 - s1.0);

  (p, angle)
}

fn arrow_projectile(
  origin: (f64, f64),
  destination: (f64, f64),
  curveh: f64,
  length: f64,
  position_percentage: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  let (p, angle) =
    make_projectile_pos_ang(origin, destination, curveh, position_percentage);
  let acos = angle.cos();
  let asin = angle.sin();

  let l1 = -length * 0.5;
  let p1 = (p.0 + acos * l1, p.1 + asin * l1);
  let l2 = length * 0.1;
  let p2 = (p.0 + acos * l2, p.1 + asin * l2);
  let l3 = length * 0.5;
  let p3 = (p.0 + acos * l3, p.1 + asin * l3);

  routes.push((0, vec![p1, p2]));
  routes.push((2, vec![p2, p3]));

  // return vec![vec![origin, destination]];
  return routes;
}

fn fireball_projectile<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  destination: (f64, f64),
  curveh: f64,
  size: f64,
  position_percentage: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  let (p, _angle) =
    make_projectile_pos_ang(origin, destination, curveh, position_percentage);

  let count = (1. + size * 10.0) as usize;
  for clr in vec![0, 2] {
    let mut route = Vec::new();
    for _i in 0..count {
      let a = rng.gen_range(0.0, 2.0 * PI);
      let acos = a.cos();
      let asin = a.sin();
      route.push((p.0 + size * acos, p.1 + size * asin));
    }
    routes.push((clr, route));

    let perc_incr = rng.gen_range(0.01, 0.02);
    let mut perc = (position_percentage - rng.gen_range(0.1, 0.3)).max(0.1);
    let mut route = vec![];
    let mut i = 0;
    while perc < position_percentage {
      let (p, angle) =
        make_projectile_pos_ang(origin, destination, curveh, perc);
      let a = angle
        + PI * rng.gen_range(0.4, 0.6) * (if i % 2 == 0 { 1.0 } else { -1.0 });
      let l = size * rng.gen_range(0.5, 1.0) * rng.gen_range(0.0, 1.0);
      let acos = a.cos();
      let asin = a.sin();
      let p1 = (p.0 + acos * l, p.1 + asin * l);
      route.push(p1);
      perc += perc_incr * rng.gen_range(0.8, 1.2);
      i += 1;
    }
    routes.push((clr, route));
  }

  // return vec![vec![origin, destination]];
  return routes;
}

fn slice_polylines(
  route: &Vec<(f64, f64)>,
  segment_length: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = vec![];
  let mut l = 0.0;
  let mut i = 1;
  let mut prev = route[0];
  let mut segment = vec![];
  segment.push(prev);
  loop {
    if i >= route.len() {
      break;
    }
    let mut next = route[i];
    let mut d = euclidian_dist(prev, next);
    while l + d < segment_length {
      segment.push(next);
      l += d;
      i += 1;
      if i >= route.len() {
        routes.push(segment);
        return routes;
      }
      prev = next;
      next = route[i];
      d = euclidian_dist(prev, next);
    }
    let current = lerp_point(prev, next, (segment_length - l) / d);
    segment.push(current);
    prev = current;
    if segment.len() > 1 {
      routes.push(segment);
      segment = vec![prev];
    }

    /*
    if l + d > segment_length {
      let p = lerp_point(prev, current, (segment_length - l) / d);
      routes.push(vec![prev, p]);
      prev = p;
      l += segment_length;
    } else {
      */
    l = 0.0;
    prev = current;
    i += 1;
  }
  routes
}

// medieval name generator
pub fn epic_title<R: Rng>(rng: &mut R) -> String {
  let mut city_start = vec![
    "An", "Cul", "Dun", "Nor", "Ship", "Tre", "Win", "Mere", "Pol", "Tarn",
    "Lin", "Man", "Baa", "Bra", "Bri", "Istan", "Bor", "Ast", "Ach", "Axe",
    "Car", "Wolf", "Chet", "Holm", "Pen", "Port", "Beck", "Buck", "Bull",
    "Bul", "Lis",
  ];
  let mut city_suffixes = vec![
    "bourg", "burg", "castle", "bul", "des", "ster", "chester", "llon", "bury",
    "borough", "by", "cott", "field", "gate", "ing", "tun", "wick", "worth",
    "caster", "burgh", "ver", "bon",
  ];
  let mut city = city_start[rng.gen_range(0, city_start.len())].to_string();
  city += city_suffixes[rng.gen_range(0, city_suffixes.len())];

  let events = vec![
    "Battle",
    "War",
    "Fall",
    "Conquest",
    "Crusade",
    "Attack",
    "Defense",
    "Siege",
    "Raid",
    "Rise",
    "Destruction",
    "Era",
    "Age",
    "Reign",
    "Empire",
    "Kingdom",
    "Doom",
    "Dawn",
    "History",
  ];
  let i =
    (rng.gen_range(0., events.len() as f64) * rng.gen_range(0.5, 1.0)) as usize;
  let mut event = events[i].to_string();
  let going_prefix = rng.gen_bool(0.5);
  let year = rng.gen_range(500, 1400);
  if going_prefix {
    return format!("{} {} - circa {}", city, event, year);
  } else {
    return format!("{} of {} - circa {}", event, city, year);
  }
}

// homemade implementation of a filling technique that will spawn random worms that eat the space to colorize it!
struct WormsFilling {
  rot: f64,
  step: f64,
  straight: f64,
  min_l: usize,
  max_l: usize,
  decrease_value: f64,
  search_max: usize,
  min_weight: f64,
  freq: f64,
  seed: f64,
}
impl WormsFilling {
  // new
  fn rand<R: Rng>(rng: &mut R) -> Self {
    let seed = rng.gen_range(-999., 999.);
    let rot = PI / rng.gen_range(1.0, 2.0);
    let step = 0.4;
    let straight = rng.gen_range(0.0, 0.1);
    let min_l = 5;
    let max_l = 20;
    let decrease_value = 1.;
    let search_max = 1000;
    let min_weight = 1.;
    let freq = 0.05;
    Self {
      rot,
      step,
      straight,
      min_l,
      max_l,
      decrease_value,
      search_max,
      min_weight,
      freq,
      seed,
    }
  }

  fn fill_in_paint<R: Rng>(
    &self,
    rng: &mut R,
    drawings: &PaintMask,
    clr: usize,
    density: f64,
    bound: (f64, f64, f64, f64),
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let f = |x, y| {
      if drawings.is_painted((x, y)) {
        density
      } else {
        0.0
      }
    };
    let coloring = |_: &Vec<(f64, f64)>| clr;
    self.fill(rng, &f, bound, &coloring, iterations)
  }

  fn fill<R: Rng>(
    &self,
    rng: &mut R,
    f: &dyn Fn(f64, f64) -> f64,
    bound: (f64, f64, f64, f64),
    clr: &dyn Fn(&Vec<(f64, f64)>) -> usize,
    iterations: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let perlin = Perlin::new();
    let w = bound.2 - bound.0;
    let h = bound.3 - bound.1;
    let precision = 0.4;
    if w <= 2. * precision || h <= 2. * precision {
      return routes;
    }
    let mut map = WeightMap::new(w, h, 0.4);

    map.fill_fn(&|p| f(p.0 + bound.0, p.1 + bound.1));

    let seed = self.seed;
    let rot = self.rot;
    let step = self.step;
    let straight = self.straight;
    let min_l = self.min_l;
    let max_l = self.max_l;
    let decrease_value = self.decrease_value;
    let search_max = self.search_max;
    let min_weight = self.min_weight;
    let freq = self.freq;

    let mut bail_out = 0;

    for _i in 0..iterations {
      let top = map.search_weight_top(rng, search_max, min_weight);
      if top.is_none() {
        bail_out += 1;
        if bail_out > 10 {
          break;
        }
      }
      if let Some(o) = top {
        let angle = perlin.get([seed, freq * o.0, freq * o.1]);

        if let Some(a) = map.best_direction(o, step, angle, PI, PI / 4.0, 0.0) {
          let route = map.dig_random_route(
            o,
            a,
            step,
            rot,
            straight,
            max_l,
            decrease_value,
          );
          if route.len() >= min_l {
            let points: Vec<(f64, f64)> = rdp(&route, 0.05);
            // remap
            let rt = points
              .iter()
              .map(|&p| (p.0 + bound.0, p.1 + bound.1))
              .collect::<Vec<_>>();
            let c = clr(&rt);
            routes.push((c, rt));
          }
        }
      }
    }

    routes
  }
}

// data model that stores values information in 2D
struct WeightMap {
  weights: Vec<f64>,
  w: usize,
  h: usize,
  width: f64,
  height: f64,
  precision: f64,
}
impl WeightMap {
  fn new(width: f64, height: f64, precision: f64) -> WeightMap {
    let w = ((width / precision) + 1.0) as usize;
    let h = ((height / precision) + 1.0) as usize;
    let weights = vec![0.0; w * h];
    WeightMap {
      weights,
      w,
      h,
      width,
      height,
      precision,
    }
  }
  fn fill_fn(&mut self, f: &impl Fn((f64, f64)) -> f64) {
    for y in 0..self.h {
      for x in 0..self.w {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let v = f(p);
        self.weights[y * self.w + x] = v;
      }
    }
  }

  // do a simple bilinear interpolation
  fn get_weight(&self, p: (f64, f64)) -> f64 {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(self.w - 1);
    let y1 = (y0 + 1).min(self.h - 1);
    let dx = x - x0 as f64;
    let dy = y - y0 as f64;
    let w00 = self.weights[y0 * self.w + x0];
    let w01 = self.weights[y0 * self.w + x1];
    let w10 = self.weights[y1 * self.w + x0];
    let w11 = self.weights[y1 * self.w + x1];
    let w0 = w00 * (1.0 - dx) + w01 * dx;
    let w1 = w10 * (1.0 - dx) + w11 * dx;
    w0 * (1.0 - dy) + w1 * dy
  }

  // apply a gaussian filter to the weights around the point p with a given radius
  fn decrease_weight_gaussian(
    &mut self,
    p: (f64, f64),
    radius: f64,
    value: f64,
  ) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = ((x - radius).floor().max(0.) as usize).min(self.w);
    let y0 = ((y - radius).floor().max(0.) as usize).min(self.h);
    let x1 = ((x + radius).ceil().max(0.) as usize).min(self.w);
    let y1 = ((y + radius).ceil().max(0.) as usize).min(self.h);
    if x0 >= self.w || y0 >= self.h {
      return;
    }
    for y in y0..y1 {
      for x in x0..x1 {
        let p = (x as f64 * self.precision, y as f64 * self.precision);
        let d = (p.0 - p.0).hypot(p.1 - p.1);
        if d < radius {
          let w = self.weights[y * self.w + x];
          let v = value * (1.0 - d / radius);
          self.weights[y * self.w + x] = w - v;
        }
      }
    }
  }

  // find the best direction to continue the route by step
  // returns None if we reach an edge or if there is no direction that can be found in the given angle += max_angle_rotation and when the weight is lower than 0.0
  fn best_direction(
    &self,
    p: (f64, f64),
    step: f64,
    angle: f64,
    max_ang_rotation: f64,
    angle_precision: f64,
    straight_factor: f64,
  ) -> Option<f64> {
    let mut best_ang = None;
    let mut best_weight = 0.0;
    let mut a = -max_ang_rotation;
    while a < max_ang_rotation {
      let ang = a + angle;
      let dx = step * ang.cos();
      let dy = step * ang.sin();
      let np = (p.0 + dx, p.1 + dy);
      if np.0 < 0.0 || np.0 > self.width || np.1 < 0.0 || np.1 > self.height {
        a += angle_precision;
        continue;
      }
      // more important when a is near 0.0 depending on straight factor
      let wmul = (1.0 - straight_factor)
        + (1.0 - a.abs() / max_ang_rotation) * straight_factor;
      let weight = self.get_weight(np) * wmul;
      if weight > best_weight {
        best_weight = weight;
        best_ang = Some(ang);
      }
      a += angle_precision;
    }
    return best_ang;
  }

  // FIXME we could optim this by keeping track of tops and not searching too random
  fn search_weight_top<R: Rng>(
    &mut self,
    rng: &mut R,
    search_max: usize,
    min_weight: f64,
  ) -> Option<(f64, f64)> {
    let mut best_w = min_weight;
    let mut best_p = None;
    for _i in 0..search_max {
      let x = rng.gen_range(0.0, self.width);
      let y = rng.gen_range(0.0, self.height);
      let p = (x, y);
      let w = self.get_weight(p);
      if w > best_w {
        best_w = w;
        best_p = Some(p);
      }
    }
    return best_p;
  }

  fn dig_random_route(
    &mut self,
    origin: (f64, f64),
    initial_angle: f64,
    step: f64,
    max_ang_rotation: f64,
    straight_factor: f64,
    max_length: usize,
    decrease_value: f64,
  ) -> Vec<(f64, f64)> {
    let mut route = Vec::new();
    let mut p = origin;
    let mut angle = initial_angle;
    for _i in 0..max_length {
      if let Some(ang) = self.best_direction(
        p,
        step,
        angle,
        max_ang_rotation,
        0.2 * max_ang_rotation,
        straight_factor,
      ) {
        angle = ang;
        let prev = p;
        p = (p.0 + step * angle.cos(), p.1 + step * angle.sin());
        route.push(p);
        self.decrease_weight_gaussian(prev, step, decrease_value);
      } else {
        break;
      }
    }

    route
  }
}

fn draw_font<R: Rng>(
  rng: &mut R,
  font: &mut Font,
  paint: &mut PaintMask,
  fontsize: f64,
  pos: (f64, f64),
  text: &str,
  clr: usize,
  iterations: usize,
  density: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let filling = WormsFilling::rand(rng);

  let mut drawing = paint.clone_empty();
  let prec = drawing.precision;

  let fonts = &[font.clone()];

  let mut routes = Vec::new();

  let px = (fontsize / prec) as f32;

  let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

  let mut settings = LayoutSettings::default();
  settings.x = (pos.0 / prec) as f32;
  settings.y = (pos.1 / prec) as f32;
  layout.reset(&settings);
  layout.append(fonts, &TextStyle::new(text, px, 0));

  for glyph in layout.glyphs() {
    let (metrics, bytes) = font.rasterize_config(glyph.key);
    if glyph.parent == '\n' {
      continue;
    }
    let o = (glyph.x as f64 * prec, glyph.y as f64 * prec);
    drawing.paint_pixels(o, &bytes, metrics.width);
  }

  routes.extend(filling.fill_in_paint(
    rng,
    &drawing,
    clr,
    density,
    (
      pos.0,
      pos.1,
      paint.width, // FIXME what is the width?
      pos.1 + layout.height() as f64,
    ),
    iterations,
  ));

  routes
}

trait BandPattern {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)>;
  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)>;

  fn render_corner(
    &self,
    clr: usize,
    position: (f64, f64),
    angle: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let untranslated = self.corner(clr, bandw);
    let acos = angle.cos();
    let asin = angle.sin();
    let mut routes = vec![];
    for (clr, route) in untranslated {
      let mut r = vec![];
      for &p in route.iter() {
        let p = (
          p.0 * acos + p.1 * asin + position.0,
          p.1 * acos - p.0 * asin + position.1,
        );
        r.push(p);
      }
      routes.push((clr, r));
    }
    routes
  }

  fn render_band(
    &self,
    clr: usize,
    from: (f64, f64),
    to: (f64, f64),
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let l = euclidian_dist(from, to);
    let untranslated = self.pattern(clr, l, bandw);
    // rotate & translate
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let a = -dy.atan2(dx);
    let acos = a.cos();
    let asin = a.sin();
    let mut routes = vec![];
    for (clr, route) in untranslated {
      let mut r = vec![];
      for &p in route.iter() {
        let p = (
          p.0 * acos + p.1 * asin + from.0,
          p.1 * acos - p.0 * asin + from.1,
        );
        r.push(p);
      }
      routes.push((clr, r));
    }
    routes
  }
}
struct MedievalBandLRectPattern {
  cellw: f64,
  padx: f64,
  pady: f64,
  offx: f64,
}
impl MedievalBandLRectPattern {
  fn new() -> Self {
    Self {
      cellw: 2.0,
      padx: 0.15,
      pady: 0.05,
      offx: 0.25,
    }
  }
}
impl BandPattern for MedievalBandLRectPattern {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;
    let padx = self.padx * cellw;
    let pady = self.pady * cellw;
    let offx = self.offx * cellw;

    let l = length + 2.0 * padx;

    // round the cellw to make the exact length
    let n = (l / cellw).round() as usize;
    let cellw = l / (n as f64);

    let mut p = -padx;
    for _i in 0..n {
      routes.push((
        clr,
        vec![
          (p + padx + offx, -bandw / 2.0 + pady),
          (p + cellw - padx, -bandw / 2.0 + pady),
          (p + cellw - padx, bandw / 2.0 - pady),
        ],
      ));
      routes.push((
        clr,
        vec![
          (p + padx, -bandw / 2.0 + pady),
          (p + padx, bandw / 2.0 - pady),
          (p + cellw - padx - offx, bandw / 2.0 - pady),
        ],
      ));
      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    let cellw = self.cellw * bandw;
    let pady = self.pady * cellw;
    let d = bandw / 2.0 - pady;
    vec![(clr, vec![(-d, -d), (d, -d), (d, d), (-d, d), (-d, -d)])]
  }
}

struct MedievalBandFeatherTrianglePattern {
  cellw: f64,
  feather_ratio: f64,
  count1: usize,
  count2: usize,
}
impl MedievalBandFeatherTrianglePattern {
  fn new() -> Self {
    Self {
      cellw: 6.0,
      count1: 3,
      count2: 3,
      feather_ratio: 0.66,
    }
  }

  fn feather(
    &self,
    clr: usize,
    a: (f64, f64),
    b: (f64, f64),
    c: (f64, f64),
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    // TODO add a pad
    let mut routes = vec![]; // array of (clr, path)
    let count1 = self.count1;
    let count2 = self.count2;
    for i in 0..count1 {
      let t = ((i + 1) as f64 / (count1 + 1) as f64) * self.feather_ratio;
      let p = lerp_point(a, b, t);
      let q = lerp_point(a, c, t);
      routes.push((clr, vec![p, q]));
    }
    for i in 0..count2 {
      let t = (i as f64 + 1.0) / (count2 + 1) as f64;
      let end_bc = lerp_point(b, c, t);
      routes
        .push((clr, vec![lerp_point(a, end_bc, self.feather_ratio), end_bc]));
    }
    routes
  }
}
impl BandPattern for MedievalBandFeatherTrianglePattern {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;

    // round the cellw to make the exact length
    let n = (length / cellw).round() as usize;
    let cellw = length / (n as f64);

    let mut p = 0.0;
    for _i in 0..n {
      let dy = bandw;
      routes
        .push((clr, vec![(p, dy), (p + cellw / 2.0, -dy), (p + cellw, dy)]));

      routes.extend(self.feather(
        clr,
        (p, dy),
        (p + cellw / 2.0, -dy),
        (p + cellw, dy),
      ));

      routes.extend(self.feather(
        clr,
        (p - cellw / 2.0, -dy),
        (p + cellw / 2.0, -dy),
        (p, dy),
      ));

      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    let cellw = self.cellw * bandw;
    let mut routes = self.feather(
      clr,
      (-bandw, cellw - bandw),
      (-bandw, -bandw),
      (bandw, bandw),
    );
    routes.push((clr, vec![(-bandw, -bandw), (bandw, bandw)]));
    routes
  }
}

struct MedievalBandForkPattern {
  cellw: f64,
  cutx: f64,
  spacex: f64,
  pady: f64,
  simplecorner: bool,
}
impl MedievalBandForkPattern {
  fn new() -> Self {
    Self {
      cellw: 2.0,
      cutx: 0.6,
      spacex: 0.3,
      pady: 0.1,
      simplecorner: false,
    }
  }
}
impl BandPattern for MedievalBandForkPattern {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;
    let cutx = cellw * self.cutx;
    let spacex = self.spacex * cellw;
    let pady = self.pady * bandw;
    let dy = bandw / 2.0 - pady;

    // round the cellw to make the exact length
    // we eat an extra space for the last fork
    let l = length + (cellw - cutx);
    let n = (l / cellw).round() as usize;
    let cellw = l / (n as f64);

    let mut p = 0.0;
    for _i in 0..n {
      routes.push((clr, vec![(p, 0.0), (p + cutx - spacex, 0.0)]));
      routes.push((clr, vec![(p + cutx, 0.0), (p + cellw, 0.0)]));

      routes.push((
        clr,
        vec![(p, -dy), (p + cutx, -dy), (p + cutx, dy), (p, dy)],
      ));

      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    if self.simplecorner {
      return vec![(clr, vec![(bandw, 0.0), (0.0, 0.0), (0.0, bandw)])];
    }
    let sz = bandw * (0.5 - 2.0 * self.pady);
    let rect = vec![(-sz, -sz), (sz, -sz), (sz, sz), (-sz, sz), (-sz, -sz)];
    vec![
      (clr, vec![(bandw, 0.0), (sz, 0.0)]),
      (clr, vec![(0.0, sz), (0.0, bandw)]),
      (clr, rect),
    ]
  }
}

struct MedievalBandConcentric {
  count: usize,
}
impl MedievalBandConcentric {
  fn new(count: usize) -> Self {
    Self { count }
  }
}
impl BandPattern for MedievalBandConcentric {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    for i in 0..self.count {
      let y =
        (i as f64 + 1.0) / (self.count as f64 + 1.0) * (2.0 * bandw) - bandw;
      routes.push((clr, vec![(0.0, y), (length, y)]));
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    for i in 0..self.count {
      let y =
        (i as f64 + 1.0) / (self.count as f64 + 1.0) * (2.0 * bandw) - bandw;
      routes.push((clr, vec![(y, bandw), (y, y), (bandw, y)]));
    }
    routes
  }
}

struct MedievalBandCurvePattern {
  xrep: f64,
  amp: f64,
  inner: f64,
  alt: bool,
}
impl MedievalBandCurvePattern {
  fn new() -> Self {
    Self {
      xrep: 4.0,
      amp: 0.5,
      inner: 0.05,
      alt: false,
    }
  }
}
impl BandPattern for MedievalBandCurvePattern {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let xrep = self.xrep * bandw;
    // round the cellw to make the exact length
    let n = (length / xrep).round() as usize;
    let xrep = length / (n as f64);

    let amp = self.amp * bandw;

    let precision = 0.2;

    let mut curve1 = vec![];
    let mut curve2 = vec![];
    let mut p = 0.0;
    while p < length {
      let phase = 2.0 * PI * p / xrep;
      if self.alt {
        curve1.push((p, amp * phase.sin()));
        curve2.push((p, amp * (phase + PI).sin()));
      } else {
        curve1.push((p, amp * phase.cos()));
        curve2.push((p, amp * (phase + PI).cos()));
      }
      p += precision;
    }
    routes.push((clr, curve1));
    routes.push((clr, curve2));

    let mut p = 0.0;
    let off = if self.alt { 0.25 } else { 0.5 };
    for _i in 0..(2 * n) {
      routes.push((
        clr,
        vec![
          (p + xrep * (off - self.inner), 0.0),
          (p + xrep * (off + self.inner), 0.0),
        ],
      ));
      p += xrep / 2.0;
    }

    routes
  }

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    let d = self.amp * bandw;
    let mut routes = vec![(
      clr,
      vec![
        (0.0, bandw),
        (0.0, 0.0),
        (bandw + self.xrep * bandw * self.inner, 0.0),
      ],
    )];
    if !self.alt {
      routes.push((clr, vec![(-d, bandw), (-d, -d), (bandw, -d)]));
      routes.push((clr, vec![(d, bandw), (d, d), (bandw, d)]));
    }
    routes
  }
}

struct MedievalBandComb {
  cellw: f64,
  twistx: f64,
  pady: f64,
  ysplits: usize,
  comblength: f64,
}
impl MedievalBandComb {
  fn new() -> Self {
    Self {
      cellw: 2.0,
      twistx: 0.4,
      pady: 0.2,
      ysplits: 4,
      comblength: 0.5,
    }
  }
}
impl BandPattern for MedievalBandComb {
  fn pattern(
    &self,
    clr: usize,
    length: f64,
    bandw: f64,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let cellw = self.cellw * bandw;
    let twistx = self.twistx * cellw;
    let pady = self.pady * bandw;
    let ysplits = self.ysplits;
    let comblength = self.comblength * cellw;

    // round the cellw to make the exact length
    let n = (length / cellw).round() as usize;
    let cellw = length / (n as f64);

    let mut p = 0.0;
    for _i in 0..(n + 1) {
      let dy = bandw;
      let maxp = (p + twistx).min(length);
      routes.push((clr, vec![(p, -dy), (maxp, dy)]));
      for j in 0..ysplits {
        let y =
          ((j as f64 + 0.5) / (ysplits as f64) - 0.5) * (2.0 * (bandw - pady));
        let x = mix(p, p + twistx, (y + bandw) / (2.0 * bandw));
        routes.push((clr, vec![(x.min(length), y), (x - comblength, y)]));
      }
      p += cellw;
    }
    routes
  }

  fn corner(&self, clr: usize, bandw: f64) -> Vec<(usize, Vec<(f64, f64)>)> {
    let mut routes = vec![];
    let pady = self.pady * bandw;
    let ysplits = self.ysplits;

    for j in 0..ysplits {
      let y =
        ((j as f64 + 0.5) / (ysplits as f64) - 0.5) * (2.0 * (bandw - pady));
      routes.push((clr, vec![(-bandw, y), (bandw, y)]));
    }
    routes
  }
}

fn framing<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  clr: usize,
  bound: (f64, f64, f64, f64),
  // pattern that will be colored for the framing
  pattern: &dyn BandPattern,
  // padding inside the frame
  padding: f64,
  // marging to exclude external
  margin: f64,
  // stroke width for the pattern
  strokew: f64,
  // density of the coloring
  density: f64,
  // nb of iteration of coloring logic
  iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

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
  let filling = WormsFilling::rand(rng);
  let routes =
    filling.fill_in_paint(rng, &drawings, clr, density, bound, iterations);

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
