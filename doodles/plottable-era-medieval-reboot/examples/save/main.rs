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
  let fontdata = std::fs::read(&"./static/PrinceValiant.ttf").unwrap();
  let hash = format!("oo{}", chars);
  let code = render(hash, 210.0, 297.0, 5.0, 0.2, fontdata, false, true);
  std::fs::write("image.svg", code).expect("Unable to write file");
}
