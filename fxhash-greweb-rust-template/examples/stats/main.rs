use greweb::*;
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let n = args
    .get(1)
    .map(|o: &String| o.parse::<usize>().ok())
    .flatten()
    .unwrap_or(1);

  let mut values = HashMap::new();

  for _i in 0..n {
    let mut rng = rand::thread_rng();
    let alphabet: Vec<char> =
      "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
        .chars()
        .collect();
    let chars: String = (0..49)
      .map(|_i| alphabet[rng.gen_range(0..alphabet.len())])
      .collect();
    let hash = format!("oo{}", chars);
    println!("{}", hash);
    let code = render(hash, 210.0, 297.0, 10.0, false);
    let start = code.find("data-traits='").unwrap() + 13;
    let end = code[start..].find("'").unwrap() + start;
    let traits = &code[start..end];
    let json = serde_json::from_str::<serde_json::Value>(traits)
      .unwrap()
      .as_object()
      .unwrap()
      .clone();
    for (k, v) in json {
      if !values.contains_key(&k) {
        values.insert(k.clone(), vec![]);
      }
      let acc = values.get_mut(&k).unwrap();
      acc.push(v.clone());
    }
  }

  // analyze the distribution of values
  for (k, v) in values {
    let mut map = HashMap::new();
    for i in v {
      let key = i.to_string();
      if !map.contains_key(&key) {
        map.insert(key.to_string(), 0);
      }
      let acc = map.get_mut(&key).unwrap();
      *acc += 1;
    }
    let mut vec: Vec<(&String, &i64)> = map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("\n{}", k);
    for (k, v) in vec {
      let percentage = (*v as f64 / n as f64 * 1000.0).round() / 10.0;
      println!("\t{}%: {}", percentage, k);
    }
  }
}
