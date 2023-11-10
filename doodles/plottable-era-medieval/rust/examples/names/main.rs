use main::*;
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
  let mut rng = rand::thread_rng();
  for _ in 0..20 {
    println!("{}", epic_title(&mut rng));
  }
}
