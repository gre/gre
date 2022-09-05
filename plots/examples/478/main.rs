use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::{path::Data, *};

/*
TODO
- work on possibility to animate the heads with a slight inner rotations of the noise warping
- ?ref à la gif loop de la danseuse caché qlq part dans le dessin?
- ?ref game of life loop
*/

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "38.0")]
  seed: f64,
  #[clap(short, long, default_value = "420.0")]
  width: f64,
  #[clap(short, long, default_value = "297.0")]
  height: f64,
  #[clap(short, long, default_value = "10")]
  seconds: i64,
  #[clap(short, long, default_value = "0")]
  index: usize,
  #[clap(short, long, default_value = "4")]
  frames: usize,
}

fn human_circle(
  radius: f64,
  pos: (f64, f64),
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut circle_path = Vec::new();
  let count = 64;
  for i in 0..(count + 1) {
    let a = 2. * PI * i as f64 / (count as f64);
    circle_path.push((pos.0 + radius * a.cos(), pos.1 + radius * a.sin()));
  }
  cordon(circle_path, 2.0, 2.0, 1.0, 5, true, 1.0, phase)
}
fn human_square(
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let square_path = vec![(x1, y1), (x2, y1), (x2, y2), (x1, y2), (x1, y1)];
  cordon(square_path, 2.0, 2.0, 1.0, 5, true, 1.0, phase)
}
fn human_path_chaos(
  seed: f64,
  path: Vec<(f64, f64)>,
  pos: (f64, f64),
  radius: f64,
  freq: f64,
  phase: f64,
) -> Vec<(f64, f64)> {
  let mut route = path;
  let perlin = Perlin::new();
  let amp = 20.0;
  let mut s = seed;
  for _i in 0..3 {
    route = path_curve(route, 1);
    route = route
      .iter()
      .enumerate()
      .map(|(i, &p)| {
        let dx = pos.0 - p.0;
        let dy = pos.1 - p.1;
        let d = (dx * dx + dy * dy).sqrt() / radius;
        let mut a = (1.1 - d) * amp;
        if i == 0 || i == route.len() - 1 {
          a /= 3.0;
        }
        let n1 = a
          * perlin.get([
            7.7 + 5. * s + 0.2 * (phase * 2.0 * PI).cos(),
            freq * p.0,
            freq * p.1,
          ]);
        let n2 = a
          * perlin.get([
            3.3 - 3. * s + 0.2 * (phase * 2.0 * PI).sin(),
            freq * p.0,
            freq * p.1,
          ]);
        (p.0 + n1, p.1 + n2)
      })
      .collect();
    s = 7.7 + s / 3.0;
  }
  route
}
fn human_body(
  seed: f64,
  radius: f64,
  pos: (f64, f64),
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let m = radius * 2.0;
  let p = (pos.0 - radius, pos.1 - radius);
  let hand_up_x = p.0 + m * 70.0 / 620.0;
  let hand_up_y = p.1 + m * 110.0 / 620.0;
  let hand_normal_x = p.0 + m * 50. / 620.;
  let hand_normal_y = p.1 + m * 200.0 / 620.0;
  let foot_up_x = p.0 + m * 160.0 / 620.0;
  let foot_up_y = p.1 + m * 580.0 / 620.0;
  let foot_normal_x = p.0 + m * 280.0 / 620.0;
  let foot_normal_y = p.1 + m * 620.0 / 620.0;
  let symx = |x| 2. * pos.0 - x;
  let path1 = vec![
    (hand_normal_x, hand_normal_y),
    (p.0, hand_normal_y + 0.05 * radius),
    (symx(hand_normal_x), hand_normal_y),
  ];
  let path2 = vec![
    (hand_up_x, hand_up_y),
    (pos.0, hand_normal_y + 0.1 * radius),
    (pos.0 - 0.08 * radius, pos.1 + 0.1 * radius),
    (symx(foot_up_x), foot_up_y),
  ];
  let path3: Vec<(f64, f64)> =
    path2.iter().map(|&(x, y)| (symx(x), y)).collect();
  let path4 = vec![
    (foot_normal_x, foot_normal_y),
    (foot_normal_x - 0.05 * radius, pos.1 + 0.2 * radius),
    (symx(foot_normal_x), pos.1),
    (pos.0 + 0.05 * radius, hand_normal_y - 0.15 * radius),
  ];
  let path5: Vec<(f64, f64)> =
    path4.iter().map(|&(x, y)| (symx(x), y)).collect();
  let w = 2.0;
  let noiseamp = w;
  let corner_pad = 2.0;
  let tracks_count = 5;
  let freq = 0.03;
  let iterations = 7;
  let mut all = Vec::new();
  let mut s = seed;
  for _i in 0..iterations {
    all = vec![
      all,
      cordon(
        human_path_chaos(s, path1.clone(), pos, radius, freq, phase),
        w,
        noiseamp,
        corner_pad,
        tracks_count,
        false,
        0.5,
        phase,
      ),
      cordon(
        human_path_chaos(s, path2.clone(), pos, radius, freq, phase),
        w,
        noiseamp,
        corner_pad,
        tracks_count,
        false,
        0.5,
        phase,
      ),
      cordon(
        human_path_chaos(s, path3.clone(), pos, radius, freq, phase),
        w,
        noiseamp,
        corner_pad,
        tracks_count,
        false,
        0.5,
        phase,
      ),
      cordon(
        human_path_chaos(s, path4.clone(), pos, radius, freq, phase),
        w,
        noiseamp,
        corner_pad,
        tracks_count,
        false,
        0.5,
        phase,
      ),
      cordon(
        human_path_chaos(s, path5.clone(), pos, radius, freq, phase),
        w,
        noiseamp,
        corner_pad,
        tracks_count,
        false,
        0.5,
        phase,
      ),
    ]
    .concat();
    s = -s * 3.3 + 7.7;
  }
  vec![
    head(
      10.0,
      0,
      pos.0,
      pos.1 - 0.72 * radius,
      0.28 * radius,
      1.5,
      phase,
    ),
    all,
  ]
  .concat()
}

