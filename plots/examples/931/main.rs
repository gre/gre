use clap::*;
use gre::letters::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

pub fn draw_text(
  letter_ref: &LetterSvgReferential,
  text: String,
  size: f64,
  path: Vec<(f64, f64)>,
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
      // TODO handle path, not just the first point as origin
      let p = (x + path[0].0, y + path[0].1);
      proj_route.push(p);
    }
    proj_routes.push(proj_route);
  }

  (proj_routes, x)
}

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub width: f64,
  #[clap(short, long, default_value = "210.0")]
  pub height: f64,
  #[clap(short, long, default_value = "10.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "2.0")]
  pub fontsize: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.fontsize;

  let mut routes = Vec::new();

  let mut rng = rng_from_seed(opts.seed);

  let letters_ref =
    LetterSvgReferential::new("images/letters.svg".to_string(), 0.1, 1.0);

  let letters = "abcdefghijklmnopqrstuvwxyz".to_string();

  let mut y = pad;

  while y < height - pad {
    let w = width - pad; //rng.gen_range(0.3 * width, width - pad);
    let mut x = pad;
    loop {
      let word_size = rng.gen_range(5, 10);
      let random_word = (0..word_size)
        .map(|_| {
          letters
            .chars()
            .nth(rng.gen_range(0, letters.len()))
            .unwrap()
        })
        .collect::<String>();

      let path = vec![(x, y), (x + 100.0, y)];

      let (rts, l) = draw_text(&letters_ref, random_word, size, path);

      x += l + size;

      if x > w {
        break;
      }

      routes.extend(rts);
    }
    y += size;
  }

  vec![(routes, "#333")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route(data, route);
      }
      let mut l = layer(format!("{} {}", i, String::from(*color)).as_str());
      l = l.add(base_path(color, 0.35, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("#eee", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
