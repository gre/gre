use main::*;
use rand::prelude::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
  let mut rng = rand::thread_rng();
  let alphabet: Vec<char> =
    "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
      .chars()
      .collect();
  let chars: String = (0..49)
    .map(|_i| alphabet[rng.gen_range(0, alphabet.len())])
    .collect();
  let hash = format!("oo{}", chars);
  let f = File::create("result.stl").unwrap();
  let mut bw = BufWriter::new(f);
  art(&Opts { hash, scale: 60.0 }, &mut bw);
}