fn head(
  seed: f64,
  i: usize,
  cx: f64,
  cy: f64,
  r: f64,
  countmul: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let precision = 0.3;
  let w = (4. * r / precision) as u32;
  let h = (4. * r / precision) as u32;
  let perlin = Perlin::new();
  let f = |(x, y): (f64, f64)| -> f64 {
    let dx: f64 = x - 0.5;
    let dy: f64 = y - 0.5;
    let mut res: f64 = (dx * dx + dy * dy).sqrt();
    let xabs = (x - 0.5).abs();
    let f1 = 1.0;
    let f2 = 1.8;
    let f3 = 3.6;
    res += 0.06
      * perlin.get([
        //
        seed
          + 0.8 * (i as f64)
          + 3.
            * perlin.get([
              // 2nd level
              2. * f2 * xabs,
              f2 * y,
              seed - 0.4 * (i as f64)
                + 0.1 * (phase * 2.0 * PI).sin()
                + 0.2 * x
                + 3.
                  * perlin.get([
                    // 3rd level
                    seed + 0.1 * (i as f64) + 0.08 * (phase * 2.0 * PI).cos(),
                    f3 * xabs,
                    f3 * y,
                  ]),
            ]),
        f1 * xabs,
        f1 * y + 0.3 * x,
      ]);
    res * 4.5
  };
  let samples = 4 + (r * 1.4 * countmul) as usize;
  let thresholds: Vec<f64> = (0..samples)
    .map(|i| (i as f64 + 1.0) / (samples as f64))
    .collect();
  let res = contour(w, h, f, &thresholds);
  let mut routes = features_to_routes(res, precision);
  routes = crop_routes(&routes, (1.0, 1.0, 4. * r - 1., 4. * r - 1.));
  routes = translate_routes(routes, (cx - 2. * r, cy - 2. * r));
  routes
}

fn organ(
  _seed: f64,
  i: usize,
  size: f64,
  angle: f64,
  (cx, cy): (f64, f64),
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let x = cx;
  let y = cy;
  let r = size;
  routes = vec![routes, head(10.0, i, x, y, r, 1.0, phase)].concat();
  // apply rotation
  routes
    .iter()
    .map(|route| {
      route
        .iter()
        .map(|p| {
          let (x, y) = p_r((p.0 - cx, p.1 - cy), angle);
          (x + cx, y + cy)
        })
        .collect()
    })
    .collect()
}

