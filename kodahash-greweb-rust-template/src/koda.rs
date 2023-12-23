use rand::prelude::*;

pub fn rng_from_hash(hash: &String) -> StdRng {
  let v = hash.chars().skip(2).take(64).collect::<Vec<char>>();
  let mut bs = [0; 32];
  for i in 0..32 {
    let mut hex = String::new();
    hex.push(v[i * 2]);
    hex.push(v[i * 2 + 1]);
    bs[i] = u8::from_str_radix(&hex, 16).unwrap();
  }
  let rng = StdRng::from_seed(bs);
  return rng;
}
