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
  #[clap(short, long, default_value = "16.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "12.0")]
  pub size: f64,
  #[clap(short, long, default_value = "0.8")]
  pub lineheight: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "0.12")]
  pub end_of_word_prob: f64,
  #[clap(short, long, default_value = "3")]
  pub min_letters: usize,
  #[clap(short, long, default_value = "4")]
  pub letter_simple_min: usize,
  #[clap(short, long, default_value = "10")]
  pub letter_simple_max: usize,
  #[clap(short, long, default_value = "20")]
  pub letter_complex_max: usize,
  #[clap(short, long, default_value = "0.4")]
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
    let y = center.1
      + rng.gen_range(-0.5, 0.5)
        * size
        * rng.gen_range(0.0, 1.0)
        * rng.gen_range(0.0, 1.0)
        * 1.5;
    route.push((x, y));
  }
  route.push((center.0 + w / 2.0, center.1));
  (route, w)
}

fn art(opts: &Opts) -> Vec<Group> {
  let width = opts.width;
  let pad = opts.pad;
  let size = opts.size;
  let mut rng = rng_from_seed(opts.seed);

  let mut routes = Vec::new();

  let mut words = vec![];

  let nb_words = rng.gen_range(1, 60);

  for _i in 0..nb_words {
    let letters = rng.gen_range(2, 12);
    let mut route = vec![];
    let y = 0.0;
    let mut x = 0.0;
    for _j in 0..letters {
      let n = (rng.gen_range(0.0, 26.0) * rng.gen_range(0.4, 1.0)) as usize;
      let (letter, step) = letter(opts, n, (x, y), opts.size);
      x += step;
      route.extend(letter);
    }
    words.push((route, x));
  }

  let mut x = pad + size / 2.0;
  let mut y = pad + size;
  for _paragraph in 0..4 {
    for _line in 0..4 {
      let xmax = width - pad - size;
      loop {
        let (word, word_length) = words[rng.gen_range(0, words.len())].clone();
        // translate word to (x,y)
        let word = word
          .iter()
          .map(|(px, py)| (px + x, py + y))
          .collect::<Vec<(f64, f64)>>();
        if x + word_length > xmax {
          break;
        }
        routes.push(word);
        x += word_length + size / 2.0;
      }
      y += opts.lineheight * size;
      x = pad + size / 2.0;
    }
    y += opts.lineheight * size;
  }

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