fn cordon(
  path: Vec<(f64, f64)>,
  width: f64,
  noiseamp: f64,
  corner_pad: f64,
  tracks_count: usize,
  reconnect: bool,
  freq_mul: f64,
  phase: f64,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let precision = 0.5;
  let r = precision;
  let mut pindex = 0;
  let mut p = path[pindex];
  let perlin = Perlin::new();
  let mut tracks = Vec::new();
  for _xi in 0..tracks_count {
    tracks.push(Vec::new());
  }
  for &next in path.iter().skip(1) {
    let dx = next.0 - p.0;
    let dy = next.1 - p.1;
    let a = dy.atan2(dx);
    let mut i = 0.0;
    let acos = a.cos();
    let asin = a.sin();
    let mut dist = (dx * dx + dy * dy).sqrt();
    if pindex != 0 {
      dist -= corner_pad;
      p.0 += corner_pad * acos;
      p.1 += corner_pad * asin;
    }
    if pindex == path.len() - 1 {
      dist -= corner_pad;
    }
    loop {
      if i >= dist {
        p = next;
        break;
      }
      p.0 += r * acos;
      p.1 += r * asin;
      for xi in 0..tracks_count {
        let variation = ((xi as f64 + (tracks_count as f64 * phase))
          % (tracks_count as f64)
          - ((tracks_count - 1) as f64 / 2.0))
          / (tracks_count as f64);
        let mut delta = variation * width;
        let noisefreq = freq_mul * (0.1 + 0.2 * (0.5 - variation.abs()));
        delta += noiseamp
          * perlin.get([
            //
            noisefreq * p.0,
            noisefreq * p.1,
            10.0 * xi as f64,
          ]);
        let a2 = a + PI / 2.0;
        let q = (p.0 + delta * a2.cos(), p.1 + delta * a2.sin());
        tracks[xi].push(q);
      }
      i += r;
    }
    pindex += 1;
  }
  for track in tracks {
    let mut track_copy = track.clone();
    if reconnect {
      track_copy.push(track[0]);
    }
    routes.push(track_copy);
  }
  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let seed = opts.seed;
  let pad = 10.0;
  let width = opts.width;
  let height = opts.height;
  let phase = opts.index as f64 / (opts.frames as f64);
  let circles_skip = 2;

  let w = 4;
  let h = 4;
  let mut cells = vec![false; w * h];
  cells[1] = true;
  cells[6] = true;
  cells[8] = true;
  cells[9] = true;
  cells[10] = true;
  let mut gol = GameOfLife {
    width: w,
    height: h,
    cells,
  };
  for _i in 0..((phase * 4.0) as usize) {
    gol = gol.next();
  }
  let lookup_game_of_life =
    |x: f64, y: f64, bounds: (f64, f64, f64, f64)| -> f64 {
      let xi =
        (w as f64 + 1.0) * (x - bounds.0) / (bounds.2 - bounds.0) + phase - 1.0;
      let yi =
        (h as f64 + 1.0) * (y - bounds.1) / (bounds.3 - bounds.1) + phase - 1.0;
      if xi >= 0.0 && yi >= 0.0 && gol.alive(xi as usize, yi as usize) {
        1.0
      } else {
        0.0
      }
    };

  let perlin = Perlin::new();
  let mut rng = rng_from_seed(seed);
  let mut head_collision = Passage2DCounter::new(0.45, width, height);
  let mut passage = Passage2DCounter::new(0.45, width, height);
  let bounds = (pad, pad, width - pad, height - pad);

  let main_r = 90.0;
  let main_circle = VCircle::new(width / 2., height / 2., 0.8 * main_r);
  let mut circles = packing(
    seed,
    1000000,
    400,
    2,
    6.0,
    bounds,
    12.0,
    24.0,
    35.0,
    vec![main_circle],
  );
  circles.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
  let duration = time::Duration::seconds(opts.seconds);
  let candidates: Vec<(f64, f64)> = circles
    .iter()
    .skip(circles_skip)
    .map(|&p| (p.x, p.y))
    .collect();

  let tour =
    travelling_salesman::simulated_annealing::solve(&candidates, duration);

  let layers: Vec<Group> = vec!["#006", "#f60", "#fa0"]
    .iter()
    .enumerate()
    .map(|(ci, &color)| {
      let mut routes = Vec::new();

      let angle_div = 1.0;

      let mut heads = Vec::new();

      if ci == 0 {
        let cr = 0.92 * main_r;
        let sq_w = 0.77 * main_r;
        let sq_h = sq_w;
        let sq_dy = 0.16 * main_r;
        routes = vec![
          routes,
          human_circle(cr, main_circle.pos(), phase),
          human_square(
            main_circle.x - sq_w,
            main_circle.y - sq_h + sq_dy,
            main_circle.x + sq_w,
            main_circle.y + sq_h + sq_dy,
            phase,
          ),
          human_body(seed, cr, main_circle.pos(), phase),
          human_square(pad, pad, width - pad, height - pad, phase),
        ]
        .concat();
      }

      // background organs and cordons
      if ci == 2 || ci == 1 {
        let highlights = circles.len() / 10;
        let subset: Vec<VCircle> = if ci == 2 {
          circles
            .iter()
            .skip(highlights + circles_skip)
            .map(|&c| c)
            .collect()
        } else {
          circles
            .iter()
            .skip(circles_skip)
            .take(highlights)
            .map(|&c| c)
            .collect()
        };
        let mut rng = rng_from_seed(seed);
        let organs: Vec<Vec<(f64, f64)>> = subset
          .iter()
          .enumerate()
          .map(|(i, &c)| {
            let angle = angle_div * rng.gen_range(-1.0, 1.0)
              + (c.x - width / 2.).atan2(c.y - height / 2.);
            let (dx, dy) = p_r((0.0, -0.5 * c.r), angle);
            heads.push((c.x + dx, c.y + dy, c.r, angle));
            organ(seed, i, c.r, angle, (c.x, c.y), phase)
          })
          .flatten()
          .collect();

        routes = vec![routes, organs].concat();
        let mut prev: Option<VCircle> = None;
        for i in tour.route.clone() {
          let to = circles[i + circles_skip];
          if let Some(from) = prev {
            let a = (to.y - from.y).atan2(to.x - from.x);
            let mut p1 = from.proj_polar(a, 0.75);
            let mut p2 = to.proj_polar(a + PI, 0.75);
            let mut connected = false;
            if ci == 1 {
              for c in subset.iter() {
                if c.x == from.x && c.y == from.y || c.x == to.x && c.y == to.y
                {
                  connected = true;
                  break;
                }
              }
              if connected {
                p1.0 += 0.1;
                p2.0 -= 0.1;
                p1.1 += 0.1;
                p2.1 -= 0.1;
              }
            } else {
              connected = true;
            }
            if connected {
              routes = vec![
                routes,
                cordon(vec![p1, p2], 4.0, 4.0, 0.0, 16, false, 1.0, phase),
              ]
              .concat();
            }
          }
          prev = Some(to);
        }
      }

      for route in routes.iter() {
        for &p in route.iter() {
          head_collision.count(p);
        }
      }

      // background noise
      if ci == 2 || ci == 0 {
        let lgol = circles[1];

        let iterations = if ci == 2 { 10 } else { 6 };
        let particles = (opts.width * opts.height
          / (if ci == 0 { 50.0 } else { 2.5 }))
          as usize;
        let precision = 0.5;
        let samples = sample_2d_candidates_f64(
          &|p| {
            let g = project_in_boundaries(p, bounds);
            let d_to_center = euclidian_dist(main_circle.pos(), g);
            if ci == 0 {
              return smoothstep(
                main_circle.r,
                0.8 * main_circle.r,
                d_to_center,
              );
            }
            let mut d = 99f64;
            for (_i, circle) in circles.iter().skip(circles_skip).enumerate() {
              d = d.min(euclidian_dist((circle.x, circle.y), g) - circle.r);
            }

            let ld = euclidian_dist((lgol.x, lgol.y), g) - lgol.r;
            if ld < 0.0 {
              return 1.
                - lookup_game_of_life(
                  g.0,
                  g.1,
                  (
                    lgol.x - lgol.r,
                    lgol.y - lgol.r,
                    lgol.x + lgol.r,
                    lgol.y + lgol.r,
                  ),
                );
            }
            smoothstep(1.5, 3.0, d).powf(2.0)
              * smoothstep(-10.0, main_circle.r, d_to_center)
          },
          1000,
          particles,
          &mut rng,
        );

        for (si, &sample) in samples.iter().enumerate() {
          let mut g = project_in_boundaries(sample, bounds);
          let mut ang = rng.gen_range(0.0, 2. * PI);

          let ld = euclidian_dist((lgol.x, lgol.y), sample) - lgol.r;

          let it = if ld < 0.0 { iterations / 2 } else { iterations };

          let mut route = Vec::new();
          for _i in 0..it {
            if out_of_boundaries(g, bounds)
              || head_collision.get(g) > 0
              || passage.count(g) > 2
            {
              break;
            }
            route.push(g);

            let mut v = (0f64, 0f64);
            for p in circles.iter().skip(1) {
              let dist = euclidian_dist((p.x, p.y), g) - p.r;
              let a = (p.y - g.1).atan2(p.x - g.0) + (si as f64 - 0.5) * PI;
              let r = smoothstep(40.0, -40.0, dist);
              v.0 += r * a.cos();
              v.1 += r * a.sin();
            }

            if v.0 != 0.0 || v.1 != 0.0 {
              let mut a = (v.1.atan2(v.0) + 2.0 * PI) % (2. * PI);
              if (a - ang).abs() > PI / 2.0 {
                a += PI;
              }
              ang = a;
            }

            let dx = g.0 / width - 0.5;
            let dy = g.1 / height - 0.5;
            let dd = 2. * (dx * dx + dy * dy).sqrt();

            ang += rng.gen_range(0.0, dd * dd)
              + 2.
                * perlin.get([
                  //
                  seed + perlin.get([0.01 * g.0, 0.01 * g.1]),
                  0.008 * g.0,
                  0.008 * g.1,
                ]);

            g.0 += precision * ang.cos();
            g.1 += precision * ang.sin();
          }

          if route.len() > 2 {
            routes.push(route);
          }
        }
      }

      let mut data = Data::new();
      println!("{} lines in {}", routes.len(), color);
      for r in routes {
        data = render_route(data, r);
      }

      Group::new()
        .set("inkscape:groupmode", "layer")
        .set("inkscape:label", color)
        .add(
          Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 0.32)
            .set("d", data),
        )
    })
    .collect();

  let mut group = Group::new();
  for l in layers {
    group = group.add(l);
  }
  vec![group]
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
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

