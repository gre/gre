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
        width: 148.5,
        height: 105.,
        pad: 5.,
      },
      false,
    );
    svg::save(format!("results/{}.svg", hash), &doc).unwrap();
    all.push(features);
  }

  let mut inks_counter = HashMap::new();
  let mut inks_count_counter = HashMap::new();
  let mut paper_counter = HashMap::new();
  let mut clouds_density_counter = HashMap::new();
  let mut eagles_density_counter = HashMap::new();
  let mut has_sun_particle_counter = HashMap::new();

  let l = all.len();

  for feature in all {
    let inks_count = inks_counter.entry(feature.inks.clone()).or_insert(0);
    *inks_count += 1;

    let inks_count_count = inks_count_counter
      .entry(feature.inks_count.clone())
      .or_insert(0);
    *inks_count_count += 1;

    let paper_count = paper_counter.entry(feature.paper.clone()).or_insert(0);
    *paper_count += 1;

    let clouds_density_count = clouds_density_counter
      .entry(feature.clouds_density.clone())
      .or_insert(0);
    *clouds_density_count += 1;

    let eagles_density_count = eagles_density_counter
      .entry(feature.eagles_density.clone())
      .or_insert(0);
    *eagles_density_count += 1;

    let has_sun_particle_count = has_sun_particle_counter
      .entry(feature.has_sun_particle.clone())
      .or_insert(0);
    *has_sun_particle_count += 1;
  }

  println!("Inks distribution:");
  for (k, v) in inks_counter.into_iter() {
    println!("{:<30} : {}%", k, (100. * (v as f32)) / (l as f32));
  }

  println!("Inks count distribution:");
  for (k, v) in inks_count_counter.into_iter() {
    println!("{:<30} : {}%", k, (100. * (v as f32)) / (l as f32));
  }

  println!("Paper distribution:");
  for (k, v) in paper_counter.into_iter() {
    println!("{:<30} : {}%", k, (100. * (v as f32)) / (l as f32));
  }

  println!("clouds_density distribution:");
  for (k, v) in clouds_density_counter.into_iter() {
    println!("{:<30} : {}%", k, (100. * (v as f32)) / (l as f32));
  }

  println!("eagles_density distribution:");
  for (k, v) in eagles_density_counter.into_iter() {
    println!("{:<30} : {}%", k, (100. * (v as f32)) / (l as f32));
  }

  println!("has_sun_particle distribution:");
  for (k, v) in has_sun_particle_counter.into_iter() {
    println!("{:<30} : {}%", k, (100. * (v as f32)) / (l as f32));
  }
}
