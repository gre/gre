use instant::Instant;
use main::*;
use rand::prelude::*;

fn main() {
  let mut rng = rand::thread_rng();
  let alphabet: Vec<char> = "0123456789ABCDEF".chars().collect();
  let chars: String = (0..64)
    .map(|_i| alphabet[rng.gen_range(0, alphabet.len())])
    .collect();
  let hash = "00E489BB34838F3A580E732CB5A7F7CB5C211B33192508381248CAF4D5C79370"
    .to_string(); //format!("{}", chars);
                  // println!("{}", hash);
  let before = Instant::now();
  let doc = art(&Opts {
    hash,
    primary_name: String::from("P"),
    secondary_name: String::from("S"),
    debug: true,
    gold_border: false,
  });
  println!("{} ms", (before.elapsed().as_secs_f64() * 1000.).round());
  svg::save("image.svg", &doc).unwrap();
}
