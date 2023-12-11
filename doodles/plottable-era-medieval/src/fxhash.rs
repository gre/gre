use rand::prelude::*;

pub fn rng_from_hash(hash: &String) -> StdRng {
  let v = bs58::decode(hash.chars().skip(2).take(43).collect::<String>())
    .into_vec()
    .unwrap();
  let mut bs = [0; 32];
  for (i, &byte) in v.iter().enumerate().take(bs.len()) {
    bs[i] = byte;
  }
  let rng = StdRng::from_seed(bs);
  return rng;
}
