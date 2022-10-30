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
  mut route: &mut Vec<(f64, f64)>,
  leftx: f64,
  lefty: f64,
  rightx: f64,
  righty: f64,
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

fn crop_routes_with_predicate(
  input_routes: Vec<(usize, Vec<(f64, f64)>)>,
  should_crop: &dyn Fn((f64, f64)) -> bool,
  cutted_points: &mut Vec<(f64, f64)>,
) -> Vec<(usize, Vec<(f64, f64)>)> {
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

  for (c, input_route) in input_routes {
    if input_route.len() < 2 {
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
          routes.push((c, route));
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
      routes.push((c, route));
    }
  }

  routes
}

fn spawn_miner<R: Rng>(pos: (f64, f64), rng: &mut R) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();

  let mut route = Vec::new();
  for _i in 0..8 {
    route.push((pos.0, pos.1 - 0.7));
  }
  route = shake(route, 1.0, rng);
  route = path_subdivide_to_curve(route, 2, 0.75);
  routes.push(route);

  for _i in 0..2 {
    routes.push(shake(
      vec![(pos.0, pos.1 - 0.6), (pos.0, pos.1 + 1.0)],
      0.2,
      rng,
    ));
  }
  routes.push(shake(
    vec![
      (pos.0 - 1.0, pos.1 + 1.0),
      (pos.0, pos.1 + 0.5),
      (pos.0 + 1.0, pos.1 + 1.0),
    ],
    0.2,
    rng,
  ));
  routes.push(shake(
    vec![(pos.0 - 0.3, pos.1 + 0.5), (pos.0 + 0.3, pos.1 + 0.5)],
    0.2,
    rng,
  ));

  routes
}

fn dig_tunnel<R: Rng>(
  rng: &mut R,
  pos: (f64, f64),
  input_routes: Vec<(usize, Vec<(f64, f64)>)>,
  main_bound: (f64, f64, f64, f64),
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let tunnelh = rng.gen_range(1.6, 2.8);
  let y1 = pos.1 + rng.gen_range(0.0, 20.0);
  let y2 = y1 + tunnelh + rng.gen_range(2.0, 20.0);

  let leftside = rng.gen_bool(0.5);

  let bounds = vec![
    (pos.0 - 0.5 * tunnelh, y1, pos.0 + 0.5 * tunnelh, y2),
    if leftside {
      (0.0, y2 - tunnelh, pos.0 + 0.5 * tunnelh, y2)
    } else {
      (pos.0 + 0.5 * tunnelh, y2 - tunnelh, main_bound.2, y2)
    },
  ];

  let should_crop =
    |p| bounds.iter().any(|&bound| strictly_in_boundaries(p, bound));

  let mut cutted_points = vec![];
  let mut routes =
    crop_routes_with_predicate(input_routes, &should_crop, &mut cutted_points);

  let tunnel_entrance_x1 = if leftside {
    let mut xmin = pos.0;
    for &p in cutted_points.iter() {
      if (p.1 - (y2 - tunnelh)).abs() < 0.5 * tunnelh {
        if p.0 < xmin {
          xmin = p.0;
        }
      }
    }
    xmin
  } else {
    let mut xmin = pos.0;
    for &p in cutted_points.iter() {
      if (p.1 - (y2 - tunnelh)).abs() < 0.5 * tunnelh {
        if p.0 > xmin {
          xmin = p.0;
        }
      }
    }
    xmin
  };
  let tunnel_entrance_x2 = if leftside {
    let mut xmin = pos.0;
    for &p in cutted_points.iter() {
      if (p.1 - y2).abs() < 0.5 * tunnelh {
        if p.0 < xmin {
          xmin = p.0;
        }
      }
    }
    xmin
  } else {
    let mut xmin = pos.0;
    for &p in cutted_points.iter() {
      if (p.1 - y2).abs() < 0.5 * tunnelh {
        if p.0 > xmin {
          xmin = p.0;
        }
      }
    }
    xmin
  };

  let x3 = if leftside {
    pos.0 + 0.5 * tunnelh
  } else {
    pos.0 - 0.5 * tunnelh
  };
  let x4 = if !leftside {
    pos.0 + 0.5 * tunnelh
  } else {
    pos.0 - 0.5 * tunnelh
  };

  routes.push((
    0,
    vec![
      (tunnel_entrance_x1, y2 - tunnelh),
      (x4, y2 - tunnelh),
      (x4, y1),
      (x3, y1),
      (x3, y2),
      (tunnel_entrance_x2, y2),
    ],
  ));

  let mut route = Vec::new();
  for _i in 0..20 {
    route.push((pos.0, y1));
  }
  route = shake(route, tunnelh, rng);
  route = path_subdivide_to_curve(route, 2, 0.75);
  routes.push((0, route));

  for _i in 0..8 {
    let p = if rng.gen_bool(
      (y2 - y1).abs() / ((y2 - y1).abs() + (tunnel_entrance_x2 - x3).abs()),
    ) {
      (pos.0, mix(y2, y1, rng.gen_range(0.05, 0.95)))
    } else {
      (
        mix(tunnel_entrance_x2, x3, rng.gen_range(0.05, 0.95)),
        y2 - tunnelh * 0.5,
      )
    };
    for route in spawn_miner(p, rng) {
      routes.push((0, route));
    }
  }

  let splits =
    (2.0 + rng.gen_range(0.0, 10.0) * rng.gen_range(0.0, 1.0)) as usize;
  for i in 0..splits {
    let x = mix(
      tunnel_entrance_x2,
      x3,
      mix(0.1, 0.85, i as f64 / (splits - 1) as f64),
    ) + rng.gen_range(-5.0, 5.0) * rng.gen_range(0.0, 1.0);
    for _i in 0..2 {
      let x1 = x + rng.gen_range(-tunnelh, tunnelh) * rng.gen_range(0.1, 1.1);
      let x2 = x + rng.gen_range(-tunnelh, tunnelh) * rng.gen_range(0.1, 1.1);
      routes.push((0, vec![(x1, y2), (x2, y2 - tunnelh)]));
    }
  }

  routes
}

