use std::{
  fs::File,
  io::{BufWriter, Write},
};

use greweb::*;
use rand::prelude::*;

fn main() {
  let mut rng = rand::thread_rng();
  let alphabet: Vec<char> =
    "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
      .chars()
      .collect();
  let chars: String = (0..49)
    .map(|_i| alphabet[rng.gen_range(0..alphabet.len())])
    .collect();
  let hash = format!("oo{}", chars);
  let art = render(hash, true);
  let f = File::create("result.stl").unwrap();
  let mut bw = BufWriter::new(f);
  let bytes = art.stl();
  bw.write_all(&bytes).unwrap();
}
