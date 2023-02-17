use clap::*;
use gre::letters::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

// TODO o, v, w => try to make it work with the ending raising and eating on the next letter a bit?
// TODO i, j => try to make a pen up in between to do the ° logically in plot order

#[derive(Parser)]
#[clap()]
pub struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "500.0")]
  pub width: f64,
  #[clap(short, long, default_value = "700.0")]
  pub height: f64,
  #[clap(short, long, default_value = "20.0")]
  pub pad: f64,
  #[clap(short, long, default_value = "0.0")]
  pub seed: f64,
  #[clap(short, long, default_value = "2.6")]
  pub size: f64,
  #[clap(short, long, default_value = "0.1")]
  pub letter_precision: f64,
  #[clap(short, long, default_value = "0.45")]
  pub vertical_chance: f64,
  #[clap(short, long, default_value = "1.0")]
  pub non_attached_pad: f64,
  #[clap(short, long, default_value = "500000")]
  pub max_iterations: usize,
  #[clap(short, long, default_value = "40")]
  pub optimized_count: usize,
  #[clap(short, long, default_value = "followers.txt")]
  list_file: String,
  #[clap(short, long, default_value = "images/letters.svg")]
  letters_file: String,
  #[clap(short, long, default_value = "images/twitter5k.png")]
  image_layer: String,
  #[clap(short, long)]
  debug: bool,
}

fn art(
  opts: &Opts,
  list: Vec<&str>,
  letters_ref: &LetterSvgReferential,
) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;
  let size = opts.size;

  let mut routes_inner = Vec::new();
  let mut routes_outer = Vec::new();
  let mut words = Vec::new();

  let mut list = list.clone();
  list.sort_by(|a, b| b.len().cmp(&a.len()));

  let mut rng = rng_from_seed(opts.seed);

  let get_color = image_get_color(opts.image_layer.as_str()).unwrap();

  let mut last_point = (width * 0.5, height * 0.5);

  let max_iterations = opts.max_iterations;
  let optimized_count = opts.optimized_count;

  for (i, item) in list.iter().enumerate() {
    if i % 100 == 0 {
      println!("{} / {}", i, list.len());
    }
    let mut word = Word::new(
      item.to_string().to_lowercase(),
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
      let sz = word.size();
      let px = (x + sz.0 * 0.5) / width;
      let py = ((y + sz.1 * 0.5) - (height - width) / 2.0) / width;
      let c = get_color((px, py));
      word.set_pos(x, y);
      last_point = word.get_pos_end_word();
      if c.2 > 0.5 {
        routes_inner.extend(word.draw(letters_ref));
      } else {
        routes_outer.extend(word.draw(letters_ref));
      }
      words.push(word);
    } else {
      println!("no location found for {} at index {}", item, i);
      break;
    }
  }

  vec![(routes_inner, "#000"), (routes_outer, "#f0f")]
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
  let list_file = opts.list_file.clone();
  let file_content = match std::fs::read_to_string(list_file.clone()) {
    Ok(list) => list,
    Err(_) => {
      let bearer = std::env::var("TWITTER_BEARER").unwrap();
      let mut list = String::new();
      let mut cursor = "-1".to_string();
      while cursor != "0" {
        let url = format!(
          "https://api.twitter.com/1.1/followers/list.json?cursor={}&skip_status=1&count=200&screen_name=greweb&skip_status=true&include_user_entities=false",
          cursor
        );
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
          reqwest::header::AUTHORIZATION,
          format!("Bearer {}", bearer).parse().unwrap(),
        );
        let client = reqwest::blocking::Client::builder()
          .default_headers(headers)
          .build()
          .unwrap();
        let response = client.get(url.as_str()).send().unwrap();
        let body = response.text().unwrap();
        let json: serde_json::Value =
          serde_json::from_str(body.as_str()).unwrap();
        cursor = json["next_cursor_str"].as_str().unwrap().to_string();
        for user in json["users"].as_array().unwrap() {
          list.push_str(
            format!("{}\n", user["screen_name"].as_str().unwrap()).as_str(),
          );
        }
      }
      std::fs::write(list_file.clone(), list.clone()).unwrap();
      list
    }
  };

  let letters_ref = LetterSvgReferential::new(
    opts.letters_file.clone(),
    opts.letter_precision,
    opts.non_attached_pad,
  );

  let list = file_content.split_ascii_whitespace().collect::<Vec<_>>();
  let groups = art(&opts, list, &letters_ref);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
