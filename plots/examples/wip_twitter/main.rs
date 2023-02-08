use clap::*;
use gre::*;
use svg::node::element::path::Data;
use svg::node::element::*;

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
  #[clap(short, long, default_value = "followers.txt")]
  list_file: String,
}

fn art(opts: &Opts, list: Vec<&str>) -> Vec<Group> {
  let width = opts.width;
  let height = opts.height;
  let pad = opts.pad;

  let mut routes = Vec::new();

  routes.push(vec![
    (pad, pad),
    (width - pad, pad),
    (width - pad, height - pad),
    (pad, height - pad),
    (pad, pad),
  ]);

  vec![(routes, "black")]
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

  let list = file_content.split_ascii_whitespace().collect::<Vec<_>>();
  let groups = art(&opts, list);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save(opts.file, &document).unwrap();
}
