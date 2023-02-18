use clap::*;
use gre::letters::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "105.0")]
  pub width: f64,
  #[clap(short, long, default_value = "105.0")]
  pub height: f64,
  #[clap(short, long, default_value = "5.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "2.6")]
  pub size: f64,
  #[clap(short, long, default_value = "0.05")]
  pub letter_precision: f64,
  #[clap(short, long, default_value = "0.3")]
  pub vertical_chance: f64,
  #[clap(short, long, default_value = "1.0")]
  pub non_attached_pad: f64,
  #[clap(short, long, default_value = "1000000")]
  pub max_iterations: usize,
  #[clap(short, long, default_value = "50")]
  pub optimized_count: usize,
  #[clap(short, long, default_value = "images/letters.svg")]
  letters_file: String,
  #[clap(short, long)]
  debug: bool,
}

fn art(opts: &Opts, letters_ref: &LetterSvgReferential) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;
  let max_iterations = opts.max_iterations;
  let optimized_count = opts.optimized_count;

  let mut rng = rng_from_seed(opts.seed);
  let mut routes = Vec::new();
  let mut words = vec![];
  let mut last_point = (width * 0.5, height * 0.5);

  for sentence in vec![" good morning ", "gm "] {
    loop {
      let mut word = TextRendering::new(
        sentence.to_string(),
        size,
        rng.gen_bool(opts.vertical_chance),
        letters_ref,
        opts.debug,
      );
      if let Some((x, y)) = word.find_location(
        &mut rng,
        &words,
        max_iterations,
        width,
        height,
        pad,
        last_point,
        optimized_count,
      ) {
        word.set_pos(x, y);
        last_point = word.get_pos_end_word();
        routes.extend(word.draw(letters_ref));
        words.push(word);
      } else {
        break;
      }
    }
  }

  vec![(routes, "#000")]
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

  let letters_ref = LetterSvgReferential::new(
    opts.letters_file.clone(),
    opts.letter_precision,
    opts.non_attached_pad,
  );

  let groups = art(&opts, &letters_ref);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