fn p_r(p: (f64, f64), a: f64) -> (f64, f64) {
  (a.cos() * p.0 + a.sin() * p.1, a.cos() * p.1 - a.sin() * p.0)
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
  fn pos(self: &Self) -> (f64, f64) {
    (self.x, self.y)
  }

  fn dist(self: &Self, c: &VCircle) -> f64 {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - c.r - self.r
  }
  fn collides(self: &Self, c: &VCircle) -> bool {
    self.dist(c) <= 0.0
  }
  fn proj_polar(self: &Self, angle: f64, factor: f64) -> (f64, f64) {
    (
      self.x + self.r * factor * angle.cos(),
      self.y + self.r * factor * angle.sin(),
    )
  }
  /*
  fn contains(self: &Self, c: &VCircle) -> bool {
    euclidian_dist((self.x, self.y), (c.x, c.y)) - self.r + c.r < 0.0
  }
  */
  fn inside_bounds(
    self: &Self,
    (x1, y1, x2, y2): (f64, f64, f64, f64),
  ) -> bool {
    x1 <= self.x - self.r
      && self.x + self.r <= x2
      && y1 <= self.y - self.r
      && self.y + self.r <= y2
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
  container_boundaries: (f64, f64, f64, f64),
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    c.inside_bounds(container_boundaries)
      && !circles.iter().any(|other| c.collides(other))
  };
  scaling_search(overlaps, min_scale, max_scale)
}

