use clap::*;
use gre::*;
use rand::prelude::*;
use rapier2d::prelude::*;
use std::f32::consts::PI;
use svg::node::element;
use svg::node::element::path::Data;

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  width: f64,
  #[clap(short, long, default_value = "210.0")]
  height: f64,
  #[clap(short, long, default_value = "10.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn art(opts: &Opts) -> Vec<element::Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let mut rng = rng_from_seed(opts.seed);
  let mut routes: Vec<(usize, Vec<(f64, f64)>)> = vec![];
  let restitution = rng.gen_range(0.0, 0.5);
  let clrs = vec!["white"];

  // global scaling
  let gm = 1.0;
  let gmx = gm;
  let gmy = -gm;
  let gdx = width / 2.0;
  let gdy = height * 0.8;

  let mut rigid_body_set = RigidBodySet::new();
  let mut collider_set = ColliderSet::new();

  /* Create the ground. */
  let collider = ColliderBuilder::cuboid(150.0, 0.1).build();
  collider_set.insert(collider);

  let mut object_handles = vec![];

  /* Create other structures necessary for the simulation. */
  let gravity = vector![0.0, -9.81];
  let integration_parameters = IntegrationParameters::default();
  let mut physics_pipeline = PhysicsPipeline::new();
  let mut island_manager = IslandManager::new();
  let mut broad_phase = BroadPhase::new();
  let mut narrow_phase = NarrowPhase::new();
  let mut impulse_joint_set = ImpulseJointSet::new();
  let mut multibody_joint_set = MultibodyJointSet::new();
  let mut ccd_solver = CCDSolver::new();
  let physics_hooks = ();
  let event_handler = ();
  let count = rng.gen_range(1000, 2000);
  let spawn_each = rng.gen_range(28, 38);
  let extra_sim_steps = 0;

  let sim_total = count * spawn_each + extra_sim_steps;
  for i in 0..sim_total {
    physics_pipeline.step(
      &gravity,
      &integration_parameters,
      &mut island_manager,
      &mut broad_phase,
      &mut narrow_phase,
      &mut rigid_body_set,
      &mut collider_set,
      &mut impulse_joint_set,
      &mut multibody_joint_set,
      &mut ccd_solver,
      None,
      &physics_hooks,
      &event_handler,
    );
    if i % spawn_each == 0 && object_handles.len() < count {
      let rigid_body = RigidBodyBuilder::dynamic()
        .rotation(rng.gen_range(-PI, PI))
        .translation(vector![0.0, 150.0])
        .build();
      let hx = 0.4 + rng.gen_range(0.0, 1.8) * rng.gen_range(0.0, 1.0);
      let hy = hx;
      let coll = ColliderBuilder::cuboid(hx, hy)
        .restitution(restitution)
        .build();
      let handle = rigid_body_set.insert(rigid_body);
      collider_set.insert_with_parent(coll, handle, &mut rigid_body_set);
      object_handles.push((handle, hx, hy));
    }
  }

  let bound = (pad, pad, width - pad, height - pad);
  for (handle, hx, hy) in &object_handles {
    let body = &rigid_body_set[*handle];
    let s = 1.0;
    let dx = body.position().translation.x as f64;
    let dy = body.position().translation.y as f64;
    let ang = body.rotation().angle() as f64;
    let w = (s * hx) as f64;
    let h = (s * hy) as f64;
    let route: Vec<(f64, f64)> =
      vec![(-w, -h), (w, -h), (w, h), (-w, h), (-w, -h)]
        .iter()
        .map(|p| {
          let p = p_r(*p, -ang);
          let p = (dx + p.0, dy + p.1);
          let p = (gmx * p.0 + gdx, gmy * p.1 + gdy);
          p
        })
        .collect();

    if route.iter().any(|p| out_of_boundaries(*p, bound)) {
      continue;
    }
    routes.push((0, route));
  }

  clrs
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let mut data = Data::new();
      for (i, route) in routes.clone() {
        if ci == i {
          data = render_route(data, route);
        }
      }
      let mut l = layer(format!("{} {}", ci, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("black", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
