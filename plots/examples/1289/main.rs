use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut rng = rng_from_seed(opts.seed);
  let precision = 0.2;
  let mut paint = PaintMask::new(precision, width, height);
  paint.paint_borders(pad);

  let ybase = height - pad;
  let ystart = rng.gen_range(0.8, 0.9) * height;
  let pos = (0.5 * width, ystart);
  let w = width * rng.gen_range(0.7, 0.85);

  let mut routes = vec![];

  // trebuchets
  let count = (1. + rng.gen_range(0., 8.) * rng.gen_range(0.0, 1.0)) as usize;
  for _i in 0..count {
    let x = rng.gen_range(0.2, 0.8) * width;
    let y = height
      - pad
      - rng.gen_range(0.0, height - ystart) * rng.gen_range(0.0, 1.0);
    let h = rng.gen_range(15.0, 22.0);
    let xflip = rng.gen_bool(0.5);
    let percent = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
    routes.extend(trebuchet(
      &mut rng,
      &mut paint,
      (x, y),
      h,
      percent,
      xflip,
      1,
    ));
  }

  // mini mountains
  let count = rng.gen_range(2, 12);
  let h = ybase - ystart;
  for i in 0..count {
    let xincr = 1.0;
    let y = ybase;
    let divmin = count as f64 * 0.3;
    let divmax = count as f64 * 0.6;
    let yamp = (i as f64 + 1.0) * h / rng.gen_range(divmin, divmax);
    let perlin = Perlin::new();
    let clr = 1;

    let f1 = rng.gen_range(0.01, 0.03) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);
    let f2 = rng.gen_range(0.0, 0.05) * rng.gen_range(0.0, 1.0);
    let amp3 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
    let f3 = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);
    let seed1 = rng.gen_range(0.0, 100.0);
    let seed2 = rng.gen_range(0.0, 100.0);
    let seed3 = rng.gen_range(0.0, 100.0);

    let valuef = |x, y| {
      let n = 0.5
        + 0.5
          * perlin.get([
            f1 * x,
            f1 * y,
            amp2
              * perlin.get([
                f2 * x,
                seed2 + amp3 * perlin.get([seed3, f3 * x, f3 * y]),
                f2 * y,
              ])
              + seed1
              + i as f64 * 55.5,
          ]);
      n
    };

    routes.extend(stroke_mountains(
      &mut paint, 0.0, width, xincr, y, yamp, &valuef, clr,
    ));
  }

  let scalef = rng.gen_range(0.0, 2.0);

  let minw = 14.0;
  let yincrlayer = rng.gen_range(6.0, 16.0);
  let mut y = ystart - height * rng.gen_range(0.05, 0.1);
  let mut w = w;

  let interbase = rng.gen_range(4.0, 6.0);
  let basescale = rng.gen_range(0.9, 1.3);

  let ptowers = rng.gen_range(0.0, 1.0);
  let pchapel = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let pdarkchapel = rng.gen_range(0.2, 0.5);
  let pdarkwall = (pdarkchapel * rng.gen_range(0.8f64, 1.2)).min(0.99);
  let pwall = rng.gen_range(0.0, 1.0);
  let pwalldestructed = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
  let pduelist = rng.gen_range(0.05, 0.3);

  let count = rng.gen_range(80, 120);
  for yi in 0..count {
    let padlayer = rng.gen_range(0.0, 20.0);
    let interincr = rng.gen_range(0.0, 3.0);

    if y < 0.2 * height {
      break;
    }
    let scale = basescale + 0.5 * scalef / (yi as f64 + 1.0);
    let inter = interbase + interincr * yi as f64;
    let splits = if yi == 0 {
      (rng.gen_range(0.0, 3.0) * rng.gen_range(0.0, 1.0)) as usize + 1
    } else {
      rng.gen_range(1, 6)
    };
    let inter_s = if splits == 1 {
      0.
    } else {
      inter * (splits as f64) / (splits as f64 - 1.) - inter
    };
    for xi in 0..splits {
      let innerw = if splits == 1 { w } else { w / (splits as f64) };
      if innerw < minw {
        continue;
      }
      if rng.gen_bool(0.1) {
        continue;
      }
      let x = pos.0 - w / 2.
        + innerw * (xi as f64 + 0.5)
        + inter_s * (xi as f64 - 0.5);
      let wallh = scale * rng.gen_range(8.0, 14.0);

      let wall = yi == 0 || rng.gen_bool(pwall);
      let destructed_wall = wall
        && rng.gen_bool(if yi == 0 {
          0.2 * pwalldestructed
        } else {
          pwalldestructed
        });
      let portcullis = !destructed_wall
        && wall
        && rng.gen_bool(if yi == 0 { 0.9 } else { 0.3 });

      let props = CastleProps {
        left_tower: yi == 0 || rng.gen_bool(ptowers),
        right_tower: yi == 0 || rng.gen_bool(ptowers),
        chapel: rng.gen_bool(pchapel),
        dark_chapel: rng.gen_bool(pdarkchapel),
        wall,
        dark_wall: rng.gen_bool(pdarkwall),
        destructed_wall,
        portcullis,
        duelist_on_wall: wall && rng.gen_bool(pduelist),
        wallh,
      };

      let size = rng.gen_range(8.0, 10.0);

      if props.duelist_on_wall {
        let cx = x + 0.2 * rng.gen_range(-0.5, 0.5) * innerw;
        routes.extend(army_fighter(
          &mut rng,
          &mut paint,
          false,
          (cx - 0.5 * size, y - wallh),
          size,
          1,
        ));
        routes.extend(army_fighter(
          &mut rng,
          &mut paint,
          true,
          (cx + 0.5 * size, y - wallh),
          size,
          1,
        ));
      }

      routes.extend(castle(
        &mut rng,
        &mut paint,
        ybase,
        (x, y),
        innerw,
        scale,
        1,
        &props,
      ));
    }
    y -= yincrlayer * rng.gen_range(0.5, 1.0);
    w -= 2. * padlayer;
  }

  // spawn eagles

  let count = rng.gen_range(1, 10);

  let circles = packing(
    &mut rng,
    vec![],
    100000,
    count,
    1,
    0.0,
    (0.0, 0.0, width, height),
    &|_c| true,
    3.0,
    5.0,
  );

  for c in circles {
    let ang = rng.gen_range(-PI, PI) * rng.gen_range(0.0, 0.1);
    let xreverse = rng.gen_bool(0.5);
    routes.extend(eagle(
      &mut rng,
      &mut paint,
      (c.x, c.y),
      c.r,
      ang,
      xreverse,
      1,
    ));
  }

  // spawn clouds
  let does_overlap = |_c: &VCircle| true;

  for _i in 0..rng.gen_range(0, 10) {
    let count = 1
      + (rng.gen_range(0., 40.) * rng.gen_range(0., 1.) * rng.gen_range(0., 1.))
        as usize;
    let bound = (0.0, 0.0, width, height * rng.gen_range(0.2, 1.0));
    let min = rng.gen_range(2.0, 10.0);
    let max = min + rng.gen_range(0.0, height * 0.2) * rng.gen_range(0., 1.);
    let circles = packing(
      &mut rng,
      vec![],
      100000,
      count,
      1,
      0.0,
      bound,
      &does_overlap,
      min,
      max,
    );

    for c in circles {
      routes.extend(cloud_in_circle(&mut rng, &mut paint, &c, 0));
    }
  }

  let c = (
    width * (0.5 + rng.gen_range(-0.3, 0.3) * rng.gen_range(0.0, 1.0)),
    height * (0.5 + rng.gen_range(-0.4, 0.2) * rng.gen_range(0.0, 1.0)),
  );
  let r = rng.gen_range(0.1, 0.2) * width.min(height);

  routes.extend(sun(&mut paint, c, r, 0.6, 2));
  routes.extend(sun(&mut paint, c, height.max(width) * 0.8, 2.0, 2));

  // border
  routes.push((
    1,
    vec![
      (pad, pad),
      (width - pad, pad),
      (width - pad, height - pad),
      (pad, height - pad),
      (pad, pad),
    ],
  ));

  /*
  let mut rng = rng_from_seed(opts.seed);
  let sq = vec![
    (0.3 * width, 0.3 * height),
    (0.3 * width, 0.7 * height),
    (0.7 * width, 0.7 * height),
    (0.7 * width, 0.3 * height),
    (0.3 * width, 0.3 * height),
  ];
  let (routes, polys) = binary_cut_and_slide(
    &vec![(
      0,
      vec![
        (0.3 * width, 0.3 * height),
        (0.3 * width, 0.7 * height),
        (0.7 * width, 0.7 * height),
        (0.7 * width, 0.3 * height),
        (0.3 * width, 0.3 * height),
      ],
    )],
    &vec![sq],
    (width * 0.5, height * 0.5),
    rng.gen_range(-PI, PI) * rng.gen_range(0.0, 1.0),
    10.0,
    3.0,
  );
  let filling = WormsFilling::rand(&mut rng);
  let mut routes = routes;
  for poly in polys {
    //    routes.push((0, poly));
    let f = |x, y| {
      if is_inside_a_polygon((x, y), &poly) {
        2.0
      } else {
        0.
      }
    };
    routes.extend(filling.fill(
      &mut rng,
      &f,
      (pad, pad, width - pad, height - pad),
      0,
      1000,
    ));
  }
  */

  vec!["#888", "#000", "#f93"]
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(color);
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

