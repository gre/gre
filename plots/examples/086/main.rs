use clap::*;
use gre::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::*;

fn art(opts: &Opts) -> Vec<Group> {
  let (width, height) = if opts.square {
    (210., 210.)
  } else if opts.portrait {
    (210., 297.)
  } else {
    (297., 210.)
  };

  let get_color = image_get_color(opts.path.as_str()).unwrap();
  let pad = 20.0;
  let boundaries = (pad, pad, width - pad, height - pad);

  let map = vec![3, 0, 2, 1];

  vec![(0.3, 1.0, 0.0), (0., 1., 1.), (1., 0., 1.), (0.3, 1.0, 0.0)]
    .iter()
    .enumerate()
    .map(|(g, &clr)| {
      let name = &format!(
        "rgb({},{},{})",
        (255f64 * clr.0).floor(),
        (255f64 * clr.1).floor(),
        (255f64 * clr.2).floor()
      );
      let cang = if g == 3 {
        PI / 2.
      } else {
        g as f64 * 2. * PI / 3.
      };
      let camp = opts.color_dist;
      let c = (0.5 + camp * cang.cos(), 0.5 + camp * cang.sin());
      let samples = opts.spins_sampling * opts.spins;
      let radius = 4.;
      let mut routes = Vec::new();
      let mut route = Vec::new();
      let mut pen_up = true;
      for i in 0..samples {
        let ii = (i as f64) / (samples as f64);
        let a = 2. * PI * (opts.spins as f64) * ii;
        let r = radius * (ii + (g as f64 / (4. * (opts.spins as f64))));
        let pn = (c.0 + r * a.cos(), c.1 + r * a.sin());

        let p = project_in_boundaries(pn, boundaries);
        let rgb = get_color(preserve_ratio_outside(pn, (width, height)));
        let dist =
          euclidian_rgb_distance(clr, rgb) + if g == 3 { 0.5 } else { 0.0 };
        let draw = dist
          < 1.4
            * ((map[((opts.spins as f64 * ii) as usize) % map.len()] as f64)
              / (map.len() as f64))
          && !out_of_boundaries(p, boundaries);
        if draw {
          if pen_up {
            if route.len() > 2 {
              routes.push(route);
            }
            route = Vec::new();
            pen_up = false;
          }
          route.push(p);
        } else {
          pen_up = true;
        }
      }
      if route.len() > 2 {
        routes.push(route);
      }

      let data = routes
        .iter()
        .fold(Data::new(), |acc, route| render_route(acc, route.clone()));

      let mut l =
        layer(name).add(base_path(name, 0.2, data).set("opacity", 0.8));
      if g == 2 {
        l = l.add(signature(
          1.0,
          (width - 35. - opts.padsig.0, height - 15. - opts.padsig.1),
          name,
        ))
      }
      l
    })
    .collect()
}

fn parse_pad(s: &str) -> Result<(f64, f64), String> {
  let all: Vec<f64> = s
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|str| str.parse::<f64>().unwrap())
    .collect();
  if all.len() == 0 {
    return Ok((0., 0.));
  }
  if all.len() == 1 {
    return Ok((all[0], all[0]));
  }
  return Ok((all[0], all[1]));
}

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "images/pattern_02_first_mint.png")]
  path: String,
  #[clap(short, long, default_value = "1000")]
  spins: usize,
  #[clap(short, long, default_value = "2000")]
  spins_sampling: usize,
  #[clap(short, long, default_value = "1.2")]
  color_dist: f64,
  #[clap(short, long)]
  portrait: bool,
  #[clap(short, long)]
  square: bool,
  #[clap(short, long, default_value = "12,5", parse(try_from_str = parse_pad))]
  padsig: (f64, f64),
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = if !opts.portrait {
    base_a4_landscape("white")
  } else {
    base_a4_portrait("white")
  };
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
