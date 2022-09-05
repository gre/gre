use clap::*;
use geo::prelude::{BoundingRect, Contains};
use geo::{Point, Polygon};
use gre::*;
use rand::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "1.0")]
  seed: f64,
}

fn lookup(base: &Vec<(f64, f64)>, x: f64) -> f64 {
  for i in 1..base.len() {
    let a = base[i - 1];
    let b = base[i];
    if x <= a.0 {
      return a.1;
    }
    if x < b.0 {
      return mix(a.1, b.1, lerp(a.0, b.0, x));
    }
  }
  return base[base.len() - 1].1;
}

fn draw_cell<R: Rng>(
  rng: &mut R,
  route: &Vec<(f64, f64)>,
  roof: &Vec<(f64, f64)>,
  ceil: &Vec<(f64, f64)>,
) -> Vec<Vec<(f64, f64)>> {
  let safeymin = roof.iter().fold(0f64, |acc, p| acc.max(p.1));
  let safeymax = ceil.iter().fold(1000f64, |acc, p| acc.min(p.1));
  let mut routes = Vec::new();
  let rts = route.clone();
  let poly = Polygon::new(rts.into(), vec![]);
  let bounds = poly.bounding_rect().unwrap();
  let width = bounds.width();
  let height = safeymax - safeymin;
  let v = bounds.min().x_y();
  let x1 = v.0;
  let y1 = safeymin;

  let effective_width = rng.gen_range(0.7, 1.) * width;
  let effective_height =
    (1.0 - 0.5 * rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0)) * height;
  let mut has_windows = false;
  let mut rects = Vec::new();
  if effective_height > 2.0 && effective_width > 2.0 {
    let sz = rng.gen_range(0.0, 1.0);
    let winx = rng.gen_range(0.8, 1.0 + 0.3 * effective_width * sz) as usize;
    let winy = rng.gen_range(0.8, 1.0 + 0.3 * effective_height * sz) as usize;
    let mut w = rng.gen_range(0.4, 0.9) * effective_width / (winx as f64 + 1.0);
    let mut h =
      rng.gen_range(0.4, 0.9) * effective_height / (winy as f64 + 1.0);
    if winx > 1 && winy > 1 && w > 1.0 && h > 1.0 {
      has_windows = true;
      let min = w.min(h);
      w = mix(w, min, rng.gen_range(0.2, 1.0));
      h = mix(h, min, rng.gen_range(0.2, 1.0));
      let modx = rng.gen_range(2, 12);
      let mody = rng.gen_range(2, 12);
      let modx2 = rng.gen_range(2, 8);
      let mody2 = rng.gen_range(2, 8);
      for x in 1..(winx + 1) {
        if x % modx == 0 {
          continue;
        };
        if (1 + x / modx) % modx2 == 0 {
          continue;
        };
        let dx = (width - effective_width) / 2.0;
        let cx = mix(
          x1 + dx,
          x1 + dx + effective_width,
          (x as f64) / (winx as f64 + 1.),
        );
        for y in 1..(winy + 1) {
          if y % mody == 0 {
            continue;
          };
          if (1 + y / mody) % mody2 == 0 {
            continue;
          };
          let cy =
            mix(y1, y1 + effective_height, (y as f64) / (winy as f64 + 1.));
          let rect = vec![
            (cx - w / 2.0, cy - h / 2.0),
            (cx + w / 2.0, cy - h / 2.0),
            (cx + w / 2.0, cy + h / 2.0),
            (cx - w / 2.0, cy + h / 2.0),
            (cx - w / 2.0, cy - h / 2.0),
          ];
          rects.push(Polygon::new(rect.clone().into(), vec![]));
          routes.push(rect);
        }
      }
    }
  }

  if !has_windows && rng.gen_range(0.0, 1.0) < 0.9
    || has_windows && rng.gen_range(0.0, 1.0) < 0.2
  {
    let min = bounds.min().x_y();
    let max = bounds.max().x_y();
    let mut y = min.1;
    let dy = if has_windows {
      rng.gen_range(3.0, 8.0)
    } else {
      rng.gen_range(1.6, 4.0)
    };
    let dx = 0.1;
    loop {
      if y > max.1 {
        break;
      }
      let mut route = Vec::new();
      let mut up = false;
      let mut x = min.0;
      loop {
        if x > max.0 {
          break;
        }
        let py = mix(lookup(roof, x), lookup(ceil, x), lerp(min.1, max.1, y));
        let point = Point::new(x, py);
        if !rects.iter().any(|r| r.contains(&point)) {
          route.push((x, py));
          up = false;
        } else {
          if !up {
            up = true;
            routes.push(route);
            route = Vec::new();
          }
        }
        x += dx;
      }
      routes.push(route);
      y += dy;
    }
  }

  /*
  if !has_windows {
      let r = rng.gen_range(0.0, 1.0);
      let area = poly.signed_area();
      let sampling = (1.0 * area) as usize;
      let dim = 200;
      let samples = samples_polygon(&poly, sampling, dim, rng);
      routes.push(route_spiral(samples));
  }
  */
  routes.push(route.clone());
  routes.push(
    route
      .iter()
      .map(|&(x, y)| {
        let dx = rng.gen_range(-1.0, 1.0) * 0.5;
        let dy = rng.gen_range(-1.0, 1.0) * 0.5;
        (x + dx, y + dy)
      })
      .collect(),
  );

  routes
}