fn castle<R: Rng>(
  peaks: &Vec<(f64, f64, f64)>,
  rng: &mut R,
  passage: &mut Passage,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  if peaks.len() == 0 {
    return routes;
  }

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
  let wallheighty = wallcenter.1 - rng.gen_range(2.0, 10.0);
  let towerwidth = rng.gen_range(3.0, 5.0);
  let maint_height = rng.gen_range(10.0, 22.0);
  let maint_width = rng.gen_range(3.0, 7.0);
  let maint_roof_height = rng.gen_range(4.0, 14.0);
  let merlonh = rng.gen_range(1.0, 2.2);

  let d1 = rng.gen_range(0.0, 3.0);
  let h1 = rng.gen_range(3.0, 5.0);

  let leftpeak = peaks[0];
  let leftpeak2 = (leftpeak.0 + towerwidth, ybase(leftpeak.0 + towerwidth));
  let rightpeak = peaks[peaks.len() - 1];
  let rightpeak2 = (rightpeak.0 - towerwidth, ybase(rightpeak.0 - towerwidth));

  // chapel

  let mut route = Vec::new();

  let x = wallcenter.0 + maint_width / 2.0;
  route.push((x, wallheighty));
  route.push((x, wallcenter.1 - maint_height));
  for shadow in wall_shadow(rng, route.clone(), -1.0) {
    routes.push((0, shadow));
  }
  let x = wallcenter.0 - maint_width / 2.0;
  route.push((x, wallcenter.1 - maint_height));
  route.push((x, wallheighty));
  routes.push((0, route));

  let w = maint_width * rng.gen_range(0.5, 0.55);
  let h = maint_roof_height;
  let y = wallcenter.1 - maint_height;
  routes.push((
    0,
    vec![
      (wallcenter.0 - w, y),
      (wallcenter.0, y - h),
      (wallcenter.0 + w, y),
    ],
  ));
  let mut l = 0.0;
  loop {
    if l > 2.0 * w {
      break;
    }
    routes.push((0, vec![(wallcenter.0, y - h), (wallcenter.0 + w - l, y)]));
    l += rng.gen_range(0.3, 0.7) + l / w;
  }

  // cross
  let x = wallcenter.0;
  let y = y - h - 2.0;
  routes.push((0, vec![(x - 0.8, y), (x + 0.8, y)]));
  routes.push((0, vec![(x, y - 1.0), (x, y + 2.0)]));

  // window
  let x = wallcenter.0;
  let y = mix(
    wallcenter.1 - maint_height,
    wallheighty,
    rng.gen_range(0.2, 0.3),
  );
  let w = 0.4;
  let h = 0.6;
  routes.push((
    0,
    vec![
      (x - w, y - h),
      (x + w, y - h),
      (x + w, y + h),
      (x - w, y + h),
      (x - w, y - h),
    ],
  ));

  // wall top
  let mut route = Vec::new();
  route.push((leftpeak2.0, wallheighty));
  merlon(
    &mut route,
    leftpeak2.0,
    wallheighty,
    rightpeak2.0,
    wallheighty,
    merlonh,
  );

  // wall texture
  let xrep = rng.gen_range(2.6, 3.2);
  let yrep = rng.gen_range(1.2, 1.6);
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
      let strokel = rng.gen_range(1.3, 1.5);
      let dx = rng.gen_range(-0.2, 0.2);
      let dy = rng.gen_range(-0.1, 0.1);
      let x1 = (x + dx).max(leftpeak.0).min(rightpeak2.0);
      let x2 = (x + dx + strokel).max(leftpeak.0).min(rightpeak2.0);
      let y1 = y + dy;
      if y1 < ybase(x1) && y1 < ybase(x2) && rng.gen_bool(0.95) {
        routes.push((0, vec![(x1, y + dy), (x2, y + dy)]));
      }
      x += xrep;
    }
    y += yrep;
    alt = !alt;
  }
  routes.push((0, route));

  for (a, b) in vec![
    // Left tower
    ((leftpeak.0, leftpeak.1), leftpeak2),
    // Right tower
    (rightpeak2, (rightpeak.0, rightpeak.1)),
  ] {
    let towerheighty = wallheighty
      - rng.gen_range(1.0, 3.0)
      - rng.gen_range(0.0, 16.0)
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
      routes.push((0, shadow));
    }
    routes.push((0, route));

    let mut y = towerheighty;
    let w = 0.25;
    let h = rng.gen_range(1.0, 1.2);
    loop {
      let x = mix(a.0, b.0, rng.gen_range(0.4, 0.6));
      let lowesty = ybase(x);
      if y > lowesty - 3.0 * h {
        break;
      }
      routes.push((
        0,
        vec![
          (x - w, y - h),
          (x + w, y - h),
          (x + w, y + h),
          (x - w, y + h),
          (x - w, y - h),
        ],
      ));
      y += 4.0 * h;
    }
  }

  for (_c, r) in routes.iter() {
    for p in path_subdivide_to_curve(r.clone(), 2, 0.8) {
      // TODO custom code to do all the lines properly
      passage.count(p);
    }
  }

  routes
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
  let stopy = rng.gen_range(0.4, 0.45) * height;
  let passage_threshold = 9;

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -30.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let precision = 0.21;
  let count = rng.gen_range(2, 8);
  for j in 0..count {
    let peakfactor = rng.gen_range(0.0002, 0.0005);
    let ampfactor = rng.gen_range(0.03, 0.04);
    let yincr = 0.5;
    let amp2 = rng.gen_range(5.0, 8.0);
    let ynoisefactor = rng.gen_range(0.05, 0.1);
    let offsetstrategy = rng.gen_range(0, 2);

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
  let highest = smooth_heights[0];
  let x = highest.0;
  let w = 25.0 + rng.gen_range(-5.0, 15.0) * rng.gen_range(0.0, 1.0);

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

  routes = vec![routes, castle(&peaks, &mut rng, &mut passage)].concat();

  routes = dig_tunnel(&mut rng, (highest.0, highest.1), routes, bound);

  let radius = rng.gen_range(1.0, 2.0);
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

  let f =
    rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let amp = rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
  let angbase = PI / 2.0 + rng.gen_range(-2.0, 2.0) * rng.gen_range(0.0, 1.0);

  for i in 0..rng.gen_range(1, 6) {
    let ppad = rng.gen_range(1.0, 2.0);
    let total_pad = pad + ppad + radius;
    let min = ppad + rng.gen_range(0.5, 1.0);
    let max = min + rng.gen_range(0.0, 4.0) * rng.gen_range(0.0, 1.0);
    let mut circles = packing(
      seed + i as f64 / 0.37,
      500000,
      1000,
      2,
      ppad,
      (total_pad, total_pad, width - total_pad, height - total_pad),
      &does_overlap,
      min,
      max,
    );
    circles.truncate(circles.len() * rng.gen_range(1, 9) / 10);

    for c in circles {
      let a = angbase + amp * perlin.get([f * c.x, f * c.y, opts.seed]);
      let dx = c.r * a.cos();
      let dy = c.r * a.sin();
      routes.push((2, vec![(c.x - dx, c.y - dy), (c.x + dx, c.y + dy)]));
      // let count = (c.r * 2.0 + 8.0) as usize;
      // routes.push((2, circle_route((c.x, c.y), c.r, count)));
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
  let colors = vec!["#000", "#000", "#aaa"];
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
