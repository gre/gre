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
  // let hash = "oooY8czwbHMtv8MEyHKNJbGFuF9AbDvcWcGeEBsTBPHa8MnsJ6L".to_string();
  let fontdata = std::fs::read(&"./static/PrinceValiant.ttf").unwrap();
  let code = render(hash, 210.0, 297.0, 5.0, 0.2, fontdata, false, true);
  std::fs::write("image.svg", code).expect("Unable to write file");
}
