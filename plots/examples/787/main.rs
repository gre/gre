use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "148.0")]
  pub width: f64,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
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
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  if peaks.len() == 0 {
    return routes;
  }

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

    let w = maint_width * rng.gen_range(0.5, 0.55);
    let h = maint_roof_height;
    let y = wallcenter.1 - maint_height;
    routes.push(vec![
      (wallcenter.0 - w, y),
      (wallcenter.0, y - h),
      (wallcenter.0 + w, y),
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

  routes
}

fn fairy<R: Rng>(
  origin: (f64, f64),
  radius: f64,
  rotation: f64,
  rng: &mut R,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();

  let scale = radius * 0.2;
  let pos = (0.0, 0.6 * radius);
  let headr = 0.7 * scale;
  let headh = 5.0 * scale;
  let armw = 2.0 * scale;
  let armh = 3.8 * scale;
  let footh = 1.6 * scale;
  let footw = 1.0 * scale;

  let leftarmdy: f64 = rng.gen_range(-2.0, 2.0) * scale;
  let rightarmdy: f64 = rng.gen_range(-2.0, 2.0) * scale;

  let repeat = 1 + (scale * 2.0) as usize;

  let dys: Vec<f64> = (0..repeat)
    .filter_map(|j| {
      if rng.gen_bool(1.0 - 0.9 * ((j as f64) / (repeat as f64)).powf(0.5)) {
        Some(rng.gen_range(-0.3, 0.3) * rng.gen_range(0.0, 1.0) * radius)
      } else {
        None
      }
    })
    .collect();

  for j in 0..repeat {
    for side in vec![-1.0, 1.0] {
      let mut route = Vec::new();
      let headcenter = (pos.0, pos.1 - headh);
      for _i in 0..6 {
        let a = rng.gen_range(-PI, PI);
        route.push((
          headcenter.0 + headr * a.cos(),
          headcenter.1 + headr * a.sin(),
        ));
      }
      route.push(headcenter);
      route = path_subdivide_to_curve_it(route, 0.75);
      route.push((pos.0, pos.1 - footh));
      route.push((pos.0 - side * footw, pos.1));
      route = shake(route, 0.4 * scale, rng);
      route = path_subdivide_to_curve_it(route, 0.8);
      routes.push(route);

      // wing
      if j < dys.len() {
        let dy = dys[j];
        let mut route = Vec::new();
        route.push((-0.2 * side * radius, 0.0));
        route.push((
          rng.gen_range(0.5, 0.6) * side * radius,
          -rng.gen_range(0.1, 0.2) * radius + dy,
        ));
        route.push((rng.gen_range(0.8, 1.0) * side * radius, dy));
        route.push((
          rng.gen_range(0.5, 0.6) * side * radius,
          rng.gen_range(0.1, 0.2) * radius + dy,
        ));
        route.push((-0.2 * side * radius, 0.0));
        route = path_subdivide_to_curve_it(route, 0.8);
        route = path_subdivide_to_curve_it(route, 0.8);
        routes.push(route);
      }
    }

    let mut route = Vec::new();
    route.push((
      pos.0 - armw + 0.3 * leftarmdy.abs(),
      pos.1 - armh + leftarmdy + rng.gen_range(-0.1, 0.1) * scale,
    ));
    let steps = 5;
    for i in 0..steps {
      let f = i as f64 / ((steps - 1) as f64);
      route.push((
        pos.0 + rng.gen_range(0.0, 0.5) * scale,
        pos.1 - mix(armh, footh, f),
      ));
    }
    for i in 0..steps {
      let f = i as f64 / ((steps - 1) as f64);
      route.push((
        pos.0 - rng.gen_range(0.0, 0.5) * scale,
        pos.1 - mix(footh, armh, f),
      ));
    }
    route.push((
      pos.0 + armw - 0.3 * rightarmdy.abs(),
      pos.1 - armh + rightarmdy + rng.gen_range(-0.1, 0.1) * scale,
    ));
    route = path_subdivide_to_curve_it(route, 0.8);
    route = shake(route, 0.3 * scale, rng);
    route = path_subdivide_to_curve_it(route, 0.7);
    routes.push(route);
  }

  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|&p| {
          let p = p_r(p, rotation);
          (scale * p.0 + origin.0, scale * p.1 + origin.1)
        })
        .collect()
    })
    .collect()
}

