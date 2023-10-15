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
  let mut paint = PaintMask::new(0.2, width, height);
  paint.paint_borders(pad);

  let ybase = height - pad;
  let ystart = rng.gen_range(0.8, 0.9) * height;
  let pos = (0.5 * width, ystart);
  let w = width * rng.gen_range(0.6, 0.8);

  let mut routes = vec![];

  let filling = WormsFilling::rand(&mut rng);

  let count = rng.gen_range(2, 6);
  let h = ybase - ystart;
  for i in 0..count {
    let xincr = 1.0;
    let y = ybase;
    let divmin = count as f64 * 0.3;
    let divmax = count as f64 * 1.0;
    let yamp = (i as f64 + 1.0) * h / rng.gen_range(divmin, divmax);
    let perlin = Perlin::new();
    let density = 1.0 + 3.0 * ((count - i) as f64) / (count as f64);
    let iterations = rng.gen_range(1000, 4000);
    let clr = 0;

    let f1 = rng.gen_range(0.01, 0.05) * rng.gen_range(0.0, 1.0);
    let amp2 = rng.gen_range(0.0, 2.0) * rng.gen_range(0.0, 1.0);
    let f2 = rng.gen_range(0.0, 0.1) * rng.gen_range(0.0, 1.0);
    let amp3 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
    let f3 = rng.gen_range(0.0, 0.2) * rng.gen_range(0.0, 1.0);
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

    routes.extend(filled_mountains(
      &mut rng,
      &mut paint,
      pad,
      width - pad,
      xincr,
      y,
      yamp,
      &valuef,
      &filling,
      iterations,
      density,
      clr,
    ));
  }

  let scalef = rng.gen_range(0.0, 2.0);

  let minw = 5.0;
  let padlayer = rng.gen_range(5.0, 20.0);
  let yincrlayer = rng.gen_range(10.0, 20.0);
  let mut y = ystart;
  let mut w = w;

  let interbase = rng.gen_range(4.0, 10.0);
  let interincr = rng.gen_range(0.0, 5.0);

  let ptowers = rng.gen_range(0.4, 0.8);
  let pchapel = rng.gen_range(0.0, 1.0) * rng.gen_range(0.1, 1.0);
  let pdarkchapel = rng.gen_range(0f64, 1.0).powf(8.0);
  let pwall = rng.gen_range(0.2, 1.0);

  let count = rng.gen_range(3, 20);
  for yi in 0..count {
    let scale = 1.0 + scalef / (yi as f64 + 1.0);
    let inter = interbase + interincr * yi as f64;
    let splits = if yi == 0 { 1 } else { rng.gen_range(2, 8) };
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
      let mut props = CastleProps {
        left_tower: yi == 0 || rng.gen_bool(ptowers),
        right_tower: yi == 0 || rng.gen_bool(ptowers),
        chapel: rng.gen_bool(if yi == 0 { 0.8 } else { pchapel }),
        dark_chapel: rng.gen_bool(pdarkchapel),
        wall: yi == 0 || rng.gen_bool(pwall),
      };
      if !props.left_tower && !props.right_tower {
        props.wall = false;
      }
      routes.extend(castle(
        &mut rng,
        &mut paint,
        ybase,
        (x, y),
        innerw,
        scale,
        0,
        &props,
      ));
    }
    y -= yincrlayer;
    w -= 2. * padlayer;
  }

  let c = (
    width * (0.5 + rng.gen_range(-0.3, 0.3) * rng.gen_range(0.0, 1.0)),
    height * (0.5 + rng.gen_range(-0.4, 0.2) * rng.gen_range(0.0, 1.0)),
  );
  let r = rng.gen_range(0.1, 0.2) * width.min(height);

  routes.extend(sun(&mut paint, c, r, 0.6, 1));

  vec!["#000", "#f93"]
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
    routes.extend(WormsFilling::rand(rng).fill(
      rng,
      &|_x, _y| 3.0,
      (
        pos.0 - width / 2.0,
        pos.1 - height,
        pos.0 + width / 2.0,
        ybase,
      ),
      clr,
      2000,
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

  // clip and paint
  let is_outside = |p| paint.is_painted(p);
  routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }

  routes
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

  // clip and paint
  let is_outside = |p| paint.is_painted(p);
  routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
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

  // clip and paint
  let is_outside = |p| paint.is_painted(p);
  routes = clip_routes_with_colors(&routes, &is_outside, 0.3, 5);
  for poly in polys.iter() {
    paint.paint_polygon(poly);
  }

  routes
}

struct CastleProps {
  left_tower: bool,
  right_tower: bool,
  chapel: bool,
  dark_chapel: bool,
  wall: bool,
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

  let wallh = scale * rng.gen_range(8.0, 14.0);
  let wallcenter = pos;

  let towerwidth = scale * rng.gen_range(3.0, 5.0);
  let maint_height = scale * rng.gen_range(14.0, 24.0);
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

fn filled_mountains<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  xfrom: f64,
  xto: f64,
  xincr: f64,
  ybase: f64,
  yamp: f64,
  valuef: &dyn Fn(f64, f64) -> f64,
  filling: &WormsFilling,
  iterations: usize,
  density: f64,
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

  // routes.push((clr, curve.clone()));

  // make the polygons
  let mut polys = vec![];
  let len = curve.len();
  for i in 0..len {
    let j = (i + 1) % len;
    let mut poly = vec![];
    let a = curve[i];
    let b = curve[j];
    poly.push(a);
    poly.push(b);
    poly.push((b.0, ybase));
    poly.push((a.0, ybase));
    polys.push(poly);
  }

  // fill them
  let f = |x, y| {
    let collides = polys
      .iter()
      .any(|poly| polygon_includes_point(poly, (x, y)));
    if collides {
      density
    } else {
      0.0
    }
  };
  let extra = 2.0;
  let bound: (f64, f64, f64, f64) = (
    xfrom - extra,
    ybase - yamp - extra,
    xto + extra,
    ybase + extra,
  );
  routes.extend(filling.fill(rng, &f, bound, clr, iterations));

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
    let rot = PI / rng.gen_range(1.0, 4.0);
    let step = 0.6;
    let straight = rng.gen_range(0.0, 0.2);
    let min_l = 5;
    let max_l = rng.gen_range(10, 80);
    let decrease_value = 1.0;
    let search_max = 500;
    let min_weight = 1.0;
    let freq = 0.02;
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
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "297")]
  height: f64,
  #[clap(short, long, default_value = "210")]
  width: f64,
  #[clap(short, long, default_value = "10")]
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

// TODO more efficient algorithm would be to paint on a mask.

#[derive(Clone)]
struct PaintMask {
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

  fn add_weight(&mut self, p: (f64, f64), v: f64) {
    let x = p.0 / self.precision;
    let y = p.1 / self.precision;
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let i = y0 * self.w + x0;
    self.weights[i] += v;
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
