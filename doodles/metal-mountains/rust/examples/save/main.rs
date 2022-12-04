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
  // println!("{}", hash);
  let before = Instant::now();
  let doc = art(&Opts {
    hash, //: String::from("oouytCqzfweCAucFs5Hgmdt5CChA8e94fftznC4crPPXohD5tLr"),
    width: 100.0,
    height: 150.0,
    pad: 5.0,
    layer1_name: String::from("P"),
    debug: true,
  });
  println!("{} ms", (before.elapsed().as_secs_f64() * 1000.).round());
  svg::save("image.svg", &doc).unwrap();
}
