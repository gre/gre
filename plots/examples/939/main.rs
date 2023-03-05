use clap::*;
use gre::letters::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "examples/939/text.txt")]
  text_file: String,
  #[clap(short, long, default_value = "images/letters.svg")]
  letters_file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "9.2")]
  pub fontsize: f64,
  #[clap(short, long, default_value = "1.3")]
  pub lineheight: f64,
  #[clap(short, long, default_value = "true")]
  pub reversed: bool,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let fontsize = opts.fontsize;
  let lineheight = opts.lineheight * fontsize;
  let text = std::fs::read_to_string(opts.text_file.clone()).unwrap();
  let letters_ref =
    LetterSvgReferential::new(opts.letters_file.clone(), 0.1, 2.0);

  let lines: Vec<&str> = text.lines().collect();
  let mut y = (height - lines.len() as f64 * lineheight) / 2.0;
  let mut routes = Vec::new();
  for line in lines {
    let path = vec![(pad, y), (width - pad, y)];
    routes.extend(
      draw_text(
        &letters_ref,
        line.to_string().to_lowercase(),
        fontsize,
        0.0,
        0.0,
        &path,
      )
      .0,
    );
    y += lineheight;
  }

  if opts.reversed {
    routes = routes
      .iter()
      .map(|route| {
        let mut route = route.clone();
        route.reverse();
        route
      })
      .collect();
    routes.reverse();
  }

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 1.0, data));
      l
    })
    .collect()
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

pub fn draw_text(
  letter_ref: &LetterSvgReferential,
  text: String,           // text to draw
  size: f64,              // font size
  xstart: f64,            // x move on the path
  yoffset: f64,           // make diff baseline
  path: &Vec<(f64, f64)>, // curve to follow
) -> (Vec<Vec<(f64, f64)>>, f64) {
  let mut routes = Vec::new();
  let mut x = 0.;
  let mut y = 0.;
  let mut can_attach = true;
  let mut last: Vec<(f64, f64)> = vec![];
  for c in text.chars() {
    if let Some(letter) = letter_ref.get_letter(&c.to_string()) {
      let (rts, (dx, dy)) = letter.render((x, y), size, false);
      if letter.can_attach && can_attach {
        let mut rts = rts.clone();

        let mut add = rts.pop().unwrap();
        // interpolate curve to attach more smoothly
        if last.len() > 0 {
          let lastp = last[last.len() - 1];
          let firstp = add[0];
          // ygap between last and first
          let ygap = firstp.1 - lastp.1;
          let mut i = 1;
          let mut maxlen = 0.5 * size;
          while i < add.len() {
            if maxlen < 0. {
              break;
            }
            let l = euclidian_dist(add[i - 1], add[i]);
            if ygap > 0.0 {
              if add[i].1 < lastp.1 {
                break;
              }
            } else {
              if add[i].1 > lastp.1 {
                break;
              }
            }
            i += 1;
            maxlen -= l;
          }
          if i == add.len() {
            i -= 1;
          }
          let stopi = i;
          add = add
            .iter()
            .enumerate()
            .map(|(i, &p)| {
              if i <= stopi {
                let y = p.1 - ygap * (1.0 - i as f64 / stopi as f64);
                (p.0, y)
              } else {
                p
              }
            })
            .collect();
        }

        last.extend(add);

        routes.extend(rts); // ° on i and j
      } else {
        if last.len() > 0 {
          routes.push(last);
          last = vec![];
        }
        routes.extend(rts);
      }
      can_attach = letter.can_attach;
      x += dx;
      y += dy;
    } else {
      println!("letter not found: {}", c);
    }
  }
  if last.len() > 0 {
    routes.push(last);
  }

  // rotate with angle and translate to origin all routes
  let mut proj_routes = Vec::new();
  for route in routes {
    let mut proj_route = Vec::new();
    for (x, y) in route {
      // use x to find position in path and project x,y
      if let Some((origin, a)) = lookup_curve_point_and_angle(&path, x + xstart)
      {
        let y = y + yoffset;
        let disp = (-y * a.sin(), y * a.cos());

        let p = (origin.0 + disp.0, origin.1 + disp.1);

        proj_route.push(p);
      } else {
        if proj_route.len() > 1 {
          proj_routes.push(proj_route);
        }
        proj_route = Vec::new();
      }
    }
    if proj_route.len() > 1 {
      proj_routes.push(proj_route);
    }
  }

  (proj_routes, x)
}

fn angle2(p1: (f64, f64), p2: (f64, f64)) -> f64 {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  let dx = x2 - x1;
  let dy = y2 - y1;
  dy.atan2(dx)
}

fn lookup_curve_point_and_angle(
  path: &Vec<(f64, f64)>,
  l: f64,
) -> Option<((f64, f64), f64)> {
  let mut i = 0;
  if l < 0.0 {
    return None;
  }
  let mut len = 0.0;
  while i < path.len() - 1 {
    let l1 = euclidian_dist(path[i], path[i + 1]);
    if len + l1 > l {
      let r = (l - len) / l1;
      let x = path[i].0 + r * (path[i + 1].0 - path[i].0);
      let y = path[i].1 + r * (path[i + 1].1 - path[i].1);
      let angle = angle2(path[i], path[i + 1]);
      return Some(((x, y), angle));
    }
    len += l1;
    i += 1;
  }
  return None;
}