// TODO more OO. each object should be a struct with args, so we can represent everything as a big struct

fn castle_chapel<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  // ybase is where the chapel foundation need to start
  ybase: f64,
  // center of the chapel base
  pos: (f64, f64),
  width: f64,
  height: f64,
  scale: f64,
  dark_fill: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let mut polys = vec![];
  let mut route = Vec::new();

  let roof_height = scale * rng.gen_range(4.0, 14.0);

  let x = pos.0 + width / 2.0;
  route.push((x, ybase));
  route.push((x, pos.1 - height));

  if dark_fill {
    let its = rng.gen_range(800, 1200);
    routes.extend(WormsFilling::rand(rng).fill(
      rng,
      &|_x, _y| 1.5,
      (
        pos.0 - width / 2.0,
        pos.1 - height,
        pos.0 + width / 2.0,
        ybase,
      ),
      clr,
      its,
    ));
  } else {
    for shadow in wall_shadow(rng, route.clone(), -scale) {
      routes.push((clr, shadow));
    }
  }
  let x = pos.0 - width / 2.0;
  route.push((x, pos.1 - height));
  route.push((x, ybase));
  routes.push((clr, route));

  // boundaries of chapel body
  polys.push(vec![
    (pos.0 - width / 2.0, ybase),
    (pos.0 + width / 2.0, ybase),
    (pos.0 + width / 2.0, pos.1 - height),
    (pos.0 - width / 2.0, pos.1 - height),
  ]);

  let w = width * rng.gen_range(0.5, 0.55);
  let h = roof_height;
  let y = pos.1 - height;
  routes.push((clr, vec![(pos.0 - w, y), (pos.0, y - h), (pos.0 + w, y)]));

  // boundaries of chapel roof
  polys.push(vec![(pos.0 - w, y), (pos.0, y - h), (pos.0 + w, y)]);
  let mut l = 0.0;
  loop {
    if l > 2.0 * w {
      break;
    }
    routes.push((clr, vec![(pos.0, y - h), (pos.0 + w - l, y)]));
    l += scale * rng.gen_range(0.7, 1.0) + l / w;
  }

  // cross
  let x = pos.0;
  let y = y - h - 2.0;
  routes.push((clr, vec![(x - scale * 0.8, y), (x + scale * 0.8, y)]));
  routes.push((clr, vec![(x, y - scale * 1.0), (x, y + scale * 2.0)]));

  // window
  let x = pos.0;
  let y = mix(pos.1 - height, pos.1, rng.gen_range(0.2, 0.3));
  let w = scale * 0.4;
  let h = scale * 0.6;
  routes.push((
    clr,
    vec![
      (x - w, y - h),
      (x + w, y - h),
      (x + w, y + h),
      (x - w, y + h),
      (x - w, y - h),
    ],
  ));

  let pushbackbase =
    rng.gen_range(0.0, 0.04) * rng.gen_range(0.0, 1.0) * height;
  let pushbackrotbase = rng.gen_range(-1.0, 1.0);
  let pushbackrotmix = rng.gen_range(0.1, 0.9);
  let sliding = scale * rng.gen_range(0.5, 2.0);
  let (routes, polys) = multicut_along_line(
    rng,
    &routes,
    &polys,
    clr,
    pos,
    (pos.0, pos.1 - height),
    |rng| rng.gen_range(2.0, 10.0),
    |rng| rng.gen_range(-PI / 2.0, PI / 2.0) * rng.gen_range(0.0, 1.0),
    |rng| sliding * rng.gen_range(-1.0, 1.0) * rng.gen_range(0.0, 1.0),
    |rng| pushbackbase * rng.gen_range(0.5, 2.0),
    |rng| 0.1 * mix(pushbackrotbase, rng.gen_range(-1.0, 1.0), pushbackrotmix),
  );

  // clip and paint
  let is_outside = |p| paint.is_painted(p);
  let mut routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }

  // make ropes behind the construction
  let count = rng.gen_range(3, 16);
  routes.extend(building_ropes(
    rng,
    paint,
    &polys,
    count,
    clr,
    2.0 * width,
    height + 50.0,
  ));

  routes
}

