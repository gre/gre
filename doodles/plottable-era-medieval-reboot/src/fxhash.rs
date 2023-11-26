use rand::prelude::*;

pub fn rng_from_hash(hash: &String) -> StdRng {
  let v = bs58::decode(hash.chars().skip(2).take(43).collect::<String>())
    .into_vec()
    .unwrap();
  let mut bs = [0; 32];
  bs.copy_from_slice(&v);
  let rng = StdRng::from_seed(bs);
  return rng;
}