fn packing(
  seed: f64,
  iterations: usize,
  desired_count: usize,
  optimize_size: usize,
  pad: f64,
  container_boundaries: (f64, f64, f64, f64),
  min_scale: f64,
  max_scale: f64,
  max_scale_first: f64,
  initial_circles: Vec<VCircle>,
) -> Vec<VCircle> {
  let mut seen = initial_circles.clone();
  let mut circles = initial_circles.clone();
  let mut tries = Vec::new();
  let mut rng = rng_from_seed(seed);
  let x1 = container_boundaries.0;
  let y1 = container_boundaries.1;
  let x2 = container_boundaries.2;
  let y2 = container_boundaries.3;
  for _i in 0..iterations {
    let x: f64 = rng.gen_range(x1, x2);
    let y: f64 = rng.gen_range(y1, y2);
    if let Some(size) = search_circle_radius(
      container_boundaries,
      &seen,
      x,
      y,
      min_scale,
      if circles.len() == initial_circles.len() {
        max_scale_first
      } else {
        max_scale
      },
    ) {
      let circle = VCircle::new(x, y, size - pad);
      tries.push(circle);
      if tries.len() > optimize_size {
        tries.sort_by(|a, b| b.r.partial_cmp(&a.r).unwrap());
        let c = tries[0];
        circles.push(c.clone());
        seen.push(c.clone());
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

fn path_curve_it(path: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
  let mut route = Vec::new();
  route.push(path[0]);
  for i in 1..(path.len() - 1) {
    let p = path[i];
    let p1 = lerp_point(path[i - 1], p, 0.8);
    let p2 = lerp_point(path[i + 1], p, 0.8);
    route.push(p1);
    route.push(p2);
  }
  route.push(path[path.len() - 1]);
  route
}

fn path_curve(path: Vec<(f64, f64)>, n: usize) -> Vec<(f64, f64)> {
  let mut route = path;
  for _i in 0..n {
    route = path_curve_it(route);
  }
  route
}

fn translate_routes(
  routes: Vec<Vec<(f64, f64)>>,
  (tx, ty): (f64, f64),
) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    .collect()
}

struct GameOfLife {
  width: usize,
  height: usize,
  cells: Vec<bool>,
}
impl GameOfLife {
  fn index(&self, x: usize, y: usize) -> usize {
    return x + y * self.width;
  }
  fn reverse(&self, i: usize) -> (usize, usize) {
    let y = i / self.width;
    let x = i - self.width * y;
    return (x, y);
  }
  fn alive(&self, x: usize, y: usize) -> bool {
    if x >= self.width || y >= self.height {
      return false;
    }
    let alive = self.cells[self.index(x, y)];
    return alive;
  }
  fn next(&self) -> GameOfLife {
    let width = self.width;
    let height = self.height;
    let mut cells = vec![false; width * height];
    for i in 0..self.cells.len() {
      let (x, y) = self.reverse(i);
      let sum: u8 = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
      ]
      .into_iter()
      .map(|(dx, dy)| {
        let xi = ((x as i32) + dx) as usize;
        let yi = ((y as i32) + dy) as usize;
        let v = self.alive(xi, yi) as u8;
        return v;
      })
      .sum();
      cells[i] = sum == 3 || sum == 2 && self.alive(x, y);
    }

    return GameOfLife {
      width,
      height,
      cells,
    };
  }
}

/*
let mut cords = Vec::new();
for &(hx, hy, hr, ha) in heads.iter() {
  let mut count = 0;
  for &h in heads.iter() {
    if count >= 2 {
      break;
    }
    if h.0 == hx && h.1 == hy {
      continue;
    }
    let a = (hy - h.1).atan2(hx - h.0);
    let da = (ha - a + 2. * PI) % (2. * PI);
    let dx = hx - h.0;
    let dy = hy - h.1;
    let d = (dx * dx + dy * dy).sqrt();
    if d < height * 0.6 {
      count += 1;
      // cords.push(vec![(hx, hy), (h.0, h.1)]);
      let mut p1 = (hx, hy, ha - PI / 2.);
      let mut p2 = (h.0, h.1, h.2 - PI / 2.);
      let precision = 1.0;
      let mut found = false;
      let mut path = Vec::new();
      let mut path2 = Vec::new();
      for i in 0..100 {
        // p1
        path.push((p1.0, p1.1));
        let atarget = (p2.1 - p1.1).atan2(p2.0 - p1.0);
        let mut da: f64 = atarget - p1.2;
        let rot: f64 = rng.gen_range(0.0, 0.01 + 0.01 * (i as f64));
        if da > PI {
          da -= 2. * PI;
        }
        if da.abs() > 0.02 {
          let r = rot.min(da.abs());
          if da > 0.0 {
            p1.2 += r;
          } else {
            p1.2 -= r;
          }
        }
        p1.0 += precision * p1.2.cos();
        p1.1 += precision * p1.2.sin();
        // p1
        path2.push((p2.0, p2.1));
        let atarget = (p1.1 - p2.1).atan2(p1.0 - p2.0);
        let mut da: f64 = atarget - p2.2;
        let rot: f64 = rng.gen_range(0.0, 0.01 + 0.01 * (i as f64));
        if da > PI {
          da -= 2. * PI;
        }
        if da.abs() > 0.02 {
          let r = rot.min(da.abs());
          if da > 0.0 {
            p2.2 += r;
          } else {
            p2.2 -= r;
          }
        }
        p2.0 += precision * p2.2.cos();
        p2.1 += precision * p2.2.sin();
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        if (dx * dx + dy * dy).sqrt() < 3. * precision {
          found = true;
          break;
        }
      }
      if found {
        let path2rev = path2.iter().rev().map(|&p| p).collect::<Vec<_>>();
        cords.push(vec![path, path2rev].concat());
      }
    }
  }
}
*/