fn building_ropes<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  polys: &Vec<Vec<(f64, f64)>>,
  count: usize,
  clr: usize,
  width: f64,
  height: f64,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let weights = polys.iter().map(|_p| rng.gen_range(0.0, 1.0)).collect();
  let mut pts = polys
    .iter()
    .map(|p| poly_centroid_weighted(p, &weights))
    .collect::<Vec<_>>();
  rng.shuffle(&mut pts);
  pts.truncate(count);
  let mut ropes = vec![];
  for p in pts {
    let rt = vec![
      p,
      (
        p.0 + 2.0 * rng.gen_range(-1.0, 1.0) * width,
        p.1 + height + rng.gen_range(0.0, 50.0),
      ),
    ];

    ropes.push((clr, rt));
  }
  regular_clip(&ropes, paint)
}

fn regular_clip(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let is_outside = |p| paint.is_painted(p);
  clip_routes_with_colors(&routes, &is_outside, 0.3, 5)
}

fn regular_clip_polys(
  routes: &Vec<(usize, Vec<(f64, f64)>)>,
  paint: &mut PaintMask,
  polys: &Vec<Vec<(f64, f64)>>,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let rts = regular_clip(routes, paint);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }
  rts
}

fn castle_wall<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  // ybase is where the chapel foundation need to start
  ybase: f64,
  // center of the wall base
  pos: (f64, f64),
  width: f64,
  height: f64,
  scale: f64,
  props: &CastleProps,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let mut polys = vec![];

  let merlonh = scale * rng.gen_range(1.0, 2.2);

  let left = (pos.0 - width / 2., ybase);
  let right = (pos.0 + width / 2., ybase);
  let wallheighty = pos.1 - height;

  let mut route = Vec::new();
  polys.push(vec![
    (left.0, ybase),
    (left.0, wallheighty + merlonh),
    (right.0, wallheighty + merlonh),
    (right.0, ybase),
  ]);
  route.push(left);
  route.push((left.0, wallheighty));
  merlon(
    &mut polys,
    &mut route,
    left.0 + 0.01,
    wallheighty,
    right.0 - 0.01,
    wallheighty,
    merlonh,
  );
  route.push(right);
  routes.push((clr, route));

  // wall texture
  if props.dark_wall {
    let its = rng.gen_range(3000, 5000);
    let density = rng.gen_range(1.0, 3.0);
    routes.extend(WormsFilling::rand(rng).fill(
      rng,
      &|x, y| {
        if is_inside_polygons((x, y), &polys) {
          density
        } else {
          0.0
        }
      },
      (
        pos.0 - width / 2.0,
        pos.1 - height,
        pos.0 + width / 2.0,
        ybase,
      ),
      clr,
      its,
    ));
  } else {
    let xrep = scale * rng.gen_range(2.6, 3.2);
    let yrep = scale * rng.gen_range(1.2, 1.6);
    let mut alt = false;
    let mut y = wallheighty + merlonh + yrep;
    loop {
      if y > ybase {
        break;
      }
      let mut x = left.0;
      if alt {
        x += xrep / 2.0;
      }
      loop {
        if x > right.0 {
          break;
        }
        let strokel = scale * rng.gen_range(1.3, 1.5);
        let dx = scale * rng.gen_range(-0.2, 0.2);
        let dy = scale * rng.gen_range(-0.1, 0.1);
        let x1 = (x + dx).max(left.0).min(right.0);
        let x2 = (x + dx + strokel).max(left.0).min(right.0);
        let y1 = y + dy;
        if y1 < ybase && y1 < ybase && rng.gen_bool(0.95) {
          routes.push((clr, vec![(x1, y + dy), (x2, y + dy)]));
        }
        x += xrep;
      }
      y += yrep;
      alt = !alt;
    }
  }

  if props.destructed_wall {
    (routes, polys) = multicut_along_line(
      rng,
      &routes,
      &polys,
      clr,
      (left.0, wallheighty),
      (right.0, wallheighty),
      |rng| rng.gen_range(8.0, 16.0),
      |rng| {
        0.5
          * rng.gen_range(-PI / 2.0, PI / 2.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0)
      },
      |rng| height * rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
      |rng| rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
      |_rng| 0.0,
    );
  }

  if props.portcullis {
    let x = pos.0;
    let h = width.min(ybase - wallheighty) * rng.gen_range(0.7, 0.85);
    let w = h * rng.gen_range(0.3, 0.5);
    let r = w / 2.0;

    let door = vec![
      vec![
        (x + w / 2., ybase),
        (x - w / 2., ybase),
        (x - w / 2., ybase - h + r),
      ],
      arc((x, ybase - h + r), r, -PI, 0.0, 32),
      vec![(x + w / 2., ybase - h + r), (x + w / 2., ybase)],
    ]
    .concat();

    let mut grids = vec![];
    let r = rng.gen_range(0.08, 0.14) * w;
    let ybottom = mix(ybase, ybase - h, rng.gen_range(0.0, 1.0));
    let mut xp = x - w / 2.0;
    let extra = 1.5;
    while xp < x + w / 2.0 {
      let grid = vec![(xp, ybottom + extra), (xp, ybase - h)];
      grids.push((clr, grid));
      xp += r;
    }
    let mut yp = ybase - h;
    while yp < ybottom {
      let grid = vec![(x - w / 2., yp), (x + w / 2., yp)];
      grids.push((clr, grid));
      yp += r;
    }

    // carve into the door
    routes = clip_routes_with_colors(
      &routes,
      &|p| polygon_includes_point(&door, p),
      0.3,
      5,
    );
    // add the door
    routes.push((clr, door.clone()));
    routes.extend(clip_routes_with_colors(
      &grids,
      &|p| !polygon_includes_point(&door, p),
      0.3,
      5,
    ));
  }

  // clip and paint
  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }

  routes
}

