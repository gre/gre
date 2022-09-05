use clap::*;
use gre::*;
use noise::*;
use rand::prelude::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let colors = vec!["red", "white"];
  let pad = 20.0;
  let width = 297.0;
  let height = 210.0;
  let size = 60.0;
  let bounds = (pad, pad, width - pad, height - pad);

  let line_length = 200.0;
  let granularity = 1.0;
  let samples = opts.samples;
  let seed = opts.seed;
  let perlin = Perlin::new();
  let mut passage = Passage2DCounter::new(0.3, width, height);

  let mut rng = SmallRng::from_seed([seed as u8; 16]);
  for _i in 0..50 {
    rng.gen::<f64>();
  }
  let a = rng.gen_range(0.1, 1.2);
  let b = rng.gen_range(0.1, 0.9);
  let c = rng.gen_range(0.1, 0.6);
  let d = rng.gen_range(0.1, 0.6);
  let e = rng.gen_range(1.0, 3.0);
  let f = rng.gen_range(0.0, 0.4);
  let g = rng.gen_range(0.0, 0.4);

  let parametric = |t: f64| {
    (
      a * (2. * PI * t).cos() + c * (18. * PI * t).cos(),
      b * (2. * PI * t).sin() - d * (10. * PI * t).cos(),
    )
  };

  let get_angle = |p: (f64, f64), initial_angle, i| {
    initial_angle
      + e * perlin.get([2. * p.0 / width, 2. * p.1 / height, 10. + seed])
      + f * perlin.get([20. * p.0 / width, 20. * p.1 / height, 10. + seed])
      + g
        * perlin.get([
          200. * p.0 / width,
          200. * p.1 / height,
          seed + i as f64 / 100.0,
        ])
  };

  let samples_data: Vec<(f64, (f64, f64))> = (0..samples)
    .map(|i| {
      let sp = i as f64 / (samples as f64);
      let o = parametric(sp);
      let mut initial_angle = if o.0 < 0. { PI } else { 0. };
      if opts.reversed {
        initial_angle += PI;
      }
      let p = (width * 0.5 + size * o.0, height * 0.5 + size * o.1);
      (initial_angle, p)
    })
    .collect();

  let initial_positions = samples_data.iter().map(|&(_a, p)| p).collect();

  let parametric_route: Vec<(f64, f64)> =
    samples_data.iter().map(|&(_a, p)| p).collect();

  let mut build_route = |p, i, j| {
    let length = i as f64 * granularity;
    if length >= line_length {
      return None; // line ends
    }
    let (initial_angle, _o) = samples_data[j];
    let angle = get_angle(p, initial_angle, j);
    let nextp = follow_angle(p, angle, granularity);
    if let Some(edge_p) = collide_segment_boundaries(p, nextp, bounds) {
      return Some((edge_p, true));
    }
    if i > 1 {
      if let Some(c) = collide_route_segment(&parametric_route, p, nextp) {
        return Some((c, true));
      }
    }
    let count = passage.count(nextp);
    if count > 2 {
      return None; // too much passage here
    }
    return Some((nextp, false));
  };

  let routes =
    // lines
    build_routes(
        initial_positions,
        &mut build_route,
    );

  // parametric curve itself
  // routes.push(parametric_route);

  // frame
  // routes.push(boundaries_route(bounds));

  colors
    .iter()
    .enumerate()
    .map(|(i, color)| {
      let data = routes
        .iter()
        .enumerate()
        .filter(|(j, _route)| j % colors.len() == i)
        .fold(Data::new(), |data, (_j, route)| {
          render_route(data, route.clone())
        });

      let mut g = layer(color);
      g = g.add(base_path(color, 0.2, data));
      if i == colors.len() - 1 {
        g = g.add(signature(1.0, (250.0, 180.0), color))
      }
      return g;
    })
    .collect()
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "11.0")]
  seed: f64,
  #[clap(short, long, default_value = "2001")]
  samples: usize,
  #[clap(short, long)]
  reversed: bool,
}
fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_landscape("black");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
