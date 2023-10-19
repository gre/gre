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
  #[clap(short, long, default_value = "8.0")]
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

  fn render(&self, clr: usize) -> Vec<(usize, Vec<(f64, f64)>)> {
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

    routes
  }
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

fn helmet(
  origin: (f64, f64),
  angle: f64,
  size: f64,
  xreverse: bool,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
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
    .collect()
}

trait MonochromeStrokable {
  fn render(&self) -> Vec<(usize, Vec<(f64, f64)>)>;
}

trait PointCheckable {
  fn includes_point(&self, point: (f64, f64)) -> bool;
}

#[derive(Clone)]
struct StrokesWithPolygonsBound {
  strokes: Vec<(usize, Vec<(f64, f64)>)>,
  polygons: Vec<Vec<(f64, f64)>>,
}

impl StrokesWithPolygonsBound {
  fn new(
    strokes: Vec<(usize, Vec<(f64, f64)>)>,
    polygons: Vec<Vec<(f64, f64)>>,
  ) -> Self {
    Self { strokes, polygons }
  }
}

impl MonochromeStrokable for StrokesWithPolygonsBound {
  fn render(&self) -> Vec<(usize, Vec<(f64, f64)>)> {
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

fn shield<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  size: f64,
  angle: f64,
  shape1: f64,
  shape2: f64,
  clr: usize,
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

  StrokesWithPolygonsBound::new(routes, polygons)
}

fn proj_point(origin: (f64, f64), angle: f64, distance: f64) -> (f64, f64) {
  let (x, y) = origin;
  let s = angle.sin();
  let c = angle.cos();
  (x + distance * c, y + distance * s)
}

fn flag<R: Rng>(
  rng: &mut R,
  mask: &mut PaintMask,
  origin: (f64, f64),
  size: f64,
  flag_length: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = Vec::new();

  let stick_len = rng.gen_range(1.2, 1.4) * size;
  let stick_w = 0.2 * size;
  let line_dist = 0.5;

  let top = (origin.0, origin.1 - stick_len);

  routes.push((
    clr,
    grow_stroke_zigzag((origin.0, origin.1), top, stick_w, line_dist),
  ));

  let flagstart = (0.05 * flag_length, 0.0);

  let barebone = subdivide(vec![flagstart, (-flag_length, 0.0)], 2);
  let shaking = rng.gen_range(0.0, 0.1) * flag_length;
  let mut barebone = shake(rng, &barebone, shaking);
  barebone[0] = flagstart;
  let barebone = path_subdivide_to_curve(barebone, 1, 0.8);

  let wmul = 2.0;
  let freq = rng.gen_range(0.5, 2.0);

  let widthf = |l: f64, _i| (1.0 + 0.5 * (l * freq).cos()) * wmul;

  routes.extend(variable_polyline_spiral_fill(
    rng, mask, top, barebone, 0.4, 0, &widthf, clr,
  ));

  routes
}

fn sword<R: Rng>(
  rng: &mut R,
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
  routes
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
    .collect()
}

fn spear<R: Rng>(
  rng: &mut R,
  origin: (f64, f64),
  size: f64,
  angle: f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
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
    .collect()
}

fn horse_with_rider<R: Rng>(
  rng: &mut R,
  mask: &mut PaintMask,
  origin: (f64, f64),
  angle: f64,
  size: f64, // reference size (height of the boat)
  xflip: bool,
  mainclr: usize,
  skinclr: usize,
  is_leader: bool,
) -> StrokesWithPolygonsBound {
  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = vec![];
  let xdir = if xflip { -1.0 } else { 1.0 };

  let x0 = -size * rng.gen_range(0.4, 0.5);
  let x1 = -size * rng.gen_range(0.3, 0.4);
  let x2 = size * rng.gen_range(0.25, 0.35);
  let x3 = size * rng.gen_range(0.4, 0.5);
  let yleft = size * rng.gen_range(0.1, 0.2);
  let yright = -size * rng.gen_range(0.6, 0.8);

  let dy_edge = 0.3;
  // horse body bottom
  let mut route = Vec::new();
  route.push((x0, yleft + dy_edge));
  route.push((x1, 0.0));
  route.push((x2, 0.0));
  route.push((x3 + 0.05 * size, yright + dy_edge + 0.05 * size));
  route = path_subdivide_to_curve(route, 2, 0.8);
  routes.push((mainclr, route));

  // horse body top
  let mut route = Vec::new();
  let y = -0.3 * size;
  route.push((x0, yleft - dy_edge));
  route.push((x1, y));
  route.push((x2, y));
  route.push((x3, yright - dy_edge));
  route = path_subdivide_to_curve(route, 2, 0.8);
  // TODO route will be used to clip people
  routes.push((mainclr, route.clone()));

  // make horse head
  let a = (x3, yright);
  let b = (x3 + rng.gen_range(0.1, 0.3) * size, yright + 0.3 * size);
  routes.push((mainclr, grow_stroke_zigzag(a, b, 0.2 * size, 0.5)));
  routes.push((mainclr, vec![a, b]));

  // make horse left feet
  let a = (x1 + 0.1 * size, y + 0.2 * size);
  let b = (x1 + rng.gen_range(-0.2, 0.2) * size, y + 0.5 * size);
  routes.push((mainclr, grow_stroke_zigzag(a, b, 0.1 * size, 0.5)));
  routes.push((mainclr, vec![a, b]));

  // make horse right feet
  let a = (x3 - 0.1 * size, y);
  let b = (x3 + rng.gen_range(-0.2, 0.1) * size, y + 0.4 * size);
  routes.push((mainclr, grow_stroke_zigzag(a, b, 0.1 * size, 0.5)));
  routes.push((mainclr, vec![a, b]));

  // humans

  let mut foreground_routes = Vec::new();
  let mask_origin = (3.0 * size, 3.0 * size);
  let mut foreground_mask =
    PaintMask::new(0.5, 2.0 * mask_origin.0, 2.0 * mask_origin.1);

  let shape1 = rng.gen_range(0.0, 1.0);
  let shape2 = rng.gen_range(0.0, 1.0);
  let x = 0.0;
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
    left_leg_bend: 0.0,
    right_leg_bend: 1.5,
  };
  let humansize = size * 0.5;
  let y = rng.gen_range(-0.1 * size, 0.0);
  let human = HumanBody::new((x, y), humansize, joints);

