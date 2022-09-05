use clap::*;
use gre::*;
use noise::*;
use rand::Rng;
use svg::node::element::{path::Data, *};

#[derive(Parser)]
#[clap()]
struct Opts {
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
  #[clap(short, long, default_value = "100.0")]
  width: f64,
  #[clap(short, long, default_value = "100.0")]
  height: f64,
}

fn art(opts: &Opts) -> Vec<Group> {
  let pad = 8.0;
  let width = opts.width;
  let height = opts.height;
  let colors = vec!["#000"];
  let stroke_width = 0.35;
  let samples = 6000;

  colors
    .iter()
    .map(|&color| {
      let mut data = Data::new();
      let perlin = Perlin::new();
      let mut rng = rng_from_seed(opts.seed);
      let mut passage = Passage2DCounter::new(0.8, width, height);

      let a = rng.gen_range(100.0, 800.0);
      let b = rng.gen_range(100.0, 800.0);
      let c = rng.gen_range(10.0, 100.0);
      let d = rng.gen_range(10.0, 100.0);
      let e = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
      let f = |i| {
        let x = width / 2.0
          + width
            * 0.48
            * perlin.get([
              0.334 + 70.7 * opts.seed / 3.,
              0.3 + i as f64 / a,
              e * perlin.get([-opts.seed, i as f64 * c]),
            ]);
        let y = height / 2.0
          + height
            * 0.48
            * perlin.get([
              i as f64 / b,
              9.1 + 40.3 * opts.seed / 7.,
              e * perlin.get([60.1 + opts.seed, i as f64 * d]),
            ]);
        (x, y)
      };

      let mut points = Vec::new();
      let mut minx = width;
      let mut miny = height;
      let mut maxx = 0.;
      let mut maxy = 0.;
      for i in 0..samples {
        let p = f(i);
        if passage.count(p) > 2 {
          continue;
        }
        points.push(p);
        if p.0 < minx {
          minx = p.0;
        }
        if p.1 < miny {
          miny = p.1;
        }
        if p.0 > maxx {
          maxx = p.0;
        }
        if p.1 > maxy {
          maxy = p.1;
        }
      }

      let w = maxx - minx;
      let h = maxy - miny;
      let dx = (width - w) / 2. - minx;
      let dy = (height - h) / 2. - miny;

      for p in points {
        let x = p.0 + dx;
        let y = p.1 + dy;
        if x < pad || y < pad || x > width - pad || y > height - pad {
          continue;
        }
        data = data.move_to((x, y));
        let angle =
          (x - width / 2.).atan2(y - height / 2.) + rng.gen_range(-0.5, 0.5);
        let amp = rng.gen_range(0.6, 1.2);
        data = data.line_to((x + amp * angle.cos(), y + amp * angle.sin()));
      }
      let mut l = layer(color);
      l = l.add(base_path(color, stroke_width, data));
      l
    })
    .collect()
}

fn main() {
  let opts: Opts = Opts::parse();
  let groups = art(&opts);
  let mut document = base_document("white", opts.width, opts.height);
  for g in groups {
    document = document.add(g);
  }
  svg::save("image.svg", &document).unwrap();
}