fn rec<R: Rng>(
  rng: &mut R,
  depth: usize,
  base: &Vec<(f64, f64)>,
  maxy: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let first = base[0];
  let last = base[base.len() - 1];
  let r = rng.gen_range(0.0, 1.0) < 0.05;
  if depth <= 0 || r {
    if r {
      let x = (first.0 + last.0) / 2.0;
      let y = lookup(base, x);
      let y2 = (y - rng.gen_range(8., 16.)).max(maxy);
      routes.push(vec![(x, y), (x, y2)]);
    }
    return routes;
  }
  let splits = if depth > 0 {
    (rng.gen_range(0., 0.08 * (last.0 - first.0)) * rng.gen_range(0.0, 1.0))
      as usize
  } else {
    0
  };

  let mut xs = vec![first.0, last.0];
  for _s in 0..splits {
    let i = rng.gen_range(1, xs.len());
    let a = xs[i - 1];
    let b = xs[i];
    if a >= b {
      println!("{:#?}", base);
    }
    xs.insert(i, rng.gen_range(a, b));
  }
  for i in 1..xs.len() {
    let a = xs[i - 1];
    let b = xs[i];
    let w = b - a;
    if w > rng.gen_range(3.0, 9.0) {
      let left = a + rng.gen_range(0.0, 0.2) * w;
      let right = b - rng.gen_range(0.0, 0.2) * w;
      let leftp = (left, lookup(base, left));
      let rightp = (right, lookup(base, right));
      let height = rng.gen_range(1.0, 20.0)
        + rng.gen_range(0., 60.) * rng.gen_range(0.0, 1.0);
      let lefth = (leftp.0, leftp.1 - height - rng.gen_range(-1.0, 4.0));
      let righth = (rightp.0, rightp.1 - height - rng.gen_range(-1.0, 4.0));
      if lefth.1 < maxy || righth.1 < maxy {
        continue;
      }
      let mut nextbase = Vec::new();
      nextbase.push(lefth);
      let count =
        (rng.gen_range(0., 4. + 0.2 * w) * rng.gen_range(0., 1.)) as usize;
      for i in 0..count {
        let p = (i as f64 + 1.) / (count as f64 + 1.);
        let x = mix(lefth.0, righth.0, p);
        let mut y = mix(lefth.1, righth.1, p);
        let f = mix((righth.0 - lefth.0) / 100.0, 1.0, rng.gen_range(0.0, 1.0));
        y += rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 1.0)
          * rng.gen_range(0.0, 10.0)
          * f;
        y -= rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 20.0) * f;
        y = y.max(maxy);
        nextbase.push((x, y));
      }
      nextbase.push(righth);

      let mut route = Vec::new();
      route.push(leftp);
      for &p in nextbase.iter() {
        route.push(p);
      }
      route.push(rightp);
      let fill = draw_cell(rng, &route, &nextbase, &base);
      for r in fill {
        routes.push(r);
      }
      let r = rec(rng, depth - 1, &nextbase, maxy);
      for route in r {
        routes.push(route);
      }
    }
  }

  routes
}

fn art(opts: Opts) -> Vec<Group> {
  let width = 297.;
  let height = 210.;
  let pad = 16.0;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();

  let ground = vec![(pad, height - pad), (width - pad, height - pad)];
  let shapes = rec(&mut rng, 8, &ground, pad);

  for i in 0..3 {
    routes.push(
      ground
        .iter()
        .map(|&(x, y)| {
          let dy = (i as f64 + rng.gen_range(-0.5, 0.5)) * 0.2;
          (x, y + dy)
        })
        .collect(),
    );
  }

  for route in shapes {
    routes.push(route);
  }

  let color = "black";
  let mut data = Data::new();
  for route in routes.iter() {
    data = render_route(data, route.clone());
  }
  let mut l = layer(color);
  l = l.add(base_path(color, 0.2, data));
  vec![l]
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
