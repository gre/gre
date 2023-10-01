use main::*;
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    println!("Please provide a number as an argument");
    std::process::exit(1);
  }

  // read count from cli params
  let count = match args[1].parse() {
    Ok(n) => n,
    Err(_) => {
      println!("Please provide a valid number as an argument");
      std::process::exit(1);
    }
  };

  let mut rng = rand::thread_rng();
  let alphabet: Vec<char> =
    "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
      .chars()
      .collect();

  let mut all = Vec::new();

  let alphabet: Vec<char> = "0123456789ABCDEF".chars().collect();
  for _i in 0..count {
    let alphabet: Vec<char> =
      "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
        .chars()
        .collect();
    let chars: String = (0..49)
      .map(|_i| alphabet[rng.gen_range(0, alphabet.len())])
      .collect();
    let hash = format!("oo{}", chars);
    let (doc, features) = art(
      &Opts {
        hash: hash.clone(),
        width: 210.,
        height: 210.,
        pad: 10.0,
      },
      false,
    );
    svg::save(format!("results/{}.svg", hash), &doc).unwrap();
    all.push(features);
  }
}