fn cell(opts: &Opts) -> Vec<(usize, Vec<(f64, f64)>)> {
  let seed = opts.seed;
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let bound = (pad, pad, width - pad, height - pad);

  // Prepare all the random values
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let stopy = rng.gen_range(0.37, 0.55) * height;
  let passage_threshold = 9;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -30.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let precision = 0.21;
  let count = rng.gen_range(3, 9);
  for j in 0..count {
    let peakfactor = rng.gen_range(-0.0003, 0.0006);
    let ampfactor = rng.gen_range(0.02, 0.05);
    let yincr = 0.5;
    let amp2 = rng.gen_range(5.0, 8.0);
    let ynoisefactor = rng.gen_range(0.05, 0.1);
    let offsetstrategy = rng.gen_range(0, 5);

    let stopy =
      mix(height, stopy, (j as f64 / ((count - 1) as f64)) * 0.7 + 0.3);

    // Build the mountains bottom-up, with bunch of perlin noises
    let mut base_y = height * 5.0;
    let mut miny = height;
    loop {
      if miny < stopy {
        break;
      }

      let mut route = Vec::new();
      let mut x = mountainpadding;
      let mut was_outside = true;
      loop {
        if x > width - mountainpadding {
          break;
        }
        let xv = (4.01 - base_y / height) * (x - width / 2.);

        let amp = height * ampfactor;
        let mut y = base_y;

        if offsetstrategy == 0 {
          y += amp * peakfactor * xv * xv;
        }

        y += -amp
          * perlin
            .get([
              //
              xv * 0.005111 + 19.9,
              y * 0.00111 + 30.1,
              77.
                + seed / 7.3
                + perlin.get([
                  //
                  55. + seed * 7.3,
                  80.3 + xv * 0.015,
                  y * 0.2 + 111.3,
                ]),
            ])
            .abs();

        if offsetstrategy == 1 {
          y += amp * peakfactor * xv * xv;
        }

        y += amp2
          * amp
          * perlin.get([
            //
            8.311 + xv * 0.00811,
            88.1 + y * ynoisefactor,
            seed * 97.311,
          ]);

        if offsetstrategy == 2 {
          y += amp * peakfactor * xv * xv;
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
              xv * 0.015 + 8.33,
              88.1 + y * 0.2,
              seed / 7.7 + 6.66,
            ])
            .min(0.0);

        if offsetstrategy == 3 {
          y += amp * peakfactor * xv * xv;
        }

        y += 0.1
          * amp
          * (1.0 - miny / height)
          * perlin.get([
            //
            66.6 + seed * 1.3,
            18.3 + xv * 0.501,
            88.1 + y * 0.503,
          ]);

        if offsetstrategy == 4 {
          y += amp * peakfactor * xv * xv;
        }

        if y < miny {
          miny = y;
        }
        let mut collides = false;
        let xi = ((x - mountainpadding) / precision).round() as usize;
        if xi >= height_map.len() {
          height_map.push(y);
        } else {
          if y > height_map[xi] {
            collides = true;
          } else {
            height_map[xi] = y;
          }
        }
        let inside = !collides && strictly_in_boundaries((x, y), bound);
        if inside && passage.get((x, y)) < passage_threshold {
          if was_outside {
            if route.len() > min_route {
              routes.push((1, route));
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
        routes.push((1, route));
      }

      base_y -= yincr;
    }
  }

  // calculate a moving average
  let smooth = 40;
  let sf = smooth as f64;
  let mut sum = 0.0;
  let mut acc = Vec::new();
  let mut smooth_heights: Vec<(f64, f64, f64)> = Vec::new();
  for (i, h) in height_map.iter().enumerate() {
    if acc.len() == smooth {
      let avg = sum / sf;
      let xtheoric = mountainpadding + (i as f64 - sf / 2.0) * precision;

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

  let sizebase = rng.gen_range(10.0, 20.0);
  let castle_target =
    (1.5 + rng.gen_range(0.0, 8.0) * rng.gen_range(0.0, 1.0)) as usize;
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
      let py = height_map
        [((px - mountainpadding) / precision) as usize % height_map.len()];
      if py > height - pad - 5.0 {
        continue;
      }
      peaks.push((px, py, 0.0));
    }

    ranges.push((left, right));
    castles.push(castle(&peaks, scale, &mut rng, &mut passage));
  }

  for all in castles {
    for r in all {
      routes.push((0, r));
    }
  }

  let radius = rng.gen_range(2.0, 5.0);
  passage.grow_passage(radius);

  let does_overlap = |c: &VCircle| {
    circle_route((c.x, c.y), c.r, 12).iter().all(|&p| {
      passage.get(p) == 0
        && strictly_in_boundaries(p, bound)
        && p.1
          < height_map
            [((p.0 - mountainpadding) / precision) as usize % height_map.len()]
    })
  };

  let f = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
  let amp = rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
  let angbase = rng.gen_range(-2.0, 2.0) * rng.gen_range(0.0, 1.0);

  let ppad = rng.gen_range(0.6, 2.0);
  let total_pad = pad + ppad + radius;
  let min = ppad + rng.gen_range(0.4, 0.6);
  let max = min + rng.gen_range(4.0, 8.0);
  let fairycount =
    (1.0 + rng.gen_range(0.0, 40.0) * rng.gen_range(0.0, 1.0)) as usize;
  let circles = packing(
    seed,
    500000,
    10000,
    1,
    ppad,
    (total_pad, total_pad, width - total_pad, height - total_pad),
    &does_overlap,
    min,
    max,
  );

  let mut fairies = 0;

  for c in circles {
    if c.r > 3.0 && fairies < fairycount {
      let a = angbase + amp * perlin.get([f * c.x, f * c.y, opts.seed]);
      for r in fairy((c.x, c.y), c.r * 0.8, a, &mut rng) {
        routes.push((2, r));
      }
      fairies += 1;
    } else {
      routes.push((2, circle_route((c.x, c.y), c.r.min(1.0), 64)));
    }
  }

  // External frame to around the whole piece
  let mut d = 0.0;
  loop {
    if d > 2.0 {
      break;
    }
    routes.push((
      1,
      vec![
        (pad + d, pad + d),
        (pad + d, height - pad - d),
        (width - pad - d, height - pad - d),
        (width - pad - d, pad + d),
        (pad + d, pad + d),
      ],
    ));
    d += 0.3;
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let routes = cell(opts);

  // Make the SVG
  let colors = vec!["#666", "#207", "#fb0"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (c, route) in routes.clone() {
        if c == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
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

fn packing(
  seed: f64,
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
  let mut rng = rng_from_seed(seed);
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
      let a = rng.gen_range(-PI, PI);
      let amp = rng.gen_range(0.0, scale);
      let dx = amp * a.cos();
      let dy = amp * a.sin();
      (x + dx, y + dy)
    })
    .collect()
}