  let human_body = human.render(mainclr);
  routes.extend(human_body);

  let left_hand = human.hand_left_pos_angle();

  let obj_strokes = if is_leader {
    flag(rng, mask, left_hand.0, size * 0.5, 4.0 * size, skinclr)
  } else if rng.gen_bool(0.5) {
    sword(rng, left_hand.0, 0.5 * size, left_hand.1, mainclr)
  } else {
    spear(rng, left_hand.0, size * 0.5, left_hand.1, mainclr)
  };
  routes.extend(obj_strokes);

  let (headpos, headangle) = human.head_pos_angle();
  let h = helmet(headpos, headangle, humansize, false, mainclr);
  routes.extend(h);

  let shield_p = human.elbow_right;

  let s = shield(rng, shield_p, size * 0.6, 0.0, shape1, shape2, mainclr);

  let is_colliding_shield = |point: (f64, f64)| s.includes_point(point);

  foreground_routes =
    clip_routes_with_colors(&foreground_routes, &is_colliding_shield, 1.0, 5);

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

    let has_foreground = |p: (f64, f64)| {
      foreground_mask.is_painted((p.0 + mask_origin.0, p.1 + mask_origin.1))
    };

    routes = clip_routes_with_colors(&routes, &has_foreground, 1.0, 5);
  }

  routes.extend(foreground_routes.clone());

  // translate routes
  routes = routes
    .iter()
    .map(|(clr, route)| {
      (
        *clr,
        route
          .iter()
          .map(|&(x, y)| {
            let x = xdir * x;
            let (x, y) = p_r((x, y), angle);
            (x + origin.0, y + origin.1)
          })
          .collect(),
      )
    })
    .collect();

  let polygons = vec![];

  StrokesWithPolygonsBound::new(routes, polygons)
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = Vec::new();
  let mut foreground_routes = Vec::new();
  let mut foreground_mask = PaintMask::new(0.5, width, height);

  let mut yi = 0;
  let mut y = pad;
  let mut xflip = false;
  while y < height - pad - size {
    let mut x = pad;
    let mut lastflagx = -999.;
    let mut xreverse = false;

    x += rng.gen_range(0.0, 2.0) * size;
    let w: f64 = size * rng.gen_range(4.0, 5.0);
    let skin = rng.gen_range(1, 4);

    while x < width - pad - size {
      let first_part = xflip && x < 0.7 * width || !xflip && x > 0.3 * width;
      if x + w > width - pad {
        break;
      }

      let leader = rng.gen_bool(if first_part { 0.7 } else { 0.001 })
        && (lastflagx - x).abs() > size * 5.0;
      if leader {
        lastflagx = x;
      }

      let origin: (f64, f64) = (x + w / 2.0, y + size);
      let objects = horse_with_rider(
        &mut rng,
        &mut foreground_mask,
        origin,
        0.0,
        size,
        xflip,
        0,
        skin,
        leader,
      );
      routes.extend(objects.strokes);

      x += 1.5 * size;

      xreverse = !xreverse;
    }
    yi += 1;
    y += 1.8 * size;
    xflip = !xflip;
  }

  let has_foreground = |p| foreground_mask.is_painted(p);

  routes = clip_routes_with_colors(&routes, &has_foreground, 1.0, 5);

  routes.extend(foreground_routes);

  let colors = vec!["black", "gold", "turquoise", "red"];

  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if i == ci {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
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

fn variable_polyline_spiral_fill<R: Rng>(
  rng: &mut R,
  paint: &mut PaintMask,
  o: (f64, f64),
  barebone: Vec<(f64, f64)>,
  spiral_fill_dr: f64,
  subsegments: usize,
  widthf: &dyn Fn(f64, usize) -> f64,
  clr: usize,
) -> Vec<(usize, Vec<(f64, f64)>)> {
  let mut routes = vec![];
  if barebone.len() < 2 {
    return routes;
  }

  // find highest radius
  let mut r = 0.0;
  for i in 1..barebone.len() {
    let d = euclidian_dist((0.0, 0.0), barebone[i]);
    if d > r {
      r = d;
    }
  }
  r += 1.0;

  // local paint dist
  let lpd = 2. * r;

  let mut widths = vec![];
  let mut l = 0.0;
  let mut prev = (0.0, 0.0);
  for i in 0..barebone.len() {
    widths.push(widthf(l, i));
    l += euclidian_dist(prev, barebone[i]);
    prev = barebone[i];
  }

  let mut fibers: Vec<Vec<(f64, f64)>> = vec![];
  let count = 2 + subsegments;
  for _ in 0..count {
    fibers.push(vec![]);
  }
  for i in 0..count {
    let df = (i as f64) / ((count - 1) as f64) - 0.5;
    for j in 0..barebone.len() {
      let a = if j > 0 {
        (barebone[j].1 - barebone[j - 1].1)
          .atan2(barebone[j].0 - barebone[j - 1].0)
      } else {
        (barebone[1].1 - barebone[0].1).atan2(barebone[1].0 - barebone[0].0)
      };
      let orthogonal = a + PI / 2.0;
      let dist = widths[j];
      let d = df * dist * 0.5;
      let p = barebone[j];
      let q = (p.0 + d * orthogonal.cos(), p.1 + d * orthogonal.sin());
      fibers[i].push(q);
    }
  }

  // TODO with fibers[0] and fibers[last], we can make polygons and fill local_paint with it
  let mut polys = vec![];
  let mut globalpolys = vec![];
  for i in 1..barebone.len() {
    let a = fibers[0][i - 1];
    let b = fibers[0][i];
    let c = fibers[count - 1][i];
    let d = fibers[count - 1][i - 1];

    {
      let a = (a.0 + lpd, a.1 + lpd);
      let b = (b.0 + lpd, b.1 + lpd);
      let c = (c.0 + lpd, c.1 + lpd);
      let d = (d.0 + lpd, d.1 + lpd);
      let polygon = vec![a, b, c, d];
      polys.push(polygon);
    }

    {
      let a = (a.0 + o.0, a.1 + o.1);
      let b = (b.0 + o.0, b.1 + o.1);
      let c = (c.0 + o.0, c.1 + o.1);
      let d = (d.0 + o.0, d.1 + o.1);
      globalpolys.push(vec![a, b, c, d]);
    }
  }

  let spiral =
    vec![(clr, spiral_optimized(r, 0.0, 2. * r, spiral_fill_dr, 0.1))];
  let local_painted = |p: (f64, f64)| {
    let q = (p.0 + lpd, p.1 + lpd);
    !polys.iter().any(|poly| polygon_includes_point(&poly, q))

    // !(euclidian_dist(p, (0.0, 0.0)) < lpd &&
    // local_paint.is_painted(q))
  };
  routes.extend(clip_routes_with_colors(&spiral, &local_painted, 0.4, 5));

  // TODO then we can colorize the paint

  for fiber in fibers {
    routes.push((clr, fiber));
  }

  // translate everything
  let mut all = vec![];
  for (clr, route) in routes {
    let mut path = vec![];
    for p in route {
      let p = (p.0 + o.0, p.1 + o.1);
      path.push(p);
    }
    all.push((clr, path));
  }

  regular_clip_polys(&all, paint, &globalpolys);

  all
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

fn shake<R: Rng>(
  rng: &mut R,
  path: &Vec<(f64, f64)>,
  scale: f64,
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