fn castle_tower<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  // ybase is where the chapel foundation need to start
  ybase: f64,
  // center of the wall base
  pos: (f64, f64),
  width: f64,
  height: f64,
  scale: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  let mut polys = vec![];

  let a = (pos.0 - width / 2., ybase);
  let b = (pos.0 + width / 2., ybase);

  let towerheighty = pos.1 - height;

  let d1 = scale * rng.gen_range(0.0, 3.0);
  let h1 = scale * rng.gen_range(3.0, 5.0);
  let merlonh = scale * rng.gen_range(1.0, 2.2);

  let mut route: Vec<(f64, f64)> = Vec::new();
  route.push(a);
  route.push((a.0, towerheighty));
  route.push((a.0 - d1, towerheighty - d1));
  route.push((a.0 - d1, towerheighty - d1 - h1));
  merlon(
    &mut polys,
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
    (a.0, a.1),
    (b.0, b.1),
    (b.0, towerheighty),
    (a.0, towerheighty),
  ]);

  // boundaries of the tower head
  polys.push(vec![
    (a.0, towerheighty),
    (b.0, towerheighty),
    (b.0 + d1, towerheighty - d1),
    (b.0 + d1, towerheighty - d1 - h1 + merlonh),
    (a.0 - d1, towerheighty - d1 - h1 + merlonh),
    (a.0 - d1, towerheighty - d1),
  ]);

  let right_side_path = vec![
    (b.0 + d1, towerheighty - d1 - h1),
    (b.0 + d1, towerheighty - d1),
    (b.0, towerheighty),
    b,
  ];
  for shadow in wall_shadow(rng, right_side_path, scale) {
    routes.push((clr, shadow));
  }
  routes.push((clr, route));

  // windows
  let mut y = towerheighty;
  let w = scale * 0.25;
  let h = scale * rng.gen_range(1.0, 1.2);
  loop {
    let x = mix(a.0, b.0, rng.gen_range(0.4, 0.6));
    let lowesty = pos.1;
    if y > lowesty - 3.0 * h {
      break;
    }
    routes.push((
      clr,
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

  let pushbackrotbase = rng.gen_range(-1.0, 1.0);
  let pushbackrotmix = 1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);

  let (routes, polys) = multicut_along_line(
    rng,
    &routes,
    &polys,
    clr,
    pos,
    (pos.0, towerheighty),
    |rng| rng.gen_range(4.0, 6.0),
    |rng| {
      rng.gen_range(-PI / 2.0, PI / 2.0)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0)
    },
    |rng| scale * rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
    |rng| rng.gen_range(0.0, 1.0),
    |rng| 0.1 * mix(pushbackrotbase, rng.gen_range(-1.0, 1.0), pushbackrotmix),
  );

  // clip and paint
  let is_outside = |p| paint.is_painted(p);
  let mut routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  let mut topy = 99999.;
  for poly in polys.iter() {
    paint.paint_polygon(poly);
    let c = poly_centroid(poly);
    if c.1 < topy {
      topy = c.1;
    }
  }

  // top fighter
  if rng.gen_bool(0.3) {
    let toppolys: Vec<_> = polys
      .iter()
      .map(poly_centroid)
      .filter(|c| c.1 < topy + 5.0)
      .collect();

    if toppolys.len() > 0 {
      let c = poly_centroid(&toppolys);
      let xreverse = rng.gen_bool(0.5);
      let sz = rng.gen_range(5.0, 8.0);
      routes.extend(army_fighter(rng, paint, xreverse, c, sz, clr));
    }
  }

  // make ropes behind the construction
  let count = rng.gen_range(3, 16);
  routes.extend(building_ropes(
    rng,
    paint,
    &polys,
    count,
    clr,
    2.0 * width,
    2.0 * height + 50.0,
  ));

  routes
}

struct CastleProps {
  left_tower: bool,
  right_tower: bool,
  chapel: bool,
  dark_chapel: bool,
  wall: bool,
  destructed_wall: bool,
  portcullis: bool,
  dark_wall: bool,
  duelist_on_wall: bool,
  wallh: f64,
}

