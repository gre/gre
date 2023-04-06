use clap::*;
use gre::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "12.0")]
  // #[clap(short, long, default_value = "24.0")]
  pub size: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
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

fn full_helmet(
  origin: (f64, f64),
  angle: f64,
  size: f64,
  xreverse: bool,
) -> Vec<Vec<(f64, f64)>> {
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

fn helmet(
  origin: (f64, f64),
  angle: f64,
  size: f64,
  xreverse: bool,
) -> Vec<Vec<(f64, f64)>> {
  let mut routes = Vec::new();
  let dx = 0.13 * size;
  let h = 0.4 * size;

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

// TODO FUTURE axe
// TODO FUTURE flag

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

fn routes_translate_rotate(
  routes: Vec<Vec<(f64, f64)>>,
  origin: (f64, f64),
  angle: f64,
) -> Vec<Vec<(f64, f64)>> {
  routes
    .iter()
    .map(|route| route_translate_rotate(route, origin, angle))
    .collect()
}

fn shield<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  size: f64,
  angle: f64,
) -> StrokesWithPolygonsBound {
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

fn proj_point(origin: (f64, f64), angle: f64, distance: f64) -> (f64, f64) {
  let (x, y) = origin;
  let s = angle.sin();
  let c = angle.cos();
  (x + distance * c, y + distance * s)
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();
  let mut foreground_routes = Vec::new();
  let mut foreground_mask = PaintMask::new(0.5, width, height);

  let mut yi = 0;
  let mut y = pad;
  let mut xflip = false;
  while y < height - pad - size {
    let mut x = pad;
    let mut xreverse = false;
    while x < width - pad - size {
      let origin = (pad + x + size / 2.0, pad + y + size);

      let ysplit = (3.0 * y / (height - pad * 2.0)).floor() as usize;
      match ysplit {
        0 => {
          // spearman

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
            right_arm_bend: 0.5,
            left_leg_bend: 1.0,
            right_leg_bend: 1.0,
          };
          let humansize = size * 0.5;
          let xcenter = origin.0 - size * 0.5;
          let human =
            HumanBody::new((xcenter, origin.1 - size * 0.5), humansize, joints);
          let mut new_routes = vec![];

          new_routes.extend(human.render());
          let (headpos, headangle) = human.head_pos_angle();
          let h = helmet(headpos, headangle, humansize, xreverse);
          new_routes.extend(h);

          // sword / shield
          let (shield_p, (object_p, _object_a)) =
            (human.elbow_left, human.hand_right_pos_angle());

          let sp = spear(&mut rng, object_p, size * 0.5, PI / 2.0);
          new_routes.extend(sp);

          let s = shield(&mut rng, shield_p, size * 0.6, 0.0);

          let is_colliding_shield = |point: (f64, f64)| s.includes_point(point);

          foreground_routes =
            clip_routes(&foreground_routes, &is_colliding_shield, 1.0, 5);

          foreground_routes.extend(s.render());

          for poly in s.polygons.iter() {
            foreground_mask.paint_polygon(poly);
          }

          routes.extend(new_routes);

          x += (0.2 + yi as f64 / 10.0) * size;
        }
        1 => {
          // bowman

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
          let human =
            HumanBody::new((xcenter, origin.1 - size * 0.5), humansize, joints);
          let mut new_routes = vec![];

          new_routes.extend(human.render());
          let (headpos, headangle) = human.head_pos_angle();
          let h = head(headpos, headangle, humansize);
          new_routes.extend(h);

          let (pos, angle) = human.hand_right_pos_angle();

          let bow = LongBow::new(&mut rng, pos, size * 0.5, -angle, phase);
          new_routes.extend(bow.render());

          if phase > 0.2 {
            let arr = arrow(bow.arrow_start, size * 0.4, bow.arrow_angle);
            new_routes.extend(arr);
          }

          routes.extend(new_routes);

          x += size;
        }
        _ => {
          // duelists

          let joints = HumanJointAngles {
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
          };
          let humansize = size * 0.5;
          let xcenter = origin.0 - size * 0.5;
          let human =
            HumanBody::new((xcenter, origin.1 - size * 0.5), humansize, joints);
          let mut new_routes = vec![];

          new_routes.extend(human.render());
          let (headpos, headangle) = human.head_pos_angle();
          if rng.gen_bool(0.5) {
            let h = full_helmet(headpos, headangle, humansize, xreverse);
            new_routes.extend(h);
          } else {
            let h = head(headpos, headangle, humansize);
            new_routes.extend(h);
          }
          // sword / shield
          let (shield_p, (object_p, object_a)) = if xreverse {
            (human.elbow_right, human.hand_left_pos_angle())
          } else {
            (human.elbow_left, human.hand_right_pos_angle())
          };

          let sw = sword(&mut rng, object_p, size * 0.5, object_a);

          let s = shield(&mut rng, shield_p, size * 0.5, 0.0);

          new_routes.extend(sw);

          let is_colliding_shield = |point: (f64, f64)| s.includes_point(point);

          foreground_routes =
            clip_routes(&foreground_routes, &is_colliding_shield, 1.0, 5);

          foreground_routes.extend(s.render());

          for poly in s.polygons.iter() {
            foreground_mask.paint_polygon(poly);
          }

          routes.extend(new_routes);

          if xreverse {
            x += 0.6 * size;
          }
          x += size;
        }
      }

      xreverse = !xreverse;
    }
    yi += 1;
    y += 1.4 * size;
    xflip = !xflip;
  }

  let has_foreground = |p| foreground_mask.is_painted(p);

  routes = clip_routes(&routes, &has_foreground, 1.0, 5);

  routes.extend(foreground_routes);

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
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

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
