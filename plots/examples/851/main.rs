use std::f64::consts::PI;

use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "420.0")]
  pub height: f64,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "2")]
  pub divx: usize,
  #[clap(short, long, default_value = "4")]
  pub divy: usize,
  #[clap(short, long, default_value = "0")]
  pub page: usize,
  #[clap(short, long, default_value = "16")]
  pub total: usize,
  #[clap(short, long, default_value = "210.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "")]
  pub testing_seeds: String,
}

pub struct Frame {
  index: usize,
  pos: (f64, f64),
  rot: f64,
  size: f64,
}

fn cell(
  seed: f64,
  width: f64,
  height: f64,
  offset: usize,
  total: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let progress = offset as f64 / (total as f64);
  let border = 3.0;
  let bound = (border, border, width - border, height - border);
  // Prepare all the random values
  let mut rng = rng_from_seed(seed);
  let perlin = Perlin::new();
  let min_route = 2;
  let stopy = rng.gen_range(0.25, 0.4) * height;
  let passage_threshold = 10;

  let gridw = 4;
  let gridh = 1;
  let pager_size = 1.5;
  let pager_pad = border + 0.5;
  let pager_ratio_scale = 1.0;
  let pgr = |xf, yf| {
    (
      pager_size * xf * pager_ratio_scale + pager_pad,
      height + pager_size * (yf - (gridh as f64)) - pager_pad,
    )
  };
  let pgr_topleft = pgr(0.0, 0.0);
  let pgr_bottomright = pgr(gridw as f64, gridh as f64);
  let safep = 0.3;
  let pgr_boundaries = (
    pgr_topleft.0 - safep,
    pgr_topleft.1 - safep,
    pgr_bottomright.0 + safep,
    pgr_bottomright.1 + safep,
  );

  // all the lines to draw are pushed here
  let mut routes = Vec::new();

  let mountainpadding = -40.0;

  let mut height_map: Vec<f64> = Vec::new();
  let mut passage = Passage::new(0.5, width, height);

  let precision = 0.2;
  let count =
    (3.0 + rng.gen_range(5.0, 10.0) * rng.gen_range(0.5, 1.0)) as usize;

  let mut xoffmen = 0.0;
  let mut smooth_heights = Vec::new();

  let mut humans = vec![rng.gen_range(0, count)];
  if rng.gen_bool(0.5) {
    humans.push(rng.gen_range(0, count));
  }

  for j in 0..count {
    let peakfactor = rng.gen_range(-0.001, 0.001)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0);
    let ampfactor = rng.gen_range(0.0, 0.12);
    let yincr = 0.35 + rng.gen_range(0.0, 2.5) * rng.gen_range(0.0, 1.0);
    let amp1 = rng.gen_range(0.0, 20.0) * rng.gen_range(0.2, 1.0);
    let amp2 = rng.gen_range(0.0, 6.0) * rng.gen_range(0.2, 1.0);
    let amp3 = rng.gen_range(0.0, 4.0) * rng.gen_range(0.2, 1.0);
    let ynoisefactor = rng.gen_range(0.1, 0.2);
    let offsetstrategy = rng.gen_range(0, 6);
    let xc = width * rng.gen_range(0.2, 0.8);

    let radius1 = rng.gen_range(0.4, 1.0);
    let radius2 = rng.gen_range(0.4, 1.0);
    let radius3 = rng.gen_range(0.4, 1.0);
    let radius4 = rng.gen_range(0.4, 1.0);

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
        let xv = (4.01 - base_y / height) * (x - xc);

        let amp = height * ampfactor;
        let mut y = base_y;

        if offsetstrategy == 0 {
          y += amp * peakfactor * xv * xv;
        }

        let p = project_cylinder_translation(
          progress,
          (x * 0.007, y * ynoisefactor),
          radius1,
          (0.0, seed),
        );

        y += amp2 * amp * perlin.get([p.0, p.1, p.2]);

        if offsetstrategy == 2 {
          y += amp * peakfactor * xv * xv;
        }

        let p = project_cylinder_translation(
          progress,
          (x * 0.01, y * 0.003),
          radius2,
          (10.0, seed),
        );

        let q = project_cylinder_translation(
          progress,
          (x * 0.022, y * 0.06),
          radius2,
          (seed, 5.0),
        );

        y += amp1
          * amp
          * perlin
            .get([p.0, p.1, p.2 + perlin.get([q.0, q.1, q.2])])
            .max(0.0);

        if offsetstrategy == 1 {
          y += amp * peakfactor * xv * xv;
        }

        let p = project_cylinder_translation(
          progress,
          (x * 0.2, y * 0.3),
          radius3,
          (seed, 50.0),
        );

        y += 0.05 * amp * perlin.get([p.0, p.1, p.2]);

        if offsetstrategy == 4 {
          y += amp * peakfactor * xv * xv;
        }

        let p = project_cylinder_translation(
          progress,
          (xv * 0.009, y * 0.07),
          radius4,
          (seed, 500.0),
        );
        y += amp * amp3 * perlin.get([p.0, p.1, p.2]).powf(2.0);

        if offsetstrategy == 3 {
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
        let p = (x, y);
        let inside = !collides
          && strictly_in_boundaries(p, bound)
          && !strictly_in_boundaries(p, pgr_boundaries);
        if inside && passage.get(p) < passage_threshold {
          if was_outside {
            if route.len() > min_route {
              routes.push((0, route));
            }
            route = Vec::new();
          }
          was_outside = false;
          route.push(p);
          passage.count(p);
        } else {
          was_outside = true;
        }

        x += precision;
      }

      if route.len() > min_route {
        routes.push((0, route));
      }

      base_y -= yincr;
    }

    // calculate a moving average to smooth the stick men positions
    let smooth = 10;
    let sf = smooth as f64;
    let mut sum = 0.0;
    let mut acc = Vec::new();
    smooth_heights = Vec::new();
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

    if !humans.contains(&j) {
      continue;
    }

    xoffmen += 0.5;

    let gif_frames = 10;
    let gif_ratio = 420. / 504.;
    let count = rng.gen_range(3, 6);

    // Calculate the "frames" that are all the rectangles to put images frame on

    let mut frames = Vec::new();
    for i in 0..count {
      let x = mountainpadding
        + ((i as f64 + xoffmen % 1.0 + offset as f64 / 10.) / (count as f64))
          * (width - 2. * mountainpadding);
      let hindex = ((x - mountainpadding) / precision) as usize;
      let p = smooth_heights[hindex % smooth_heights.len()];
      let rot = p.2 * 0.7;
      let pos = (p.0, p.1);
      let size = mix(5.0, 15.0, p.1 / height);
      frames.push(Frame {
        index: (i + offset) % gif_frames,
        pos,
        rot,
        size,
      });
    }

    for f in frames {
      let get_color =
        image_gif_get_color("images/YoungGrossHoopoe.gif", f.index).unwrap();

      // 4 corners of the image to project
      let x1 = f.pos.0 - f.size / 2.0;
      let x2 = f.pos.0 + f.size / 2.0;
      let y1 = f.pos.1 - 0.9 * f.size / gif_ratio;
      let y2 = f.pos.1 + 0.1 * f.size / gif_ratio;

      // stroke a lot of lines to plot the image

      /*
      // contouring
      let prec = 0.01;
      let p = 0.04;
      let bds = (p, p, 1.0 - p, 1.0 - p);
      let thresholds = vec![0.5];
      let lookup = |v: (f64, f64)| {
        let c = get_color(v);
        if !strictly_in_boundaries(v, (0.01, 0.01, 0.99, 0.99)) {
          return 1.0;
        }
        c.0
      };
      let w = (1.0 / prec) as u32;
      let h = (1.0 / prec) as u32;
      let res = contour(w, h, lookup, &thresholds);
      let mut r = features_to_routes(res, prec);
      r = crop_routes(&r, bds);
      for r in r {
        let mut route = Vec::new();
        for v in r {
          let p = (mix(x1, x2, v.0), mix(y1, y2, v.1));
          let q = (p.0 - f.pos.0, p.1 - f.pos.1);
          let p = p_r(q, f.rot);
          let p = (p.0 + f.pos.0, p.1 + f.pos.1);
          route.push(p);
        }
        routes.push((0, route));
      }
      */

      // pixel lines
      let res = (f.size / 0.2) as usize;
      for x in 0..res {
        let mut route = Vec::new();
        for y in 0..res {
          let v = (x as f64 / (res as f64), y as f64 / (res as f64));
          let p = (mix(x1, x2, v.0), mix(y1, y2, v.1));
          let q = (p.0 - f.pos.0, p.1 - f.pos.1);
          let p = p_r(q, f.rot);
          let p = (p.0 + f.pos.0, p.1 + f.pos.1);
          let c = get_color(v);
          if c.0 < 0.5 && strictly_in_boundaries(p, (0.0, 0.0, width, height)) {
            route.push(p);
          } else {
            if route.len() > 0 {
              routes.push((0, route));
            }
            route = Vec::new();
          }
        }
        if route.len() > 0 {
          routes.push((0, route));
        }
      }
    }
  }

  let ytopavg = smooth_heights.iter().map(|&(_x, y, _rot)| y).sum::<f64>()
    / (smooth_heights.len() as f64);

  let sky_rot_ycenter = 1.2 * ytopavg;
  let sky_rot_amp = 0.1 * height + ytopavg;
  let sky_rot_x_diff = rng.gen_range(0.25, 0.5);

  // sun
  {
    let p = progress * 2.0 * PI;
    let center = (
      (0.5 + sky_rot_x_diff * p.cos()) * width,
      p.sin() * sky_rot_amp + sky_rot_ycenter,
    );

    let approx = 0.05;
    let dr = 0.4;
    let radius = rng.gen_range(8.0, 16.0);
    let c = center;
    let two_pi = 2.0 * PI;
    let mut route = Vec::new();
    let mut r: f64 = radius + 2.0 * dr;
    let mut a = 0f64;
    loop {
      let ar = r.min(radius);
      let p = round_point((c.0 + ar * a.cos(), c.1 + ar * a.sin()), 0.01);
      let l = route.len();
      if l == 0 || euclidian_dist(route[l - 1], p) > approx {
        if !strictly_in_boundaries(p, bound)
          || p.1 > height_map[((p.0 - mountainpadding) / precision) as usize]
        {
          if route.len() > 1 {
            routes.push((1, route));
          }
          route = vec![];
        } else {
          passage.count(p);
          route.push(p);
        }
      }
      let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
      a = (a + da) % two_pi;
      r -= dr * da / two_pi;
      if r < 0.05 {
        break;
      }
    }
    if route.len() > 1 {
      routes.push((1, route));
    }
  }

  // moon
  {
    let p = progress * 2.0 * PI;
    let center = (
      (0.5 - sky_rot_x_diff * p.cos()) * width,
      -p.sin() * sky_rot_amp + sky_rot_ycenter,
    );
    let radius = rng.gen_range(8.0, 10.0);
    //let intersectr = 1.8 * radius;
    //let intersectp = (center.0 - radius, center.1 + radius);

    let approx = 0.05;
    let dr = 1.0;
    let c = center;
    let two_pi = 2.0 * PI;
    let mut route = Vec::new();
    let mut r: f64 = radius + 2.0 * dr;
    let mut a = 2.0 * PI * progress;
    loop {
      let ar = r.min(radius);
      let p = round_point((c.0 + ar * a.cos(), c.1 + ar * a.sin()), 0.01);
      let l = route.len();
      if l == 0 || euclidian_dist(route[l - 1], p) > approx {
        if !strictly_in_boundaries(p, bound)
          || p.1 > height_map[((p.0 - mountainpadding) / precision) as usize]
        //|| euclidian_dist(intersectp, p) > intersectr
        {
          if route.len() > 1 {
            routes.push((2, route));
          }
          route = vec![];
        } else {
          passage.count(p);
          route.push(p);
        }
      }
      let da = 1.0 / (r + 8.0); // bigger radius is more we have to do angle iterations
      a = (a + da) % two_pi;
      r -= dr * da / two_pi;
      if r < 0.05 {
        break;
      }
    }
    if route.len() > 1 {
      routes.push((2, route));
    }
  }

  // machines

  let machinespeed = 2.0;
  let perlinamp = rng.gen_range(150.0, 350.0);
  for j in 0..3 {
    // local rng to have the same machines repeating (to make it loop)
    let mut rng = rng_from_seed(4.4 + seed);
    let mut full_rng = rng_from_seed(4.4 + seed + 77. * progress);
    let mut peaks = Vec::new();
    let x = width * (j as f64 + (machinespeed * progress) % 1.0 - 0.5) / 2.0;
    let index = ((x - mountainpadding) / precision) as usize;
    for i in 0..8 {
      let center_index = index
        + (perlinamp * perlin.get([0.15 * x, i as f64 * 7.3, seed])) as usize;
      if center_index < smooth_heights.len() {
        let c = smooth_heights[center_index];
        peaks.push(c);
      }
    }
    let topmix = 0.4 + 0.16 * (4.0 * PI * x / width).cos();
    routes = vec![
      routes,
      make_machine(
        &peaks,
        bound,
        topmix,
        &mut rng,
        &mut full_rng,
        &mut passage,
      ),
    ]
    .concat();
  }

  // prepare the sky area
  let radius = rng.gen_range(3.0, 4.0);
  passage.grow_passage(radius);

  // sky
  let does_overlap = |p| {
    passage.get(p) == 0
      && p.1
        < height_map
          [((p.0 - mountainpadding) / precision) as usize % height_map.len()]
  };

  for f in 0..3 {
    let total_pad = radius;
    let ppad = 0.0;
    let min = ppad + rng.gen_range(0.5, 2.0);
    let max = min + rng.gen_range(0.0, 2.0);
    let circles = packing(
      seed + f as f64 + offset as f64 * 7.7,
      100000,
      1000,
      rng.gen_range(0, 4),
      ppad,
      (total_pad, total_pad, width - total_pad, height - total_pad),
      &does_overlap,
      min,
      max,
    );

    for c in circles {
      let a = 2.4 * perlin.get([c.x * 0.02, c.y * 0.02, seed]);
      let p1 = (c.x - c.r * a.cos(), c.y - c.r * a.sin());
      let p2 = (c.x + c.r * a.cos(), c.y + c.r * a.sin());
      routes.push((2, vec![p1, p2]));
    }
  }

  // pager
  let mut pager = Vec::new();
  for xj in vec![0, gridw] {
    pager.push(vec![pgr(xj as f64, 0.0), pgr(xj as f64, gridh as f64)]);
  }
  for yj in vec![0, gridh] {
    pager.push(vec![pgr(0.0, yj as f64), pgr(gridw as f64, yj as f64)]);
  }
  for yi in 0..gridh {
    for xi in 0..gridw {
      let i = gridw * gridh - 1 - (xi + yi * gridw);
      let mask = 2usize.pow(i);
      let fill = offset & mask != 0;
      if fill {
        let lines = 5;
        for l in 0..lines {
          let f = (l as f64 + 0.5) / (lines as f64);
          pager.push(vec![
            pgr(xi as f64, yi as f64 + f),
            pgr(xi as f64 + 1.0, yi as f64 + f),
          ]);
        }
      }
    }
  }

  for r in pager {
    routes.push((1, r));
  }

  // External frame to around the whole piece
  let mut d = border;
  loop {
    if d < 0.1 {
      break;
    }
    routes.push((
      0,
      vec![
        (d, d),
        (d, height - d),
        (width - d, height - d),
        (width - d, d),
        (d, d),
      ],
    ));
    d -= 0.2;
  }

  routes
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = opts.pad;
  let divx = opts.divx;
  let divy = opts.divy;
  let pageoff = opts.page * divx * divy;
  let w = (opts.width) / (divx as f64);
  let h = (opts.height) / (divy as f64);
  let total = opts.total;

  let testing_seeds = Some(
    opts
      .testing_seeds
      .split(",")
      .filter(|s| !s.is_empty())
      .map(|s| s.parse().unwrap())
      .collect::<Vec<f64>>(),
  )
  .and_then(|v| if v.is_empty() { None } else { Some(v) });

  let indexes: Vec<(usize, usize)> = (0..divx)
    .flat_map(|xi| (0..divy).map(|yi| (xi, yi)).collect::<Vec<_>>())
    .collect();

  let all = indexes
    .par_iter()
    .map(|&(xi, yi)| {
      let offset = yi + xi * divy;
      let dx = pad + xi as f64 * w;
      let dy = pad + yi as f64 * h;
      if let Some(seed) = match testing_seeds.clone() {
        None => Some(opts.seed),
        Some(array) => array.get(offset).map(|&o| o),
      } {
        let mut routes =
          cell(seed, w - 2.0 * pad, h - 2.0 * pad, pageoff + offset, total);
        routes = routes
          .iter()
          .map(|(ci, route)| {
            let r: (usize, Vec<(f64, f64)>) =
              (*ci, route.iter().map(|&p| (p.0 + dx, p.1 + dy)).collect());
            r
          })
          .collect();
        return routes;
      }
      vec![]
    })
    .collect::<Vec<_>>();

  let routes = all.concat();
  // Make the SVG
  let colors = vec!["#105", "#F90", "#bbb"];
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
  circles: &Vec<VCircle>,
  x: f64,
  y: f64,
  min_scale: f64,
  max_scale: f64,
) -> Option<f64> {
  let overlaps = |size| {
    let c = VCircle::new(x, y, size);
    does_overlap((x, y)) && !circles.iter().any(|other| c.collides(other))
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
  does_overlap: &dyn Fn((f64, f64)) -> bool,
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

// See https://twitter.com/greweb/status/1524490017531432961
// slide a 2D point on a cylinder in 3D space along with a progress loop
fn project_cylinder_translation(
  // frame index / total frames
  progress: f64,
  // position on a 2D rectangle that have to be loop-translated on X
  point: (f64, f64),
  //radius of the cylinder to use
  radius: f64,
  // allow value to be injected to change the "seed" in the space lookup
  seed: (f64, f64),
) -> (f64, f64, f64) {
  let angle = 2. * PI * progress + point.0 / radius;
  let y = point.1;
  let x = seed.0 + radius * angle.cos();
  let z = seed.1 + radius * angle.sin();
  (x, y, z)
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

fn make_machine<R: Rng>(
  peaks: &Vec<(f64, f64, f64)>,
  bound: (f64, f64, f64, f64),
  topmix: f64,
  rng: &mut R,
  full_rng: &mut R,
  passage: &mut Passage,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();
  if peaks.len() == 0 {
    return Vec::new();
  }

  let mut avgx = 0.0;
  let mut highest_peak = peaks[0];
  for &p in peaks.iter() {
    if p.1 < highest_peak.1 {
      highest_peak = p;
    }
    avgx += p.0;
  }
  avgx /= peaks.len() as f64;

  avgx = mix(mix(bound.0, bound.2, 0.5), avgx, 0.8);

  /*
  if highest_peak.1 < bound.1 + 30.0 {
    return routes;
  }
  */

  let topy = mix(bound.1 + 4.0, highest_peak.1 - 20.0, topmix);

  let w = rng.gen_range(4.0, 6.0);
  let h = rng.gen_range(2.0, 3.0);
  let x1 = avgx - w;
  let x2 = avgx + w;
  let y1 = topy;
  let y2 = topy + 2.0 * h;
  let mut y = y1;
  let mut reverse = false;
  let mut route = Vec::new();
  loop {
    if y > y2 {
      break;
    }
    if reverse {
      route.push((x1, y));
    } else {
      route.push((x2, y));
    }
    y += 0.1
      + full_rng.gen_range(0.0, 0.5)
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0);
    reverse = !reverse;
  }
  routes.push(shake(path_subdivide_to_curve(route, 2, 0.9), 1.0, rng));

  for (i, p) in peaks.iter().enumerate() {
    let dx = (i as f64 / (peaks.len() as f64) - 0.5) * w * 1.0;
    let route = path_subdivide_to_curve(
      vec![
        (p.0, p.1),
        (
          mix(avgx + dx, p.0, 0.1 + 0.7 * rng.gen_range(0.0, 1.0)),
          mix(p.1, topy, rng.gen_range(0.3, 0.7)),
        ),
        (avgx + dx, topy + h),
      ],
      3,
      0.9,
    );
    routes.push(route.clone());
    routes.push(shake(route.clone(), 0.5, full_rng));
    routes.push(shake(route.clone(), 0.5, rng));
  }

  let mut routes_safe = Vec::new();
  for r in routes.iter() {
    let mut route = Vec::new();
    for p in subdivide(r.clone(), 1) {
      if strictly_in_boundaries(p, bound) {
        route.push(p);
        passage.count(p);
      } else {
        if route.len() == 1 {
          route = Vec::new();
        } else if route.len() > 1 {
          routes_safe.push((0, route));
          route = Vec::new();
        }
      }
    }
    routes_safe.push((0, route));
  }

  routes_safe
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