fn castle<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  // ybase is where the chapel foundation need to start
  ybase: f64,
  // center of the castle base
  pos: (f64, f64),
  width: f64,
  scale: f64,
  clr: usize,
  props: &CastleProps,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  let wallcenter = pos;
  let wallh = props.wallh;

  let towerwidth = scale * rng.gen_range(3.0, 6.0);
  let maint_height = scale * rng.gen_range(14.0, 32.0);
  let maint_width = scale * rng.gen_range(4.0, 8.0);

  for (p, skip) in vec![
    (
      (pos.0 - width / 2. + towerwidth / 2., pos.1),
      !props.left_tower,
    ),
    (
      (pos.0 + width / 2. - towerwidth / 2., pos.1),
      !props.right_tower,
    ),
  ] {
    if skip {
      continue;
    }
    let towerheight = wallh + scale * rng.gen_range(4.0, 8.0);
    routes.extend(castle_tower(
      rng,
      paint,
      ybase,
      p,
      towerwidth,
      towerheight,
      scale,
      clr,
    ));
  }

  if props.wall {
    routes.extend(castle_wall(
      rng,
      paint,
      ybase,
      wallcenter,
      width - towerwidth * 2.,
      wallh,
      scale,
      &props,
      clr,
    ));
  }

  // chapel
  if props.chapel {
    routes.extend(castle_chapel(
      rng,
      paint,
      ybase,
      (wallcenter.0, wallcenter.1 - maint_height),
      maint_width,
      maint_height,
      scale,
      props.dark_chapel,
      clr,
    ));
  }

  routes
}

fn merlon(
  polys: &mut Vec<Vec<(f64, f64)>>,
  route: &mut Vec<(f64, f64)>,
  leftx: f64,
  lefty: f64,
  rightx: f64,
  _righty: f64,
  h: f64,
) {
  let mut count = ((rightx - leftx) / h).ceil();
  count = (count / 2.0).floor() * 2.0 + 1.0;
  if count <= 0.0 {
    return;
  }
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
      if route.len() > 1 {
        let last = route[route.len() - 1];
        let minx = last.0;
        let miny = last.1;
        let maxx = x;
        let maxy = y + h;
        polys.push(vec![
          (minx, miny),
          (maxx, miny),
          (maxx, maxy),
          (minx, maxy),
        ]);
      }
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

    l += rng.gen_range(0.8, 1.2) * stroke_len.abs();
  }
}

fn sun(
  paint: &mut PaintMask,
  c: (f64, f64),
  r: f64,
  dr: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let routes = vec![
    (clr, spiral_optimized(c.0, c.1, r, dr, 0.1)),
    (clr, circle_route(c, r, (r * 2. + 8.) as usize)),
  ];
  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  paint.paint_circle(c.0, c.1, r);
  routes
}

fn stroke_mountains(
  paint: &mut PaintMask,
  xfrom: f64,
  xto: f64,
  xincr: f64,
  ybase: f64,
  yamp: f64,
  valuef: &dyn Fn(f64, f64) -> f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];

  // sample the curve with f
  let mut curve = vec![];
  let mut x = xfrom;
  while x < xto {
    let y = ybase - yamp * valuef(x, ybase);
    curve.push((x, y));
    x += xincr;
  }
  if x > xto {
    let y = ybase - yamp * valuef(xto, ybase);
    curve.push((xto, y));
  }

  if curve.len() < 2 {
    return routes;
  }

  // make the polygons
  let mut polys = vec![];
  let len = curve.len();
  for j in 1..len {
    let i = j - 1;
    let mut poly = vec![];
    let a = curve[i];
    let b = curve[j];
    poly.push(a);
    poly.push(b);
    poly.push((b.0, ybase));
    poly.push((a.0, ybase));
    polys.push(poly);
  }

  routes.push((clr, curve.clone()));

  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }
  routes
}

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
    let step = 0.5;
    let straight = rng.gen_range(0.0, 0.1);
    let min_l = 5;
    let max_l = 20;
    let decrease_value = 1.;
    let search_max = 500;
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

  fn fill<R: Rng>(
    &self,
    rng: &mut R,
    f: &dyn Fn(f64, f64) -> f64,
    bound: (f64, f64, f64, f64),
    clr: usize,
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
            let rt = rdp(&route, 0.05);
            // remap
            let rt =
              rt.iter().map(|p| (p.0 + bound.0, p.1 + bound.1)).collect();
            routes.push((clr, rt));
          }
        }
      }
    }

    routes
  }
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "516.0")]
  seed: f64,
  #[clap(short, long, default_value = "400")]
  width: f64,
  #[clap(short, long, default_value = "300")]
  height: f64,
  #[clap(short, long, default_value = "20")]
  pad: f64,
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

#[derive(Clone)]
pub struct PaintMask {
  mask: Vec<bool>,
  precision: f64,
  width: f64,
  height: f64,
}

impl PaintMask {
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
    let precision = self.precision;
    let wi = (self.width / precision) as usize;
    let hi = (self.height / precision) as usize;
    let x = ((point.0.max(0.) / precision) as usize).min(wi - 1);
    let y = ((point.1.max(0.) / precision) as usize).min(hi - 1);
    self.mask[x + y * wi]
  }

  fn paint_circle(&mut self, cx: f64, cy: f64, cr: f64) {
    let (minx, miny, maxx, maxy) = (
      (cx - cr).max(0.),
      (cy - cr).max(0.),
      (cx + cr).min(self.width),
      (cy + cr).min(self.height),
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
        if euclidian_dist(point, (cx, cy)) < cr {
          self.mask[x + y * wi] = true;
        }
      }
    }
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
    let minx = ((minx).max(0.).min(self.width) / precision) as usize;
    let miny = ((miny).max(0.).min(self.height) / precision) as usize;
    let maxx =
      ((maxx + precision).max(0.).min(self.width) / precision) as usize;
    let maxy =
      ((maxy + precision).max(0.).min(self.height) / precision) as usize;
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

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn clip_routes_with_colors(
  input_routes: &Vec<(usize, Vec<(f64, f64)>)>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
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

  for (clrp, input_route) in input_routes.iter() {
    let clr = *clrp;
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
              routes.push((clr, route));
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
      routes.push((clr, route));
    }
  }

  routes
}

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

