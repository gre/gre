use main::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

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

  for i in 0..count {
    let chars: String = (0..49)
      .map(|_i| alphabet[rng.gen_range(0, alphabet.len())])
      .collect();
    let hash = format!("oo{}", chars);
    let f = File::create(format!("results/{}.stl", i)).unwrap();
    let mut bw = BufWriter::new(f);
    let features = art(&Opts { hash, scale: 60.0 }, &mut bw);
    all.push(features);
  }

  let mut resolution_style_counter = HashMap::new();
  let mut complexity_counter = HashMap::new();
  let mut crystal_counter = HashMap::new();
  let mut style_counter = HashMap::new();

  let l = all.len();

  for feature in all {
    let resolution_style_count = resolution_style_counter
      .entry(feature.resolution_style.clone())
      .or_insert(0);
    *resolution_style_count += 1;
    let complexity_count = complexity_counter
      .entry(feature.complexity.clone())
      .or_insert(0);
    *complexity_count += 1;
    let crystal_count =
      crystal_counter.entry(feature.crystal.clone()).or_insert(0);
    *crystal_count += 1;
    let style_count = style_counter.entry(feature.style.clone()).or_insert(0);
    *style_count += 1;
  }

  println!("Resolution style distribution:");
  for (k, v) in resolution_style_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }
  println!("Complexity distribution:");
  for (k, v) in complexity_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }
  println!("Crystal distribution:");
  for (k, v) in crystal_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }
  println!("Style:");
  for (k, v) in style_counter.into_iter() {
    println!("{:<30} : {}%", k, (100 * v) / l);
  }
}
