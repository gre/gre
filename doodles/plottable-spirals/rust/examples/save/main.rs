use main::*;
use rand::prelude::*;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  // read count from cli params
  let width = match args.get(1).and_then(|a| a.parse().ok()) {
    Some(n) => n,
    None => 210.,
  };
  let pad = match args.get(2).and_then(|a| a.parse().ok()) {
    Some(n) => n,
    None => 10.,
  };

  let mut rng = rand::thread_rng();
  let alphabet: Vec<char> =
    "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
      .chars()
      .collect();
  let chars: String = (0..49)
    .map(|_i| alphabet[rng.gen_range(0, alphabet.len())])
    .collect();
  let hash = format!("oo{}", chars);
  let (doc, _) = art(
    &Opts {
      hash,
      width: width,
      height: width,
      pad: pad,
    },
    false,
  );
  svg::save("image.svg", &doc).unwrap();
}
