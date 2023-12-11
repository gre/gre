use greweb::*;
use instant::Instant;
use rand::prelude::*;
use std::collections::HashMap;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let n = args
    .get(1)
    .map(|o: &String| o.parse::<usize>().ok())
    .flatten()
    .unwrap_or(50);

  let fontdata = std::fs::read(&"./static/PrinceValiant.ttf").unwrap();

  let bs = [0; 32];
  let mut rng = StdRng::from_seed(bs);
  let mut duration_per_hash = vec![];

  let jsons = (0..n)
    .map(|_| {
      let alphabet: Vec<char> =
        "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
          .chars()
          .collect();
      let chars: String = (0..49)
        .map(|_i| alphabet[rng.gen_range(0..alphabet.len())])
        .collect();
      let hash = format!("oo{}", chars);
      let before = Instant::now();
      let code = render(
        hash.clone(),
        210.0,
        297.0,
        5.0,
        0.2,
        fontdata.clone(),
        false,
        true,
      );
      let elapsed = before.elapsed().as_millis();
      duration_per_hash.push((hash.clone(), elapsed));
      let start = code.find("data-perf='").unwrap() + 11;
      let end = code[start..].find("'").unwrap() + start;
      let perf = &code[start..end];
      let json: Vec<(String, i64)> =
        serde_json::from_str::<serde_json::Value>(perf)
          .unwrap()
          .as_object()
          .unwrap()
          .get("per_label")
          .unwrap()
          .as_array()
          .unwrap()
          .iter()
          .map(|entry| {
            let label =
              entry.get("label").unwrap().as_str().unwrap().to_string();
            let duration_ms =
              entry.get("duration_ms").unwrap().as_i64().unwrap();
            (label, duration_ms)
          })
          .collect();
      json
    })
    .collect::<Vec<_>>();

  let mut values = HashMap::new();
  for json in jsons {
    for (k, v) in json {
      if !values.contains_key(&k) {
        values.insert(k.clone(), vec![]);
      }
      let acc = values.get_mut(&k).unwrap();
      acc.push(v.clone());
    }
  }

  duration_per_hash.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

  for (hash, duration) in duration_per_hash.iter().take(5) {
    println!("{} = {} ms", hash, duration);
  }
  if duration_per_hash.len() > 5 {
    println!(
      "median is {} ms",
      duration_per_hash[duration_per_hash.len() / 2].1
    );
  }

  let mut avgs: Vec<_> = values
    .iter()
    .map(|(k, v)| {
      let avg = v.iter().sum::<i64>() as f32 / v.len() as f32;
      (k.clone(), avg)
    })
    .collect();

  avgs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

  // analyze the distribution of values
  for (k, avg) in &avgs {
    println!("{} = {} ms", k, (avg * 10.).round() / 10.);
  }
  println!(
    "sum = {} ms",
    avgs.iter().map(|(_, v)| v).sum::<f32>().round()
  );
}