pub fn trebuchet_people<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  xreverse: bool,
  origin: (f64, f64),
  humansize: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let joints = HumanJointAngles::rand(rng, xreverse);
  let xcenter = origin.0;
  let human = HumanBody::new((xcenter, origin.1), humansize, joints);

  let mut new_routes = vec![];

  let (headpos, headangle) = human.head_pos_angle();

  let h = head(paint, headpos, headangle, humansize, clr);

  new_routes.extend(h);

  new_routes.extend(human.render(paint, clr));

  new_routes
}

pub fn army_fighter<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  xreverse: bool,
  origin: (f64, f64),
  size: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let joints = HumanJointAngles::rand(rng, xreverse);
  let humansize = size * 0.5;
  let xcenter = origin.0;
  let human = HumanBody::new((xcenter, origin.1), humansize, joints);

  let (shield_p, (object_p, object_a)) = if xreverse {
    (human.elbow_right, human.hand_left_pos_angle())
  } else {
    (human.elbow_left, human.hand_right_pos_angle())
  };

  let mut new_routes = vec![];

  let s = shield(rng, paint, shield_p, size * 0.5, 0.0, clr);
  new_routes.extend(s);

  new_routes.extend(human.render(paint, clr));
  let (headpos, headangle) = human.head_pos_angle();

  if rng.gen_bool(0.5) {
    let h = full_helmet(paint, headpos, headangle, humansize, xreverse, clr);
    new_routes.extend(h);
  } else {
    let h = head(paint, headpos, headangle, humansize, clr);
    new_routes.extend(h);
  }
  // sword / shield

  let sw = sword(rng, paint, object_p, size * 0.5, object_a, clr);

  new_routes.extend(sw);

  new_routes
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

impl HumanJointAngles {
  pub fn rand<R: Rng>(rng: &mut R, xreverse: bool) -> Self {
    HumanJointAngles {
      body_angle: -PI / 2.0
        + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
      head_angle: -PI / 2.0
        + rng.gen_range(-0.5, 0.5) * rng.gen_range(0.0, 1.0),
      shoulder_right_angle: if xreverse || rng.gen_bool(0.2) {
        0.0
      } else {
        PI
      } + rng.gen_range(-1.0, 1.0),
      shoulder_left_angle: if xreverse || rng.gen_bool(0.2) {
        PI
      } else {
        0.0
      } + rng.gen_range(-1.0, 1.0),
      elbow_right_angle: rng.gen_range(-1.0, 2.0),
      elbow_left_angle: PI / 2.0 + rng.gen_range(-0.8, 2.0),
      hip_right_angle: PI / 2.0 - rng.gen_range(0.0, 1.0),
      hip_left_angle: PI / 2.0 + rng.gen_range(0.0, 1.0),
      knee_right_angle: PI / 2.0 - rng.gen_range(-0.5, 0.5),
      knee_left_angle: PI / 2.0 - rng.gen_range(-0.5, 0.5),

      left_arm_bend: if xreverse {
        1.0
      } else {
        rng.gen_range(0.0, 1.0)
      },
      right_arm_bend: if xreverse {
        rng.gen_range(0.0, 1.0)
      } else {
        1.0
      },
      left_leg_bend: 1.0,
      right_leg_bend: 1.0,
    }
  }
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

  fn render(
    &self,
    paint: &mut PaintMask,
    clr: usize,
  ) -> Vec<(usize, Vec<(f64, f64)>)> {
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

    routes.push((clr, vec![hip, shoulder, head]));

    routes.push((clr, vec![shoulder, shoulder_right, elbow_right]));
    routes.push((clr, vec![shoulder, shoulder_left, elbow_left]));

    routes.push((clr, vec![hip, hip_right, knee_right]));
    routes.push((clr, vec![hip, hip_left, knee_left]));

    regular_clip(&routes, paint)
  }
}

fn full_helmet(
  paint: &mut PaintMask,
  origin: (f64, f64),
  angle: f64,
  size: f64,
  xreverse: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = 0.4 * size;
  let extrax = 0.1 * size;
  routes.push(vec![
    (-dx, 0.0),
    (-dx, -h),
    (dx, -h),
    (dx + extrax, -0.5 * h),
    (dx, 0.0),
    (-dx, 0.0),
  ]);

  routes.push(vec![
    (dx + extrax, -0.5 * h),
    (0.2 * dx, -1.3 * h),
    (0.2 * dx, 0.3 * h),
  ]);
  routes.push(vec![(-dx, -0.5 * h), (dx + 0.6 * extrax, -0.4 * h)]);
  routes.push(vec![(-dx, -0.5 * h), (dx + 0.6 * extrax, -0.6 * h)]);

  let ang = angle + PI / 2.0;
  // translate and rotate routes
  regular_clip(
    &routes
      .iter()
      .map(|route| {
        (
          clr,
          route
            .iter()
            .map(|&(x, y)| {
              let x = if xreverse { -x } else { x };
              let (x, y) = p_r((x, y), ang);
              (x + origin.0, y + origin.1)
            })
            .collect(),
        )
      })
      .collect(),
    paint,
  )
}

fn sword<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  origin: (f64, f64),
  size: f64,
  angle: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
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
  regular_clip(
    &routes
      .iter()
      .map(|route| {
        (
          clr,
          route
            .iter()
            .map(|&(x, y)| {
              let (x, y) = p_r((x, y), angle);
              (x + origin.0, y + origin.1)
            })
            .collect(),
        )
      })
      .collect(),
    paint,
  )
}

fn shield<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  origin: (f64, f64),
  size: f64,
  angle: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  let dx = 0.2 * size;
  let dy = 0.4 * size;
  let mut route = vec![];
  let mut route2 = vec![];
  for v in vec![
    (0.0, -dy),
    (0.5 * dx, -dy),
    (
      dx,
      -(1.0 - rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0)) * dy,
    ),
    (dx, 0.0),
    (dx, rng.gen_range(0.0, 1.0) * dy),
    (0.0, dy),
  ] {
    route.push(v);
    route2.push((-v.0, v.1));
  }
  route2.reverse();
  route.extend(route2);

  route = route_translate_rotate(&route, origin, angle);
  let polygons = vec![route.clone()];
  routes.push((clr, route));

  let tick = rng.gen_range(0.2, 0.3);
  let y = rng.gen_range(-0.2, 0.0) * dy;
  routes.push((
    clr,
    route_translate_rotate(
      &vec![(0.0, -tick * dy + y), (tick * dx, y), (0.0, tick * dy + y)],
      origin,
      angle,
    ),
  ));

  regular_clip_polys(&routes, paint, &polygons)
}

