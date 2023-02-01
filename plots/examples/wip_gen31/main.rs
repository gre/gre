use clap::*;
use gre::*;
use kiss3d::nalgebra::*;
use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::Document;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn main() {
  let opts: Opts = Opts::parse();
  let document = art(&opts);
  svg::save(opts.file, &document).unwrap();
}

#[derive(Clone, Debug)]
pub struct Feature {
  pub splits: String,
  pub turns: String,
  pub alignments: String,
  pub sliding: String,
  pub inks: String,
  pub paper: String,
}

struct Color(&'static str, &'static str, &'static str);

fn art(opts: &Opts) -> Document {
  let width = opts.width;
  let height = opts.height;
  let pad = 10.0;

  let mut rng = rng_from_seed(opts.seed);

  let seibokublue = Color("blue", "Sailor Sei-boku", "#1060a3");
  let inaho = Color("inaho", "iroshizuku ina-ho", "#ad9c6a");
  let gold_gel = Color("gold", "Gold Gel", "#e6c100");
  let white_gel = Color("white", "White Gel", "#ffffff");
  let black = Color("black", "Black", "#000000");
  let grey = Color("grey", "Grey", "#808080");
  let amber = Color("amber", "Amber", "#f7a800");
  let red = Color("red", "Brillant Red", "#ee3300");
  let blue_gel = Color("blue", "Blue Gel", "#4466ee");

  let white_paper = Color("white", "White", "#ffffff");
  let red_paper = Color("red", "Red", "#cc0000");
  let darkblue_paper = Color("darkblue", "Dark Blue", "#000033");
  let black_paper = Color("darkblue", "Dark Blue", "#000000");

  let (mut colors, paper) = if rng.gen_bool(0.25) {
    (vec![seibokublue, inaho], white_paper)
  } else if rng.gen_bool(0.25) {
    (vec![white_gel, blue_gel], black_paper)
  } else if rng.gen_bool(0.25) {
    (vec![white_gel, gold_gel], black_paper)
  } else if rng.gen_bool(0.2) {
    (vec![black, grey], white_paper)
  } else if rng.gen_bool(0.15) {
    (vec![red, amber], white_paper)
  } else if rng.gen_bool(0.25) {
    (vec![white_gel, gold_gel], darkblue_paper)
  } else {
    (vec![white_gel], red_paper)
  };

  if rng.gen_bool(0.2) {
    colors.reverse();
  }

  let colors_count = colors.len().min(rng.gen_range(1, 3));

  // global random values that drives the variation
  let a_delta = rng.gen_range(-PI, PI);
  let disp = rng.gen_range(2.0, 4.0);
  let adisp = rng.gen_range(0.4, 1.0);
  let dr = disp + rng.gen_range(10.0, 20.0) * rng.gen_range(0.0, 1.0);
  let r = 80.0;
  let count = (8.0 * (disp + adisp)) as usize;

  let mut routes = vec![];
  for _i in 0..count {
    // randomly offset of the position
    let x = width / 2.0 + rng.gen_range(-disp, disp);
    let y = height / 2.0 + rng.gen_range(-disp, disp);
    // randomly offset of the initial angle
    let start_a = a_delta + rng.gen_range(-adisp, adisp);
    let points = spiral(x, y, r, dr, start_a);
    routes.push((0, points));
  }

  let count = rng.gen_range(2, 10);
  let mut prev_a = PI / 2.0;
  let rotations = (0..count)
    .map(|_i| {
      let a = if rng.gen_bool(0.3) {
        prev_a + PI / 2.0
      } else if rng.gen_bool(0.3) {
        prev_a + PI / 4.0
      } else {
        prev_a
          + rng.gen_range(-PI, PI)
            * rng.gen_range(0.0, 1.0)
            * rng.gen_range(0.0, 1.0)
      };
      prev_a = a;
      a
    })
    .collect::<Vec<f64>>();

  // statistic way to store the rots used
  let mut dedup_rot = vec![];

  let count = rng.gen_range(1usize, 20);
  let split = rng.gen_range(0.2, 1.2);
  let max_slide = rng.gen_range(0.0, 20.0);
  let shake = rng.gen_range(0.0, 1.0);
  let mut total_displacement = 0.0;
  for i in 0..count {
    let (x, y) = if i == 0 {
      (width / 2.0, height / 2.0)
    } else {
      (
        width * rng.gen_range(0.3, 0.7),
        height * rng.gen_range(0.3, 0.7),
      )
    };
    let a = rotations[(rng.gen_range(0., rotations.len() as f64)
      * rng.gen_range(0.0, 1.0)) as usize];
    let dx = a.cos();
    let dy = a.sin();
    let amp = 200.0;
    let left = (x - amp * dx, y - amp * dy);
    let right = (x + amp * dx, y + amp * dy);
    let slice = slice_routes(routes.clone(), left, right);
    let slide = rng.gen_range(0.0, max_slide)
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(shake, 1.0);
    let l = euclidian_dist(slice.a, slice.b);

    let v = if l == 0.0 {
      (0.0, 0.0)
    } else {
      ((slice.b.0 - slice.a.0) / l, (slice.b.1 - slice.a.1) / l)
    };
    let n = (v.1, -v.0);

    if slice.routes_above.len() > 0 && slice.routes_below.len() > 0 {
      let r = (((a / PI + 100.0) % 1.0) * 360.0) as usize;
      if !dedup_rot.contains(&r) {
        dedup_rot.push(r);
      }
      total_displacement += slide;
    }

    routes = vec![
      translate_routes(
        slice.routes_above,
        (v.0 * slide + n.0 * split, v.1 * slide + n.1 * split),
      )
      .iter()
      .map(|(ci, route)| ((ci + 1) % colors_count, route.clone()))
      .collect(),
      translate_routes(
        slice.routes_below,
        (-v.0 * slide - n.0 * split, -v.1 * slide - n.1 * split),
      ),
    ]
    .concat();
  }

  let mut min_x = width;
  let mut min_y = height;
  let mut max_x = 0.0;
  let mut max_y = 0.0;
  for (_, route) in routes.iter() {
    for &(x, y) in route.iter() {
      min_x = min_x.min(x);
      min_y = min_y.min(y);
      max_x = max_x.max(x);
      max_y = max_y.max(y);
    }
  }
  let w = max_x - min_x;
  let h = max_y - min_y;

  let scale = if h < w {
    ((width - 2.0 * pad) / w).min((height - 2.0 * pad) / h)
  } else {
    ((width - 2.0 * pad) / h).min((height - 2.0 * pad) / w)
  };

  let mut color_presence = vec![false, false];

  routes = routes
    .iter()
    .flat_map(|(ci, route)| {
      if route.len() == 0 {
        return None;
      }
      color_presence[*ci] = true;
      let route = route.iter().map(|&(x, y)| {
        let mut p = (x - min_x - w / 2., y - min_y - h / 2.);
        if h > w {
          // rotate 90°
          p = (p.1, -p.0);
        }
        p = round_point(
          (scale * p.0 + width / 2., scale * p.1 + height / 2.),
          0.01,
        );
        p
      });
      Some((*ci, route.collect()))
    })
    .collect();

  // Infer from the generated pieces the main features

  let mut inks = vec![];
  for (i, &present) in color_presence.iter().enumerate() {
    if present {
      inks.push(colors[i].1);
    }
  }
  let feature = Feature {
    splits: (match count {
      0..=5 => "Low",
      6..=10 => "Medium",
      _ => "High",
    })
    .to_string(),
    turns: (match (r / dr).ceil() as usize {
      0..=7 => "Low",
      8..=15 => "Medium",
      _ => "High",
    })
    .to_string(),
    alignments: (match dedup_rot.len() {
      0 => "None",
      1 => "One",
      2 => "Two",
      3 => "Three",
      4 => "Four",
      5 => "Five",
      _ => "Many",
    })
    .to_string(),
    sliding: (match total_displacement.round() as usize {
      0..=1 => "None",
      2..=10 => "Low",
      11..=25 => "Medium",
      26..=50 => "High",
      _ => "Extreme",
    })
    .to_string(),
    inks: inks.join(", "),
    paper: paper.1.to_string(),
  };
  println!("feature: {:?}", feature);

  // Generate the SVG

  let mut document = base_document(paper.2, opts.width, opts.height);
  for (i, &Color(_id, name, color)) in colors.iter().enumerate() {
    let mut data = Data::new();
    let mut l = layer(name);

    for (ci, route) in routes.iter() {
      if *ci == i {
        data = render_route(data, route.clone());
      }
    }

    l = l.add(base_path(color, 0.35, data));
    document = document.add(l);
  }
  document
}

fn spiral(
  x: f64,
  y: f64,
  radius: f64,
  dr: f64,
  start_a: f64,
) -> Vec<(f64, f64)> {
  let two_pi = 2.0 * PI;
  let mut route = Vec::new();
  let mut r = radius;
  let mut a = start_a;
  loop {
    route.push((x + r * a.cos(), y + r * a.sin()));
    let da = 1.0 / (r + 8.0);
    a = (a + da) % two_pi;
    r -= dr * da / two_pi;
    if r < 0.1 {
      break;
    }
  }
  route
}

struct Slice {
  routes_above: Vec<(usize, Vec<(f64, f64)>)>,
  routes_below: Vec<(usize, Vec<(f64, f64)>)>,
  a: (f64, f64),
  b: (f64, f64),
}

fn slice_routes(
  routes: Vec<(usize, Vec<(f64, f64)>)>,
  cuta: (f64, f64),
  cutb: (f64, f64),
) -> Slice {
  let mut routes_above = Vec::new();
  let mut routes_below = Vec::new();

  let mut amin = lerp_point(cuta, cutb, 0.5);
  let mut bmin = amin;
  let mut dista = 99999.0;
  let mut distb = 0.0;

  for (clr, r) in routes.clone() {
    if r.len() < 2 {
      continue;
    }
    let mut prev = r[0];
    let mut route = vec![prev];
    for &p in r.iter().skip(1) {
      if let Some(c) = collides_segment(prev, p, cuta, cutb) {
        let la = euclidian_dist(c, cuta);
        if la > distb {
          distb = la;
          bmin = c;
        }
        if la < dista {
          dista = la;
          amin = c;
        }

        route.push(c);
        if route.len() > 1 {
          if !is_left(cuta, cutb, prev) {
            routes_above.push((clr, route));
          } else {
            routes_below.push((clr, route));
          }
        }
        route = vec![c, p];
      } else {
        route.push(p);
      }
      prev = p;
    }
    if route.len() > 1 {
      if !is_left(cuta, cutb, prev) {
        routes_above.push((clr, route));
      } else {
        routes_below.push((clr, route));
      }
    }
  }

  Slice {
    routes_above,
    routes_below,
    a: amin,
    b: bmin,
  }
}

fn lerp_point(a: (f64, f64), b: (f64, f64), m: f64) -> (f64, f64) {
  (a.0 * (1. - m) + b.0 * m, a.1 * (1. - m) + b.1 * m)
}

fn is_left(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
  ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)) > 0.0
}

fn translate_routes(
  routes: Vec<(usize, Vec<(f64, f64)>)>,
  (tx, ty): (f64, f64),
) -> Vec<(usize, Vec<(f64, f64)>)> {
  routes
    .iter()
    .map(|(i, route)| {
      (*i, route.iter().map(|&(x, y)| (x + tx, y + ty)).collect())
    })
    .collect()
}
