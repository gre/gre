use instant::Instant;
use main::*;
use rand::prelude::*;

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
  let before = Instant::now();
  let (doc, _) = art(
    &Opts {
      hash,
      width: 210.,
      height: 297.,
      pad: 10.,
    },
    false,
  );
  println!("{} ms", (before.elapsed().as_secs_f64() * 1000.).round());
  svg::save("image.svg", &doc).unwrap();
}