fn head(
  _paint: &mut PaintMask,
  origin: (f64, f64),
  angle: f64,
  size: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let dx = 0.13 * size;
  let h = 0.4 * size;
  let route = vec![(-dx, 0.0), (-dx, -h), (dx, -h), (dx, 0.0), (-dx, 0.0)];

  let ang = angle + PI / 2.0;

  let route = route
    .iter()
    .map(|&(x, y)| {
      let (x, y) = p_r((x, y), ang);
      (x + origin.0, y + origin.1)
    })
    .collect::<Vec<_>>();

  let routes = vec![(clr, route)];

  // FIXME something not working here
  // let polys = vec![route.clone()];
  // let routes = regular_clip_polys(&routes, paint, &polys);

  routes
}

fn grow_stroke_zigzag(
  from: (f64, f64),
  to: (f64, f64),
  width: f64,
  line_dist: f64,
) -> Vec<(f64, f64)> {
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

fn proj_point(origin: (f64, f64), angle: f64, distance: f64) -> (f64, f64) {
  let (x, y) = origin;
  let s = angle.sin();
  let c = angle.cos();
  (x + distance * c, y + distance * s)
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

fn same_point(a: (f64, f64), b: (f64, f64)) -> bool {
  (a.0 - b.0).abs() < 0.0001 && (a.1 - b.1).abs() < 0.0001
}

// a simple implementation of cutting a convex polygon in 2 with a line
fn cut_polygon(
  poly: &Vec<(f64, f64)>,
  a: (f64, f64),
  b: (f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  let poly = if !same_point(poly[0], poly[poly.len() - 1]) {
    let mut poly = poly.clone();
    poly.push(poly[0]);
    poly
  } else {
    poly.clone()
  };
  let mut prev: Option<(f64, f64)> = None;
  let mut first = Vec::new();
  let mut second = Vec::new();
  let mut on_first = true;
  for p in poly.clone() {
    let to = p;
    if let Some(from) = prev {
      let collision = collides_segment(from, to, a, b);
      if let Some(c) = collision {
        first.push(c);
        second.push(c);
        on_first = !on_first;
      }
    }
    if on_first {
      first.push(to);
    } else {
      second.push(to);
    }
    prev = Some(to);
  }
  if second.len() < 2 {
    return vec![poly.clone()];
  }
  return vec![first, second];
}

fn binary_cut_and_slide(
  routes_in: &Vec<(usize, Vec<(f64, f64)>)>,
  polys_in: &Vec<Vec<(f64, f64)>>,
  center: (f64, f64),
  ang: f64,
  sliding: f64,
  pushback: f64,
  pushback_rotation: f64,
  clr: usize,
) -> (Vec<(usize, Vec<(f64, f64)>)>, Vec<Vec<(f64, f64)>>) {
  let mut routes = vec![];
  let mut polys = vec![];

  let dx = ang.cos();
  let dy = ang.sin();
  let amp = 1000.0;
  let a = (center.0 + amp * dx, center.1 + amp * dy);
  let b = (center.0 - amp * dx, center.1 - amp * dy);

  let is_left =
    |(x, y)| (x - center.0) * (b.1 - a.1) - (y - center.1) * (b.0 - a.0) > 0.0;

  let is_right = |p| !is_left(p);

  let project = |(x, y), leftmul| {
    let local = (x - center.0, y - center.1);
    let local = p_r(local, pushback_rotation * leftmul);

    (
      center.0 + local.0 + (sliding * dx - pushback * dy) * leftmul,
      center.1 + local.1 + (sliding * dy + pushback * dx) * leftmul,
    )
  };

  for poly in polys_in.clone() {
    let out = cut_polygon(&poly, a, b);
    for p in out {
      let mut c = (0., 0.);
      for point in p.iter() {
        c.0 += point.0;
        c.1 += point.1;
      }
      let len = p.len() as f64;
      c = (c.0 / len, c.1 / len);

      let leftmul = if is_left(c) { 1.0 } else { -1.0 };

      let p = p.iter().map(|&p| project(p, leftmul)).collect();

      polys.push(p);
    }
  }

  let mut left_routes = clip_routes_with_colors(&routes_in, &is_right, 0.3, 4);
  let mut right_routes = clip_routes_with_colors(&routes_in, &is_left, 0.3, 4);

  let out_of_polys = |p| !is_inside_polygons(p, polys_in);

  let cut_routes =
    clip_routes_with_colors(&vec![(clr, vec![a, b])], &out_of_polys, 0.3, 4);

  left_routes.extend(cut_routes.clone());
  right_routes.extend(cut_routes.clone());

  let data = vec![(1.0, left_routes), (-1.0, right_routes)];
  for (leftmul, rts) in data {
    for (clr, rt) in rts {
      let newrt = rt.iter().map(|&p| project(p, leftmul)).collect();
      routes.push((clr, newrt));
    }
  }

  (routes, polys)
}

fn is_inside_polygons(p: (f64, f64), polygons: &Vec<Vec<(f64, f64)>>) -> bool {
  for polygon in polygons {
    if is_inside_a_polygon(p, polygon) {
      return true;
    }
  }
  false
}

fn is_inside_a_polygon(p: (f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
  let mut inside = false;
  let mut j = polygon.len() - 1;
  for i in 0..polygon.len() {
    let pi = polygon[i];
    let pj = polygon[j];
    if (pi.1 > p.1) != (pj.1 > p.1)
      && p.0 < (pj.0 - pi.0) * (p.1 - pi.1) / (pj.1 - pi.1) + pi.0
    {
      inside = !inside;
    }
    j = i;
  }
  inside
}

fn multicut_along_line<R: Rng>(
  rng: &mut R,
  routes_in: &Vec<(usize, Vec<(f64, f64)>)>,
  polys_in: &Vec<Vec<(f64, f64)>>,
  clr: usize,
  from: (f64, f64),
  to: (f64, f64),
  mut increment_f: impl FnMut(&mut R) -> f64,
  mut angle_delta_f: impl FnMut(&mut R) -> f64,
  mut sliding_f: impl FnMut(&mut R) -> f64,
  mut pushback_f: impl FnMut(&mut R) -> f64,
  mut pushback_rotation_f: impl FnMut(&mut R) -> f64,
) -> (Vec<(usize, Vec<(f64, f64)>)>, Vec<Vec<(f64, f64)>>) {
  let mut routes = routes_in.clone();
  let mut polys = polys_in.clone();
  let initial = increment_f(rng) / 2.0;
  let mut d = initial;
  let l = euclidian_dist(from, to);
  let dx = to.0 - from.0;
  let dy = to.1 - from.1;
  let a = dy.atan2(dx);
  while d < l - initial {
    let p = lerp_point(from, to, d / l);
    let ang = a + PI / 2.0 + angle_delta_f(rng);
    let sliding = sliding_f(rng);
    let pushback = pushback_f(rng);
    let pushback_rotation = pushback_rotation_f(rng);
    let o = binary_cut_and_slide(
      &routes,
      &polys,
      p,
      ang,
      sliding,
      pushback,
      pushback_rotation,
      clr,
    );
    routes = o.0;
    polys = o.1;
    d += increment_f(rng);
  }
  (routes, polys)
}

fn cloud_in_circle<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  circle: &VCircle,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
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
      input_routes.push((clr, arc((x, y), r, start, end, count)));
      r -= dr;
    }

    routes.extend(clip_routes_with_colors(&input_routes, &should_crop, 0.3, 4));

    circles.push(circle);
  }

  let is_outside = |p| paint.is_painted(p);
  let routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for c in circles {
    paint.paint_circle(c.x, c.y, c.r);
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
  initial_circles: Vec<VCircle>,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  bound: (f64, f64, f64, f64),
  does_overlap: &dyn Fn(&VCircle) -> bool,
  min_scale: f64,
  max_scale: f64,
) -> Vec<VCircle> {
  let mut circles = initial_circles.clone();
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

fn eagle<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  origin: (f64, f64),
  sz: f64,
  rotation: f64,
  xreverse: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
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

  let mut circles = vec![];
  let border = 1.2;

  // scale, rotate & translate
  let out = routes
    .iter()
    .map(|route| {
      (
        clr,
        route
          .iter()
          .map(|&p| {
            let p = p_r(p, rotation);
            let p = (xmul * scale * p.0 + origin.0, scale * p.1 + origin.1);
            circles.push((p.0, p.1, border));
            p
          })
          .collect(),
      )
    })
    .collect();
  let out = regular_clip(&out, paint);
  for (x, y, r) in circles {
    paint.paint_circle(x, y, r);
  }
  out
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

fn trebuchet<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  origin: (f64, f64),
  height: f64,
  action_percent: f64,
  xflip: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = Vec::new();

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
  routes.push((clr, route));

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
  routes.push((clr, route));

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
    routes.push((clr, route));
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
  routes.push((clr, route));

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
  routes.push((clr, route));

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
          routes.push((clr, route));
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
    routes.push((clr, route));
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
      routes.push((clr, route));
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
      routes.push((clr, route));
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
      routes.push((clr, route));
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
  routes.push((clr, vec![pivot1, p]));
  routes.push((clr, vec![center2, p, center1]));

  let mut r = line_width;
  while r > line_dist / 2.0 {
    routes.push((clr, circle_route(center, r, 16)));
    r -= 0.8;
  }

  // rope to attach the beam on a wheel

  let wheel_radius = 0.04 * height;
  let wheel_center = (
    origin.0 - 0.2 * xmul * height,
    origin.1 - wheel_radius - 0.06 * height,
  );
  routes.push((
    clr,
    vec![
      (wheel_center.0, origin.1),
      wheel_center,
      (wheel_center.0 - 0.1 * xmul * height, origin.1),
    ],
  ));

  let mut r = 0.3;
  while r < wheel_radius {
    routes.push((clr, circle_route(wheel_center, r, 10)));
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
    routes.push((clr, vec![a, beam_anchor_half, b]));
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

  routes.push((clr, ropes));

  for _i in 0..rng.gen_range(1, 5) {
    let p = (
      origin.0
        + (if xflip { 1. } else { -1. })
          * if rng.gen_bool(0.3) {
            rng.gen_range(1.0 * height, 1.8 * height)
          } else {
            rng.gen_range(-1.0 * height, -0.3 * height)
          },
      origin.1,
    );
    routes.extend(trebuchet_people(rng, paint, xflip, p, 0.2 * height, clr));
  }

  routes = regular_clip(&routes, paint);

  routes
}

fn poly_centroid(poly: &Vec<(f64, f64)>) -> (f64, f64) {
  let mut c = (0., 0.);
  let mut l = poly.len();
  if same_point(poly[0], poly[l - 1]) {
    l -= 1;
  }
  for i in 0..l {
    let a = poly[i];
    c.0 += a.0;
    c.1 += a.1;
  }
  c.0 /= l as f64;
  c.1 /= l as f64;
  c
}

fn poly_centroid_weighted(
  poly: &Vec<(f64, f64)>,
  weights: &Vec<f64>,
) -> (f64, f64) {
  let mut c = (0., 0.);
  let mut l = poly.len();
  if same_point(poly[0], poly[l - 1]) {
    l -= 1;
  }
  let mut sum = 0.;
  for i in 0..l {
    let a = poly[i];
    let w = weights[i];
    c.0 += w * a.0;
    c.1 += w * a.1;
    sum += w;
  }
  c.0 /= sum;
  c.1 /= sum;
  c
}
