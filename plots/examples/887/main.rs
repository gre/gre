use clap::*;
use gre::*;
use rand::Rng;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "297.0")]
  pub height: f64,
  #[clap(short, long, default_value = "210.0")]
  pub width: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "6.0")]
  pub size: f64,
  #[clap(short, long, default_value = "1.2")]
  pub lineheight: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.12")]
  pub end_of_word_prob: f64,
  #[clap(short, long, default_value = "3")]
  pub min_letters: usize,
  #[clap(short, long, default_value = "1")]
  pub letter_simple_min: usize,
  #[clap(short, long, default_value = "8")]
  pub letter_simple_max: usize,
  #[clap(short, long, default_value = "12")]
  pub letter_complex_max: usize,
  #[clap(short, long, default_value = "0.7")]
  pub letter_ratio: f64,
}

fn letter(
  opts: &Opts,
  n: usize,
  center: (f64, f64),
  size: f64,
) -> (Vec<(f64, f64)>, f64) {
  let w = size * opts.letter_ratio;
  let mut rng = rng_from_seed(opts.seed + n as f64 / 7.77);
  let mut route = vec![];
  let count = rng.gen_range(opts.letter_simple_min, opts.letter_simple_max)
    + (rng.gen_range(0, opts.letter_complex_max) as f64
      * rng.gen_range(0.0, 1.0)
      * rng.gen_range(0.0, 1.0)) as usize;
  for _i in 0..count {
    let x = center.0 + rng.gen_range(-0.5, 0.5) * w;
    let y = center.1 + rng.gen_range(-0.5, 0.5) * size;
    route.push((x, y));
  }
  route.push((center.0 + w / 2.0, center.1));
  (route, w)
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();

  let lineheight = opts.lineheight;

  let mut x = pad + size / 2.0;
  let mut y = pad + size;
  let mut word_length = 0;
  let mut line_is_title = true;
  let mut title_length = rng.gen_range(width * 0.3, width);

  let mut route = vec![];

  while y < height - pad - size {
    let n = (rng.gen_range(0.0, 26.0) * rng.gen_range(0.4, 1.0)) as usize;
    let (letter, step) = letter(opts, n, (x, y), opts.size);
    x += step;

    let end_of_paragraph = rng.gen_bool(1.0 / 500.0);

    let end_of_line = end_of_paragraph
      || x > width - pad - size / 2.0
      || line_is_title && x > title_length;

    if !end_of_line {
      route.extend(letter);
    }

    let end_of_word =
      word_length >= opts.min_letters && rng.gen_bool(opts.end_of_word_prob);

    let should_add_word = end_of_word || end_of_line || end_of_paragraph;

    if should_add_word {
      if word_length >= opts.min_letters {
        route.push(route[route.len() - 1]);
        if line_is_title {
          routes.push(route.clone());
          routes.push(translate_route(route.clone(), (0.4, 0.0)));
          routes.push(translate_route(route.clone(), (0.2, 0.2)));
        } else {
          routes.push(route);
        }
      }
      route = vec![];
      word_length = 0;
    } else {
      word_length += 1;
    }

    if end_of_word {
      x += size;
    }

    if end_of_line {
      x = pad + size / 2.0;
      y += opts.size * lineheight;

      if line_is_title {
        y += opts.size * lineheight * 0.2;
      }

      // randomly add one empty line
      if end_of_paragraph {
        y += opts.size * lineheight;
      }

      if !line_is_title {
        title_length = rng.gen_range(width * 0.3, width);
        line_is_title = rng.gen_bool(1.0 / 20.0);
        if line_is_title {
          y += opts.size * lineheight;
        }
      } else {
        line_is_title = false;
      }
    }

    if end_of_paragraph && rng.gen_bool(0.2) {
      break;
    }
  }

  routes.push(route);

  vec![(routes, "black")]
    .iter()
    .enumerate()
    .map(|(i, (routes, color))| {
      let mut data = Data::new();
      for route in routes.clone() {
        data = render_route_curve(data, route);
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
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}

fn translate_route(
  route: Vec<(f64, f64)>,
  (tx, ty): (f64, f64),
) -> Vec<(f64, f64)> {
  route.iter().map(|&(x, y)| (x + tx, y + ty)).collect()
}
