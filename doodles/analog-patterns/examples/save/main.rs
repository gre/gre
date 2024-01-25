use image::io::Reader as ImageReader;
use main::*;
use rand::prelude::*;

fn main() {
  let mut rng = rand::thread_rng();
  let alphabet: Vec<char> = "0123456789ABCDEF".chars().collect();
  let chars: String = (0..64)
    .map(|_i| alphabet[rng.gen_range(0, alphabet.len())])
    .collect();
  let hash = format!("{}", chars).to_string();
  println!("hash: {}", hash);
  let images = vec![];
  /*
    let img = ImageReader::open("../eiffel_silhouette_clipart.png").unwrap().decode().unwrap();
  let rgba = img.to_rgba8();
  let data = rgba.to_vec();
  let (w, h) = rgba.dimensions();
  let width = w as usize;
  let height = h as usize;
  let image = ImageData { data, width, height };
   */
  let (doc, _) = art(
    &Opts {
      hash,
      width: 210.,
      height: 210.,
      pad: 10.0,
      images,
    },
    false,
  );
  svg::save("image.svg", &doc).unwrap();
}
