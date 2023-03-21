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
  let hash = //"oopdnZVfpiciFNXXLPXqULvXXah95W5Pnvkgt3E9rVbBDfQeHfJ".to_string(); 
  format!("oo{}", chars);
  let before = Instant::now();
  let lightness = 0.0;
  let color_cutoff = 3;
  let layers_count = 3;
  let noise_effect = rng.gen_range(-1.0f64, 1.0);
  let (doc, _) = art(
    &Opts {
      hash,
      width: 297.,
      height: 210.,
      pad: 10.,
      lightness,
      color_cutoff,
      color_offset: 0,
      layers_count,
      noise_effect,
      kaleidoscope: false,
      kaleidoscope_mod: 0,
    },
    false,
  );
  println!("{} ms", (before.elapsed().as_secs_f64() * 1000.).round());
  svg::save("image.svg", &doc).unwrap();
}
