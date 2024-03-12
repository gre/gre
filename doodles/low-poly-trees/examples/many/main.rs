use greweb::*;
use rand::prelude::*;
use rayon::prelude::*;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let n = args
    .get(1)
    .map(|o: &String| o.parse::<usize>().ok())
    .flatten()
    .unwrap_or(1);
  let alphabet: Vec<char> =
    "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
      .chars()
      .collect();

  std::fs::create_dir_all("results").unwrap();

  (0..n).into_par_iter().for_each(|_| {
    let mut rng = rand::thread_rng();
    let chars: String = (0..49)
      .map(|_i| alphabet[rng.gen_range(0..alphabet.len())])
      .collect();
    let hash = format!("oo{}", chars);
    println!("{}", hash);
    let code = render(hash.clone(), 210.0, 297.0, 5.0, 0.2, false, false);
    let filename = format!("results/{}.svg", hash);
    std::fs::write(filename.clone(), code).expect("Unable to write file");
    println!("Wrote {}", filename);
  });
}
