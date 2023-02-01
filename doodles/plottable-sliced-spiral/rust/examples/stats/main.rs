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

  for _i in 0..count {
    let chars: String = (0..49)
      .map(|_i| alphabet[rng.gen_range(0, alphabet.len())])
      .collect();
    let hash = format!("oo{}", chars);
    let (doc, features) = art(
      &Opts {
        hash: hash.clone(),
        width: 297.,
        height: 210.,
        pad: 10.,
      },
      false,
    );
    svg::save(format!("results/{}.svg", hash), &doc).unwrap();
    all.push(features);
  }

  let mut splits_counter = HashMap::new();
  let mut spins_counter = HashMap::new();
  let mut axes_counter = HashMap::new();
  let mut sliding_counter = HashMap::new();
  let mut inks_counter = HashMap::new();
  let mut inks_count_counter = HashMap::new();
  let mut paper_counter = HashMap::new();

  let l = all.len();

  for feature in all {
    let splits_count =
      splits_counter.entry(feature.splits.clone()).or_insert(0);
    *splits_count += 1;

    let spins_count = spins_counter.entry(feature.spins.clone()).or_insert(0);
    *spins_count += 1;

    let axes_count = axes_counter.entry(feature.axes.clone()).or_insert(0);
    *axes_count += 1;

    let sliding_count =
      sliding_counter.entry(feature.sliding.clone()).or_insert(0);
    *sliding_count += 1;

    let inks_count = inks_counter.entry(feature.inks.clone()).or_insert(0);
    *inks_count += 1;

    let inks_count_count = inks_count_counter
      .entry(feature.inks_count.clone())
      .or_insert(0);
    *inks_count_count += 1;

    let paper_count = paper_counter.entry(feature.paper.clone()).or_insert(0);
    *paper_count += 1;
  }

  println!("Splits distribution:");
  for (k, v) in splits_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }

  println!("spins distribution:");
  for (k, v) in spins_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }

  println!("axes distribution:");
  for (k, v) in axes_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }

  println!("Sliding distribution:");
  for (k, v) in sliding_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }

  println!("Inks distribution:");
  for (k, v) in inks_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }

  println!("Inks count distribution:");
  for (k, v) in inks_count_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }

  println!("Paper distribution:");
  for (k, v) in paper_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }
}
