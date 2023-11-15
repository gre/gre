use crate::algo::{clipping::regular_clip, paintmask::PaintMask};

#[derive(Clone, Copy)]
pub struct HumanJointAngles {
  pub body_angle: f64,
  pub head_angle: f64,
  // shoulders (left, right)
  pub shoulder_right_angle: f64,
  pub shoulder_left_angle: f64,
  // elbows (left, right)
  pub elbow_right_angle: f64,
  pub elbow_left_angle: f64,
  // hips
  pub hip_right_angle: f64,
  pub hip_left_angle: f64,
  // knees (left, right)
  pub knee_right_angle: f64,
  pub knee_left_angle: f64,

  pub left_arm_bend: f64,
  pub left_leg_bend: f64,
  pub right_arm_bend: f64,
  pub right_leg_bend: f64,
}

#[derive(Clone, Copy)]
pub struct HumanBody {
  pub joints: HumanJointAngles,
  pub height: f64,
  pub hip: (f64, f64),
  pub shoulder: (f64, f64),
  pub shoulder_right: (f64, f64),
  pub shoulder_left: (f64, f64),
  pub elbow_right: (f64, f64),
  pub elbow_left: (f64, f64),
  pub hip_right: (f64, f64),
  pub hip_left: (f64, f64),
  pub knee_right: (f64, f64),
  pub knee_left: (f64, f64),
  pub head: (f64, f64),
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
  /*
  pub fn foot_left_pos_angle(&self) -> ((f64, f64), f64) {
    (self.knee_left, self.joints.knee_left_angle)
  }
  pub fn foot_right_pos_angle(&self) -> ((f64, f64), f64) {
    (self.knee_right, self.joints.knee_right_angle)
  }
  pub fn get_size(&self) -> f64 {
    self.height
  }
  */

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

  pub fn render(
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

fn proj_point(origin: (f64, f64), angle: f64, distance: f64) -> (f64, f64) {
  let (x, y) = origin;
  let s = angle.sin();
  let c = angle.cos();
  (x + distance * c, y + distance * s)
}
