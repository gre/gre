use clap::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn parametric(p: f64) -> (f64, f64) {
  (
    3. * (p - 0.5) + 0.5 * (10.0 * PI * p).sin(),
    (10.0 * PI * p).cos(),
  )
}

fn art(opts: Opts) -> Vec<Group> {
  let colors = vec!["darkturquoise"];
  let pad = 20.0;
  let width = 297.0;
  let height = 210.0;
  let size = 60.0;
  let bounds = (pad, pad, width - pad, height - pad);

  let line_length = 1000.0;
  let granularity = 1.0;
  let samples = 1000;

  let perlin = Perlin::new();
  let get_angle = |(x, y): (f64, f64), initial_angle, length| -> f64 {
    initial_angle
      - 5.
        * perlin.get([
          (10. * x).floor() / 10.,
          (10. * y).floor() / 10.,
          opts.seed,
        ])
  };

  let initial_data: Vec<((f64, f64), f64)> = (0..samples)
    .map(|s| {
      let sp = s as f64 / (samples as f64);
      let o = parametric(sp);
      let dt = 0.0001;
      let o2 = parametric(sp + dt);
      let initial_angle = (o.1 - o2.1).atan2(o.0 - o2.0);
      let p = (width * 0.5 + size * o.0, height * 0.5 + size * o.1);
      (p, initial_angle)
    })
    .collect();

  let initial_positions: Vec<(f64, f64)> =
    initial_data.iter().map(|&(p, a)| p).collect();

  let initial_angles: Vec<f64> = initial_data.iter().map(|&(p, a)| a).collect();

  let mut parametric = initial_positions.clone();

  let mut build_route = |p: (f64, f64), l, route_i| {
    let normalized = normalize_in_boundaries(p, bounds);
    let initial_angle = initial_angles[route_i];
    let angle = get_angle(normalized, initial_angle, l as f64 * granularity);
    let next = (
      p.0 + granularity * angle.cos(),
      p.1 + granularity * angle.sin(),
    );
    let ends = l as f64 / granularity > line_length;
    /*
    if let Some(c) =
        collide_route_segment(&parametric, p, next)
    {
        return Some((c, true));
    }
    */
    if let Some(c) = collide_segment_boundaries(p, next, bounds) {
      return Some((c, true));
    }
    if ends {
      None
    } else {
      Some((next, false))
    }
  };

  let mut routes =
    build_routes_with_collision_par(initial_positions.clone(), &build_route);

  routes = routes
    .iter()
    .map(|route| round_route(route.clone(), 0.01))
    .collect();

  // routes.push(parametric);

  routes.push(boundaries_route(bounds));

  let mut groups = Vec::new();

  for (i, color) in colors.iter().enumerate() {
    let mut data = Data::new();
    for (j, route) in routes.iter().enumerate() {
      if j % colors.len() == i {
        data = render_route(data, route.clone());
      }
    }

    let mut g = layer(color);

    g = g.add(base_path(color, 0.2, data));

    if i == colors.len() - 1 {
      g = g.add(signature(1.0, (250.0, 190.0), color))
    }

    groups.push(g);
  }

  groups
}
#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "36.0")]
  seed: f64,
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
