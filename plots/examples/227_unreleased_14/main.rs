use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
struct Opts {}

fn art(_opts: Opts) -> Vec<Group> {
  let get_color = image_get_color("images/pattern_02_e.png").unwrap();
  let colors = vec!["red", "black"];
  colors
    .iter()
    .enumerate()
    .map(|(ci, color)| {
      let pad = 20.0;
      let width = 297.0;
      let height = 210.0;
      let boundaries = (pad, pad, width - pad, height - pad);
      let f = |point: (f64, f64)| {
        let p = preserve_ratio_outside(
          point,
          (boundaries.2 - boundaries.0, boundaries.3 - boundaries.1),
        );
        let rgb = get_color(p);
        let c = if ci == 0 { rgb.0 } else { rgb.2 };
        1.0 - c
      };
      let mut routes = Vec::new(); // all the lines
      let xdivisions = 200; // how much to split the width space
      let lines = 60; // how much to split the height space
      let sublines = 10; // for each line, how much do we make "sublines" to make it grow
      for i in 0..lines {
        let ypi = i as f64 / ((lines - 1) as f64); // y=0..1
        for j in 0..sublines {
          if j % 2 == ci {
            continue;
          }
          let yp = ypi + (j as f64) / ((lines * sublines) as f64); // y=0..1 of the resp subline
          let mut route = Vec::new(); // one line (points to make a curve)
          for k in 0..xdivisions {
            let xp = (k as f64) / ((xdivisions - 1) as f64); // x=0..1
            let v = f((xp, yp)); // lookup from a normalized function
            let p = (
              // our final point (normalized in 0..1)
              xp,
              mix(ypi, yp, v), // interp the position based on f value
            );
            route.push(project_in_boundaries(p, boundaries));
          }
          route.push(route[route.len() - 1]); // as it's a curve, we need to add last point again
          routes.push(route);
        }
      }

      let mut l = layer(color);
      for r in routes {
        let data = render_route_curve(Data::new(), r);
        l = l.add(base_path(color, 0.35, data));
      }
      if ci == 1 {
        l = l.add(signature(0.8, (255.0, 190.0), color));
      }
      l
    })
    .collect()
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
