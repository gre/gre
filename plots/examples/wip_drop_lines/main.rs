use clap::*;
use gre::*;
use noise::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: Opts) -> Vec<Group> {
  let colors = vec!["green", "green"];
  let pad = 20.0;
  let width = 210.0;
  let height = 297.0;
  let bounds = (pad, pad, width - pad, height - pad);

  let line_length = 300.0;
  let granularity = 1.0;
  let samples = 200;

  let perlin = Perlin::new();

  let initial_positions: Vec<(f64, f64)> = (0..samples)
    .map(|s| {
      let sp = s as f64 / (samples as f64);
      (pad + (width - 2. * pad) * sp, pad + 0.1)
    })
    .collect();

  let get_angle = |(x, y): (f64, f64), _length, i| -> f64 {
    PI / 2.0 + 0.02 * perlin.get([10. * x, 10. * y, 1.0])
  };

  let build_route = |p: (f64, f64), l, route_i| {
    let normalized = normalize_in_boundaries(p, bounds);
    let angle = get_angle(normalized, l as f64 * granularity, route_i);
    let next = (
      p.0 + granularity * angle.cos(),
      p.1 + granularity * angle.sin(),
    );
    let len =
      line_length * (0.5 + 0.5 * perlin.get([4. * p.0, 4. * p.1, opts.seed]));
    let ends = l as f64 * granularity > len;
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
      g = g.add(signature(1.0, (165.0, 278.0), color))
    }

    groups.push(g);
  }

  groups
}
#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "26.0")]
  seed: f64,
  #[clap(short, long, default_value = "30.0")]
  divisor: f64,
  #[clap(short, long, default_value = "1.0")]
  power: f64,
}
fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(opts);
  let mut document = base_a4_portrait("white");
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
